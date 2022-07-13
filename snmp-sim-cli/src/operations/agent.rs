use crate::cli::CliContext;
use crate::subcommands::agent::{Agent, CreateAgent, UpdateAgent};
use rust_client_snmp_sim_lib::apis::agents_api::*;
use rust_client_snmp_sim_lib::apis::configuration::Configuration;
use rust_client_snmp_sim_lib::models::RequestAgent;
use tracing::{self, trace};

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn list_agents(ctx: &CliContext<'_>) -> Result<(), anyhow::Error> {
    trace!("List all agents");
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let response = agents_get(&configuration, None, None).await?;
    for agent in response.items.iter() {
        println!("{:#?}", agent);
    }

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn create_agent(ctx: &CliContext<'_>, create_agent: CreateAgent) -> Result<(), anyhow::Error> {
    trace!("Create a new instance of agent={:#?}", create_agent);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let mut agent = RequestAgent::new(
        create_agent.name,
        create_agent
            .snmp_data_file
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    agent.description = create_agent.description;

    let created_agent = agents_post(&configuration, agent).await?;
    println!("{:#?}", created_agent);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn update_agent(ctx: &CliContext<'_>, update_agent: UpdateAgent) -> Result<(), anyhow::Error> {
    trace!("Update an existing agent={:#?}", update_agent);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let mut agent = RequestAgent::new(
        update_agent.name,
        update_agent
            .snmp_data_file
            .into_os_string()
            .into_string()
            .unwrap(),
    );
    agent.description = update_agent.description;

    let updated_agent = agents_id_put(&configuration, &update_agent.id, agent).await?;
    println!("{:#?}", updated_agent);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn delete_agent(ctx: &CliContext<'_>, agent: Agent) -> Result<(), anyhow::Error> {
    trace!("Delete an existing agent={:#?}", agent);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let deleted_agent = agents_id_delete(&configuration, &agent.id).await?;
    println!("{:#?}", deleted_agent);

    Ok(())
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) async fn get_agent(ctx: &CliContext<'_>, agent: Agent) -> Result<(), anyhow::Error> {
    trace!("Delete an existing agent={:#?}", agent);
    let mut configuration = Configuration::new();
    configuration.base_path = ctx.url();

    let agent = agents_id_get(&configuration, &agent.id).await?;
    println!("{:#?}", agent);

    Ok(())
}
