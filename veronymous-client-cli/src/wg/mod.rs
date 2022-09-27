use crate::error::ClientError;
use crate::error::ClientError::{CommandError, IoError};
use rustc_serialize::hex::ToHex;
use std::env::temp_dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use veronymous_client::client::state::VpnConnection;

pub fn wg_down() -> Result<(), ClientError> {
    // Delete the interface. Ignore error (thrown if the interface does not exists)
    delete_wg_interface().ok();

    // TODO: Delete routing configuration

    Ok(())
}

pub fn wg_up(connection: &VpnConnection, tunnel_only: bool) -> Result<(), ClientError> {
    // Tear down existing connection
    wg_down()?;

    // Create the wireguard interface
    create_wg_interface()?;

    // Assign the ip addresses
    assign_wg_addresses(
        &connection.client_addresses[0],
        &connection.client_addresses[1],
    )?;

    configure_wg(
        &connection.client_private_key,
        &connection.client_public_key,
        &connection.wg_public_key,
        &connection.wg_endpoint,
    )?;

    start_wg_interface()?;

    if !tunnel_only {
        configure_routing()?;
    }

    Ok(())
}

pub fn wg_refresh(connection: &VpnConnection, tunnel_only: bool) -> Result<(), ClientError> {
    // Flush wireguard addresses
    flush_wg_addresses()?;

    // Assign the ip addresses
    assign_wg_addresses(
        &connection.client_addresses[0],
        &connection.client_addresses[1],
    )?;

    // Configure the wireguard interface
    configure_wg(
        &connection.client_private_key,
        &connection.client_public_key,
        &connection.wg_public_key,
        &connection.wg_endpoint,
    )?;

    if !tunnel_only {
        configure_routing()?;
    }

    Ok(())
}

/*
* Create wireguard network interface
*/
fn create_wg_interface() -> Result<(), ClientError> {
    run_command(&format!("ip link add dev veron0 type wireguard"))?;

    Ok(())
}

fn delete_wg_interface() -> Result<(), ClientError> {
    run_command(&format!("ip link delete dev veron0 type wireguard"))?;

    Ok(())
}

/*
* Assign the client wireguard addresses
*/
fn assign_wg_addresses(ip4_address: &String, ip6_address: &String) -> Result<(), ClientError> {
    // Set the ipv4 address
    run_command(&format!("ip address add {} dev veron0", ip4_address))?;

    // Set the ipv6 address
    run_command(&format!("ip address add {} dev veron0", ip6_address))?;

    Ok(())
}

fn flush_wg_addresses() -> Result<(), ClientError> {
    run_command(&format!("ip addr flush dev veron0"))?;

    Ok(())
}

/*
* Configure wireguard
* TODO: Put private key file generation in different function
*/
fn configure_wg(
    private_key: &String,
    public_key: &String,
    peer: &String,
    endpoint: &String,
) -> Result<(), ClientError> {
    // wg set wg0 private-key privatekey.txt peer /ZjSUjxcDiHHxBifHX0yVekKklDmczNv8k7M3AgmXXg= allowed-ips 0.0.0.0/0,::/0 endpoint wg1.ny.veronymous.io:51820

    // Create temp dir
    let temp = create_temp_dir(&public_key.as_bytes()[0..10].to_hex())?;

    let private_key_file = save_private_temp(&temp, private_key, public_key)?;

    match run_command(&format!(
        "wg set veron0 private-key {} peer {} allowed-ips 0.0.0.0/0,::/0 endpoint {}",
        private_key_file.to_str().unwrap(),
        peer,
        endpoint
    )) {
        Ok(_) => {
            fs::remove_dir_all(temp).map_err(|e| IoError(e.to_string()))?;

            Ok(())
        }
        Err(e) => {
            fs::remove_dir_all(temp).map_err(|e| IoError(e.to_string()))?;

            Err(e)
        }
    }
}

/*
* Start the wireguard interface
*/
fn start_wg_interface() -> Result<(), ClientError> {
    run_command(&format!("ip link set mtu 1420 up dev veron0"))?;

    Ok(())
}

// Configure routing of all traffic through the wireguard interface
// TODO: Select different table number?
fn configure_routing() -> Result<(), ClientError> {
    run_command(&format!("wg set veron0 fwmark 51820"))?;
    run_command(&format!("ip route add default dev veron0 table 51820"))?;
    run_command(&format!("ip rule add not fwmark 51820 table 51820"))?;
    run_command(&"ip rule add table main suppress_prefixlength 0".to_string())?;

    Ok(())
}

fn run_command(command: &String) -> Result<String, ClientError> {
    debug!("{}", command);
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

// TODO: umask 777
fn save_private_temp(
    temp: &PathBuf,
    private_key: &String,
    public_key: &String,
) -> Result<PathBuf, ClientError> {
    // Create temporary directory
    let mut private_key_file = temp.clone();
    private_key_file.push(format!("{}.priv", public_key.as_bytes()[0..10].to_hex()));

    fs::write(&private_key_file, private_key).map_err(|e| IoError(e.to_string()))?;

    Ok(private_key_file)
}

fn create_temp_dir(name: &String) -> Result<PathBuf, ClientError> {
    // Create temporary directory
    let mut temp = temp_dir();
    temp.push(name);

    fs::create_dir_all(&temp).map_err(|e| IoError(e.to_string()))?;

    Ok(temp)
}
