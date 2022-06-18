mod snmp_v1;
mod snmp_v2;

// #[cfg_attr(feature = "integration-tests", visibility::make(pub))]
// pub(crate) use snmp_v1::*;

// #[cfg_attr(feature = "integration-tests", visibility::make(pub))]
// pub(crate) use snmp_v2::*;

use crate::snmp::codec::generic_snmp_message::GenericSnmpMessage;
use crate::snmp::codec::snmp_codec::SnmpCodec;
use bytes::Bytes;
use rasn::prelude::{ObjectIdentifier, OctetString};
use snmp_data_parser::parser::snmp_data::component::DataType;

#[inline]
pub fn to_string_default(bytes: &OctetString, default: &str) -> String {
    std::str::from_utf8(bytes).unwrap_or(default).to_string()
}

//#[inline]
pub fn try_to_i32<T: num_traits::ToPrimitive + std::fmt::Display>(value: &T) -> Result<i32, ValidationError> {
    value
        .to_i32()
        .ok_or_else(|| ValidationError::Invalid(format!("request_id is not i32 {}", value)))
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("{0}")]
    Invalid(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(strum_macros::Display, Debug)]
pub enum SnmpProtocolVersion {
    SNMPV1(String),
    SNMPV2C(String),
    //    SNMPV3,
}

#[derive(strum_macros::Display, Debug)]
pub enum ErrorStatus {
    NoError = 0,
    TooBig = 1,
    NoSuchName = 2,
    BadValue = 3,
    ReadOnly = 4,
    GenErr = 5,
}

#[derive(Debug)]
pub struct Variable {
    pub name: ObjectIdentifier,
    pub data_type: DataType,
    pub value: String,
}

#[derive(Debug)]
pub struct GetRequest {
    pub request_id: i32,
    pub objects: Vec<ObjectIdentifier>,
}

#[derive(Debug)]
pub struct GetNextRequest {
    pub request_id: i32,
    pub objects: Vec<ObjectIdentifier>,
}

#[derive(Debug)]
pub struct GetResponse {
    pub request_id: i32,
    pub variable_values: Vec<Variable>,
}

#[derive(Debug)]
pub struct GetResponseError {
    pub request_id: i32,
    pub error_status: ErrorStatus,
    pub error_index: usize,
    pub name: Option<ObjectIdentifier>,
}

impl From<(&SnmpProtocolVersion, GetRequest)> for GenericSnmpMessage {
    fn from((protocol_version, request): (&SnmpProtocolVersion, GetRequest)) -> Self {
        match protocol_version {
            SnmpProtocolVersion::SNMPV1(community) => GenericSnmpMessage::V1Message(rasn_snmp::v1::Message {
                version: SnmpCodec::SNMP_VERSION1.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: request.into(),
            }),
            SnmpProtocolVersion::SNMPV2C(community) => GenericSnmpMessage::V2Message(rasn_snmp::v2c::Message {
                version: SnmpCodec::SNMP_VERSION2.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: request.into(),
            }),
        }
    }
}

impl From<(&SnmpProtocolVersion, GetNextRequest)> for GenericSnmpMessage {
    fn from((protocol_version, request): (&SnmpProtocolVersion, GetNextRequest)) -> Self {
        match protocol_version {
            SnmpProtocolVersion::SNMPV1(community) => GenericSnmpMessage::V1Message(rasn_snmp::v1::Message {
                version: SnmpCodec::SNMP_VERSION1.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: request.into(),
            }),
            SnmpProtocolVersion::SNMPV2C(community) => GenericSnmpMessage::V2Message(rasn_snmp::v2c::Message {
                version: SnmpCodec::SNMP_VERSION2.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: request.into(),
            }),
        }
    }
}

impl From<(&SnmpProtocolVersion, GetResponse)> for GenericSnmpMessage {
    fn from((protocol_version, response): (&SnmpProtocolVersion, GetResponse)) -> Self {
        match protocol_version {
            SnmpProtocolVersion::SNMPV1(community) => GenericSnmpMessage::V1Message(rasn_snmp::v1::Message {
                version: SnmpCodec::SNMP_VERSION1.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: response.into(),
            }),
            SnmpProtocolVersion::SNMPV2C(community) => GenericSnmpMessage::V2Message(rasn_snmp::v2c::Message {
                version: SnmpCodec::SNMP_VERSION2.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: response.into(),
            }),
        }
    }
}

impl From<(&SnmpProtocolVersion, GetResponseError)> for GenericSnmpMessage {
    fn from((protocol_version, response): (&SnmpProtocolVersion, GetResponseError)) -> Self {
        match protocol_version {
            SnmpProtocolVersion::SNMPV1(community) => GenericSnmpMessage::V1Message(rasn_snmp::v1::Message {
                version: SnmpCodec::SNMP_VERSION1.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: response.into(),
            }),
            SnmpProtocolVersion::SNMPV2C(community) => GenericSnmpMessage::V2Message(rasn_snmp::v2c::Message {
                version: SnmpCodec::SNMP_VERSION2.into(),
                community: Bytes::from(community.as_bytes().to_vec()),
                data: response.into(),
            }),
        }
    }
}
