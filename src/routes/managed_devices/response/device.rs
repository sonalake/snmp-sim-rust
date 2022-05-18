use crate::domain;
use crate::routes::agents::response::Agent;
use crate::routes::SnmpProtocolAttributes;
use paperclip::actix::Apiv2Schema;
use uuid_dev::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Apiv2Schema)]
#[openapi(rename = "ResponseDevice")]
/// An managed device as a response body.
pub struct Device {
    /// The unique identifier of this managed device.
    pub id: Uuid,

    /// Device's name.
    pub name: String,

    /// Device's optional description.
    pub description: Option<String>,

    pub agent: Agent,

    pub snmp_host: String,

    pub snmp_port: u16,

    pub snmp_protocol_attributes: SnmpProtocolAttributes,
}

impl From<domain::ManagedDevice> for Device {
    fn from(managed_device: domain::ManagedDevice) -> Self {
        let agent: Agent = match managed_device.agent {
            domain::ManagedDeviceAgent::Agent(agent) => agent.into(),
            _ => panic!(),
        };
        Self {
            id: managed_device.id,
            name: managed_device.name.clone(),
            description: managed_device.description.clone(),
            agent,
            snmp_protocol_attributes: managed_device.snmp_protocol_attributes.into(),
            snmp_host: managed_device.snmp_host,
            snmp_port: managed_device.snmp_port,
        }
    }
}
