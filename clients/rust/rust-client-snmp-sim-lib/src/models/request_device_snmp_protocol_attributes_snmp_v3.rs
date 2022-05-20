/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestDeviceSnmpProtocolAttributesSnmpV3 {
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "authentication_password")]
    pub authentication_password: String,
    #[serde(rename = "encryption", skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "encryption_key")]
    pub encryption_key: String,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl RequestDeviceSnmpProtocolAttributesSnmpV3 {
    pub fn new(authentication_password: String, encryption_key: String) -> RequestDeviceSnmpProtocolAttributesSnmpV3 {
        RequestDeviceSnmpProtocolAttributesSnmpV3 {
            authentication: None,
            authentication_password,
            encryption: None,
            encryption_key,
            user: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Authentication {
    #[serde(rename = "MD5")]
    MD5,
    #[serde(rename = "SHA")]
    SHA,
}

impl Default for Authentication {
    fn default() -> Authentication {
        Self::MD5
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Encryption {
    #[serde(rename = "DES")]
    DES,
    #[serde(rename = "AES")]
    AES,
}

impl Default for Encryption {
    fn default() -> Encryption {
        Self::DES
    }
}

