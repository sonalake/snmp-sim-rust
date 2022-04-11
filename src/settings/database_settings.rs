use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub connection_uri: String,
    #[serde(default = "default_tests_skip_drop")]
    pub tests_skip_drop: bool,
}

fn default_tests_skip_drop() -> bool {
    false
}

impl DatabaseSettings {
    pub fn options(&self) -> sqlx::Result<SqliteConnectOptions> {
        Ok(SqliteConnectOptions::from_str(&self.connection_uri)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true))
    }
}
