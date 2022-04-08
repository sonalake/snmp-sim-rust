use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use paperclip::actix::web::Json;
use paperclip::actix::CreatedJson;
use paperclip::actix::OperationModifier;

use paperclip::v2::models::DefaultOperationRaw;
use paperclip::v2::models::DefaultSchemaRaw;
use paperclip::v2::schema::Apiv2Schema as Apiv2SchemaTrait;
use serde::Serialize;
use std::collections::BTreeMap;

pub enum PostResponse<T: Serialize + Apiv2SchemaTrait> {
    Created(T),
    Exists(T),
}

impl<T: Serialize + Apiv2SchemaTrait> Responder for PostResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Self::Created(c) => CreatedJson(c).respond_to(req).map_into_boxed_body(),
            Self::Exists(c) => Json(c).respond_to(req).map_into_boxed_body(),
        }
    }
}

impl<Schema: Serialize + Apiv2SchemaTrait> Apiv2SchemaTrait for PostResponse<Schema> {}

impl<Schema: Serialize + Apiv2SchemaTrait> OperationModifier for PostResponse<Schema> {
    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        Json::<Schema>::update_definitions(map);
        CreatedJson::<Schema>::update_definitions(map);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        Json::<Schema>::update_response(op);
        CreatedJson::<Schema>::update_response(op);
    }
}
