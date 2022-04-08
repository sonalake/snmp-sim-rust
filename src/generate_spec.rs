use crate::app;
use crate::configuration;
use actix_web::App;
use paperclip::actix::OpenApiExt;
use std::path::PathBuf;

pub fn generate_spec(path: Option<PathBuf>) -> serde_json::Value {
    let settings = configuration::get_configuration(path).expect("failed to create settings");
    let app = App::new().wrap_api();
    let app = app::register_services(app, &settings.application.uri_prefix).with_spec(app::spec_modifier);
    serde_json::to_value(app.into_spec()).unwrap()
}
