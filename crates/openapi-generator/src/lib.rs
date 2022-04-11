//! A convenient library to run openapi-generator from build.rs script with all
//! the template files in the openapi-contrib repository.

mod find;
mod generator;

pub use self::find::*;
pub use self::generator::*;
