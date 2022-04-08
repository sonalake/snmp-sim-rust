use crate::domain;
use paperclip::actix::Apiv2Schema;
use uuid_dev::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Apiv2Schema)]
#[openapi(rename = "ResponseAgent")]
/// An agent as a response body.
pub struct Agent {
    /// The unique identifier of this agent.
    pub id: Uuid,

    /// The name of this agent.
    pub name: String,
}

impl From<crate::domain::Agent> for Agent {
    fn from(agent: crate::domain::Agent) -> Self {
        Self {
            id: agent.id,
            name: agent.name,
        }
    }
}

impl From<&domain::Agent> for Agent {
    fn from(agent: &domain::Agent) -> Self {
        Self {
            id: agent.id,
            name: agent.name.clone(),
        }
    }
}
