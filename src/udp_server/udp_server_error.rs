use std::convert::Infallible;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum UdpServerError {
    #[error("Start failed with error {0}")]
    StartFailed(String),

    #[error("Device is Already Running")]
    DeviceAlreadyRunning,

    #[error("Device is Not Running")]
    DeviceNotRunning,

    #[error("MailboxError {0}")]
    MailboxError(String),
}

impl From<Infallible> for UdpServerError {
    fn from(_: Infallible) -> Self {
        unreachable!("could not convert Infallible to UdpServerError")
    }
}
