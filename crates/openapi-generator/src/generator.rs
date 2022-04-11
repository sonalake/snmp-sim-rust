use anyhow::Result;
use std::process::{Command, Output};
use typed_builder::TypedBuilder;

pub trait OpenApiGeneratorTrait {
    /// Returns Ok(generator log) if successful
    fn generate(&self) -> Result<Output>;
}

#[derive(TypedBuilder)]
pub struct OpenApiGenerator {
    #[builder(setter(into))]
    generator: String,

    #[builder(setter(into))]
    output_path: String,

    #[builder(default, setter(into))]
    package_name: String,

    #[builder(default, setter(into))]
    package_version: String,

    #[builder(setter(into))]
    spec_path: String,
}

impl OpenApiGenerator {
    fn command_line(&self) -> Vec<String> {
        [
            "generate",
            "-g",
            &self.generator,
            "-o",
            &self.output_path,
            "--package-name",
            &self.package_name,
            "--additional-properties",
            &format!("packageVersion={}", self.package_version),
            "-i",
            &self.spec_path,
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }
}

impl OpenApiGeneratorTrait for OpenApiGenerator {
    fn generate(&self) -> Result<Output> {
        let cmdline = self.command_line();
        Command::new(crate::find_openapi_generator()?)
            .args(&cmdline)
            .output()
            .map_err(anyhow::Error::from)
    }
}
