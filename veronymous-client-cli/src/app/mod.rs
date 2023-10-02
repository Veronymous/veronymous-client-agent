use crate::constants::cli::{
    ABOUT, APP_NAME, APP_VERSION_01, AUTHOR, CONNECT_COMMAND, CONNECT_COMMAND_ABOUT,
    CONNECT_COMMAND_VERSION, LIST_SERVERS, LIST_SERVERS_ABOUT, LIST_SERVERS_VERSION, SERVER_NAME,
    TUNNEL_ONLY_ARG,
};
use crate::error::CliClientError;
use crate::utils::cli_utils::{get_password, get_user_input};
use crate::vpn_client::CliVpnClient;
use crate::wg::wg_down;
use clap::{App, Arg, ArgMatches, SubCommand};
use veronymous_client::error::VeronymousClientError;

type RedoRequired = bool;

pub async fn run() {
    // Get the CLI
    let matches = get_matches();

    if let Some(matches) = matches.subcommand_matches(CONNECT_COMMAND) {
        run_connect(matches).await;
    } else if let Some(matches) = matches.subcommand_matches(LIST_SERVERS) {
        run_list_servers(matches).await;
    } else {
        debug!("Command is not supported.");
    }
}

pub async fn run_connect(matches: &ArgMatches) {
    let server_name = matches.value_of(SERVER_NAME).unwrap().to_string();
    let tunnel_only = matches.is_present(TUNNEL_ONLY_ARG);
    let mut vpn_client = CliVpnClient::create().await.unwrap();

    // Set the Ctrl-C handler
    set_disconnect_handler();

    while connect(&server_name, tunnel_only, &mut vpn_client).await {
        // Redo
        connect(&server_name, tunnel_only, &mut vpn_client).await;
    }

    // An error has occurred, disconnect
    disconnect();
}

async fn run_list_servers(_matches: &ArgMatches) {
    let vpn_client = CliVpnClient::create().await.unwrap();

    // Set the Ctrl-C handler
    set_disconnect_handler();

    let servers = match vpn_client.get_servers(None).await {
        Ok(servers) => servers,
        Err(error) => {
            println!("An error has occurred.");
            debug!("Could not get servers. {:?}", error);
            return;
        }
    };

    println!("VPN Servers");

    // Print the servers
    for server in servers {
        println!("\t* {}", server);
    }
}

async fn connect(
    server_name: &String,
    tunnel_only: bool,
    client: &mut CliVpnClient,
) -> RedoRequired {
    match client.connect(server_name.to_string(), tunnel_only).await {
        Ok(_) => {}
        Err(error) => match error {
            CliClientError::VeronymousClientError(error) => match error {
                VeronymousClientError::AuthRequired() => {
                    user_auth(&client).await;
                    return true;
                }
                VeronymousClientError::SubscriptionRequired() => {
                    debug!("Subscription is required");
                    return false;
                }
                _ => {
                    error!("An error has occurred. {:?}", error);
                }
            },
            CliClientError::SubscriptionRequired => {
                error!("VPN Subscription is required.");
            }
            _ => {
                error!("An error has occurred. {:?}", error);
            }
        },
    }

    false
}

async fn user_auth(client: &CliVpnClient) {
    println!("Enter username:");
    let user_name = get_user_input();

    println!("Enter password:");
    let password = get_password();

    match client.authenticate(user_name, password).await {
        Ok(_) => {}
        Err(e) => match e {
            CliClientError::VeronymousClientError(e) => match e {
                VeronymousClientError::OidcError(_) => {
                    error!("Authentication failed.");
                }
                VeronymousClientError::SubscriptionRequired() => {
                    error!("VPN subscription is required.");
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
            SubCommand::with_name(CONNECT_COMMAND)
                .about(CONNECT_COMMAND_ABOUT)
                .version(CONNECT_COMMAND_VERSION)
                .author(AUTHOR)
                .arg(
                    Arg::with_name(SERVER_NAME)
                        .help("Server name.")
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
        .subcommand(
            SubCommand::with_name(LIST_SERVERS)
                .about(LIST_SERVERS_ABOUT)
                .version(LIST_SERVERS_VERSION)
                .author(AUTHOR),
        )
        .get_matches()
}
