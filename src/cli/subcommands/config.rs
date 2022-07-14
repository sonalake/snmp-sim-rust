use clap::{Args, Subcommand};
use crate::cli::operations::config::write_default_config;
use async_trait::async_trait;

use crate::cli::cli::{CommandHandler, CliContext};

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum ConfigCommands {
    /// Create default configuration stored in base.yaml file
    Create(CreateConfig),
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CreateConfig {
    // agent name
    // short has to be a 'y' character, check documentation on short macro
    #[clap(short)]
    pub overwrite: bool,
}

#[async_trait]
impl CommandHandler for ConfigCommands {
    async fn handle(self, _ctx: &CliContext) -> Result<(), anyhow::Error> {
        match self {
            ConfigCommands::Create(args) => write_default_config(args.overwrite).await,
        }
    }
}
