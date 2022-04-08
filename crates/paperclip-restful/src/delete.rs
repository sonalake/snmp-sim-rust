use actix_web::body::BoxBody;
use actix_web::{web::Json, HttpRequest, HttpResponse, Responder};
use paperclip::actix::{NoContent, OperationModifier};
use serde::Serialize;

use paperclip::v2::models::DefaultOperationRaw;
use paperclip::v2::models::DefaultSchemaRaw;
use paperclip::v2::schema::Apiv2Schema as Apiv2SchemaTrait;
use std::collections::BTreeMap;

pub enum DeleteResponse<T: Serialize + Apiv2SchemaTrait> {
    Deleted(T),
    NoContent,
}

impl<Schema: Serialize + Apiv2SchemaTrait> Apiv2SchemaTrait for DeleteResponse<Schema> {}
impl<Schema: Serialize + Apiv2SchemaTrait> OperationModifier for DeleteResponse<Schema> {
    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        Json::<Schema>::update_definitions(map);
        NoContent::update_definitions(map);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        Json::<Schema>::update_response(op);
        NoContent::update_response(op);
    }
}

impl<T: Serialize + Apiv2SchemaTrait> Responder for DeleteResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Self::Deleted(d) => Json(d).respond_to(req).map_into_boxed_body(),
            Self::NoContent => NoContent {}.respond_to(req),
        }
    }
}
