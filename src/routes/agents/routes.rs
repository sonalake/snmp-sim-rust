use super::{request, response};
use crate::domain::{CreateResult, UpdateResult};
use crate::routes::{AgentError, GetAgentsQuery};
use paperclip::actix::{
    api_v2_operation, delete, get, post, put,
    web::{self, Data, Json, ServiceConfig},
};
use paperclip_restful::{DeleteResponse, GetResponse, JsonError, PostResponse, PutResponse};
use sea_orm::DatabaseConnection;
use std::convert::TryInto;
use uuid_dev::Uuid;

#[post("/agents")]
#[api_v2_operation(tags("Agents"), consumes = "application/json")]
/// Create a new agent
async fn post_agent(
    form: Json<request::Agent>,
    conn: Data<DatabaseConnection>,
) -> Result<PostResponse<response::Agent>, JsonError<AgentError>> {
    let agent: crate::domain::Agent = form.0.try_into()?;

    let result = crate::domain::create_agent(conn.as_ref(), &agent)
        .await
        .map_err(AgentError::from)?;
    match result {
        CreateResult::Created(x) => Ok(PostResponse::Created(response::Agent::from(x))),
        CreateResult::Duplicate(x) => Ok(PostResponse::Exists(response::Agent::from(x))),
    }
}

#[get("/agents/{id}")]
#[api_v2_operation(tags("Agents"))]
/// Get agent by ID
async fn get_agent(
    id: web::Path<Uuid>,
    conn: Data<DatabaseConnection>,
) -> Result<GetResponse<response::Agent>, JsonError<AgentError>> {
    let agent = crate::domain::get_agent(conn.as_ref(), id.as_ref())
        .await
        .map_err(AgentError::from)
        .map_err(JsonError::from)?;

    Ok(agent).map(response::Agent::from).map(GetResponse::Ok)
}

#[delete("/agents/{id}")]
#[api_v2_operation(tags("Agents"))]
/// Delete agent by ID
async fn delete_agent(
    id: web::Path<Uuid>,
    conn: Data<DatabaseConnection>,
) -> Result<DeleteResponse<response::Agent>, JsonError<AgentError>> {
    let result = crate::domain::delete_agent(conn.as_ref(), id.as_ref())
        .await
        .map_err(AgentError::from)
        .map_err(JsonError::from)?;
    match result {
        None => Ok(DeleteResponse::NoContent),
        Some(x) => Ok(DeleteResponse::Deleted(x.into())),
    }
}

#[get("/agents")]
#[api_v2_operation(tags("Agents"))]
/// List agents
async fn list_agents(
    conn: Data<DatabaseConnection>,
    web::Query(query): web::Query<GetAgentsQuery>,
) -> Result<GetResponse<Vec<response::Agent>>, JsonError<AgentError>> {
    let results = crate::domain::list_agents(conn.as_ref(), query.page.unwrap(), query.page_size.unwrap())
        .await
        .map_err(AgentError::from)?;

    Ok(GetResponse::Ok(results.iter().map(response::Agent::from).collect()))
}

#[put("/agents/{id}")]
#[api_v2_operation(tags("Agents"))]
/// Update agent
async fn update_agent(
    conn: Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    form: Json<request::Agent>,
) -> Result<PutResponse<response::Agent>, JsonError<AgentError>> {
    let requested_agent = (id.into_inner(), form.0).try_into()?;
    let result = crate::domain::update_agent(conn.as_ref(), requested_agent)
        .await
        .map_err(AgentError::from)?;

    match result {
        UpdateResult::Created(x) => Ok(PutResponse::Created(response::Agent::from(x))),
        UpdateResult::Updated(x) => Ok(PutResponse::Updated(response::Agent::from(x))),
    }
}

pub fn agents_config(cfg: &mut ServiceConfig) {
    cfg.service(post_agent);
    cfg.service(get_agent);
    cfg.service(delete_agent);
    cfg.service(list_agents);
    cfg.service(update_agent);
}
