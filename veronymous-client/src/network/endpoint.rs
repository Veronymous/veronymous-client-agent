use regex::Regex;
use std::net::SocketAddr;

lazy_static! {
    static ref ENDPOINT_REGEX: Regex = Regex::new("^(.+):([0-9]+)$").unwrap();
}

#[derive(Clone, Debug)]
pub struct Endpoint {
    // NOTE: Veronymous servers will only have ipv4 addresses
    pub address: SocketAddr,

    pub domain: String,
}

impl Endpoint {
    pub fn new(address: SocketAddr, domain: String) -> Self {
        Self { address, domain }
    }
}
