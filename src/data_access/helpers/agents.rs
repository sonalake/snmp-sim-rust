use crate::data_access::entity::agents::{
    ActiveModel as AgentsActiveModel, Column as AgentsColumn, Entity as Agents, Model as AgentsModel,
};
use crate::domain::CreateResult;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait, DbErr, Delete, DeleteResult, EntityTrait};
use uuid_dev::Uuid;

#[tracing::instrument(name = "[DA] Create a new instance of agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn create_agent<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
    agent_name: &str,
) -> Result<CreateResult<AgentsModel>, DbErr> {
    let result = Agents::find()
        .filter(AgentsColumn::Id.eq(id.as_bytes().to_vec()))
        .one(conn)
        .await?;

    if let Some(result) = result {
        return Ok(CreateResult::Duplicate(result));
    }

    let mut agent = AgentsActiveModel {
        id: ActiveValue::set(id.as_bytes().to_vec()),
        name: ActiveValue::set(agent_name.to_string()),
        ..Default::default()
    };

    let insert_result = Agents::insert(agent.clone())
        .exec(conn)
        .await
        .map_err(|e| DbErr::Custom(format!("CONFLICT, error={}", e)))?;
    agent.id = ActiveValue::set(insert_result.last_insert_id);
    Ok(CreateResult::Created(agent.into()))
}

#[tracing::instrument(name = "[DA] Finding an agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn get_agent<'db>(conn: &'db impl ConnectionTrait, id: &Uuid) -> Result<Option<AgentsModel>, DbErr> {
    Agents::find_by_id(id.as_bytes().to_vec()).one(conn).await
}

#[tracing::instrument(name = "[DA] Deleting an agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn delete_agent<'db>(conn: &'db impl ConnectionTrait, id: &Uuid) -> Result<DeleteResult, DbErr> {
    Delete::one(AgentsActiveModel {
        id: ActiveValue::set(id.as_bytes().to_vec()),
        // keep the default here, since we want to delete the entity by key only, all the resut of the fields are unset
        ..Default::default()
    })
    .exec(conn)
    .await
}

#[tracing::instrument(name = "[DA] Listing agents", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn list_agents<'db>(
    conn: &'db impl ConnectionTrait,
    page: usize,
    page_size: usize,
) -> Result<Vec<AgentsModel>, DbErr> {
    let paginator = Agents::find().paginate(conn, page_size);
    paginator.fetch_page(page - 1).await
}

#[tracing::instrument(name = "[DA] Updating agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn update_agent<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
    agent_name: &str,
) -> Result<AgentsModel, DbErr> {
    let agent = AgentsActiveModel {
        id: ActiveValue::set(id.as_bytes().to_vec()),
        name: ActiveValue::set(agent_name.to_string()),
        ..Default::default()
    };
    agent.update(conn).await
}
