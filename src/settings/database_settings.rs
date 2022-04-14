use anyhow::Context;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    connection_uri: String,
    #[serde(default = "default_tests_skip_drop")]
    pub tests_skip_drop: bool,
}

fn default_tests_skip_drop() -> bool {
    false
}

impl DatabaseSettings {
    pub fn connection_uri(&self) -> anyhow::Result<String> {
        Ok(self.connection_uri.replace(
            "~",
            dirs::home_dir()
                .context("Failed to get the HOME directory")?
                .into_os_string()
                .into_string()
                .unwrap()
                .as_str(),
        ))
    }

    pub fn options(&self) -> anyhow::Result<SqliteConnectOptions> {
        if let Some(target_ri) = self.connection_uri()?.strip_prefix("sqlite://") {
            fs::create_dir_all(
                Path::new(&target_ri)
                    .parent()
                    .unwrap_or(Path::new(&target_ri)),
            )?;
        }

        Ok(SqliteConnectOptions::from_str(&self.connection_uri()?)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true))
    }
}
