use crate::routes::{first, twenty, DeviceError, SnmpProtocolAttributes};
use paperclip::actix::Apiv2Schema;
use serde::Deserialize;
use std::convert::TryFrom;
use uuid_dev::Uuid;

#[derive(Debug, Deserialize, Apiv2Schema)]
#[openapi(rename = "RequestDevice")]
/// An agent as a response body.
pub struct Device {
    /// The name of this agent.
    name: String,

    description: Option<String>,

    agent_id: Uuid,

    snmp_protocol_attributes: SnmpProtocolAttributes,

    snmp_host: String,

    snmp_port: u16,
}

#[derive(Debug, Deserialize, Apiv2Schema)]
pub struct GetDevicesQuery {
    #[serde(default = "first")]
    /// Page index starts from zero, default value is 1.
    pub page: Option<usize>,

    /// Number of results on a page, default value is 20.
    #[serde(default = "twenty")]
    pub page_size: Option<usize>,
}

impl TryFrom<Device> for crate::domain::ManagedDevice {
    type Error = DeviceError;

    fn try_from(managed_device: Device) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            name: managed_device.name,
            description: managed_device.description,
            agent: crate::domain::ManagedDeviceAgent::Id(managed_device.agent_id),
            snmp_protocol_attributes: crate::domain::SnmpProtocolAttributes::try_from(
                managed_device.snmp_protocol_attributes,
            )?,
            snmp_host: managed_device.snmp_host,
            snmp_port: managed_device.snmp_port,
        })
    }
}

impl TryFrom<(Uuid, Device)> for crate::domain::ManagedDevice {
    type Error = DeviceError;

    fn try_from((id, managed_device): (Uuid, Device)) -> Result<Self, Self::Error> {
        Ok(Self {
            id,
            name: managed_device.name,
            description: managed_device.description,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            agent: crate::domain::ManagedDeviceAgent::Id(managed_device.agent_id),
            snmp_protocol_attributes: crate::domain::SnmpProtocolAttributes::try_from(
                managed_device.snmp_protocol_attributes,
            )?,
            snmp_host: managed_device.snmp_host,
            snmp_port: managed_device.snmp_port,
        })
    }
}
