pub mod agents;
pub mod managed_devices;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) use agents::*;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) use managed_devices::*;

impl From<crate::data_access::entity::agents::ActiveModel> for crate::data_access::entity::agents::Model {
    fn from(am: crate::data_access::entity::agents::ActiveModel) -> Self {
        Self {
            id: am.id.unwrap(),
            created_at: am.created_at.unwrap(),
            modified_at: am.modified_at.unwrap(),
            name: am.name.unwrap(),
            description: am.description.unwrap(),
            snmp_data_url: am.snmp_data_url.unwrap(),
        }
    }
}

impl From<crate::data_access::entity::managed_devices::ActiveModel>
    for crate::data_access::entity::managed_devices::Model
{
    fn from(am: crate::data_access::entity::managed_devices::ActiveModel) -> Self {
        Self {
            id: am.id.unwrap(),
            created_at: am.created_at.unwrap(),
            modified_at: am.modified_at.unwrap(),
            name: am.name.unwrap(),
            description: am.description.unwrap(),
            agent_id: am.agent_id.unwrap(),
            snmp_protocol_attributes: am.snmp_protocol_attributes.unwrap(),
        }
    }
}
