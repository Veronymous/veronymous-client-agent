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
            token_endpoint: "https://token-service.192.168.2.41.veronymous.io".to_string(),
            // token_endpoint_ca: "-----BEGIN CERTIFICATE-----\nMIIDyzCCArOgAwIBAgIUANb3hm6n1wwhGkjB0XN2fctauGUwDQYJKoZIhvcNAQEL\nBQAwdTELMAkGA1UEBhMCQ0ExEDAOBgNVBAgMB09udGFyaW8xDzANBgNVBAcMBk90\ndGF3YTEhMB8GA1UECgwYSW50ZXJuZXQgV2lkZ2l0cyBQdHkgTHRkMSAwHgYDVQQD\nDBdsb2NhbGhvc3QudmVyb255bW91cy5pbzAeFw0yMjEyMjExMzI3NDFaFw0yNzEy\nMjAxMzI3NDFaMHUxCzAJBgNVBAYTAkNBMRAwDgYDVQQIDAdPbnRhcmlvMQ8wDQYD\nVQQHDAZPdHRhd2ExITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDEg\nMB4GA1UEAwwXbG9jYWxob3N0LnZlcm9ueW1vdXMuaW8wggEiMA0GCSqGSIb3DQEB\nAQUAA4IBDwAwggEKAoIBAQCxx0+i60ptd2flxcBw+OpQM2oBm/riL0wGqOWc6j2F\nhEDJkfjcK4Fcc+8hcyGNNy11f2l59yuCY7wJhyZPXhyXi0lrkN328hPo19rYzYze\n83AQYKcq9XucAGbv9kRRSVyyeKu45DqSinClgfZzgB6qRNMB8yZl7cqhVwjLpa47\nVUH4zhDHYfKfH8cBMXGlW2gPexJWqGeusXhuXCd8dHoCzzGr6+NCxkzffpsLI3FN\nLPNXPaq8cYynyi/tO4A3QX6gTOCmKnwlNtZTpHUBy4BKV2HZ4XRVojfH+lOuylL3\nqgzYkQWsqaizZEIzlg5iEh4py50HsTq/JOXpXgfD7eadAgMBAAGjUzBRMB0GA1Ud\nDgQWBBT1Rui71l7VsTyoZvYmkSOTxZFz8TAfBgNVHSMEGDAWgBT1Rui71l7VsTyo\nZvYmkSOTxZFz8TAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUAA4IBAQAl\n+zDuALuo50w4PClws1ZGRGVYZQqgDKU32oR1zo+rSGbrqcO5yH2aanCeOX5oIJqC\nC1VPyjAbZ6x8kUTfzp+OtT2J3RJTA/jTaP2opR9QHZZ+uYQkalZky/djjsNNw2+X\nvlw2UZ+OfZI/hVEArEo7tc+qUvzcdhbthJOtSFhcQaY04Jd659Cj4svsZm8Jui+v\ngjZpJE1Ezp2hVVMAU7zO1Joe/CqcUnbpQXCPdZ0Wk2XxDwSXKtgY3VyAFJrS/DP7\ngdqvcZekbRaQmNXsK0CUjw5n2pDdgiu4XfN+FL0RN6nuC1ZRw3zNM6Y0qynib697\neFfhCIv9u/9vLexDNm0o\n-----END CERTIFICATE-----".to_string(),
            token_endpoint_ca: "-----BEGIN CERTIFICATE-----\nMIIEPTCCAyWgAwIBAgIUMtcvG69O61fUIz0bbv97vK9oW6kwDQYJKoZIhvcNAQEL\nBQAwga0xCzAJBgNVBAYTAkNBMRAwDgYDVQQIDAdPbnRhcmlvMQ8wDQYDVQQHDAZP\ndHRhd2ExJDAiBgNVBAoMG1Zlcm9ueW1vdXMgVGVjaG5vbG9naWVzIEluYzEUMBIG\nA1UECwwLRGV2ZWxvcG1lbnQxGjAYBgNVBAMMEWRldi52ZXJvbnltb3VzLmlvMSMw\nIQYJKoZIhvcNAQkBFhRuYm91bWFAdmVyb255bW91cy5pbzAeFw0yMjEyMDgxMTMz\nNDFaFw0yNzEyMDcxMTMzNDFaMIGtMQswCQYDVQQGEwJDQTEQMA4GA1UECAwHT250\nYXJpbzEPMA0GA1UEBwwGT3R0YXdhMSQwIgYDVQQKDBtWZXJvbnltb3VzIFRlY2hu\nb2xvZ2llcyBJbmMxFDASBgNVBAsMC0RldmVsb3BtZW50MRowGAYDVQQDDBFkZXYu\ndmVyb255bW91cy5pbzEjMCEGCSqGSIb3DQEJARYUbmJvdW1hQHZlcm9ueW1vdXMu\naW8wggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQC5DMCIXm8A6xuMaQof\nJr0f2u27HcKhCwr1ch83HtY1+YB6x+7l+ALdsjE7+Ifb3h0p25t4kwpUYj7kyI+k\nqKzaWwZI2SpiyC8ComNO8KI6fRs8rxgxHI9PeF2J7TwiJ7KxdTqhJ/avGkBmDnOt\n6nXo7m/szakMY3EzywFtAeKaV74QFcVKWLdvC6DnwvXxIV7VAG5odfDPMoZbK/Um\nG5I4IXGbSJ9dOjeEnZZbDa6lOsv3vkvRbgWaa5aWGPkKbPy6Jq4qQuKuIqJQB2eX\n5xzS0fV9U8GTOHNymFdMS/f3KMSGp0e7ATod3E8QJEHA761FvkC2rttPlKma7Km9\n+B1rAgMBAAGjUzBRMB0GA1UdDgQWBBT5a9ZBITxCBAa6JGhqgHx6WiJvITAfBgNV\nHSMEGDAWgBT5a9ZBITxCBAa6JGhqgHx6WiJvITAPBgNVHRMBAf8EBTADAQH/MA0G\nCSqGSIb3DQEBCwUAA4IBAQAFn3Wrc/Mj+OJEq8Nr5VOzDveNjzj2an4qZjtwP5lt\n6XOPBNFAFwjd9Cncby6maFNwfTwluPOmP0fcbXh5/hKJtd5FY1kzHcx64rlN0vNJ\n1BleDCNDq5pQfVs+mCm4+SlruqTzeKSnUZvcB0valEWSL/5ApjSdq9112USQHLXn\nIKx/xHR1TWI/NcQ99ONdjMC1YH4EfciwpQDl1UHhSLu+xzxbpwTGxIiZwyvqAhHt\nl7WEy76k+nrcUg/AdUHqg1zoxWam2V7ONuGVnYW78NhloKmtUFLb9/JkxN63xLt3\nq5PoTRvpiVzN3kKHEq3BeafutFTqfBiu6rl9gHKVD3ER\n-----END CERTIFICATE-----".to_string(),
            servers_endpoint: "http://servers.192.168.2.41.veronymous.io/servers.json".to_string()
        }
    }
}
