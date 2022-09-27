mod app;
mod constants;
mod error;
mod utils;
mod vpn_client;
mod wg;

#[tokio::main]
async fn main() {
    app::run().await;
}
