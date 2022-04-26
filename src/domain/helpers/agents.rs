use crate::domain::{Agent, CreateResult, DomainError, UpdateResult};
use sea_orm::ConnectionTrait;
use uuid_dev::Uuid;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(name = "[BL] Create an instance of Agent", skip(conn))]
pub(crate) async fn create_agent<'db>(
    conn: &'db impl ConnectionTrait,
    agent: &Agent,
) -> Result<CreateResult<Agent>, DomainError> {
    let result = crate::data_access::helpers::create_agent(
        conn,
        &agent.id,
        &agent.name,
        &agent.description,
        &agent.snmp_data_url,
    )
    .await
    .map_err(DomainError::from)?;

    Ok(result.map(|agent| agent.into()))
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(name = "[BL] Finding an agent", skip(conn))]
pub(crate) async fn get_agent<'db>(conn: &'db impl ConnectionTrait, id: &Uuid) -> Result<Agent, DomainError> {
    crate::data_access::helpers::get_agent(conn, id)
        .await?
        .map(|agent| agent.into())
        .ok_or_else(|| DomainError::NotFound(format!("AgentId={} not exists", id)))
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(name = "[BL] Deleting an agent", skip(conn))]
pub(crate) async fn delete_agent<'db>(
    conn: &'db impl ConnectionTrait,
    id: &Uuid,
) -> Result<Option<Agent>, DomainError> {
    match crate::data_access::helpers::get_agent(conn, id).await? {
        Some(agent) => {
            crate::data_access::helpers::delete_agent(conn, id).await?;
            Ok(Some(Agent::from(agent)))
        }
        None => Ok(None),
    }
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(name = "[BL] Listing agents", skip(conn))]
pub(crate) async fn list_agents<'db>(
    conn: &'db impl ConnectionTrait,
    page: usize,
    page_size: usize,
) -> Result<Vec<Agent>, DomainError> {
    let result = crate::data_access::helpers::list_agents(conn, page, page_size).await?;

    Ok(result.into_iter().map(Agent::from).collect())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[tracing::instrument(name = "[BL] Updating agent", skip(conn))]
pub(crate) async fn update_agent<'db>(
    conn: &'db impl ConnectionTrait,
    agent: Agent,
) -> Result<UpdateResult<Agent>, DomainError> {
    if let Err(DomainError::NotFound(_)) = get_agent(conn, &agent.id).await {
        // Agent not exists yet => create a new instance
        let result = create_agent(conn, &agent).await?;
        if let CreateResult::Created(created_agent) = result {
            return Ok(UpdateResult::Created(created_agent));
        }
    };

    let result = crate::data_access::helpers::update_agent(
        conn,
        &agent.id,
        &agent.name,
        &agent.description,
        &agent.snmp_data_url,
    )
    .await?;

    Ok(UpdateResult::Updated(result.into()))
}
