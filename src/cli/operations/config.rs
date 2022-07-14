use crate::cli::write_config_error::WriteConfigError;
use crate::configuration::Settings;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

pub async fn write_default_config(overwrite: bool) -> Result<(), anyhow::Error> {
    // Path to file
    let path = Path::new("./configuration/base.yaml");

    // Insert code for variable holding default values
    let settings = Settings::default();

    // Insert code for serializing variable
    let s = serde_yaml::to_string(&settings)?;

    // Creates file in directory
    let mut f = match std::fs::File::create(&path) {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists if overwrite => Ok(OpenOptions::new().write(true).truncate(true).open(path)?),
            ErrorKind::AlreadyExists => Err(WriteConfigError::AlreadyExists),
            _ => Err(WriteConfigError::IoError(error)),
        },
    }?;

    f.write_all(&s.as_bytes())?;

    Ok(())
}
