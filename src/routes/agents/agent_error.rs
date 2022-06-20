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
pub enum AgentError {
    #[error("Forbidden")]
    Forbidden,

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    //#[error(transparent)]
    #[error("{0}")]
    Unexpected(String),
}

impl std::fmt::Debug for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for AgentError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to AgentError")
    }
}

impl ResponseError for AgentError {
    fn status_code(&self) -> StatusCode {
        match self {
            AgentError::Forbidden => StatusCode::FORBIDDEN,
            AgentError::Validation(_) => StatusCode::BAD_REQUEST,
            AgentError::NotFound(_) => StatusCode::NOT_FOUND,
            AgentError::Conflict(_) => StatusCode::CONFLICT,
            AgentError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DomainError> for AgentError {
    fn from(domain_error: DomainError) -> Self {
        match domain_error {
            DomainError::Validation(details) => AgentError::Validation(details),
            DomainError::NotFound(details) => AgentError::NotFound(details),
            DomainError::Conflict(details) => AgentError::Conflict(details),
            DomainError::Unexpected(details) => AgentError::Unexpected(details.to_string()),
            DomainError::UdpServerError(details) => AgentError::Unexpected(details.to_string()),
        }
    }
}
