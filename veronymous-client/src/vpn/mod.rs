use crate::network::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub struct VpnProfile {
    pub domain: String,

    pub agent_endpoint: Endpoint,

    pub root_cert: Vec<u8>,

    // Wireguard endpoint
    pub wg_endpoint: String,

    // Wireguard server public key
    pub wg_key: String,
}

impl VpnProfile {
    pub fn new(
        domain: String,
        agent_endpoint: Endpoint,
        root_cert: Vec<u8>,
        wg_endpoint: String,
        wg_key: String,
    ) -> Self {
        Self {
            domain,
            agent_endpoint,
            root_cert,
            wg_endpoint,
            wg_key,
        }
    }
}
