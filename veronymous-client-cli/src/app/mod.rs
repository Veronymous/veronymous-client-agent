use crate::constants::cli::{
    ABOUT, APP_NAME, APP_VERSION_01, AUTHOR, CONNECT_COMMAND, CONNECT_COMMAND_ABOUT,
    CONNECT_COMMAND_VERSION, TUNNEL_ONLY_ARG, VPN_PROFILE_ARG,
};
use crate::error::ClientError;
use crate::utils::cli_utils::{get_password, get_user_input};
use crate::vpn_client::VpnClient;
use crate::wg::wg_down;
use clap::{App, Arg, ArgMatches, SubCommand};
use veronymous_client::error::VeronymousClientError;

type RedoRequired = bool;

pub async fn run() {
    // Get the CLI
    let matches = get_matches();

    if let Some(matches) = matches.subcommand_matches(CONNECT_COMMAND) {
        run_connect(matches).await;
    } else {
        debug!("Command is not supported.");
    }
}

pub async fn run_connect(matches: &ArgMatches) {
    let vpn_profile = matches.value_of(VPN_PROFILE_ARG).unwrap().to_string();
    let tunnel_only = matches.is_present(TUNNEL_ONLY_ARG);
    let mut vpn_client = VpnClient::create().await.unwrap();

    // Set the Ctrl-C handler
    set_disconnect_handler();

    while connect(&vpn_profile, tunnel_only, &mut vpn_client).await {
        // Redo
        connect(&vpn_profile, tunnel_only, &mut vpn_client).await;
    }

    // An error has occurred, disconnect
    disconnect();
}

async fn connect(vpn_profile: &String, tunnel_only: bool, client: &mut VpnClient) -> RedoRequired {
    match client.connect(vpn_profile.to_string(), tunnel_only).await {
        Ok(_) => {}
        Err(error) => match error {
            ClientError::VeronymousClientError(error) => match error {
                VeronymousClientError::AuthRequired() => {
                    user_auth(&client).await;
                    return true;
                }
                _ => {
                    error!("An error has occurred. {:?}", error);
                }
            },
            _ => {
                error!("An error has occurred. {:?}", error);
            }
        },
    }

    false
}

async fn user_auth(client: &VpnClient) {
    println!("Enter username:");
    let user_name = get_user_input();

    println!("Enter password:");
    let password = get_password();

    match client.authenticate(user_name, password).await {
        Ok(_) => {}
        Err(e) => match e {
            ClientError::VeronymousClientError(e) => match e {
                VeronymousClientError::OidcError(_) => {
                    error!("Authentication failed.");
                }
                _ => {
                    error!("An error has occurred. {:?}", e);
                }
            },
            _ => {
                error!("An error has occurred. {:?}", e);
            }
        },
    }
}

fn set_disconnect_handler() {
    ctrlc::set_handler(move || {
        info!("Received Exit Signal!");

        disconnect();
    })
    .expect("Could not set ctrl-c handler.");
}

fn disconnect() {
    match wg_down() {
        Ok(_) => {
            std::process::exit(0);
        }
        Err(e) => {
            error!(
                "Encountered an error when tearing down the connection. {:?}",
                e
            );
            std::process::exit(1);
        }
    }
}

pub fn get_matches() -> ArgMatches {
    App::new(APP_NAME)
        .version(APP_VERSION_01)
        .author(AUTHOR)
        .about(ABOUT)
        .subcommand(
            SubCommand::with_name("connect")
                .about(CONNECT_COMMAND_ABOUT)
                .version(CONNECT_COMMAND_VERSION)
                .author(AUTHOR)
                .arg(
                    Arg::with_name(VPN_PROFILE_ARG)
                        .help("VPN profile file.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name(TUNNEL_ONLY_ARG)
                        .help("Don't forward all traffic through the VPN server.")
                        .long("--tunnel-only")
                        .short('t')
                        .required(false)
                        .takes_value(false),
                ),
        )
        .get_matches()
}