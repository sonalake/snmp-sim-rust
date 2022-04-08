use crate::routes::agents_config;
use actix_web::dev::ServiceFactory;
use paperclip::actix::web::scope;
use paperclip::v2::models::DefaultApiRaw;

pub fn register_services<Sf>(app: paperclip::actix::App<Sf>, uri_prefix: &str) -> paperclip::actix::App<Sf>
where
    Sf: ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
{
    app.service(scope(uri_prefix).configure(agents_config))
}

pub fn spec_modifier(spec: &mut DefaultApiRaw) {
    paperclip_restful::add_json_error(spec);
}
