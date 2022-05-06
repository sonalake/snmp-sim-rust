use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Apiv2Schema, Eq, PartialEq)]
pub enum SnmpProtocolVersion {
    SnmpV1,
    SnmpV2c,
    SnmpV3,
}

impl From<&crate::domain::SnmpProtocolVersion> for SnmpProtocolVersion {
    fn from(spv: &crate::domain::SnmpProtocolVersion) -> Self {
        match spv {
            crate::domain::SnmpProtocolVersion::SnmpV1 => SnmpProtocolVersion::SnmpV1,
            crate::domain::SnmpProtocolVersion::SnmpV2c => SnmpProtocolVersion::SnmpV2c,
            crate::domain::SnmpProtocolVersion::SnmpV3 => SnmpProtocolVersion::SnmpV3,
        }
    }
}

impl From<&SnmpProtocolVersion> for crate::domain::SnmpProtocolVersion {
    fn from(spv: &SnmpProtocolVersion) -> Self {
        match spv {
            SnmpProtocolVersion::SnmpV1 => crate::domain::SnmpProtocolVersion::SnmpV1,
            SnmpProtocolVersion::SnmpV2c => crate::domain::SnmpProtocolVersion::SnmpV2c,
            SnmpProtocolVersion::SnmpV3 => crate::domain::SnmpProtocolVersion::SnmpV3,
        }
    }
}
