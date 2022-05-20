use crate::data_access::entity::agents::{Entity as Agents, Model as AgentsModel};
use crate::data_access::entity::managed_devices::{
    ActiveModel as DevicesActiveModel, Entity as ManagedDevices, Model as DevicesModel,
};
use crate::data_access::helpers::get_agent;
use crate::domain::CreateResult;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait, DbErr, Delete, DeleteResult, EntityTrait};
use uuid_dev::Uuid;

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(level = "debug", name = "[DA] Create a new instance of managed device", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn create_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
    managed_device_name: &str,
    description: &Option<String>,
    agent_id: &Uuid,
    snmp_protocol_attributes: &str,
    snmp_host: &str,
    snmp_port: u16,
) -> Result<CreateResult<(DevicesModel, Option<AgentsModel>)>, DbErr> {
    let result = ManagedDevices::find_by_id(id.to_string())
        .find_with_related(Agents)
        .one(conn)
        .await?;

    if let Some(result) = result {
        return Ok(CreateResult::Duplicate(result));
    }

    let mut managed_device = DevicesActiveModel {
        id: ActiveValue::set(id.to_string()),
        created_at: ActiveValue::set(chrono::Utc::now()),
        modified_at: ActiveValue::set(chrono::Utc::now()),
        name: ActiveValue::set(managed_device_name.to_string()),
        description: ActiveValue::set(description.clone()),
        agent_id: ActiveValue::set(agent_id.to_string()),
        snmp_protocol_attributes: ActiveValue::set(snmp_protocol_attributes.to_string()),
        snmp_host: ActiveValue::set(snmp_host.to_string()),
        snmp_port: ActiveValue::set(snmp_port.into()),
    };

    let insert_result = ManagedDevices::insert(managed_device.clone())
        .exec(conn)
        .await
        .map_err(|e| DbErr::Custom(format!("CONFLICT, error={}", e)))?;
    managed_device.id = ActiveValue::set(insert_result.last_insert_id);
    let agent = get_agent(conn, agent_id).await?;
    Ok(CreateResult::Created((managed_device.into(), agent)))
}

#[tracing::instrument(level = "debug", name = "[DA] Finding a managed device", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn get_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<Option<(DevicesModel, Option<AgentsModel>)>, DbErr> {
    ManagedDevices::find_by_id(id.to_string())
        .find_with_related(Agents)
        .one(conn)
        .await
}

#[tracing::instrument(level = "debug", name = "[DA] Deleting a managed device", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn delete_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<DeleteResult, DbErr> {
    Delete::one(DevicesActiveModel {
        id: ActiveValue::set(id.to_string()),
        // keep the default here, since we want to delete the entity by key only, all the resut of the fields are unset
        ..Default::default()
    })
    .exec(conn)
    .await
}

#[tracing::instrument(level = "debug", name = "[DA] Listing managed devices", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn list_managed_devices<'db>(
    conn: &'db impl ConnectionTrait,
    page: usize,
    page_size: usize,
) -> Result<Vec<(DevicesModel, Vec<AgentsModel>)>, DbErr> {
    //let paginator =
    ManagedDevices::find()
        .find_with_related(Agents)
        .all(conn)
        .await
    //     .paginate(conn, page_size);
    // paginator.fetch_page(page - 1).await
}

#[allow(clippy::too_many_arguments)]
#[tracing::instrument(level = "debug", name = "[DA] Updating managed device", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn update_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
    managed_device_name: &str,
    managed_device_description: &Option<String>,
    agent_id: &Uuid,
    snmp_protocol_attributes: &str,
    snmp_host: &str,
    snmp_port: u16,
) -> Result<(DevicesModel, Option<AgentsModel>), DbErr> {
    let am: Option<DevicesModel> = ManagedDevices::find_by_id(id.to_string()).one(conn).await?;
    let mut managed_device: DevicesActiveModel = am.unwrap().into();

    managed_device.name = ActiveValue::set(managed_device_name.to_string());
    managed_device.description = ActiveValue::set(managed_device_description.clone());
    managed_device.agent_id = ActiveValue::set(agent_id.to_string());
    managed_device.snmp_protocol_attributes = ActiveValue::set(snmp_protocol_attributes.to_string());
    managed_device.snmp_host = ActiveValue::set(snmp_host.to_string());
    managed_device.snmp_port = ActiveValue::set(snmp_port.into());

    let device: DevicesModel = managed_device.clone().into();

    managed_device.update(conn).await?;
    let agent = get_agent(conn, agent_id).await?;

    Ok((device, agent))
}
