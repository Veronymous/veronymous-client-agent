use crypto_common::rand_non_zero_fr;
use ps_signatures::keys::{PsParams, PsPublicKey};
use ps_signatures::serde::Serializable;
use rand::thread_rng;
use tonic::metadata::{Ascii, MetadataKey, MetadataValue};
use tonic::transport::Channel;
use veronymous_token::root::RootVeronymousToken;
use veronymous_token::root_exchange::{complete_root_token, create_root_token_request, RootTokenResponse};
use veronymous_token::serde::Serializable as TokenSerializable;
use crate::config::VERONYMOUS_CLIENT_CONFIG;
use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::TokenClientError;
use crate::veronymous_token::grpc::veronymous_user_token_service::{TokenInfo, TokenInfoRequest, TokenRequest};
use crate::veronymous_token::grpc::veronymous_user_token_service::veronymous_user_token_service_client::VeronymousUserTokenServiceClient;

lazy_static! {
    static ref AUTHORIZATION_BEARER: MetadataKey<Ascii> =
        MetadataKey::from_bytes(b"Authorization-Bearer").unwrap();
}

type AccessToken = MetadataValue<Ascii>;

pub struct VeronymousTokenClient {
    grpc_client: VeronymousUserTokenServiceClient<Channel>,
}

impl VeronymousTokenClient {
    pub async fn create(endpoint: String) -> Result<Self, VeronymousClientError> {
        let grpc_client = VeronymousUserTokenServiceClient::connect(endpoint)
            .await
            .map_err(|e| TokenClientError(format!("Could not connect to token issuer. {:?}", e)))?;

        Ok(Self { grpc_client })
    }

    pub async fn fetch_token(
        &mut self,
        issuer_key_params: &PsParams,
        issuer_key: &PsPublicKey,
        access_token: &String,
        key_epoch: u64,
        epoch: u64,
    ) -> Result<RootVeronymousToken, VeronymousClientError> {
        // Generate the token id and blinding
        let token_id = rand_non_zero_fr(&mut thread_rng());
        let blinding = rand_non_zero_fr(&mut thread_rng());

        let is_next_epoch = Self::is_next_epoch(key_epoch, epoch);

        let access_token = Self::assemble_access_token(access_token)?;

        // Assemble the token request
        let token_request =
            create_root_token_request(&token_id, &blinding, issuer_key, issuer_key_params)
                .map_err(|e| {
                    TokenClientError(format!("Could not create token request. {:?}", e))
                })?;
        let token_request = token_request.serialize();

        // Get the token response
        let token_response = match is_next_epoch {
            true => self.get_next_token(token_request, &access_token).await?,
            false => self.get_token(token_request, &access_token).await?,
        };

        let token_response = RootTokenResponse::deserialize(&token_response).map_err(|e| {
            TokenClientError(format!("Could not deserialize token response. {:?}", e))
        })?;

        // Complete the token
        let root_token = complete_root_token(
            &token_response,
            &token_id,
            &blinding,
            &issuer_key,
            &issuer_key_params,
        )
        .map_err(|e| TokenClientError(format!("Could not complete root token. {:?}", e)))?;

        Ok(root_token)
    }

    pub async fn get_token_info(
        &mut self,
        epoch: u64,
        key_epoch: u64,
        access_token: &String,
    ) -> Result<(PsPublicKey, PsParams), VeronymousClientError> {
        let is_next_epoch = Self::is_next_epoch(key_epoch, epoch);

        let access_token = Self::assemble_access_token(access_token)?;

        let token_info = match is_next_epoch {
            true => self.get_next_token_info(&access_token).await?,
            false => self.get_current_token_info(&access_token).await?,
        };

        // Decode the values
        let public_key = PsPublicKey::deserialize(&token_info.public_key).map_err(|e| {
            TokenClientError(format!("Could not decode token issuer public key. {:?}", e))
        })?;

        let params = PsParams::deserialize(&token_info.params).map_err(|e| {
            TokenClientError(format!("Could not decode token info params. {:?}", e))
        })?;

        Ok((public_key, params))
    }

    async fn get_token(
        &mut self,
        token_request: Vec<u8>,
        access_token: &AccessToken,
    ) -> Result<Vec<u8>, VeronymousClientError> {
        let request = TokenRequest { token_request };
        let mut request = tonic::Request::new(request);
        request
            .metadata_mut()
            .insert(AUTHORIZATION_BEARER.clone(), access_token.clone());

        let token_response = self
            .grpc_client
            .get_token(request)
            .await
            .map_err(|e| TokenClientError(format!("Could not get token. {:?}", e)))?
            .into_inner();

        Ok(token_response.token_response)
    }

    async fn get_next_token(
        &mut self,
        token_request: Vec<u8>,
        access_token: &AccessToken,
    ) -> Result<Vec<u8>, VeronymousClientError> {
        let request = TokenRequest { token_request };
        let mut request = tonic::Request::new(request);
        request
            .metadata_mut()
            .insert(AUTHORIZATION_BEARER.clone(), access_token.clone());

        let token_response = self
            .grpc_client
            .get_next_token(request)
            .await
            .map_err(|e| TokenClientError(format!("Could not get token. {:?}", e)))?
            .into_inner();

        Ok(token_response.token_response)
    }

    async fn get_current_token_info(
        &mut self,
        access_token: &AccessToken,
    ) -> Result<TokenInfo, VeronymousClientError> {
        let request = TokenInfoRequest {};
        let mut request = tonic::Request::new(request);
        request
            .metadata_mut()
            .insert(AUTHORIZATION_BEARER.clone(), access_token.clone());

        let token_info = self
            .grpc_client
            .get_token_info(request)
            .await
            .map_err(|e| TokenClientError(format!("Could not fetch token info. {:?}", e)))?
            .into_inner();

        Ok(token_info)
    }

    async fn get_next_token_info(
        &mut self,
        access_token: &AccessToken,
    ) -> Result<TokenInfo, VeronymousClientError> {
        let request = TokenInfoRequest {};
        let mut request = tonic::Request::new(request);
        request
            .metadata_mut()
            .insert(AUTHORIZATION_BEARER.clone(), access_token.clone());

        let token_info = self
            .grpc_client
            .get_next_token_info(request)
            .await
            .map_err(|e| TokenClientError(format!("Could not fetch token info. {:?}", e)))?
            .into_inner();

        Ok(token_info)
    }

    fn assemble_access_token(access_token: &String) -> Result<AccessToken, VeronymousClientError> {
        let access_token = access_token
            .parse()
            .map_err(|e| TokenClientError(format!("Could not encode access token. {:?}", e)))?;

        Ok(access_token)
    }

    // Check if the epoch belongs to the next key epoch
    fn is_next_epoch(key_epoch: u64, epoch: u64) -> bool {
        let next_key_epoch = key_epoch + VERONYMOUS_CLIENT_CONFIG.key_lifetime;

        return epoch >= next_key_epoch;
    }
}
