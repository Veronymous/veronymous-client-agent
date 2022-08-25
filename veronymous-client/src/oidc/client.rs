use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::{DeserializationError, OidcError};
use crate::oidc::credentials::{OidcCredentials, UserCredentials};
use std::collections::HashMap;

const GRANT_TYPE: &str = "grant_type";
const CLIENT_ID: &str = "client_id";
const USERNAME: &str = "username";
const PASSWORD: &str = "password";
const REFRESH_TOKEN: &str = "refresh_token";
const PASSWORD_GRANT: &str = "password";
const REFRESH_TOKEN_GRANT: &str = "refresh_token";

// TODO: Put in http constants module
const STATUS_OK: u16 = 200;

pub struct OidcClient {
    token_endpoint: String,

    // Public OIDC client
    client_id: String,
}

impl OidcClient {
    pub fn new(token_endpoint: String, client_id: String) -> Self {
        Self {
            token_endpoint,
            client_id,
        }
    }

    pub async fn fetch_tokens(
        &self,
        credentials: &UserCredentials,
    ) -> Result<OidcCredentials, VeronymousClientError> {
        // Http client
        let client = reqwest::Client::new();

        // Request form
        let mut body = HashMap::new();
        body.insert(GRANT_TYPE, PASSWORD_GRANT.to_string());
        body.insert(CLIENT_ID, self.client_id.clone());
        body.insert(USERNAME, credentials.username.clone());
        body.insert(PASSWORD, credentials.password.clone());

        // Post
        let response = client
            .post(&self.token_endpoint)
            .form(&body)
            .send()
            .await
            .map_err(|e| OidcError(format!("Could not fetch user tokens. {:?}", e)))?;

        // Response code must be 200
        if response.status().as_u16() != STATUS_OK {
            return Err(OidcError(format!(
                "Got bad response code. Expected {}, but got {}.",
                STATUS_OK,
                response.status()
            )));
        }

        // Parse the body
        let oidc_credentials: OidcCredentials = response.json().await.map_err(|e| {
            DeserializationError(format!("Could not decode token info object. {:?}", e))
        })?;

        Ok(oidc_credentials)
    }

    pub async fn refresh_tokens(
        &self,
        credentials: &mut OidcCredentials,
    ) -> Result<(), VeronymousClientError> {
        // Http client
        let client = reqwest::Client::new();

        // Request form
        let mut body = HashMap::new();
        body.insert(GRANT_TYPE, REFRESH_TOKEN_GRANT.to_string());
        body.insert(CLIENT_ID, self.client_id.clone());
        body.insert(REFRESH_TOKEN, credentials.refresh_token.clone());

        // Post
        let response = client
            .post(&self.token_endpoint)
            .form(&body)
            .send()
            .await
            .map_err(|e| OidcError(format!("Could not refresh user tokens. {:?}", e)))?;

        // Response code must be 200
        if response.status().as_u16() != STATUS_OK {
            return Err(OidcError(format!(
                "Got bad response code. Expected {}, but got {}.",
                STATUS_OK,
                response.status()
            )));
        }

        // Parse the body
        let refreshed_credentials: OidcCredentials = response.json().await.map_err(|e| {
            DeserializationError(format!("Could not decode token info object. {:?}", e))
        })?;

        // Set the updated values
        credentials.access_token = refreshed_credentials.access_token;
        credentials.refresh_token = refreshed_credentials.refresh_token;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::oidc::client::OidcClient;
    use crate::oidc::credentials::UserCredentials;

    #[tokio::test]
    async fn test_oidc_client() {
        let token_endpoint: &str =
            "http://172.20.0.3:8080/realms/veronymous-vpn/protocol/openid-connect/token";
        let oidc_client_id: &str = "auth-client";

        let client = OidcClient::new(token_endpoint.to_string(), oidc_client_id.to_string());

        let credentials = UserCredentials::new("user1".to_string(), "password".to_string());

        let mut credentials = client.fetch_tokens(&credentials).await.unwrap();

        client.refresh_tokens(&mut credentials).await.unwrap();
    }
}
