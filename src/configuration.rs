use crate::settings::*;
use config::Config;
use config::Environment;
use config::File;
use std::path::PathBuf;
use std::default::Default;

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
/// Represents the implemented settings of the SNMP Simulator service.
pub struct Settings {
    //#[serde(default = "default_application_config")]
    pub application: ApplicationSettings,

    //#[serde(default = "default_database_config")]
    pub database: DatabaseSettings,
}

/// Returns the SNMP Simulator configuration
///
/// # Arguments
///
/// * `path_override` - An optional path to the configuration file(s). Current
///   directory is used if path is not provided. The function is expecting a
///   mandatory configuration file `base.yaml` and an optional `local.yaml`,
///   which can be used to override the `base.yaml` configuration.
///
/// # Errors
///
/// If configuration read fails, be it technical reasons or related to inability
/// to read data as Config for different reasons, this method returns error.
///
/// # Panics
///
/// Panics if the service is executed with insufficient permissions to access
/// the current directory.
///
/// # Examples
///
/// ```
/// use snmp_sim::configuration::get_configuration;
/// let config = get_configuration(None);
/// ```
#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub fn get_configuration(path_override: Option<PathBuf>) -> Result<Settings, config::ConfigError> {
    let base_path =
        path_override.unwrap_or_else(|| std::env::current_dir().expect("Failed to determine the current directory"));

    let configuration_directory = base_path.join("configuration");

    Config::builder()
        .add_source(File::from(configuration_directory.join("base.yaml")).required(true))
        .add_source(File::from(configuration_directory.join("local.yaml")).required(false))
        .add_source(Environment::with_prefix("app").separator("__"))
        .build()?
        .try_deserialize()
}
