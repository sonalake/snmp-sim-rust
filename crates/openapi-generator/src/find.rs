use anyhow::Result;
use std::process::Command;

pub fn find_openapi_generator() -> Result<&'static str> {
    if Command::new("openapi-generator")
        .arg("version")
        .output()
        .is_ok()
    {
        return Ok("openapi-generator");
    }

    if Command::new("openapi-generator-cli")
        .arg("version")
        .output()
        .is_ok()
    {
        return Ok("openapi-generator-cli");
    }

    Err(anyhow::anyhow!(
        "Please install openapi-generator. See https://openapi-generator.tech/docs/installation for more information"
    ))
}
