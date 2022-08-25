use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::DecodingError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenPayload {
    pub exp: u64,
}

pub fn decode_jwt_payload(jwt: &String) -> Result<TokenPayload, VeronymousClientError> {
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

    // Decode json
    let payload: TokenPayload = serde_json::from_slice(&payload)
        .map_err(|e| DecodingError(format!("Could not decode token json payload. {:?}", e)))?;

    Ok(payload)
}
