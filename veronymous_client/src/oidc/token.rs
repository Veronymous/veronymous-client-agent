use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::DecodingError;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ClientResourceAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenPayload {
    pub exp: u64,
    pub resource_access: HashMap<String, ClientResourceAccess>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenPayload {
    pub exp: u64,
}

pub fn decode_jwt_payload<T: DeserializeOwned>(jwt: &String) -> Result<T, VeronymousClientError> {
    // Split the jwt
    let split = jwt.split(".");
    let parts: Vec<&str> = split.collect();

    if parts.len() != 3 {
        return Err(DecodingError(format!(
            "JWT has the wrong number of delimiters."
        )));
    }

    let payload = parts[1];

    // Base64 decode the payload
    let payload = base64::decode(payload)
        .map_err(|e| DecodingError(format!("Could not decode JWT payload. {:?}", e)))?;
    let payload = payload.clone();

    // Decode json
    let payload: T = serde_json::from_slice(&payload)
        .map_err(|e| DecodingError(format!("Could not decode token json payload. {:?}", e)))?;

    Ok(payload)
}
