pub mod configuration;
pub mod settings;
pub mod snmp;
pub mod telemetry;
pub mod udp_server;

use crate::configuration::get_configuration;
use crate::telemetry::{get_subscriber, init_subscriber};

use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Context;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // read the configuration file
    let configuration = get_configuration(None).context("Failed to read configuration")?;

    // setup application telemetry
    let subscriber = get_subscriber(
        "snmp-sim".into(),
        configuration.application.level.clone(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let binding_address = format!("{}:{}", configuration.application.host, configuration.application.port);
    tracing::debug!("HttpServer binding address: {}", binding_address);

    // start an instance of http restful api
    HttpServer::new(move || {
        App::new().wrap(TracingLogger::default()).service(
            web::scope(&configuration.application.uri_prefix).service(web::resource("/").to(|| HttpResponse::Ok())),
        )
    })
    .bind(binding_address)?
    .disable_signals()
    .run()
    .await?;

    Ok(())
}
