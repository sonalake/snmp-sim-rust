use crate::cli::subcommands::config::ConfigCommands;
use async_trait::async_trait;
use clap::Parser;
use clap::Subcommand;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[async_trait]
pub(crate) trait CommandHandler {
    async fn handle(self, ctx: &CliContext) -> anyhow::Result<()>;
}

#[derive(Debug)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CliContext();

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum SnmpServiceCliCommands {
    /// Manage Config
    #[clap(subcommand)]
    Config(ConfigCommands),
}

#[async_trait]
impl CommandHandler for SnmpServiceCliCommands {
    async fn handle(self, ctx: &CliContext) -> anyhow::Result<()> {
        match self {
            SnmpServiceCliCommands::Config(config_cmd) => config_cmd.handle(ctx).await,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct SnmpServiceCli {
    #[clap(subcommand)]
    command: SnmpServiceCliCommands,
}
