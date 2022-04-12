use sea_orm::DatabaseConnection;
use snmp_sim::app::get_database_connection;
use snmp_sim::settings::DatabaseSettings;

#[derive(Debug, Clone)]
pub struct TestApp {
    pub address: String,
    pub db_conn: Option<DatabaseConnection>,
}

impl TestApp {
    pub async fn new(address: &str, database: &DatabaseSettings) -> Self {
        let db_conn = get_database_connection(database)
            .await
            .expect("Failed to connect to the database");

        TestApp {
            address: address.to_string(),
            db_conn,
        }
    }
}
