use crate::cli::{CliContext, CommandHandler};
use crate::operations::device::*;
use async_trait::async_trait;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) enum DeviceCommands {
    /// List SNMP Devices
    Ls,

    /// Get Device by ID
    Get(Device),

    /// Create a new instance of SNMP Device
    Add(CreateDevice),

    /// Update an existing instance of SNMP Device
    Update(UpdateDevice),

    /// Remove Device by ID
    Rm(Device),

    /// Start a Device by ID
    Start(Device),

    /// Stop a Device by ID
    Stop(Device),
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct CreateDevice {
    // device name
    #[clap(long)]
    pub name: String,

    // device description
    #[clap(long)]
    pub description: Option<String>,

    // the referenced SNMP Agent identifier
    #[clap(long)]
    pub agent_id: String,

    // SNMP Protocol parameters as JSON string
    #[clap(long)]
    pub protocol: String,
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct UpdateDevice {
    // unique identifier of an existing device
    #[clap(long)]
    pub id: String,

    // device name
    #[clap(long)]
    pub name: String,

    // device description
    #[clap(long)]
    pub description: Option<String>,

    // the referenced SNMP Agent identifier
    #[clap(long)]
    pub agent_id: String,

    // SNMP Protocol parameters as JSON string
    #[clap(long)]
    pub protocol: String,
}

#[derive(Debug, Args)]
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct Device {
    // unique identifier of an existing device
    #[clap(short, long)]
    pub id: String,
}

#[async_trait]
impl CommandHandler for DeviceCommands {
    async fn handle(self, ctx: &CliContext) -> Result<(), anyhow::Error> {
        match self {
            DeviceCommands::Ls => list_devices(ctx).await,
            DeviceCommands::Add(args) => create_device(ctx, args).await,
            DeviceCommands::Update(args) => update_device(ctx, args).await,
            DeviceCommands::Rm(args) => delete_device(ctx, args).await,
            DeviceCommands::Get(args) => get_device(ctx, args).await,
            DeviceCommands::Start(args) => start_device(ctx, args).await,
            DeviceCommands::Stop(args) => stop_device(ctx, args).await,
        }
    }
}
