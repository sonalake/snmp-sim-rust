use clap::Parser
use config::Config

fn main() {
    let settings = Config::builder()
        .add_source(File::new("./configuration/base", FileFormat::yaml));

    match settings.build() {
        Ok(config) => {},

        Err(e) => {}
    }
}
