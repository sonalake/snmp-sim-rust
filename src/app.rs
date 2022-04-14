use crate::configuration::Settings;
use crate::routes::agents_config;
use crate::settings::DatabaseSettings;
use actix_web::dev::ServiceFactory;
use actix_web::middleware::{Compat, NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer};
use anyhow::Context;
use paperclip::actix::web::scope;
use paperclip::actix::OpenApiExt;
use paperclip::v2::models::DefaultApiRaw;
use paperclip_restful::extractor_config::*;
use sea_orm::{Database, DatabaseConnection};
use sqlx::{Connection, SqliteConnection};
use telemetry::{get_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

pub fn register_services<Sf>(app: paperclip::actix::App<Sf>, uri_prefix: &str) -> paperclip::actix::App<Sf>
where
    Sf: ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
{
    app.service(scope(uri_prefix).configure(agents_config))
}

pub fn spec_modifier(spec: &mut DefaultApiRaw) {
    paperclip_restful::add_json_error(spec);
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub async fn get_database_connection(configuration: &DatabaseSettings) -> anyhow::Result<Option<DatabaseConnection>> {
    if !configuration.connection_uri()?.is_empty() {
        Ok(Some(Database::connect(&configuration.connection_uri()?).await?))
    } else {
        Ok(None)
    }
}

pub struct Service;

impl Service {
    pub async fn run(configuration: Settings) -> anyhow::Result<()> {
        // setup application telemetry
        let subscriber = get_subscriber(
            "snmp-sim".into(),
            configuration.application.level.clone(),
            std::io::stdout,
        );
        init_subscriber(subscriber);

        let database_options = configuration.database.options()?;

        let mut connection = SqliteConnection::connect_with(&database_options)
            .await
            .context(format!("connection URI {}", &configuration.database.connection_uri()?))
            .context("Failed to connect to SQLite database.")?;

        sqlx::migrate!("./migrations/")
            .run(&mut connection)
            .await
            .context(format!("connection_uri {}", &configuration.database.connection_uri()?))
            .context("Failed to migrate the database")?;

        connection.close();

        let db_conn = get_database_connection(&configuration.database)
            .await
            .context(format!("connection_uri {}", &configuration.database.connection_uri()?))
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
        .await
        .context("Failed to start service")
    }
}
