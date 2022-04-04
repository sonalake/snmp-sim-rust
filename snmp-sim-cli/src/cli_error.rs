use color_eyre::Report;
use shared_common::error_chain_fmt;
use std::convert::Infallible;

#[derive(thiserror::Error)]
pub enum CliError {
    #[error("{0}")]
    Eyre(#[from] Report),

    #[error(transparent)]
    CommandHandler(#[from] anyhow::Error),
}

impl std::fmt::Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for CliError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to CliError")
    }
}
