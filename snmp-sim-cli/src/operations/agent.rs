use crate::subcommands::agent::{CreateAgent, DeleteAgent, UpdateAgent};
use tracing::{self, trace};

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) fn list_agents() -> Result<(), anyhow::Error> {
    trace!("List all agents");
    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) fn create_agent(agent: CreateAgent) -> Result<(), anyhow::Error> {
    trace!("Create a new instance of agent={:?}", agent);
    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) fn update_agent(agent: UpdateAgent) -> Result<(), anyhow::Error> {
    trace!("Update an existing agent={:?}", agent);
    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) fn delete_agent(agent: DeleteAgent) -> Result<(), anyhow::Error> {
    trace!("Delete an existing agent={:?}", agent);
    Ok(())
}
