use crate::routes::DeviceError;
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
pub struct SnmpProtocolAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_v1: Option<SnmpV1Attributes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_v2c: Option<SnmpV2cAttributes>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_v3: Option<SnmpV3Attributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
pub struct SnmpV1Attributes {
    #[serde(default = "default_public")]
    pub community: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
pub struct SnmpV2cAttributes {
    #[serde(default = "default_public")]
    pub community: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
pub struct SnmpV3Attributes {
    #[serde(default = "default_public")]
    pub user: Option<String>,

    #[serde(default = "default_sha")]
    pub authentication: Option<AuthenticationAlgorithm>,

    pub authentication_password: String,

    #[serde(default = "default_aes")]
    pub encryption: Option<EncryptionAlgorithm>,

    pub encryption_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthenticationAlgorithm {
    Md5,
    Sha,
}

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EncryptionAlgorithm {
    Des,
    Aes,
}

fn default_public() -> Option<String> {
    Some("public".to_string())
}

fn default_sha() -> Option<AuthenticationAlgorithm> {
    Some(AuthenticationAlgorithm::Sha)
}

fn default_aes() -> Option<EncryptionAlgorithm> {
    Some(EncryptionAlgorithm::Aes)
}

impl From<crate::domain::SnmpProtocolAttributes> for SnmpProtocolAttributes {
    fn from(spa: crate::domain::SnmpProtocolAttributes) -> Self {
        match spa {
            crate::domain::SnmpProtocolAttributes::SnmpV1(attr) => SnmpProtocolAttributes {
                snmp_v1: Some(SnmpV1Attributes {
                    community: Some(attr.community),
                }),
                snmp_v2c: None,
                snmp_v3: None,
            },
            crate::domain::SnmpProtocolAttributes::SnmpV2c(attr) => SnmpProtocolAttributes {
                snmp_v1: None,
                snmp_v2c: Some(SnmpV2cAttributes {
                    community: Some(attr.community),
                }),
                snmp_v3: None,
            },
            crate::domain::SnmpProtocolAttributes::SnmpV3(attr) => SnmpProtocolAttributes {
                snmp_v1: None,
                snmp_v2c: None,
                snmp_v3: Some(SnmpV3Attributes {
                    user: Some(attr.user),
                    authentication: Some(attr.authentication.into()),
                    authentication_password: attr.authentication_password,
                    encryption: Some(attr.encryption.into()),
                    encryption_key: attr.encryption_key,
                }),
            },
        }
    }
}

impl TryFrom<SnmpProtocolAttributes> for crate::domain::SnmpProtocolAttributes {
    type Error = DeviceError;

    fn try_from(spa: SnmpProtocolAttributes) -> Result<Self, Self::Error> {
        match (spa.snmp_v1, spa.snmp_v2c, spa.snmp_v3) {
            (Some(attr), None, None) => Ok(crate::domain::SnmpProtocolAttributes::SnmpV1(
                crate::domain::SnmpV1Attributes {
                    community: attr.community.unwrap(),
                },
            )),
            (None, Some(attr), None) => Ok(crate::domain::SnmpProtocolAttributes::SnmpV2c(
                crate::domain::SnmpV2cAttributes {
                    community: attr.community.unwrap(),
                },
            )),
            (None, None, Some(attr)) => Ok(crate::domain::SnmpProtocolAttributes::SnmpV3(
                crate::domain::SnmpV3Attributes {
                    user: attr.user.unwrap(),
                    authentication: attr.authentication.unwrap().into(),
                    authentication_password: attr.authentication_password,
                    encryption: attr.encryption.unwrap().into(),
                    encryption_key: attr.encryption_key,
                },
            )),
            _ => Err(DeviceError::Validation(
                "exactly one of 'snmp_v1', 'snmp_v2c', 'snmp_v3' must be supplied in SnmpProtocolAttributes".into(),
            )),
        }
    }
}

impl From<crate::domain::AuthenticationAlgorithm> for AuthenticationAlgorithm {
    fn from(tt: crate::domain::AuthenticationAlgorithm) -> Self {
        match tt {
            crate::domain::AuthenticationAlgorithm::Md5 => Self::Md5,
            crate::domain::AuthenticationAlgorithm::Sha => Self::Sha,
        }
    }
}

impl From<AuthenticationAlgorithm> for crate::domain::AuthenticationAlgorithm {
    fn from(tt: AuthenticationAlgorithm) -> Self {
        match tt {
            AuthenticationAlgorithm::Md5 => Self::Md5,
            AuthenticationAlgorithm::Sha => Self::Sha,
        }
    }
}

impl From<crate::domain::EncryptionAlgorithm> for EncryptionAlgorithm {
    fn from(sa: crate::domain::EncryptionAlgorithm) -> Self {
        match sa {
            crate::domain::EncryptionAlgorithm::Des => Self::Des,
            crate::domain::EncryptionAlgorithm::Aes => Self::Aes,
        }
    }
}

impl From<EncryptionAlgorithm> for crate::domain::EncryptionAlgorithm {
    fn from(sa: EncryptionAlgorithm) -> Self {
        match sa {
            EncryptionAlgorithm::Des => Self::Des,
            EncryptionAlgorithm::Aes => Self::Aes,
        }
    }
}
