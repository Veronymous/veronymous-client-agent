use crate::config::VERONYMOUS_CLIENT_CONFIG;
use crate::error::VeronymousClientError;
use crate::error::VeronymousClientError::{DeserializationError, HttpError, IllegalArgumentError, NotFoundError, ParseError};
use crate::vpn::VpnProfile;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

type DomainId = String;
type ServerId = String;
type ServersMap = HashMap<DomainId, HashMap<ServerId, VpnProfile>>;

const DIGEST_HEADER: &str = "Digest";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VpnServers {
    // <domain, <server-id, server>>
    pub servers: ServersMap,

    // The digest of the servers file
    pub digest: Option<String>,
}

impl VpnServers {
    pub fn new() -> Self {
        Self {
            servers: ServersMap::new(),
            digest: None,
        }
    }

    pub fn list_domains(&self) -> Vec<DomainId> {
        let mut domains: Vec<DomainId> = Vec::with_capacity(self.servers.len());

        for domain in &self.servers {
            domains.push(domain.0.clone());
        }

        domains
    }

    pub async fn update(&mut self) -> Result<bool, VeronymousClientError> {
        let servers_endpoint = &VERONYMOUS_CLIENT_CONFIG.servers_endpoint;

        if self.is_update_required(servers_endpoint).await? {
            let response = reqwest::get(servers_endpoint)
                .await
                .map_err(|e| HttpError(format!("Could not fetch servers. {:?}", e)))?;

            let digest = Self::get_digest(&response)?;

            let servers = response
                .json::<ServersMap>()
                .await
                .map_err(|e| ParseError(format!("Could not parse servers response. {:?}", e)))?;

            self.servers = servers;
            self.digest = Some(digest);

            Ok(true)
        } else {
            // Update is not required, do nothing
            Ok(false)
        }
    }

    pub fn find_server(&self, domain: &DomainId) -> Result<&VpnProfile, VeronymousClientError> {
        let vpn_profiles = match self.servers.get(domain) {
            Some(vpn_profiles) => vpn_profiles,
            None => return Err(NotFoundError(format!("Could not find server for name {}", domain)))
        };

        // Get a random profile
        let mut rng = rand::thread_rng();

        let vpn_profile_index = rng.gen_range(0, vpn_profiles.len());

        let mut current_index: usize = 0;

        for vpn_profile in vpn_profiles {
            if vpn_profile_index == current_index {
                return Ok(vpn_profile.1);
            }
            current_index += 1;
        }

        return Err(NotFoundError(format!("Could not find server")));
    }

    async fn is_update_required(
        &self,
        servers_endpoint: &String,
    ) -> Result<bool, VeronymousClientError> {
        // Get the current digest
        let digest = match &self.digest {
            None => return Ok(true),
            Some(digest) => digest,
        };

        let file_metadata = Self::get_servers_metadata(servers_endpoint).await?;

        Ok(digest.to_string() != file_metadata.digest)
    }

    async fn get_servers_metadata(
        servers_endpoint: &String,
    ) -> Result<FileMetadata, VeronymousClientError> {
        let metadata_endpoint = servers_endpoint.to_string() + "/metadata";

        let metadata = reqwest::get(metadata_endpoint)
            .await
            .map_err(|e| HttpError(format!("Could not get file metadata. {:?}", e)))?
            .json::<FileMetadata>()
            .await
            .map_err(|e| HttpError(format!("Could not parse file metadata. {:?}", e)))?;

        Ok(metadata)
    }

    // Get the digest header value
    fn get_digest(response: &Response) -> Result<String, VeronymousClientError> {
        let digest = match response.headers().get(DIGEST_HEADER) {
            None => {
                return Err(IllegalArgumentError(format!(
                    "Response is missing the 'Digest' header."
                )));
            }
            Some(digest) => digest,
        };

        let digest = digest
            .to_str()
            .map_err(|e| DeserializationError(format!("Could not decode digest. {:?}", e)))?;

        Ok(digest.to_string())
    }
}

#[derive(Deserialize, Debug, Clone)]
struct FileMetadata {
    digest: String,
}

// #[cfg(test)]
// mod tests {
//     use crate::servers::VpnServers;
//
//     #[tokio::test]
//     async fn servers_update_test() {
//         let mut servers = VpnServers::new();
//
//         servers
//             .update()
//             .await
//             .unwrap();
//
//         println!("{:?}", servers);
//
//         servers
//             .update()
//             .await
//             .unwrap();
//
//         println!("{:?}", servers);
//     }
// }
