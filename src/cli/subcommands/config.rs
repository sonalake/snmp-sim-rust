use crate::cli::operations::config::write_default_config;
use async_trait::async_trait;
use clap::{Args, Subcommand};

use crate::cli::cli::{CliContext, CommandHandler};

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum ConfigCommands {
    /// Create default configuration stored in base.yaml file
    Create(CreateConfig),
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CreateConfig {
    // Overwrite status, false as default
    // short has to be a 'y' character, check documentation on short macro
    #[clap(short = 'y')]
    // pub overwrite: Option<bool>,
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
