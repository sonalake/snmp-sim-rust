use actix_web::middleware::{Compat, NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer};
use anyhow::Context;
use paperclip::actix::OpenApiExt;
use paperclip_restful::extractor_config::*;
use sea_orm::{Database, DatabaseConnection};
use snmp_sim::app::register_services;
use snmp_sim::configuration::get_configuration;
use snmp_sim::settings::DatabaseSettings;
use sqlx::{Connection, SqliteConnection};
use telemetry::{get_subscriber, init_subscriber};
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

    let mut connection = SqliteConnection::connect_with(&configuration.database.options()?)
        .await
        .context(format!("connection URI {}", &configuration.database.connection_uri))
        .context("Failed to connect to SQLite database.")?;

    sqlx::migrate!("./migrations/")
        .run(&mut connection)
        .await
        .context(format!("connection_uri {}", &configuration.database.connection_uri))
        .context("Failed to migrate the database")?;

    connection.close();

    let db_conn = get_database_connection(&configuration.database)
        .await
        .context(format!("connection_uri {}", &configuration.database.connection_uri))
        .context("Failed to initialize sea-orm, SQLite database failure!")?
        .expect("Failed to initiate a database connection");

    let binding_address = format!("{}:{}", configuration.application.host, configuration.application.port);
    tracing::debug!("HttpServer binding address: {}", binding_address);

    // start an instance of http restful api
    HttpServer::new(move || {
        let app = App::new()
            .wrap(Compat::new(TracingLogger::default()))
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(json_extractor_config())
            .app_data(path_extractor_config())
            .app_data(query_extractor_config())
            .app_data(actix_web::web::Data::new(db_conn.clone()))
            .wrap_api()
            .with_json_spec_at("/api/spec/v2")
            .with_json_spec_v3_at("/api/spec/v3")
            .with_swagger_ui_at("/swagger");
        register_services(app, &configuration.application.uri_prefix).build()
    })
    .bind(binding_address)?
    .run()
    .await?;

    Ok(())
}

pub async fn get_database_connection(configuration: &DatabaseSettings) -> anyhow::Result<Option<DatabaseConnection>> {
    if !configuration.connection_uri.is_empty() {
        Ok(Some(Database::connect(&configuration.connection_uri).await?))
    } else {
        Ok(None)
    }
}
