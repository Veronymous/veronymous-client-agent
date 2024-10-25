#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub mod client;
pub mod config;
pub mod error;
pub mod oidc;
pub mod servers;
pub mod veronymous_token;
pub mod vpn;
mod wg;
