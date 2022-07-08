use crate::data_access::entity::agents::{
    ActiveModel as AgentsActiveModel, Column as AgentsColumn, Entity as Agents, Model as AgentsModel,
};
use crate::domain::CreateResult;
use sea_orm::{entity::prelude::*, ActiveValue, ConnectionTrait, DbErr, Delete, DeleteResult, EntityTrait};
use uuid_dev::Uuid;

#[tracing::instrument(level = "debug", name = "[DA] Create a new instance of agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn create_agent<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
    agent_name: &str,
    description: &Option<String>,
    snmp_data_url: &str,
) -> Result<CreateResult<AgentsModel>, DbErr> {
    let result = Agents::find()
        .filter(AgentsColumn::Id.eq(id.to_string()))
        .one(conn)
        .await?;

    if let Some(result) = result {
        return Ok(CreateResult::Duplicate(result));
    }

    let mut agent = AgentsActiveModel {
        id: ActiveValue::set(id.to_string()),
        name: ActiveValue::set(agent_name.to_string()),
        description: ActiveValue::set(description.clone()),
        snmp_data_url: ActiveValue::set(snmp_data_url.to_string()),
        created_at: ActiveValue::set(chrono::Utc::now()),
        modified_at: ActiveValue::set(chrono::Utc::now()),
    };

    let insert_result = Agents::insert(agent.clone())
        .exec(conn)
        .await
        .map_err(|e| DbErr::Custom(format!("CONFLICT, error={}", e)))?;
    agent.id = ActiveValue::set(insert_result.last_insert_id);
    Ok(CreateResult::Created(agent.into()))
}

#[tracing::instrument(level = "debug", name = "[DA] Finding an agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn get_agent<'db>(conn: &'db impl ConnectionTrait, id: &Uuid) -> Result<Option<AgentsModel>, DbErr> {
    Agents::find_by_id(id.to_string()).one(conn).await
}

#[tracing::instrument(level = "debug", name = "[DA] Deleting an agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn delete_agent<'db>(conn: &'db impl ConnectionTrait, id: &Uuid) -> Result<DeleteResult, DbErr> {
    Delete::one(AgentsActiveModel {
        id: ActiveValue::set(id.to_string()),
        // keep the default here, since we want to delete the entity by key only, all the resut of the fields are unset
        ..Default::default()
    })
    .exec(conn)
    .await
}

#[tracing::instrument(level = "debug", name = "[DA] Listing agents", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn list_agents<'db>(
    conn: &'db impl ConnectionTrait,
    page: usize,
    page_size: usize,
) -> Result<(usize, Vec<AgentsModel>), DbErr> {
    let paginator = Agents::find().paginate(conn, page_size);

    Ok((paginator.num_items().await?, paginator.fetch_page(page - 1).await?))
}

#[tracing::instrument(level = "debug", name = "[DA] Updating agent", skip(conn))]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn update_agent<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
    agent_name: &String,
    agent_description: &Option<String>,
    snmp_data_url: &str,
) -> Result<AgentsModel, DbErr> {
    let am: Option<AgentsModel> = Agents::find_by_id(id.to_string()).one(conn).await?;
    let mut agent: AgentsActiveModel = am.unwrap().into();

    agent.name = ActiveValue::set(agent_name.to_string());
    agent.description = ActiveValue::set(agent_description.clone());
    agent.snmp_data_url = ActiveValue::set(snmp_data_url.to_string());

    agent.update(conn).await
}
