use crate::snmp::codec::snmp_codec_error::CodecError;
use shared_common::error_chain_fmt;
use std::convert::Infallible;

#[derive(thiserror::Error)]
pub enum ClientError {
    #[error("Stream closed by peer")]
    StreamClosed,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    CodecError(#[from] CodecError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for ClientError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to ClientError")
    }
}
