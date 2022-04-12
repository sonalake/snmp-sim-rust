use crate::subcommands::agent::AgentCommands;
use async_trait::async_trait;
use clap::Parser;
use clap::Subcommand;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[async_trait]
pub(crate) trait CommandHandler {
    async fn handle(self) -> anyhow::Result<()>;
}

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum SnmpSimCliCommands {
    /// List SNMP Agents
    Agents,

    /// Manage SNMP Agents
    #[clap(subcommand)]
    Agent(AgentCommands),
}

#[async_trait]
impl CommandHandler for SnmpSimCliCommands {
    async fn handle(self) -> anyhow::Result<()> {
        match self {
            SnmpSimCliCommands::Agents => AgentCommands::Ls.handle().await,
            SnmpSimCliCommands::Agent(agent_cmd) => agent_cmd.handle().await,
        }
    }
}

/// SNMP Simulator Management CLI
#[derive(Debug, Parser)]
#[clap(name = "snmp-sim-cli", version)]
#[clap(about, long_about = None)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct SnmpSimCli {
    #[clap(env = "SNMP_SIM_URL")]
    url: String,

    #[clap(subcommand)]
    /// SNMP Simulator CLI Commands
    command: SnmpSimCliCommands,
}

impl SnmpSimCli {
    pub async fn run(self) -> anyhow::Result<()> {
        self.command.handle().await
    }
}
