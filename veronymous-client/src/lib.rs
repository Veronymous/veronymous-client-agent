#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

pub mod client;
pub mod constants;
pub mod error;
pub mod oidc;
pub mod veronymous_token;
pub mod vpn;
mod wg;
