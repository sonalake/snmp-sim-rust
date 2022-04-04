mod cli_commands;
mod command_handler;
mod sub_commands;

use anyhow::Error;
use clap::Parser;
pub use cli_commands::*;
pub use command_handler::*;
pub use sub_commands::*;

/// SNMP Simulator Management CLI
#[derive(Debug, Parser)]
#[clap(name = "snmp-sim-cli", version)]
#[clap(about, long_about = None)]
struct SnmpSimCli {
    #[clap(subcommand)]
    /// SNMP Simulator CLI Commands
    command: SnmpSimCliCommands,
}

impl SnmpSimCli {
    pub fn run(self) -> Result<(), Error> {
        self.command.handle()
    }
}

fn main() -> Result<(), Error> {
    SnmpSimCli::parse().run()
}
