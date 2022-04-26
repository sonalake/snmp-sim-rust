use crate::routes::AgentError;
use crate::routes::{first, twenty};
use paperclip::actix::Apiv2Schema;
use serde::Deserialize;
use std::convert::TryFrom;
use uuid_dev::Uuid;

#[derive(Debug, serde::Deserialize, Apiv2Schema)]
#[openapi(rename = "RequestAgent")]
/// An agent as a response body.
pub struct Agent {
    /// The name of this agent.
    name: String,

    /// The URL to the SNMP data f.e. "file://./os/linux.dat"
    snmp_data_url: String,

    description: Option<String>,
}

#[derive(Debug, Deserialize, Apiv2Schema)]
pub struct GetAgentsQuery {
    #[serde(default = "first")]
    /// Page index starts from zero, default value is 1.
    pub page: Option<usize>,

    /// Number of results on a page, default value is 20.
    #[serde(default = "twenty")]
    pub page_size: Option<usize>,
}

impl TryFrom<Agent> for crate::domain::Agent {
    type Error = AgentError;

    fn try_from(value: Agent) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            name: value.name,
            description: value.description,
            snmp_data_url: value.snmp_data_url,
        })
    }
}

impl TryFrom<(Uuid, Agent)> for crate::domain::Agent {
    type Error = AgentError;

    fn try_from((id, value): (Uuid, Agent)) -> Result<Self, Self::Error> {
        Ok(Self {
            id,
            name: value.name,
            description: value.description,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            snmp_data_url: value.snmp_data_url,
        })
    }
}
