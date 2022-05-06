use crate::subcommands::agent::AgentCommands;
use crate::subcommands::device::DeviceCommands;
use async_trait::async_trait;
use clap::Parser;
use clap::Subcommand;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[async_trait]
pub(crate) trait CommandHandler {
    async fn handle(self, ctx: &CliContext) -> anyhow::Result<()>;
}

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum SnmpSimCliCommands {
    /// List SNMP Agents
    Agents,

    /// Manage SNMP Agents
    #[clap(subcommand)]
    Agent(AgentCommands),

    /// List Devices
    Devices,

    /// Manage Devices
    #[clap(subcommand)]
    Device(DeviceCommands),
}

#[async_trait]
impl CommandHandler for SnmpSimCliCommands {
    async fn handle(self, ctx: &CliContext) -> anyhow::Result<()> {
        match self {
            SnmpSimCliCommands::Agents => AgentCommands::Ls.handle(ctx).await,
            SnmpSimCliCommands::Agent(agent_cmd) => agent_cmd.handle(ctx).await,
            SnmpSimCliCommands::Devices => DeviceCommands::Ls.handle(ctx).await,
            SnmpSimCliCommands::Device(device_cmd) => device_cmd.handle(ctx).await,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CliContext<'a> {
    url: &'a str,
}

impl<'a> CliContext<'a> {
    pub fn new(url: &'a str) -> Self {
        CliContext { url }
    }

    pub fn url(&self) -> String {
        self.url.to_string()
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
        self.command.handle(&CliContext::new(&self.url)).await
    }
}
