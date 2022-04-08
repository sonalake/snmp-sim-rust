use actix_web::web;
use actix_web::HttpResponse;
use paperclip::actix::get;
use paperclip::actix::web::ServiceConfig;
use rust_embed::RustEmbed;
use serde::Serialize;
use std::borrow::Cow;
use tracing::warn;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct ServiceAssets;

#[derive(Serialize)]
pub struct OpenApiJson(pub serde_json::Value);

#[get("/openapi.yaml")]
#[tracing::instrument(level = "TRACE", name = "get_openapi_json", skip(spec))]
fn get_openapi_json(spec: web::Data<OpenApiJson>) -> HttpResponse {
    HttpResponse::Ok().json(spec)
}

#[tracing::instrument(level = "TRACE", name = "swagger_ui_config", skip(cfg))]
pub fn swagger_ui_config(cfg: &mut ServiceConfig) {
    use swagger_ui::Assets;

    // only register if running in actix
    for file in Assets::iter() {
        // we'll supply our own index.html later, so don't include it!
        if file == "index.html" {
            continue;
        }

        let path = format!("/swagger/{}", file.as_ref());
        let content = Assets::get(&file).unwrap();
        cfg.service_raw(web::resource(&path).route(web::get().to(move || match content.clone() {
            Cow::Borrowed(b) => HttpResponse::Ok().body(b),
            Cow::Owned(owned) => HttpResponse::Ok().body(owned),
        })));
    }

    cfg.service_raw(web::resource("/swagger").route(web::get().to(move || {
        let content = ServiceAssets::get("swagger_ui.html").unwrap().data;
        match content {
            Cow::Borrowed(b) => HttpResponse::Ok().body(b),
            Cow::Owned(b) => HttpResponse::Ok().body(b),
        }
    })));
    cfg.service_raw(get_openapi_json);
}
