use serde::Deserialize;

lazy_static! {
    pub static ref VERONYMOUS_CLIENT_CONFIG: VeronymousClientConfig =
        VeronymousClientConfig::default();
}

#[derive(Clone, Debug, Deserialize)]
pub struct VeronymousClientConfig {
    pub epoch_length: u64,

    pub epoch_buffer: u64,

    pub key_lifetime: u64,

    pub oidc_endpoint: String,

    pub oidc_client_id: String,

    pub token_endpoint: String,
}

#[cfg(feature = "dev-local")]
#[cfg(not(feature = "dev-env"))]
impl Default for VeronymousClientConfig {
    fn default() -> Self {
        Self {
            // 10 minutes
            epoch_length: 600,
            // 1 minute
            epoch_buffer: 60,
            // 12 hours
            key_lifetime: 43200,
            oidc_endpoint:
                "http://172.20.0.3:8080/realms/veronymous-vpn/protocol/openid-connect/token"
                    .to_string(),
            oidc_client_id: "auth-client".to_string(),
            token_endpoint: "http://127.0.0.1:9123".to_string(),
        }
    }
}

#[cfg(feature = "dev-env")]
impl Default for VeronymousClientConfig {
    fn default() -> Self {
        Self {
            // 10 minutes
            epoch_length: 600,
            // 1 minute
            epoch_buffer: 60,
            // 12 hours
            key_lifetime: 43200,
            oidc_endpoint: "http://keycloak.192.168.2.41.veronymous.io/realms/veronymous-vpn/protocol/openid-connect/token".to_string(),
            oidc_client_id: "auth-client".to_string(),
            token_endpoint: "http://192.168.2.41.veronymous.io:30001".to_string()
        }
    }
}
