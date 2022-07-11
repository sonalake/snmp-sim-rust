#[derive(thiserror::Error)]
pub enum WriteConfigError {
    #[error("This file already exists, use the '-y' argument to overwrite the output.")]
    AlreadyExists,

    #[error(transparent)]
    SerializationError(#[from] serde_yaml::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl std::fmt::Debug for WriteConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        shared_common::error_chain_fmt(self, f)
    }
}
