use std::convert::Infallible;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UdpServerError {
    #[error("Start failed with error {error:?}")]
    StartFailed { error: std::io::Error },

    #[error("Device is Already Running")]
    DeviceAlreadyRunning,

    #[error("Device is Not Running")]
    DeviceNotRunning,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    MailboxError(#[from] actix::MailboxError),
}

impl From<Infallible> for UdpServerError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to UdpServerError")
    }
}
