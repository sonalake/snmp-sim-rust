use shared_common::error_chain_fmt;
use std::convert::Infallible;

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
#[derive(thiserror::Error)]
pub(crate) enum DomainError {
    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for DomainError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to DomainError")
    }
}

impl From<sea_orm::DbErr> for DomainError {
    fn from(db_error: sea_orm::DbErr) -> Self {
        match db_error {
            sea_orm::DbErr::Conn(_) => DomainError::Unexpected(db_error.into()),
            sea_orm::DbErr::Exec(_) => DomainError::Unexpected(db_error.into()),
            sea_orm::DbErr::Query(_) => DomainError::Unexpected(db_error.into()),
            sea_orm::DbErr::RecordNotFound(_) => DomainError::NotFound(db_error.to_string()),
            sea_orm::DbErr::Custom(ref details) if details.starts_with("CONFLICT") => {
                DomainError::Conflict(db_error.to_string())
            }
            sea_orm::DbErr::Custom(ref details) if details.starts_with("VALIDATION") => {
                DomainError::Validation(db_error.to_string())
            }
            sea_orm::DbErr::Custom(_) => DomainError::Unexpected(db_error.into()),
            sea_orm::DbErr::Type(_) => DomainError::Validation(db_error.to_string()),
            sea_orm::DbErr::Json(json_error) => DomainError::Validation(json_error),
            sea_orm::DbErr::Migration(_) => DomainError::Unexpected(db_error.into()),
        }
    }
}
