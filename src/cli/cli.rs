use configuration::Settings;
use clap::Parser;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::ErrorKind;

// fn write_data_to_file() -> Result<(), serde::yaml::Error> {

//     // Assigns path to base.yaml file to variable. Immutable
//     let path = Path::new("./configuration/base.yaml")

//     // Insert code for variable holding default values
//     let settings = Settings::default();

//     // Insert code for serializing variable
//     let s = serde_yaml::to_string(&settings)?;

//     // Assigns the file to mutable variable
//     let mut buffer = File::create("base.yaml")?;

//     // If file does not exist, write serialized data to it
//     if !std::path::Path::new(path).exists(){

//         // Assigns the file to mutable variable
//         let mut buffer = File::create(path)?;

//         buffer.write(s)?;

//         Ok(())
//     }

//     Ok(())
// }

fn generate_default_config(overwrite:bool) -> Result<(), serde::yaml::Error> {

    // Path to file
    let path = Path::new("./configuration/base.yaml");

    // Insert code for variable holding default values
    let settings = Settings::default();

    // Insert code for serializing variable
    let s = serde_yaml::to_string(&settings)?;
    
    // Creates file in directory
    let f = File::create(&path);

    // match statement, if file creates
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("This file already exists {:?}", error),
    // };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => match File::open(path) {
                Ok(fo) => {
                    if overwrite == True {
                        fo,
                    };
                }
                Err(err) => panic!("This file already exists {:?}", err);
            }
        },
    };
}
