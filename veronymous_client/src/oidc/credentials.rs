use crate::config::VERONYMOUS_CLIENT_CONFIG;
use crate::error::VeronymousClientError;
use crate::oidc::token::{decode_jwt_payload, AccessTokenPayload, RefreshTokenPayload};
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
        let access_token: AccessTokenPayload = decode_jwt_payload(&self.access_token)?;
        let refresh_token: RefreshTokenPayload = decode_jwt_payload(&self.refresh_token)?;

        debug!("Getting oidc credentials status.");
        debug!("Access token: {}", self.access_token);
        debug!("Refresh token: {}", self.refresh_token);
        debug!("Now: {}", now);
        debug!("Next epoch: {}", next_epoch);
        debug!("Access token: {:?}", access_token);
        debug!("Refresh token: {:?}", refresh_token);

        return if !Self::token_has_subscription(&access_token)? {
            debug!("Does not have a subscription");
            Ok(OidcCredentialsStatus::SubscriptionRequired)
        } else if refresh_token.exp <= now {
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

    pub fn has_subscription(&self) -> Result<bool, VeronymousClientError> {
        // Decode the access token
        let access_token: AccessTokenPayload = decode_jwt_payload(&self.access_token)?;

        Self::token_has_subscription(&access_token)
    }

    fn token_has_subscription(token: &AccessTokenPayload) -> Result<bool, VeronymousClientError> {
        // Get the resource access for the subscription oidc client
        let resource_access = match token
            .resource_access
            .get(&VERONYMOUS_CLIENT_CONFIG.sub_oidc_client_id)
        {
            Some(resource_access) => resource_access,
            None => {
                return Ok(false);
            }
        };

        // Check the subscription role
        Ok(resource_access
            .roles
            .contains(&VERONYMOUS_CLIENT_CONFIG.sub_oidc_role))
    }
}

#[derive(Debug)]
pub enum OidcCredentialsStatus {
    OK,
    RefreshRequired,
    AuthRequired,
    SubscriptionRequired,
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
