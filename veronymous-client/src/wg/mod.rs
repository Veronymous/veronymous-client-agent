use crate::error::VeronymousClientError;

use crate::error::VeronymousClientError::CommandError;
use std::process::Command;

/*
* NOTE: This will not work for non-linux platforms
 */

pub fn generate_keypair() -> Result<(String, String), VeronymousClientError> {
    let private_key = run_command(&"wg genkey".to_string())?.replace("\n", "");
    let public_key = run_command(&format!("printf {} | wg pubkey", private_key))?.replace("\n", "");

    Ok((private_key, public_key))
}

pub fn run_command(command: &String) -> Result<String, VeronymousClientError> {
    let out = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| CommandError(e.to_string()))?;

    if !out.status.success() {
        return Err(CommandError(format!(
            "Received an error: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use crate::wg::generate_keypair;

    #[test]
    fn generate_keypair_test() {
        generate_keypair().unwrap();
    }
}
