use super::{request, response};
use crate::domain::{CreateResult, UpdateResult};
use crate::routes::{DeviceError, GetAgentsQuery};
use crate::udp_server::udp_server_delegate::UdpServerDelegate;
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{self, Data, Json, ServiceConfig},
};
use paperclip_restful::{DeleteResponse, GetResponse, JsonError, PostResponse, PutResponse};
use sea_orm::DatabaseConnection;
use std::convert::TryInto;
use uuid_dev::Uuid;

#[post("/devices")]
#[api_v2_operation(tags("Devices"), consumes = "application/json")]
/// Create a new managed device
async fn post_device(
    form: Json<request::Device>,
    conn: Data<DatabaseConnection>,
) -> Result<PostResponse<response::Device>, JsonError<DeviceError>> {
    let managed_device: crate::domain::ManagedDevice = form.0.try_into()?;

    let result = crate::domain::create_managed_device(conn.as_ref(), &managed_device)
        .await
        .map_err(DeviceError::from)?;
    match result {
        CreateResult::Created(x) => Ok(PostResponse::Created(response::Device::from(x))),
        CreateResult::Duplicate(x) => Ok(PostResponse::Exists(response::Device::from(x))),
    }
}

#[get("/devices/{id}")]
#[api_v2_operation(tags("Devices"))]
/// Get managed device by ID
async fn get_device(
    id: web::Path<Uuid>,
    conn: Data<DatabaseConnection>,
) -> Result<GetResponse<response::Device>, JsonError<DeviceError>> {
    let managed_device = crate::domain::get_managed_device(conn.as_ref(), id.as_ref())
        .await
        .map_err(DeviceError::from)
        .map_err(JsonError::from)?;

    Ok(managed_device)
        .map(response::Device::from)
        .map(GetResponse::Ok)
}

#[delete("/devices/{id}")]
#[api_v2_operation(tags("Devices"))]
/// Delete managed device by ID
async fn delete_device(
    id: web::Path<Uuid>,
    conn: Data<DatabaseConnection>,
) -> Result<DeleteResponse<response::Device>, JsonError<DeviceError>> {
    let result = crate::domain::delete_managed_device(conn.as_ref(), id.as_ref())
        .await
        .map_err(DeviceError::from)
        .map_err(JsonError::from)?;
    match result {
        None => Ok(DeleteResponse::NoContent),
        Some(x) => Ok(DeleteResponse::Deleted(x.into())),
    }
}

#[get("/devices")]
#[api_v2_operation(tags("Devices"))]
/// List managed devices
async fn list_devices(
    conn: Data<DatabaseConnection>,
    web::Query(query): web::Query<GetAgentsQuery>,
) -> Result<GetResponse<Vec<response::Device>>, JsonError<DeviceError>> {
    let results = crate::domain::list_managed_devices(conn.as_ref(), query.page.unwrap(), query.page_size.unwrap())
        .await
        .map_err(DeviceError::from)?;

    Ok(GetResponse::Ok(
        results.into_iter().map(response::Device::from).collect(),
    ))
}

#[put("/devices/{id}")]
#[api_v2_operation(tags("Devices"))]
/// Update managed device
async fn update_device(
    conn: Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    form: Json<request::Device>,
) -> Result<PutResponse<response::Device>, JsonError<DeviceError>> {
    let requested_device = (id.into_inner(), form.0).try_into()?;
    let result = crate::domain::update_managed_device(conn.as_ref(), requested_device)
        .await
        .map_err(DeviceError::from)?;

    match result {
        UpdateResult::Created(x) => Ok(PutResponse::Created(response::Device::from(x))),
        UpdateResult::Updated(x) => Ok(PutResponse::Updated(response::Device::from(x))),
    }
}

#[put("/devices/{id}/start")]
#[api_v2_operation(tags("Devices"))]
/// Start an existing managed device
async fn post_device_start(
    id: web::Path<Uuid>,
    conn: Data<DatabaseConnection>,
    udp_server: web::Data<UdpServerDelegate>,
) -> Result<PutResponse<bool>, JsonError<DeviceError>> {
    let result = crate::domain::start_managed_device(conn.as_ref(), id.as_ref(), udp_server.as_ref())
        .await
        .map_err(DeviceError::from)?;

    match result {
        UpdateResult::Created(x) => Ok(PutResponse::Created(x)),
        UpdateResult::Updated(x) => Ok(PutResponse::Updated(x)),
    }
}

#[put("/devices/{id}/stop")]
#[api_v2_operation(tags("Devices"))]
/// Stop an existing managed device
async fn post_device_stop(
    id: web::Path<Uuid>,
    conn: Data<DatabaseConnection>,
    udp_server: web::Data<UdpServerDelegate>,
) -> Result<PutResponse<bool>, JsonError<DeviceError>> {
    let result = crate::domain::stop_managed_device(conn.as_ref(), id.as_ref(), udp_server.as_ref())
        .await
        .map_err(DeviceError::from)?;

    match result {
        UpdateResult::Created(x) => Ok(PutResponse::Created(x)),
        UpdateResult::Updated(x) => Ok(PutResponse::Updated(x)),
    }
}

pub fn devices_config(cfg: &mut ServiceConfig) {
    cfg.service(post_device);
    cfg.service(get_device);
    cfg.service(delete_device);
    cfg.service(list_devices);
    cfg.service(update_device);
    cfg.service(post_device_start);
    cfg.service(post_device_stop);
}
