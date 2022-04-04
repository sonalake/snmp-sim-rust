use crate::AgentCommands;
use crate::CommandHandler;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SnmpSimCliCommands {
    /// List SNMP Agents
    Agents,

    /// Manage SNMP Agents
    #[clap(subcommand)]
    Agent(AgentCommands),
}

impl CommandHandler for SnmpSimCliCommands {
    fn handle(self) -> std::result::Result<(), anyhow::Error> {
        match self {
            SnmpSimCliCommands::Agents => AgentCommands::Ls.handle(),
            SnmpSimCliCommands::Agent(agent_cmd) => agent_cmd.handle(),
        }
    }
}
