use crate::data_access::entity::agents::Model as AgentsModel;
use crate::data_access::entity::managed_devices::{ActiveModel, Model};
use crate::domain::Agent;
use crate::domain::SnmpProtocolAttributes;
use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;
use std::str::FromStr;
use uuid_dev::Uuid;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(Debug, Clone)]
pub(crate) struct ManagedDevice {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub agent: ManagedDeviceAgent,
    pub snmp_protocol_attributes: SnmpProtocolAttributes,
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(Debug, Clone)]
pub(crate) enum ManagedDeviceAgent {
    Id(Uuid),
    Agent(Agent),
}

impl ManagedDevice {
    pub fn agent_id(&self) -> &Uuid {
        match &self.agent {
            ManagedDeviceAgent::Id(id) => id,
            ManagedDeviceAgent::Agent(agent) => &agent.id,
        }
    }
}

impl From<(Model, Option<AgentsModel>)> for ManagedDevice {
    fn from((model, agent): (Model, Option<AgentsModel>)) -> ManagedDevice {
        Self {
            id: Uuid::from_str(&model.id).unwrap(),
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            modified_at: model.modified_at,
            agent: ManagedDeviceAgent::Agent(agent.unwrap().into()),
            snmp_protocol_attributes: serde_json::from_str(&model.snmp_protocol_attributes).unwrap(),
        }
    }
}

impl From<(Model, Vec<AgentsModel>)> for ManagedDevice {
    fn from((model, agent): (Model, Vec<AgentsModel>)) -> ManagedDevice {
        Self {
            id: Uuid::from_str(&model.id).unwrap(),
            name: model.name,
            description: model.description,
            created_at: model.created_at,
            modified_at: model.modified_at,
            agent: ManagedDeviceAgent::Agent(agent.first().unwrap().clone().into()),
            snmp_protocol_attributes: serde_json::from_str(&model.snmp_protocol_attributes).unwrap(),
        }
    }
}

impl From<(ActiveModel, Option<AgentsModel>)> for ManagedDevice {
    fn from((am, agent): (ActiveModel, Option<AgentsModel>)) -> Self {
        Self {
            id: Uuid::from_str(&am.id.unwrap()).unwrap(),
            name: am.name.unwrap(),
            description: am.description.unwrap(),
            created_at: am.created_at.unwrap(),
            modified_at: am.modified_at.unwrap(),
            agent: ManagedDeviceAgent::Agent(agent.unwrap().into()),
            snmp_protocol_attributes: serde_json::from_str(&am.snmp_protocol_attributes.unwrap()).unwrap(),
        }
    }
}

impl From<ManagedDevice> for Model {
    fn from(managed_device: ManagedDevice) -> Self {
        let agent_id = managed_device.agent_id().to_string();
        Self {
            id: managed_device.id.to_string(),
            created_at: managed_device.created_at,
            modified_at: managed_device.modified_at,
            name: managed_device.name,
            description: managed_device.description,
            agent_id,
            snmp_protocol_attributes: serde_json::to_string(&managed_device.snmp_protocol_attributes).unwrap(),
        }
    }
}

impl From<ManagedDevice> for ActiveModel {
    fn from(managed_device: ManagedDevice) -> Self {
        let agent_id = managed_device.agent_id().to_string();
        Self {
            id: ActiveValue::set(managed_device.id.to_string()),
            created_at: ActiveValue::set(managed_device.created_at),
            modified_at: ActiveValue::set(managed_device.modified_at),
            name: ActiveValue::set(managed_device.name),
            description: ActiveValue::set(managed_device.description),
            agent_id: ActiveValue::set(agent_id),
            snmp_protocol_attributes: ActiveValue::set(
                serde_json::to_string(&managed_device.snmp_protocol_attributes).unwrap(),
            ),
        }
    }
}
