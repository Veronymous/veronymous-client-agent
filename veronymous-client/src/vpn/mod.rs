use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VpnProfile {
    pub domain: String,

    // pub agent_endpoint: Endpoint,
    pub agent_endpoint: String,

    pub root_cert: Vec<u8>,

    // Wireguard endpoint
    pub wg_endpoint: String,

    // Wireguard server public key
    pub wg_key: String,
}

impl VpnProfile {
    pub fn new(
        domain: String,
        agent_endpoint: String,
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

#[cfg(test)]
mod tests {
    use crate::vpn::VpnProfile;

    #[test]
    fn test_vpn_profile_json() {
        let vpn_profile = vpn_profile();

        let vpn_profile_json = serde_json::to_string(&vpn_profile).unwrap();

        let vpn_profile_parsed: VpnProfile =
            serde_json::from_str(vpn_profile_json.as_str()).unwrap();

        assert_eq!(vpn_profile, vpn_profile_parsed)
    }

    fn vpn_profile() -> VpnProfile {
        let root_cert = "-----BEGIN CERTIFICATE-----
MIID0TCCArmgAwIBAgIUCVuNppf++HHklyxMgrGWTPNTKMgwDQYJKoZIhvcNAQEL
BQAweDELMAkGA1UEBhMCQ0ExEDAOBgNVBAgMB09udGFyaW8xDzANBgNVBAcMBk90
dGF3YTEkMCIGA1UECgwbVmVyb255bW91cyBUZWNobm9sb2dpZXMgSW5jMSAwHgYD
VQQDDBdWZXJvbnltb3VzIFRlY2hub2xvZ2llczAeFw0yMjA4MTExMzIwNTJaFw0y
NzA4MTAxMzIwNTJaMHgxCzAJBgNVBAYTAkNBMRAwDgYDVQQIDAdPbnRhcmlvMQ8w
DQYDVQQHDAZPdHRhd2ExJDAiBgNVBAoMG1Zlcm9ueW1vdXMgVGVjaG5vbG9naWVz
IEluYzEgMB4GA1UEAwwXVmVyb255bW91cyBUZWNobm9sb2dpZXMwggEiMA0GCSqG
SIb3DQEBAQUAA4IBDwAwggEKAoIBAQCw1TQI538AkY5IEC2FTzhb4/ZtErWAAKAZ
U744847DN0xHzeXMSBFm4RmVi5j8bOAMhnV11tqh5XfbhHGmEBO9i85XJoXUuGes
MUIIMna3eHS889SeIp0xo0TBtVHUwuvAofE4vts6/ZI6Ip5sHSEXl41n93VkwUPO
1E+0TBZVy+3pvguzbQa/tjgXDyYwv03Sy1JQQXUEHOVghpRdc+tL+GzzhXLoVmAr
07vBj5cnICCN7g/WkJbhoi7WxGUxkjNX3ibkmQjxTTSsNnbQ3fvAY8lKmkt6uPzz
Yt/Xyx/Is0f58FxFoiGYyTvqi4ShXb614VhUg43kCMwmKPd86YWTAgMBAAGjUzBR
MB0GA1UdDgQWBBTMGM+KXB5CpEeAZwSakqvvb9P8pDAfBgNVHSMEGDAWgBTMGM+K
XB5CpEeAZwSakqvvb9P8pDAPBgNVHRMBAf8EBTADAQH/MA0GCSqGSIb3DQEBCwUA
A4IBAQBmJWu/5nCGZbxkVBTxhdhyXiXn5xJbssYOdgfCnIl8fnMgDJjSjBlmGrc4
JpJ6EKjsvGdeVdOQ86Up8SQR7gHrW2MeWcTejNDwsmBGKdQDh/U1mozwoFnMX7oH
gz6Gqz8T5XSGTfbxNwQPBDp3fDNaISgEM7DeZhxBR10oSkwa0c6WA1HkyBUTbNn9
2xdjDAr1Uj70P05qwyWlSHdBXYBaWOEYe0jkCKxR5xwF4+lYtmyYmpQqVxCojk8p
WWkQ8PIncGsunqvO9NW+ShTkrYR3NNa4yDTVcqy9Us1bycnCfpTnYIRt4BWH1IIj
wIieRFPJFKt7IAQE8g3/2VF12EeS
-----END CERTIFICATE-----";

        VpnProfile {
            domain: "dev_domain".to_string(),
            agent_endpoint: "localhost.veronymous.io:7777".to_string(),
            root_cert: root_cert.as_bytes().into(),
            wg_endpoint: "wg1.ny.veronymous.io:51820".to_string(),
            wg_key: "/ZjSUjxcDiHHxBifHX0yVekKklDmczNv8k7M3AgmXXg=".to_string(),
        }
    }
}
