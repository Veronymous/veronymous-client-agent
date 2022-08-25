pub mod state;

use crate::client::state::{ClientState, IssuerInfo, IssuerInfos, RootTokens, VpnConnection};
use crate::constants::{EPOCH_BUFFER, EPOCH_LENGTH};
use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::{
    AuthRequired, ConnectError, DomainInUseError, MissingIssuerInfoError, MissingTokenError, TokenError,
};
use crate::oidc::client::OidcClient;
use crate::oidc::credentials::{OidcCredentials, OidcCredentialsStatus, UserCredentials};
use crate::veronymous_token::client::VeronymousTokenClient;
use crate::vpn::VpnProfile;
use rand::thread_rng;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use veronymous_connection::model::PublicKey;
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

    pub async fn connect(
        &mut self,
        vpn_profile: &VpnProfile,
        client_state: &mut ClientState,
        public_key: PublicKey,
    ) -> Result<VpnConnection, VeronymousClientError> {
        // Get the current epoch
        let current_epoch = Self::get_current_epoch();

        // Make sure that the domain is not used
        if client_state
            .connections
            .has_connection(&current_epoch, &vpn_profile.domain)
        {
            return Err(DomainInUseError());
        }

        // Ensure the oidc tokens
        let access_token;
        match &mut client_state.oidc_credentials {
            None => {
                return Err(AuthRequired());
            }
            Some(credentials) => {
                self.ensure_oidc_credentials(current_epoch, credentials)
                    .await?;
                access_token = &credentials.access_token;
            }
        };

        // Ensure that the client state contains the issuer's token info
        self.ensure_issuer_info(&mut client_state.issuer_infos, access_token, current_epoch)
            .await?;

        // Ensure root token
        self.ensure_root_token(
            &mut client_state.root_tokens,
            &mut client_state.issuer_infos,
            access_token,
            current_epoch,
        )
            .await?;

        // Derive the authentication token
        let auth_token = Self::derive_auth_token(
            current_epoch,
            &vpn_profile.domain,
            &mut client_state.root_tokens,
            &mut client_state.issuer_infos,
        )?;

        // Send a connection request to the router agent
        let vpn_connection = self
            .create_connection(public_key, vpn_profile, auth_token)
            .await?;

        // Save the connection state
        client_state.connections.add_connection(
            vpn_connection.clone(),
            current_epoch,
            vpn_profile.domain.clone(),
        );

        Ok(vpn_connection)
    }

    pub async fn create_connection(
        &mut self,
        public_key: PublicKey,
        vpn_profile: &VpnProfile,
        auth_token: VeronymousToken,
    ) -> Result<VpnConnection, VeronymousClientError> {
        // Create the client
        let router_client = VeronymousRouterClient::new(
            vpn_profile.agent_endpoint.address,
            vpn_profile.agent_endpoint.domain.as_str(),
            &vec![vpn_profile.root_cert.clone()],
        )
            .map_err(|e| ConnectError(format!("Could not create router agent client. {:?}", e)))?;

        // Send a connection request
        let connection_response = router_client
            .connect(public_key, auth_token)
            .await
            .map_err(|e| ConnectError(format!("Could not create connection. {:?}", e)))?;

        if !connection_response.accepted {
            return Err(ConnectError(format!("Connection request denied.")));
        }

        let vpn_connection = VpnConnection::new(
            vec![
                Ipv4Addr::from(connection_response.ipv4_address).to_string(),
                Ipv6Addr::from(connection_response.ipv6_address).to_string(),
            ],
            vpn_profile.wg_key.clone(),
            vpn_profile.wg_endpoint.clone(),
        );

        Ok(vpn_connection)
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

    /*
     * Ensure that the client state contains the required token info
     */
    async fn ensure_issuer_info(
        &mut self,
        token_infos: &mut IssuerInfos,
        access_token: &String,
        epoch: u64,
    ) -> Result<(), VeronymousClientError> {
        if !token_infos.issuer_infos.contains_key(&epoch) {
            let issuer_info = self
                .token_client
                .get_token_info(epoch, access_token)
                .await?;
            let issuer_info = IssuerInfo::new(issuer_info.0, issuer_info.1);

            token_infos.issuer_infos.insert(epoch, issuer_info);
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
        current_epoch: u64,
    ) -> Result<(), VeronymousClientError> {
        // Check if a root token exists
        if !root_tokens.tokens.contains_key(&current_epoch) {
            // Does not contain a root token for the epoch

            let issuer_info = match issuer_infos.issuer_infos.get(&current_epoch) {
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
                    current_epoch,
                )
                .await?;

            root_tokens.tokens.insert(current_epoch, root_token);
        }

        Ok(())
    }

    /*
     * Check the oidc credentials. Refresh if needed.
     */
    async fn ensure_oidc_credentials(
        &self,
        now: u64,
        credentials: &mut OidcCredentials,
    ) -> Result<(), VeronymousClientError> {
        match credentials.status(now)? {
            OidcCredentialsStatus::OK => Ok(()),
            OidcCredentialsStatus::RefreshRequired => {
                // Refresh the tokens
                self.oidc_client.refresh_tokens(credentials).await?;

                Ok(())
            }
            OidcCredentialsStatus::AuthRequired => {
                return Err(AuthRequired());
            }
        }
    }

    fn derive_auth_token(
        epoch: u64,
        domain: &String,
        root_tokens: &RootTokens,
        issuer_infos: &IssuerInfos,
    ) -> Result<VeronymousToken, VeronymousClientError> {
        let root_token = match root_tokens.tokens.get(&epoch) {
            None => return Err(MissingTokenError("Missing root token".to_string())),
            Some(root_token) => root_token,
        };

        let token_info = match issuer_infos.issuer_infos.get(&epoch) {
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

    fn get_current_epoch() -> u64 {
        get_current_epoch(Self::now(), EPOCH_LENGTH, EPOCH_BUFFER)
    }

    fn now() -> u64 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

#[cfg(test)]
mod tests {
    use crate::client::state::ClientState;
    use crate::client::VeronymousClient;
    use crate::error::VeronymousClientError::AuthRequired;
    use crate::network::endpoint::Endpoint;
    use crate::oidc::client::OidcClient;
    use crate::oidc::credentials::UserCredentials;
    use crate::veronymous_token::client::VeronymousTokenClient;
    use crate::vpn::VpnProfile;
    use veronymous_connection::model::PublicKey;

    const ROUTER_AGENT_ROOT: &str = "-----BEGIN CERTIFICATE-----
MIID0TCCArmgAwIBAgIUCVuNppf++HHklyxMgrGWTPNTKMgwDQYJKoZIhvcNAQEL
BQAweDELMAkGA1UEBhMCQ0ExEDAOBgNVBAgMB09udGFyaW8xDzANBgNVBAcMBk90
dGF3YTEkMCIGA1UECgwbVmVyb255bW91cyBUZWNobm9sb2dpZXMgSW5jMSAwHgYD
VQQDDBdWZXJvbnltb3VzIFRlY2hub2xvZ2llczAeFw0yMjA4MTExMzIwNTJaFw0y
NzA4MTAxMzIwNTJaMHgxCzAJBgNVBAYTAkNBMRAwDgYDVQQIDAdPbnRhcmlvMQ8w
DQYDVQQHDAZPdHRhd2ExJDAiBgNVBAoMG1Zlcm9ueW1vdXMgVGVjaG5vbG9naWVz
IEluYzEgMB4GA1UEAwwXVmVyb255bW91cyBUZWNobm9sb2dpZXMwggEiMA0GCSqG
SIb3DQEBAQUAA4IBDwAwggEKAoIBAQCw1TQI538AkY5IEC2FTzhb4/ZtErWAAKAZ
U744847DN0xHzeXMSBFm4RmVi5j8bOAMhnV11tqh5XfbhHGmEBO9i85XJoXUuGes
MUIIMna3eHS889SeIp0xo0TBtVHUwuvAofE4vts6/ZI6Ip5sHSEXl41n93VkwUPO
1E+0TBZVy+3pvguzbQa/tjgXDyYwv03Sy1JQQXUEHOVghpRdc+tL+GzzhXLoVmAr
07vBj5cnICCN7g/WkJbhoi7WxGUxkjNX3ibkmQjxTTSsNnbQ3fvAY8lKmkt6uPzz
Yt/Xyx/Is0f58FxFoiGYyTvqi4ShXb614VhUg43kCMwmKPd86YWTAgMBAAGjUzBR
MB0GA1UdDgQWBBTMGM+KXB5CpEeAZwSakqvvb9P8pDAfBgNVHSMEGDAWgBTMGM+K
XB5CpEeAZwSakqvvb9P8pDAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUA
A4IBAQBmJWu/5nCGZbxkVBTxhdhyXiXn5xJbssYOdgfCnIl8fnMgDJjSjBlmGrc4
JpJ6EKjsvGdeVdOQ86Up8SQR7gHrW2MeWcTejNDwsmBGKdQDh/U1mozwoFnMX7oH
gz6Gqz8T5XSGTfbxNwQPBDp3fDNaISgEM7DeZhxBR10oSkwa0c6WA1HkyBUTbNn9
2xdjDAr1Uj70P05qwyWlSHdBXYBaWOEYe0jkCKxR5xwF4+lYtmyYmpQqVxCojk8p
WWkQ8PIncGsunqvO9NW+ShTkrYR3NNa4yDTVcqy9Us1bycnCfpTnYIRt4BWH1IIj
wIieRFPJFKt7IAQE8g3/2VF12EeS
-----END CERTIFICATE-----";

    #[tokio::test]
    async fn connect() {
        // OIDC client
        let oidc_token_endpoint: &str =
            "http://172.20.0.3:8080/realms/veronymous-vpn/protocol/openid-connect/token";
        let oidc_client_id: &str = "auth-client";

        let oidc_client =
            OidcClient::new(oidc_token_endpoint.to_string(), oidc_client_id.to_string());

        let token_endpoint = "http://127.0.0.1:9123";
        let token_client = VeronymousTokenClient::create(token_endpoint.to_string())
            .await
            .unwrap();

        // Client state
        let mut client_state = ClientState::empty();

        // VPN Values
        let agent_endpoint = Endpoint::new(
            "127.0.0.1:7777".parse().unwrap(),
            "localhost.veronymous.io".to_string(),
        );

        let vpn_profile = VpnProfile::new(
            "dev_domain".to_string(),
            agent_endpoint,
            ROUTER_AGENT_ROOT.as_bytes().into(),
            "wg1.ny.veronymous.io:51820".to_string(),
            "/ZjSUjxcDiHHxBifHX0yVekKklDmczNv8k7M3AgmXXg=".to_string(),
        );

        let client_wg_key: PublicKey =
            base64::decode("lDvZ18A8W94xceJcz08SOSoXFwhAlWlAVVZ5Dw3UA0E=")
                .unwrap()
                .try_into()
                .unwrap();

        // Veronymous client
        let mut client = VeronymousClient::new(oidc_client, token_client);

        // Connect without credentials
        let connection_result = client
            .connect(&vpn_profile, &mut client_state, client_wg_key.clone())
            .await;

        assert!(connection_result.is_err());
        assert_eq!(connection_result.err(), Some(AuthRequired()));

        // Authenticate
        let user_credentials = UserCredentials::new("user1".to_string(), "password".to_string());
        client
            .authenticate(&user_credentials, &mut client_state)
            .await
            .unwrap();

        // OIDC credentials must be set
        assert!(client_state.oidc_credentials.is_some());

        // Connect
        let connection = client
            .connect(&vpn_profile, &mut client_state, client_wg_key.clone())
            .await
            .unwrap();

        println!("{:?}", connection);
    }
}
