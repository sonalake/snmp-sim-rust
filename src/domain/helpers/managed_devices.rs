use crate::domain::{CreateResult, DomainError, ManagedDevice, UpdateResult};
use sea_orm::ConnectionTrait;
use uuid_dev::Uuid;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Create an instance of managed device", skip(conn))]
pub(crate) async fn create_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    managed_device: &ManagedDevice,
) -> Result<CreateResult<ManagedDevice>, DomainError> {
    let agent_id = managed_device.agent_id();
    let result = crate::data_access::helpers::create_managed_device(
        conn,
        &managed_device.id,
        &managed_device.name,
        &managed_device.description,
        agent_id,
        serde_json::to_string(&managed_device.snmp_protocol_attributes)
            .unwrap()
            .as_ref(),
    )
    .await
    .map_err(DomainError::from)?;

    Ok(result.map(|managed_device| managed_device.into()))
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Finding an managed device", skip(conn))]
pub(crate) async fn get_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<ManagedDevice, DomainError> {
    crate::data_access::helpers::get_managed_device(conn, id)
        .await?
        .map(|managed_device| managed_device.into())
        .ok_or_else(|| DomainError::NotFound(format!("ManagedDeviceId={} not exists", id)))
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Deleting an managed device", skip(conn))]
pub(crate) async fn delete_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<Option<ManagedDevice>, DomainError> {
    match crate::data_access::helpers::get_managed_device(conn, id).await? {
        Some(managed_device) => {
            crate::data_access::helpers::delete_managed_device(conn, id).await?;
            Ok(Some(ManagedDevice::from(managed_device)))
        }
        None => Ok(None),
    }
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Listing managed devices", skip(conn))]
pub(crate) async fn list_managed_devices<'db>(
    conn: &'db impl ConnectionTrait,
    page: usize,
    page_size: usize,
) -> Result<Vec<ManagedDevice>, DomainError> {
    let result = crate::data_access::helpers::list_managed_devices(conn, page, page_size).await?;

    Ok(result.into_iter().map(ManagedDevice::from).collect())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Updating managed device", skip(conn))]
pub(crate) async fn update_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    managed_device: ManagedDevice,
) -> Result<UpdateResult<ManagedDevice>, DomainError> {
    if let Err(DomainError::NotFound(_)) = get_managed_device(conn, &managed_device.id).await {
        // ManagedDevice not exists yet => create a new instance
        let result = create_managed_device(conn, &managed_device).await?;
        if let CreateResult::Created(created_managed_device) = result {
            return Ok(UpdateResult::Created(created_managed_device));
        }
    };
    let agent_id = managed_device.agent_id();
    let result = crate::data_access::helpers::update_managed_device(
        conn,
        &managed_device.id,
        &managed_device.name,
        &managed_device.description,
        agent_id,
        serde_json::to_string(&managed_device.snmp_protocol_attributes)
            .unwrap()
            .as_ref(),
    )
    .await?;

    Ok(UpdateResult::Updated(result.into()))
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Starting managed device", skip(conn))]
pub(crate) async fn start_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<UpdateResult<bool>, DomainError> {
    let _device = get_managed_device(conn, id).await?;

    // TODO: ManagedDevice exists => start it

    Ok(UpdateResult::Updated(true))
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(level = "debug", name = "[BL] Stopping managed device", skip(conn))]
pub(crate) async fn stop_managed_device<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<UpdateResult<bool>, DomainError> {
    let _device = get_managed_device(conn, id).await?;

    // TODO: ManagedDevice exists => stop it

    Ok(UpdateResult::Updated(true))
}
