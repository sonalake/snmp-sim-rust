use crate::domain::DomainError;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use paperclip::actix::api_v2_errors;
use shared_common::error_chain_fmt;
use std::convert::Infallible;

#[api_v2_errors(
    code = 400,
    description = "Bad request format",
    code = 404,
    description = "Not Found",
    code = 409,
    description = "Conflict",
    code = 500,
    description = "Internal server error"
)]
#[derive(thiserror::Error)]
pub enum DeviceError {
    #[error("Forbidden")]
    Forbidden,

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for DeviceError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to DeviceError")
    }
}

impl ResponseError for DeviceError {
    fn status_code(&self) -> StatusCode {
        match self {
            DeviceError::Forbidden => StatusCode::FORBIDDEN,
            DeviceError::Validation(_) => StatusCode::BAD_REQUEST,
            DeviceError::NotFound(_) => StatusCode::NOT_FOUND,
            DeviceError::Conflict(_) => StatusCode::CONFLICT,
            DeviceError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DomainError> for DeviceError {
    fn from(domain_error: DomainError) -> Self {
        match domain_error {
            DomainError::Validation(details) => DeviceError::Validation(details),
            DomainError::NotFound(details) => DeviceError::NotFound(details),
            DomainError::Conflict(details) => DeviceError::Conflict(details),
            DomainError::Unexpected(details) => DeviceError::Unexpected(details),
            DomainError::UdpServerError(details) => DeviceError::Conflict(details.to_string()),
        }
    }
}
