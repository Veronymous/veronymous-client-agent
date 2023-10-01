use crate::error::VeronymousClientError;

use curve25519_dalek::{EdwardsPoint, Scalar};
use rand_core::OsRng;

/*
* NOTE: This will not work for non-linux platforms
 */

pub fn generate_keypair() -> Result<(String, String), VeronymousClientError> {
    let mut csprng = OsRng;

    let private_key = Scalar::random(&mut csprng).to_bytes();
    let public_key = EdwardsPoint::mul_base_clamped(private_key).to_montgomery();

    Ok((
        base64::encode(private_key),
        base64::encode(public_key.to_bytes()),
    ))
}

#[cfg(test)]
mod tests {
    use crate::wg::generate_keypair;

    #[test]
    fn generate_keypair_test() {
        generate_keypair().unwrap();
    }
}
