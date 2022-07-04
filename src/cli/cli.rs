use configuration::Settings;
use clap::Parser;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), serde::yaml::Error> {

    // Assigns path to base.yaml file to variable. Immutable
    let path = Path::new("./configuration/base.yaml")

    // Insert code for variable holding default values
    let settings = Settings::default();

    // Insert code for serializing variable
    let s = serde_yaml::to_string(&settings)?;

    // Assigns the file to mutable variable
    let mut buffer = File::create("base.yaml")?;

    // If file does not exist, write serialized data to it
    if !std::path::Path::new(path).exists(){
        
        // Assigns the file to mutable variable
        let mut buffer = File::create("base.yaml")?;

        buffer.write(s)?;
    }

    Ok(())
}
