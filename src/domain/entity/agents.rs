use crate::data_access::entity::agents::{ActiveModel, Model};
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use std::str::FromStr;
use uuid_dev::Uuid;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub name: String,
    pub snmp_data_url: String,
    pub description: Option<String>,
}

impl From<Model> for Agent {
    fn from(model: Model) -> Agent {
        Self {
            id: Uuid::from_str(&model.id).unwrap(),
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            modified_at: model.modified_at,
            snmp_data_url: model.snmp_data_url,
        }
    }
}

impl From<ActiveModel> for Agent {
    fn from(am: ActiveModel) -> Self {
        Self {
            id: Uuid::from_str(&am.id.unwrap()).unwrap(),
            name: am.name.unwrap(),
            description: am.description.unwrap(),
            created_at: am.created_at.unwrap(),
            modified_at: am.modified_at.unwrap(),
            snmp_data_url: am.snmp_data_url.unwrap(),
        }
    }
}

impl From<Agent> for Model {
    fn from(agent: Agent) -> Self {
        Self {
            id: agent.id.to_string(),
            created_at: agent.created_at,
            modified_at: agent.modified_at,
            name: agent.name,
            description: agent.description,
            snmp_data_url: agent.snmp_data_url,
        }
    }
}

impl From<Agent> for ActiveModel {
    fn from(agent: Agent) -> Self {
        Self {
            id: ActiveValue::set(agent.id.to_string()),
            created_at: ActiveValue::set(agent.created_at),
            modified_at: ActiveValue::set(agent.modified_at),
            name: ActiveValue::set(agent.name),
            description: ActiveValue::set(agent.description),
            snmp_data_url: ActiveValue::set(agent.snmp_data_url),
        }
    }
}
