use crate::cli::{CliContext, CommandHandler};
use crate::operations::agent::{create_agent, delete_agent, get_agent, list_agents, update_agent};
use async_trait::async_trait;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum AgentCommands {
    /// List SNMP Agents
    Ls,

    /// Get Agent by ID
    Get(Agent),

    /// Create a new instance of SNMP Agent
    Add(CreateAgent),

    /// Update an existing instance of SNMP Agent
    Update(UpdateAgent),

    /// Remove Agent by ID
    Rm(Agent),
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CreateAgent {
    // agent name
    #[clap(long)]
    pub name: String,

    // agent description
    #[clap(long)]
    pub description: Option<String>,

    // path to the SNMPWalk output file
    #[clap(long, parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    pub snmp_data_file: std::path::PathBuf,
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UpdateAgent {
    // unique identifier of an existing agent
    #[clap(long)]
    pub id: String,

    // agent name
    #[clap(long)]
    pub name: String,

    // agent description
    #[clap(long)]
    pub description: Option<String>,

    // path to the SNMPWalk output file
    #[clap(long, parse(from_os_str), value_hint = clap::ValueHint::FilePath)]
    pub snmp_data_file: std::path::PathBuf,
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct Agent {
    // unique identifier of an existing agent
    #[clap(short, long)]
    pub id: String,
}

#[async_trait]
impl CommandHandler for AgentCommands {
    async fn handle(self, ctx: &CliContext) -> Result<(), anyhow::Error> {
        match self {
            AgentCommands::Ls => list_agents(ctx).await,
            AgentCommands::Add(args) => create_agent(ctx, args).await,
            AgentCommands::Update(args) => update_agent(ctx, args).await,
            AgentCommands::Rm(args) => delete_agent(ctx, args).await,
            AgentCommands::Get(args) => get_agent(ctx, args).await,
        }
    }
}
