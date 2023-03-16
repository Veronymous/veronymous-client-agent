use serde::Deserialize;

lazy_static! {
    pub static ref VERONYMOUS_CLIENT_CONFIG: VeronymousClientConfig =
        VeronymousClientConfig::default();
}

#[derive(Clone, Debug, Deserialize)]
pub struct VeronymousClientConfig {
    pub epoch_length: u64,

    pub epoch_buffer: u64,

    pub key_lifetime: u64,

    pub oidc_endpoint: String,

    pub oidc_client_id: String,

    pub token_endpoint: String,

    pub token_endpoint_ca: String,

    pub servers_endpoint: String,
}

#[cfg(feature = "dev-local")]
#[cfg(not(feature = "dev-env"))]
impl Default for VeronymousClientConfig {
    fn default() -> Self {
        Self {
            // 10 minutes
            epoch_length: 600,
            // 1 minute
            epoch_buffer: 60,
            // 12 hours
            // key_lifetime: 43200,
            // 10 minutes
            key_lifetime: 600,
            oidc_endpoint:
                "http://172.20.0.3:8080/realms/veronymous-vpn/protocol/openid-connect/token"
                    .to_string(),
            oidc_client_id: "auth-client".to_string(),
            token_endpoint: "https://localhost.veronymous.io:9123".to_string(),
            token_endpoint_ca: "-----BEGIN CERTIFICATE-----\nMIIDyzCCArOgAwIBAgIUANb3hm6n1wwhGkjB0XN2fctauGUwDQYJKoZIhvcNAQEL\nBQAwdTELMAkGA1UEBhMCQ0ExEDAOBgNVBAgMB09udGFyaW8xDzANBgNVBAcMBk90\ndGF3YTEhMB8GA1UECgwYSW50ZXJuZXQgV2lkZ2l0cyBQdHkgTHRkMSAwHgYDVQQD\nDBdsb2NhbGhvc3QudmVyb255bW91cy5pbzAeFw0yMjEyMjExMzI3NDFaFw0yNzEy\nMjAxMzI3NDFaMHUxCzAJBgNVBAYTAkNBMRAwDgYDVQQIDAdPbnRhcmlvMQ8wDQYD\nVQQHDAZPdHRhd2ExITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDEg\nMB4GA1UEAwwXbG9jYWxob3N0LnZlcm9ueW1vdXMuaW8wggEiMA0GCSqGSIb3DQEB\nAQUAA4IBDwAwggEKAoIBAQCxx0+i60ptd2flxcBw+OpQM2oBm/riL0wGqOWc6j2F\nhEDJkfjcK4Fcc+8hcyGNNy11f2l59yuCY7wJhyZPXhyXi0lrkN328hPo19rYzYze\n83AQYKcq9XucAGbv9kRRSVyyeKu45DqSinClgfZzgB6qRNMB8yZl7cqhVwjLpa47\nVUH4zhDHYfKfH8cBMXGlW2gPexJWqGeusXhuXCd8dHoCzzGr6+NCxkzffpsLI3FN\nLPNXPaq8cYynyi/tO4A3QX6gTOCmKnwlNtZTpHUBy4BKV2HZ4XRVojfH+lOuylL3\nqgzYkQWsqaizZEIzlg5iEh4py50HsTq/JOXpXgfD7eadAgMBAAGjUzBRMB0GA1Ud\nDgQWBBT1Rui71l7VsTyoZvYmkSOTxZFz8TAfBgNVHSMEGDAWgBT1Rui71l7VsTyo\nZvYmkSOTxZFz8TAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQAl\n+zDuALuo50w4PClws1ZGRGVYZQqgDKU32oR1zo+rSGbrqcO5yH2aanCeOX5oIJqC\nC1VPyjAbZ6x8kUTfzp+OtT2J3RJTA/jTaP2opR9QHZZ+uYQkalZky/djjsNNw2+X\nvlw2UZ+OfZI/hVEArEo7tc+qUvzcdhbthJOtSFhcQaY04Jd659Cj4svsZm8Jui+v\ngjZpJE1Ezp2hVVMAU7zO1Joe/CqcUnbpQXCPdZ0Wk2XxDwSXKtgY3VyAFJrS/DP7\ngdqvcZekbRaQmNXsK0CUjw5n2pDdgiu4XfN+FL0RN6nuC1ZRw3zNM6Y0qynib697\neFfhCIv9u/9vLexDNm0o\n-----END CERTIFICATE-----".to_string(),
            servers_endpoint: "http://localhost:9090/servers.json".to_string(),
        }
    }
}

#[cfg(feature = "dev-env")]
impl Default for VeronymousClientConfig {
    fn default() -> Self {
        Self {
            // 10 minutes
            epoch_length: 600,
            // 1 minute
            epoch_buffer: 60,
            // 12 hours
            key_lifetime: 43200,
            oidc_endpoint: "http://keycloak.192.168.2.41.veronymous.io/realms/veronymous-vpn/protocol/openid-connect/token".to_string(),
            oidc_client_id: "auth-client".to_string(),
            token_endpoint: "https://192.168.2.41.veronymous.io:30001".to_string(),
            token_endpoint_ca: "-----BEGIN CERTIFICATE-----\nMIIDyzCCArOgAwIBAgIUANb3hm6n1wwhGkjB0XN2fctauGUwDQYJKoZIhvcNAQEL\nBQAwdTELMAkGA1UEBhMCQ0ExEDAOBgNVBAgMB09udGFyaW8xDzANBgNVBAcMBk90\ndGF3YTEhMB8GA1UECgwYSW50ZXJuZXQgV2lkZ2l0cyBQdHkgTHRkMSAwHgYDVQQD\nDBdsb2NhbGhvc3QudmVyb255bW91cy5pbzAeFw0yMjEyMjExMzI3NDFaFw0yNzEy\nMjAxMzI3NDFaMHUxCzAJBgNVBAYTAkNBMRAwDgYDVQQIDAdPbnRhcmlvMQ8wDQYD\nVQQHDAZPdHRhd2ExITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDEg\nMB4GA1UEAwwXbG9jYWxob3N0LnZlcm9ueW1vdXMuaW8wggEiMA0GCSqGSIb3DQEB\nAQUAA4IBDwAwggEKAoIBAQCxx0+i60ptd2flxcBw+OpQM2oBm/riL0wGqOWc6j2F\nhEDJkfjcK4Fcc+8hcyGNNy11f2l59yuCY7wJhyZPXhyXi0lrkN328hPo19rYzYze\n83AQYKcq9XucAGbv9kRRSVyyeKu45DqSinClgfZzgB6qRNMB8yZl7cqhVwjLpa47\nVUH4zhDHYfKfH8cBMXGlW2gPexJWqGeusXhuXCd8dHoCzzGr6+NCxkzffpsLI3FN\nLPNXPaq8cYynyi/tO4A3QX6gTOCmKnwlNtZTpHUBy4BKV2HZ4XRVojfH+lOuylL3\nqgzYkQWsqaizZEIzlg5iEh4py50HsTq/JOXpXgfD7eadAgMBAAGjUzBRMB0GA1Ud\nDgQWBBT1Rui71l7VsTyoZvYmkSOTxZFz8TAfBgNVHSMEGDAWgBT1Rui71l7VsTyo\nZvYmkSOTxZFz8TAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQAl\n+zDuALuo50w4PClws1ZGRGVYZQqgDKU32oR1zo+rSGbrqcO5yH2aanCeOX5oIJqC\nC1VPyjAbZ6x8kUTfzp+OtT2J3RJTA/jTaP2opR9QHZZ+uYQkalZky/djjsNNw2+X\nvlw2UZ+OfZI/hVEArEo7tc+qUvzcdhbthJOtSFhcQaY04Jd659Cj4svsZm8Jui+v\ngjZpJE1Ezp2hVVMAU7zO1Joe/CqcUnbpQXCPdZ0Wk2XxDwSXKtgY3VyAFJrS/DP7\ngdqvcZekbRaQmNXsK0CUjw5n2pDdgiu4XfN+FL0RN6nuC1ZRw3zNM6Y0qynib697\neFfhCIv9u/9vLexDNm0o\n-----END CERTIFICATE-----".to_string(),
            servers_endpoint: "http://servers.192.168.2.41.veronymous.io/servers.json".to_string()
        }
    }
}
