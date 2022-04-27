use openapi_generator::OpenApiGenerator;
use openapi_generator::OpenApiGeneratorTrait;
use service::generate_spec;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let base_path = String::from_utf8_lossy(
        &Command::new("git")
            .args(&["rev-parse", "--show-toplevel"])
            .output()
            .unwrap()
            .stdout,
    )
    .trim()
    .to_string();

    let openapi_spec_path = format!("{}/docs/openapi.json", &base_path);

    let spec = generate_spec(Some(PathBuf::from(&base_path)));
    fs::write(&openapi_spec_path, spec.to_string()).unwrap();

    let name_lib = format!("{}-lib", env!("CARGO_PKG_NAME"));
    let package_version = env!("CARGO_PKG_VERSION");
    let output_path_lib = format!("./{}", name_lib);

    let gen = OpenApiGenerator::builder()
        .generator("rust")
        .output_path(output_path_lib)
        .package_name(name_lib)
        .package_version(package_version)
        .spec_path(openapi_spec_path)
        .build();

    let output = gen.generate().unwrap();
    if !output.status.success() {
        panic!(
            "unsuccessful API generate: {:?}\nstderr: {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}
