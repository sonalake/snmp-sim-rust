use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum SnmpProtocolAttributes {
    SnmpV1(SnmpV1Attributes),
    SnmpV2c(SnmpV2cAttributes),
    SnmpV3(SnmpV3Attributes),
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct SnmpV1Attributes {
    pub community: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct SnmpV2cAttributes {
    pub community: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct SnmpV3Attributes {
    pub user: String,
    pub authentication: AuthenticationAlgorithm,
    pub authentication_password: String,
    pub encryption: EncryptionAlgorithm,
    pub encryption_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum AuthenticationAlgorithm {
    Md5,
    Sha,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum EncryptionAlgorithm {
    Des,
    Aes,
}
