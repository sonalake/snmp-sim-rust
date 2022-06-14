use core::{any::Any, fmt};

use std::error;

use tokio::sync::AcquireError;

/// Collection of error type from supervisor.
pub enum SupervisorError {
    /// supervisor worker is closed unexpectedly.
    WorkerClosed,
    /// thread error from supervisor worker.
    WorkerJoin(Box<dyn Any + Send>),
    /// wrap io error.
    Io(std::io::Error),
    /// wrap error object.
    Std(Box<dyn error::Error + Send + Sync>),
}

impl From<AcquireError> for SupervisorError {
    fn from(_: AcquireError) -> Self {
        Self::WorkerClosed
    }
}

impl From<Box<dyn Any + Send>> for SupervisorError {
    fn from(e: Box<dyn Any + Send>) -> Self {
        Self::WorkerJoin(e)
    }
}

impl fmt::Debug for SupervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WorkerClosed => write!(f, "Worker thread closed unexpectedly"),
            Self::WorkerJoin(ref e) => {
                let msg = e.downcast_ref::<&str>().unwrap_or(&"(..)");
                write!(f, "Worker thread join error: {:?}", msg)
            }
            Self::Io(ref e) => write!(f, "{:?}", e),
            Self::Std(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Display for SupervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::WorkerClosed => write!(f, "Supervisor worker thread closed unexpectedly"),
            Self::WorkerJoin(ref e) => {
                let msg = e.downcast_ref::<&str>().unwrap_or(&"(..)");
                write!(f, "Worker thread join error: {}", msg)
            }
            Self::Io(ref e) => write!(f, "{}", e),
            Self::Std(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for SupervisorError {}

impl PartialEq for SupervisorError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::WorkerJoin(_), Self::WorkerJoin(_)) => true,
            (Self::WorkerClosed, Self::WorkerClosed) => true,
            (Self::Std(_), Self::Std(_)) => true,
            (Self::Io(ref e), Self::Io(ref e2)) => e.kind() == e2.kind(),
            _ => false,
        }
    }
}
