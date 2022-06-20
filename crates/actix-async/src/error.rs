use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(PartialEq)]
pub enum ActixAsyncError {
    /// actor's channel is closed. happens when actor is shutdown.
    Closed,

    /// failed to send message to actor in time.
    SendTimeout,

    /// failed to receive result from actor in time.
    ReceiveTimeout,

    /// fail to receive result for given message. happens when actor is blocked or the
    /// thread it runs on panicked.
    Receiver,

    #[cfg(feature = "tokio-rt")]
    SuperVisor(super::supervisor::SupervisorError),
}

impl Debug for ActixAsyncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut fmt = f.debug_struct("ActixAsyncError");

        match self {
            ActixAsyncError::Closed => fmt
                .field("cause", &"Closed")
                .field("description", &"Actor is already closed")
                .finish(),
            ActixAsyncError::SendTimeout => fmt
                .field("cause", &"SendTimeout")
                .field(
                    "description",
                    &"MessageRequest is timed out. (Failed to send message to actor in time.)",
                )
                .finish(),
            ActixAsyncError::ReceiveTimeout => fmt
                .field("cause", &"ReceiveTimeout")
                .field(
                    "description",
                    &"MessageRequest is timed out. (Failed to receive result from actor in time.)",
                )
                .finish(),
            ActixAsyncError::Receiver => fmt
                .field("cause", &"Receive")
                .field("description", &"Fail to receive result for given message.")
                .finish(),

            #[cfg(feature = "tokio-rt")]
            ActixAsyncError::SuperVisor(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl Display for ActixAsyncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            #[cfg(feature = "tokio-rt")]
            ActixAsyncError::SuperVisor(ref e) => write!(f, "{}", e),
            this => write!(f, "{:?}", this),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ActixAsyncError {}
