use crate::subcommands::agent::AgentCommands;
use clap::Parser;
use clap::Subcommand;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) trait CommandHandler {
    fn handle(self) -> anyhow::Result<()>;
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

impl CommandHandler for SnmpSimCliCommands {
    fn handle(self) -> anyhow::Result<()> {
        match self {
            SnmpSimCliCommands::Agents => AgentCommands::Ls.handle(),
            SnmpSimCliCommands::Agent(agent_cmd) => agent_cmd.handle(),
        }
    }
}

/// SNMP Simulator Management CLI
#[derive(Debug, Parser)]
#[clap(name = "snmp-sim-cli", version)]
#[clap(about, long_about = None)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct SnmpSimCli {
    #[clap(subcommand)]
    /// SNMP Simulator CLI Commands
    command: SnmpSimCliCommands,
}

impl SnmpSimCli {
    pub async fn run(self) -> anyhow::Result<()> {
        self.command.handle()
    }
}
