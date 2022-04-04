use shared_common::error_chain_fmt;
use std::convert::Infallible;

#[derive(thiserror::Error)]
pub enum CodecError {
    #[error("{0}")]
    Decoder(rasn::ber::de::Error),

    #[error("{0}")]
    Encoder(rasn::ber::enc::Error),

    #[error("Invalid protocol version {0}")]
    InvalidVersion(u32),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<Infallible> for CodecError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to CodecError")
    }
}

impl From<std::io::Error> for CodecError {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}
