use anyhow::Context;
use snmp_sim::app::Service;
use snmp_sim::configuration::get_configuration;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Value {
    /* To be filled with args/values */
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // read the configuration file
    let configuration = get_configuration(None).context("Failed to read configuration")?;

    Service::run(configuration).await
}