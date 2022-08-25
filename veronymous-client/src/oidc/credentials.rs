use crate::error::VeronymousClientError;
use crate::oidc::token::decode_jwt_payload;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OidcCredentials {
    pub access_token: String,

    pub refresh_token: String,
}

impl OidcCredentials {
    pub fn status(&self, now: u64) -> Result<OidcCredentialsStatus, VeronymousClientError> {
        // Decode the access and refresh tokens
        let access_token = decode_jwt_payload(&self.access_token)?;
        let refresh_token = decode_jwt_payload(&self.refresh_token)?;

        // Get the current time
        //let now = Self::now();

        // If access token is not expired
        return if access_token.exp > now {
            Ok(OidcCredentialsStatus::OK)
        } else if refresh_token.exp > now {
            // Refresh token is not expired
            Ok(OidcCredentialsStatus::RefreshRequired)
        } else {
            // Both access token and the refresh token are expired
            Ok(OidcCredentialsStatus::AuthRequired)
        };
    }
}

#[derive(Debug)]
pub enum OidcCredentialsStatus {
    OK,
    RefreshRequired,
    AuthRequired,
}

#[derive(Clone, Debug)]
pub struct UserCredentials {
    pub(crate) username: String,

    pub(crate) password: String,
}

impl UserCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}
