use crate::constants::app::{CLIENT_FILE_PATH, VPN_SERVERS_FILE_PATH};
use crate::error::CliClientError;
use crate::error::CliClientError::{
    EncodingError, InitializationError, IoError, ParseError, ReadFileError, VeronymousClientError,
};
use crate::utils::path_utils::get_home_path;
use crate::wg::{wg_refresh, wg_up};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, thread};
use veronymous_client::client::state::{ClientState, VpnConnection};
use veronymous_client::client::VeronymousClient;
use veronymous_client::config::VERONYMOUS_CLIENT_CONFIG;
use veronymous_client::oidc::client::OidcClient;
use veronymous_client::oidc::credentials::UserCredentials;
use veronymous_client::servers::VpnServers;
use veronymous_client::veronymous_token::client::VeronymousTokenClient;
use veronymous_token::token::get_next_epoch;

pub struct CliVpnClient {
    veronymous_client: VeronymousClient,
}

impl CliVpnClient {
    pub async fn create() -> Result<Self, CliClientError> {
        let oidc_client = OidcClient::new(
            VERONYMOUS_CLIENT_CONFIG.oidc_endpoint.clone(),
            VERONYMOUS_CLIENT_CONFIG.oidc_client_id.clone(),
        );
        let token_client =
            VeronymousTokenClient::create(VERONYMOUS_CLIENT_CONFIG.token_endpoint.clone())
                .await
                .map_err(|e| InitializationError(e.to_string()))?;

        let veronymous_client = VeronymousClient::new(oidc_client, token_client);
        Ok(Self { veronymous_client })
    }

    pub async fn authenticate(
        &self,
        username: String,
        password: String,
    ) -> Result<(), CliClientError> {
        let credentials = UserCredentials::new(username, password);

        // read the client state
        let mut client_state = Self::read_client_state(None)?;

        self.veronymous_client
            .authenticate(&credentials, &mut client_state)
            .await
            .map_err(|e| CliClientError::VeronymousClientError(e))?;

        Self::save_client_state(&mut client_state, None)?;

        Ok(())
    }

    pub async fn get_servers(&self) -> Result<Vec<String>, CliClientError> {
        let mut vpn_servers = Self::read_vpn_servers(None)?;

        Self::update_servers(&mut vpn_servers).await?;

        Ok(vpn_servers.list_domains())
    }

    pub async fn connect(
        &mut self,
        server: String,
        tunnel_only: bool,
    ) -> Result<(), CliClientError> {
        info!("Connecting...");

        let connection = self.create_connection(&server).await?;

        wg_up(&connection, tunnel_only)?;
        info!("Connected.");

        loop {
            let delay = Self::get_refresh_start();

            info!("Updating connection in {}s", delay.as_secs());

            thread::sleep(delay);

            info!("Update connection...");

            let connection = self.create_connection(&server).await?;

            wg_refresh(&connection, tunnel_only)?;

            info!("Connected.");
        }
    }

    /*
     * Connect to a Veronymous VPN Server.
     * TODO: Optional client file path
     * TODO: Optional auth file (user name and password)
     */
    async fn create_connection(
        &mut self,
        domain: &String,
    ) -> Result<VpnConnection, CliClientError> {
        // Read and update the vpn servers
        let mut vpn_servers = Self::read_vpn_servers(None)?;
        Self::update_servers(&mut vpn_servers).await?;

        // read the client state
        let mut client_state = Self::read_client_state(None)?;

        // Establish connection with the vpn router
        let connection = match self
            .veronymous_client
            .connect(domain, &mut client_state, &vpn_servers)
            .await
        {
            Ok(connection) => {
                Self::save_client_state(&mut client_state, None)?;
                connection
            }
            Err(e) => {
                Self::save_client_state(&mut client_state, None)?;

                return Err(VeronymousClientError(e));
            }
        };

        Ok(connection)
    }

    async fn update_servers(vpn_servers: &mut VpnServers) -> Result<(), CliClientError> {
        let updated = vpn_servers
            .update()
            .await
            .map_err(|e| VeronymousClientError(e))?;

        if updated {
            Self::save_vpn_servers(vpn_servers, None)?;
        }

        Ok(())
    }

    fn read_client_state(path: Option<String>) -> Result<ClientState, CliClientError> {
        let path = match path {
            None => get_home_path(CLIENT_FILE_PATH),
            Some(path) => path,
        };

        let client_state;
        if !Path::new(&path).exists() {
            // Does not exists, create
            client_state = ClientState::empty();
        } else {
            let contents = fs::read(path)
                .map_err(|e| ReadFileError(format!("Could not read client file. {:?}", e)))?;

            client_state =
                serde_json::from_slice(&contents).map_err(|e| ParseError(format!("{:?}", e)))?;
        }

        Ok(client_state)
    }

    fn save_client_state(
        client_state: &mut ClientState,
        path: Option<String>,
    ) -> Result<(), CliClientError> {
        // Clear old connections
        client_state.clear_old(
            VeronymousClient::get_current_key_epoch(None),
            VeronymousClient::get_current_epoch(None),
        );

        let path = match path {
            None => get_home_path(CLIENT_FILE_PATH),
            Some(path) => path,
        };

        let parent = Path::new(&path).parent().unwrap();

        // Create parent directory if it does not exist
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| IoError(e.to_string()))?;
        }

        let contents =
            serde_json::to_vec(client_state).map_err(|e| EncodingError(e.to_string()))?;

        fs::write(path, contents).map_err(|e| IoError(e.to_string()))?;

        Ok(())
    }

    fn read_vpn_servers(path: Option<String>) -> Result<VpnServers, CliClientError> {
        let path = match path {
            None => get_home_path(VPN_SERVERS_FILE_PATH),
            Some(path) => path,
        };

        let vpn_servers;
        if !Path::new(&path).exists() {
            // Does not exists, create
            vpn_servers = VpnServers::new();
        } else {
            let contents = fs::read(path)
                .map_err(|e| ReadFileError(format!("Could not read vpn servers file. {:?}", e)))?;

            vpn_servers =
                serde_json::from_slice(&contents).map_err(|e| ParseError(format!("{:?}", e)))?;
        }

        Ok(vpn_servers)
    }

    fn save_vpn_servers(
        client_state: &mut VpnServers,
        path: Option<String>,
    ) -> Result<(), CliClientError> {
        let path = match path {
            None => get_home_path(VPN_SERVERS_FILE_PATH),
            Some(path) => path,
        };

        let parent = Path::new(&path).parent().unwrap();

        // Create parent directory if it does not exist
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| IoError(e.to_string()))?;
        }

        let contents =
            serde_json::to_vec(client_state).map_err(|e| EncodingError(e.to_string()))?;

        fs::write(path, contents).map_err(|e| IoError(e.to_string()))?;

        Ok(())
    }

    fn get_refresh_start() -> Duration {
        let now = Self::now();

        let mut next_epoch = get_next_epoch(now, VERONYMOUS_CLIENT_CONFIG.epoch_length);

        // Check if currently in buffer
        if VERONYMOUS_CLIENT_CONFIG.epoch_buffer
            > (VERONYMOUS_CLIENT_CONFIG.epoch_length
                - (now % VERONYMOUS_CLIENT_CONFIG.epoch_length))
        {
            // Go to the subsequent epoch
            next_epoch += VERONYMOUS_CLIENT_CONFIG.epoch_length;
        }
        // + 15 for wiggle room
        let refresh_start = next_epoch - VERONYMOUS_CLIENT_CONFIG.epoch_buffer - now + 15;

        Duration::from_secs(refresh_start)
    }

    fn now() -> u64 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}
