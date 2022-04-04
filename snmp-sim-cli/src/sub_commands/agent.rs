use crate::CommandHandler;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    /// List SNMP Agents
    Ls,

    /// Create a new instance of SNMP Agent
    Add(CreateAgent),

    /// Update an existing instance of SNMP Agent
    Update(UpdateAgent),

    /// Remove one or more SNMP Agents
    Rm(DeleteAgent),
}

#[derive(Debug, Args)]
pub struct CreateAgent {
    // name of the agent to be created
    #[clap(short, long)]
    name: String,
}

#[derive(Debug, Args)]
pub struct UpdateAgent {
    // unique identifier of an existing agent
    #[clap(short, long)]
    id: String,

    // new name of an agent
    #[clap(short, long)]
    name: String,
}

#[derive(Debug, Args)]
pub struct DeleteAgent {
    // unique identifier of an existing agent
    #[clap(short, long)]
    id: String,
}

impl CommandHandler for AgentCommands {
    fn handle(self) -> Result<(), anyhow::Error> {
        match self {
            AgentCommands::Ls => list_agents(),
            AgentCommands::Add(args) => create_agent(args),
            AgentCommands::Update(args) => update_agent(args),
            AgentCommands::Rm(args) => delete_agent(args),
        }
    }
}

fn list_agents() -> Result<(), anyhow::Error> {
    println!("Get all agents");
    Ok(())
}

fn create_agent(agent: CreateAgent) -> Result<(), anyhow::Error> {
    println!("Creating agent {:?}", agent);
    Ok(())
}

fn update_agent(agent: UpdateAgent) -> Result<(), anyhow::Error> {
    println!("Updating agent {:?}", agent);
    Ok(())
}

fn delete_agent(agent: DeleteAgent) -> Result<(), anyhow::Error> {
    println!("Deleting agent {:?}", agent);
    Ok(())
}
