use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use paperclip::actix::Apiv2Schema;
use paperclip::v2::models::DefaultOperationRaw;
use paperclip::v2::schema::Apiv2Errors;
use serde::Serialize;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct JsonError<E> {
    error: E,
}

#[derive(Serialize, Apiv2Schema)]
pub(crate) struct JsonErrorResponse {
    error: String,
}

impl<E: fmt::Display> From<&JsonError<E>> for JsonErrorResponse {
    fn from(je: &JsonError<E>) -> Self {
        Self {
            error: je.error.to_string(),
        }
    }
}

impl<E> From<E> for JsonError<E> {
    fn from(e: E) -> Self {
        Self { error: e }
    }
}

impl<E: Display> Display for JsonError<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "error: '{}'", self.error)
    }
}

impl<E: ResponseError> ResponseError for JsonError<E> {
    fn status_code(&self) -> StatusCode {
        self.error.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(JsonErrorResponse::from(self))
    }
}

impl<E: Apiv2Errors> Apiv2Errors for JsonError<E> {
    const ERROR_MAP: &'static [(u16, &'static str)] = E::ERROR_MAP;

    fn update_error_definitions(op: &mut DefaultOperationRaw) {
        E::update_error_definitions(op);
    }
}
