mod app;
mod constants;
mod error;
mod utils;
mod vpn_client;
mod wg;

#[macro_use]
extern crate log;

use env_logger::Env;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    app::run().await;
}
