use actix_web::body::EitherBody;
use actix_web::{web::Json, HttpRequest, HttpResponse, Responder};
use paperclip::actix::OperationModifier;
use serde::Serialize;

use paperclip::v2::models::DefaultOperationRaw;
use paperclip::v2::models::DefaultSchemaRaw;
use paperclip::v2::schema::Apiv2Schema as Apiv2SchemaTrait;
use std::collections::BTreeMap;

pub enum GetResponse<T: Serialize + Apiv2SchemaTrait> {
    Ok(T),
}

impl<Schema: Serialize + Apiv2SchemaTrait> Apiv2SchemaTrait for GetResponse<Schema> {}
impl<Schema: Serialize + Apiv2SchemaTrait> OperationModifier for GetResponse<Schema> {
    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        Json::<Schema>::update_definitions(map);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        Json::<Schema>::update_response(op);
    }
}

impl<T: Serialize + Apiv2SchemaTrait> Responder for GetResponse<T> {
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Self::Ok(d) => Json(d).respond_to(req),
        }
    }
}
