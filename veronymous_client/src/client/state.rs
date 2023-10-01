use ps_signatures::keys::{PsParams, PsPublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use veronymous_token::root::RootVeronymousToken;

use crate::oidc::credentials::OidcCredentials;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientState {
    pub oidc_credentials: Option<OidcCredentials>,

    pub connections: VpnConnections,

    pub root_tokens: RootTokens,

    pub issuer_infos: IssuerInfos,
}

impl ClientState {
    pub fn new(
        oidc_credentials: Option<OidcCredentials>,
        connections: VpnConnections,
        root_tokens: RootTokens,
        issuer_infos: IssuerInfos,
    ) -> Self {
        Self {
            oidc_credentials,
            connections,
            root_tokens,
            issuer_infos,
        }
    }

    pub fn empty() -> Self {
        Self::new(
            None,
            VpnConnections::empty(),
            RootTokens::empty(),
            IssuerInfos::empty(),
        )
    }

    // Clear old connections and tokens
    pub fn clear_old(&mut self, key_epoch: u64, epoch: u64) {
        self.connections.clear_old(epoch);
        self.issuer_infos.clear_old(key_epoch);
        self.root_tokens.clear_old_connections(key_epoch);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VpnConnection {
    pub client_addresses: Vec<String>,

    pub wg_public_key: String,

    pub wg_endpoint: String,

    pub client_private_key: String,

    pub client_public_key: String,

    pub domain: String,
}

impl VpnConnection {
    pub fn new(
        client_addresses: Vec<String>,
        wg_public_key: String,
        wg_endpoint: String,
        client_private_key: String,
        client_public_key: String,
        domain: String,
    ) -> Self {
        Self {
            client_addresses,
            wg_public_key,
            wg_endpoint,
            client_private_key,
            client_public_key,
            domain,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VpnConnections {
    // <epoch, <domain, connection>>
    pub connections: HashMap<u64, HashMap<String, VpnConnection>>,
}

impl VpnConnections {
    pub fn new(connections: HashMap<u64, HashMap<String, VpnConnection>>) -> Self {
        Self { connections }
    }

    pub fn empty() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn has_connection(&self, epoch: &u64, domain: &String) -> bool {
        match self.connections.get(epoch) {
            None => false,
            Some(connections) => connections.contains_key(domain),
        }
    }

    pub fn get_connection(&self, epoch: &u64, domain: &String) -> Option<&VpnConnection> {
        match self.connections.get(epoch) {
            None => None,
            Some(connections) => connections.get(domain),
        }
    }

    pub fn add_connection(&mut self, connection: VpnConnection, epoch: u64, domain: String) {
        if let Some(connections) = self.connections.get_mut(&epoch) {
            connections.insert(domain, connection);
        } else {
            let mut connections = HashMap::new();
            connections.insert(domain, connection);

            self.connections.insert(epoch, connections);
        }
    }

    pub fn clear_old(&mut self, current_epoch: u64) {
        self.connections.retain(|&epoch, _| epoch >= current_epoch);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RootTokens {
    // epoch/token
    pub tokens: HashMap<u64, RootVeronymousToken>,
}

impl RootTokens {
    pub fn new(tokens: HashMap<u64, RootVeronymousToken>) -> Self {
        Self { tokens }
    }

    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }
}

pub trait EpochMap<T> {
    fn clear_old_connections(&mut self, current_epoch: u64) {
        let epoch_map = self.get_epoch_map();
        epoch_map.retain(|epoch, _| epoch >= &current_epoch);
    }

    fn get_epoch_map(&mut self) -> &mut HashMap<u64, T>;
}

impl EpochMap<HashMap<String, VpnConnection>> for VpnConnections {
    fn get_epoch_map(&mut self) -> &mut HashMap<u64, HashMap<String, VpnConnection>> {
        &mut self.connections
    }
}

impl EpochMap<RootVeronymousToken> for RootTokens {
    fn get_epoch_map(&mut self) -> &mut HashMap<u64, RootVeronymousToken> {
        &mut self.tokens
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssuerInfo {
    pub public_key: PsPublicKey,

    pub params: PsParams,
}

impl IssuerInfo {
    pub fn new(public_key: PsPublicKey, params: PsParams) -> Self {
        Self { public_key, params }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssuerInfos {
    // epoch/key
    pub issuer_infos: HashMap<u64, IssuerInfo>,
}

impl IssuerInfos {
    pub fn new(issuer_infos: HashMap<u64, IssuerInfo>) -> Self {
        Self { issuer_infos }
    }

    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    pub fn clear_old(&mut self, current_epoch: u64) {
        self.issuer_infos.retain(|&epoch, _| epoch >= current_epoch);
    }
}

impl EpochMap<IssuerInfo> for IssuerInfos {
    fn get_epoch_map(&mut self) -> &mut HashMap<u64, IssuerInfo> {
        &mut self.issuer_infos
    }
}

#[cfg(test)]
mod tests {
    use crate::client::state::EpochMap;
    use std::collections::HashMap;

    struct TestEpochMap {
        map: HashMap<u64, u16>,
    }

    impl EpochMap<u16> for TestEpochMap {
        fn get_epoch_map(&mut self) -> &mut HashMap<u64, u16> {
            &mut self.map
        }
    }

    #[test]
    fn test_epoch_map() {
        let mut map = HashMap::new();
        map.insert(10, 10);
        map.insert(20, 20);
        map.insert(30, 30);

        let mut epoch_map = TestEpochMap { map };

        // Remove epoch smaller than 20
        epoch_map.clear_old_connections(20);
        assert_eq!(2, epoch_map.map.len());
        assert!(epoch_map.map.contains_key(&30));
        assert!(!epoch_map.map.contains_key(&10));
    }
}
