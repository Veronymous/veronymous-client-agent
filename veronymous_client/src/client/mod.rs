pub mod state;

use crate::client::state::{ClientState, IssuerInfo, IssuerInfos, RootTokens, VpnConnection};
use crate::config::VERONYMOUS_CLIENT_CONFIG;
use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::{AuthRequired, ConnectError, MissingIssuerInfoError, MissingTokenError, ParseError, SubscriptionRequired, TokenError};
use crate::oidc::client::OidcClient;
use crate::oidc::credentials::{OidcCredentials, OidcCredentialsStatus, UserCredentials};
use crate::servers::VpnServers;
use crate::veronymous_token::client::VeronymousTokenClient;
use crate::vpn::VpnProfile;
use crate::wg::generate_keypair;
use rand::thread_rng;
use std::time::{SystemTime, UNIX_EPOCH};
use veronymous_router_client::VeronymousRouterClient;
use veronymous_token::token::{get_current_epoch, VeronymousToken};

pub struct VeronymousClient {
    oidc_client: OidcClient,

    token_client: VeronymousTokenClient,
}

impl VeronymousClient {
    pub fn new(oidc_client: OidcClient, token_client: VeronymousTokenClient) -> VeronymousClient {
        Self {
            oidc_client,
            token_client,
        }
    }

    pub async fn authenticate(
        &self,
        credentials: &UserCredentials,
        client_state: &mut ClientState,
    ) -> Result<(), VeronymousClientError> {
        let oidc_credentials = self.oidc_client.fetch_tokens(credentials).await?;

        client_state.oidc_credentials = Some(oidc_credentials);

        Ok(())
    }

    pub async fn connect(
        &mut self,
        domain: &String,
        client_state: &mut ClientState,
        servers: &VpnServers,
    ) -> Result<VpnConnection, VeronymousClientError> {
        // Get the current epoch
        let now = Self::now();
        let current_epoch = Self::get_current_epoch(Some(now));
        let next_epoch = Self::get_next_epoch(current_epoch);
        // let next_epoch = Self::get_next_epoch(now);
        let current_key_epoch = Self::get_current_key_epoch(Some(now));
        let active_key_epoch = Self::get_active_key_epoch(Some(now), Some(current_key_epoch));

        debug!("Current epoch: {}", current_epoch);
        debug!("Next key epoch: {}", next_epoch);
        debug!("Current key epoch: {}", current_key_epoch);
        debug!("Active key epoch: {}", active_key_epoch);

        // Ensure the oidc tokens
        let access_token;
        match &mut client_state.oidc_credentials {
            None => {
                return Err(AuthRequired());
            }
            Some(credentials) => {
                self.ensure_oidc_credentials(now, next_epoch, credentials)
                    .await?;
                access_token = &credentials.access_token;
            }
        };

        // Make sure that the domain is not used
        if client_state
            .connections
            .has_connection(&current_epoch, domain)
        {
            return Ok(client_state
                .connections
                .get_connection(&current_epoch, domain)
                .unwrap()
                .clone());
        }

        // TODO: Get the vpn profile
        let vpn_profile = servers.find_server(domain)?;

        // Ensure that the client state contains the issuer's token info
        self.ensure_issuer_info(
            &mut client_state.issuer_infos,
            access_token,
            current_key_epoch,
            active_key_epoch,
        )
            .await?;

        // Ensure root token
        self.ensure_root_token(
            &mut client_state.root_tokens,
            &mut client_state.issuer_infos,
            access_token,
            current_key_epoch,
            active_key_epoch,
        )
            .await?;

        // Derive the authentication token
        let auth_token = Self::derive_auth_token(
            active_key_epoch,
            current_epoch,
            &vpn_profile.domain,
            &mut client_state.root_tokens,
            &mut client_state.issuer_infos,
        )?;

        // Generate a wireguard keypair
        let (private_key, public_key) = generate_keypair()?;

        // Send a connection request to the router agent
        let vpn_connection = self
            .create_connection(private_key, public_key, vpn_profile, auth_token)
            .await?;

        // Save the connection state
        client_state.connections.add_connection(
            vpn_connection.clone(),
            current_epoch,
            vpn_profile.domain.clone(),
        );

        Ok(vpn_connection)
    }

    async fn create_connection(
        &mut self,
        private_key: String,
        public_key: String,
        vpn_profile: &VpnProfile,
        auth_token: VeronymousToken,
    ) -> Result<VpnConnection, VeronymousClientError> {
        let root_cert = match &vpn_profile.root_cert {
            None => None,
            Some(cert) => Some(cert.as_bytes()),
        };

        // Create the client
        let mut router_client = VeronymousRouterClient::new(&vpn_profile.agent_endpoint, root_cert)
            .await
            .map_err(|e| ConnectError(format!("Could not create router agent client. {:?}", e)))?;

        // Send a connection request
        let public_key_decoded = base64::decode(&public_key)
            .map_err(|e| ParseError(format!("Could not decode public key. {:?}", e)))?
            .try_into()
            .map_err(|e| ParseError(format!("Could not decode public key. {:?}", e)))?;

        let connection = router_client
            .connect(public_key_decoded, auth_token)
            .await
            .map_err(|e| ConnectError(format!("Could not create connection. {:?}", e)))?;

        let vpn_connection = VpnConnection::new(
            vec![
                connection.ipv4_address.to_string(),
                connection.ipv6_address.to_string(),
            ],
            vpn_profile.wg_key.clone(),
            vpn_profile.wg_endpoint.clone(),
            private_key,
            public_key,
            vpn_profile.domain.clone(),
        );

        Ok(vpn_connection)
    }

    /*
     * Ensure that the client state contains the required token info
     */
    async fn ensure_issuer_info(
        &mut self,
        token_infos: &mut IssuerInfos,
        access_token: &String,
        current_key_epoch: u64,
        active_key_epoch: u64,
    ) -> Result<(), VeronymousClientError> {
        if !token_infos.issuer_infos.contains_key(&active_key_epoch) {
            let issuer_info = self
                .token_client
                .get_token_info(active_key_epoch, current_key_epoch, access_token)
                .await?;
            let issuer_info = IssuerInfo::new(issuer_info.0, issuer_info.1);

            token_infos
                .issuer_infos
                .insert(active_key_epoch, issuer_info);
        }

        Ok(())
    }

    /*
     * Check root token. Fetch if needed.
     */
    async fn ensure_root_token(
        &mut self,
        root_tokens: &mut RootTokens,
        issuer_infos: &mut IssuerInfos,
        access_token: &String,
        current_key_epoch: u64,
        active_key_epoch: u64,
    ) -> Result<(), VeronymousClientError> {
        // Check if a root token exists
        if !root_tokens.tokens.contains_key(&active_key_epoch) {
            // Does not contain a root token for the epoch

            let issuer_info = match issuer_infos.issuer_infos.get(&active_key_epoch) {
                None => return Err(MissingIssuerInfoError()),
                Some(issuer_info) => issuer_info,
            };

            // Fetch the root token
            let root_token = self
                .token_client
                .fetch_token(
                    &issuer_info.params,
                    &issuer_info.public_key,
                    access_token,
                    current_key_epoch,
                    active_key_epoch,
                )
                .await?;

            root_tokens.tokens.insert(active_key_epoch, root_token);
        }

        Ok(())
    }

    /*
     * Check the oidc credentials. Refresh if needed.
     */
    async fn ensure_oidc_credentials(
        &self,
        now: u64,
        next_epoch: u64,
        credentials: &mut OidcCredentials,
    ) -> Result<(), VeronymousClientError> {
        match credentials.status(now, next_epoch)? {
            OidcCredentialsStatus::OK => Ok(()),
            OidcCredentialsStatus::RefreshRequired => {
                // Refresh the tokens
                self.oidc_client.refresh_tokens(credentials).await?;

                Ok(())
            }
            OidcCredentialsStatus::AuthRequired => {
                return Err(AuthRequired());
            }
            OidcCredentialsStatus::SubscriptionRequired => {
                return Err(SubscriptionRequired());
            }
        }
    }

    fn derive_auth_token(
        active_key_epoch: u64,
        epoch: u64,
        domain: &String,
        root_tokens: &RootTokens,
        issuer_infos: &IssuerInfos,
    ) -> Result<VeronymousToken, VeronymousClientError> {
        let root_token = match root_tokens.tokens.get(&active_key_epoch) {
            None => {
                return Err(MissingTokenError(format!(
                    "Missing root token for key epoch: {}",
                    active_key_epoch
                )));
            }
            Some(root_token) => root_token,
        };

        let token_info = match issuer_infos.issuer_infos.get(&active_key_epoch) {
            None => return Err(MissingIssuerInfoError()),
            Some(token_info) => token_info,
        };

        let auth_token = root_token
            .derive_token(
                domain.as_bytes(),
                epoch,
                &token_info.public_key,
                &token_info.params,
                &mut thread_rng(),
            )
            .map_err(|e| TokenError(format!("Could not derive auth token. {:?}", e)))?;

        Ok(auth_token)
    }

    pub fn get_current_epoch(now: Option<u64>) -> u64 {
        let now = match now {
            None => Self::now(),
            Some(now) => now,
        };

        get_current_epoch(
            now,
            VERONYMOUS_CLIENT_CONFIG.epoch_length,
            VERONYMOUS_CLIENT_CONFIG.epoch_buffer,
        )
    }

    pub fn get_current_key_epoch(now: Option<u64>) -> u64 {
        let now = match now {
            None => Self::now(),
            Some(now) => now,
        };

        let current_epoch = now - (now % VERONYMOUS_CLIENT_CONFIG.key_lifetime);

        current_epoch
    }

    pub fn get_active_key_epoch(now: Option<u64>, current_key_epoch: Option<u64>) -> u64 {
        let now = match now {
            None => Self::now(),
            Some(now) => now,
        };

        let current_key_epoch = match current_key_epoch {
            None => Self::get_current_key_epoch(Some(now)),
            Some(current_key_epoch) => current_key_epoch,
        };

        // Return next if in buffer
        return if VERONYMOUS_CLIENT_CONFIG.epoch_buffer
            > VERONYMOUS_CLIENT_CONFIG.key_lifetime - (now % VERONYMOUS_CLIENT_CONFIG.key_lifetime)
        {
            debug!("In buffer, returning next key epoch.");
            current_key_epoch + VERONYMOUS_CLIENT_CONFIG.key_lifetime
        } else {
            current_key_epoch
        };
    }

    fn get_next_epoch(current_epoch: u64) -> u64 {
        return current_epoch + VERONYMOUS_CLIENT_CONFIG.epoch_length;
        // return now + VERONYMOUS_CLIENT_CONFIG.epoch_buffer;
    }

    fn now() -> u64 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}
