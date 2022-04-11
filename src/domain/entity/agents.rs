use crate::data_access::entity::agents::{ActiveModel, Model};
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::ActiveValue;
use uuid_dev::Uuid;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub name: String,
}

impl From<Model> for Agent {
    fn from(model: Model) -> Agent {
        Self {
            id: Uuid::from_slice(&model.id).unwrap(),
            name: model.name,
            created_at: model
                .created_at
                .parse::<DateTime<FixedOffset>>()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
            modified_at: model
                .modified_at
                .parse::<DateTime<FixedOffset>>()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
        }
    }
}

impl From<ActiveModel> for Agent {
    fn from(am: ActiveModel) -> Self {
        Self {
            id: Uuid::from_slice(&am.id.unwrap()).unwrap(),
            name: am.name.unwrap(),
            created_at: am
                .created_at
                .unwrap()
                .parse::<DateTime<FixedOffset>>()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
            modified_at: am
                .modified_at
                .unwrap()
                .parse::<DateTime<FixedOffset>>()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
        }
    }
}

impl From<Agent> for Model {
    fn from(agent: Agent) -> Self {
        Self {
            id: agent.id.as_bytes().to_vec(),
            created_at: agent.created_at.to_string(),
            modified_at: agent.modified_at.to_string(),
            name: agent.name,
        }
    }
}

impl From<Agent> for ActiveModel {
    fn from(agent: Agent) -> Self {
        Self {
            id: ActiveValue::set(agent.id.as_bytes().to_vec()),
            created_at: ActiveValue::set(agent.created_at.to_string()),
            modified_at: ActiveValue::set(agent.modified_at.to_string()),
            name: ActiveValue::set(agent.name),
        }
    }
}