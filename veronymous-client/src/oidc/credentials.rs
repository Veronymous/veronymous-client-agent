use crate::error::VeronymousClientError;
use crate::oidc::token::decode_jwt_payload;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OidcCredentials {
    pub access_token: String,

    pub refresh_token: String,
}

impl OidcCredentials {
    pub fn status(
        &self,
        now: u64,
        next_epoch: u64,
    ) -> Result<OidcCredentialsStatus, VeronymousClientError> {
        // Decode the access and refresh tokens
        let access_token = decode_jwt_payload(&self.access_token)?;
        let refresh_token = decode_jwt_payload(&self.refresh_token)?;

        debug!("Getting oidc credentials status.");
        debug!("Access token: {}", self.access_token);
        debug!("Refresh token: {}", self.refresh_token);
        debug!("Now: {}", now);
        debug!("Next epoch: {}", next_epoch);
        debug!("Access token: {:?}", access_token);
        debug!("Refresh token: {:?}", refresh_token);

        return if refresh_token.exp <= now {
            debug!("Refresh token is expired.");
            Ok(OidcCredentialsStatus::AuthRequired)
        } else if refresh_token.exp <= next_epoch - 60 {
            debug!("Refresh token will be expired at the next epoch.");
            Ok(OidcCredentialsStatus::RefreshRequired)
        } else if access_token.exp <= now {
            debug!("Access token is expired.");
            Ok(OidcCredentialsStatus::RefreshRequired)
        } else {
            debug!("Tokens are all good.");
            Ok(OidcCredentialsStatus::OK)
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
