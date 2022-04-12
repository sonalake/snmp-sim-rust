use crate::subcommands::agent::{CreateAgent, DeleteAgent, UpdateAgent};
use rust_client_snmp_sim_lib::apis::agents_api::*;
use rust_client_snmp_sim_lib::apis::configuration::Configuration;
use tracing::{self, trace};

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn list_agents() -> Result<(), anyhow::Error> {
    trace!("List all agents");
    let mut configuration = Configuration::default();
    configuration.base_path = "http://127.0.0.1:8180".to_string();
    let agents = agents_get(&configuration, None, None).await?;
    for agent in agents.iter() {
        println!("{:?}", agent);
    }

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn create_agent(agent: CreateAgent) -> Result<(), anyhow::Error> {
    trace!("Create a new instance of agent={:?}", agent);
    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn update_agent(agent: UpdateAgent) -> Result<(), anyhow::Error> {
    trace!("Update an existing agent={:?}", agent);
    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn delete_agent(agent: DeleteAgent) -> Result<(), anyhow::Error> {
    trace!("Delete an existing agent={:?}", agent);
    Ok(())
}
