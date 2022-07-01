use configuration::Settings;
use clap::Parser;

fn main() -> Result<(), serde::yaml::Error> {

    // Insert code for variable holding default values
    let settings = Settings::default();

    // Insert code for serializing variable
    let s = serde_yaml::to_string(&settings)?;

    Ok(())
}
