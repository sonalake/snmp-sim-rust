use crate::domain;
use paperclip::actix::Apiv2Schema;
use uuid_dev::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Apiv2Schema)]
#[openapi(rename = "ResponseAgent")]
/// An agent as a response body.
pub struct Agent {
    /// The unique identifier of this agent.
    pub id: Uuid,

    /// Agent's name.
    pub name: String,

    /// Agent's optional description.
    pub description: Option<String>,

    /// The URL to the SNMP data f.e. "file://./os/linux.dat"
    pub snmp_data_url: String,
}

impl From<crate::domain::Agent> for Agent {
    fn from(agent: crate::domain::Agent) -> Self {
        Self {
            id: agent.id,
            name: agent.name,
            description: agent.description,
            snmp_data_url: agent.snmp_data_url,
        }
    }
}

impl From<&domain::Agent> for Agent {
    fn from(agent: &domain::Agent) -> Self {
        Self {
            id: agent.id,
            name: agent.name.clone(),
            description: agent.description.clone(),
            snmp_data_url: agent.snmp_data_url.clone(),
        }
    }
}
