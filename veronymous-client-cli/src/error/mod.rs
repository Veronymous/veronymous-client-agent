use thiserror::Error;
use veronymous_client::error::VeronymousClientError;

#[derive(Clone, Debug, Error)]
pub enum CliClientError {
    #[error("{0}")]
    ParseError(String),

    #[error("{0}")]
    EncodingError(String),

    #[error("{0}")]
    IoError(String),

    #[error("{0}")]
    ReadFileError(String),

    #[error("{0}")]
    InitializationError(String),

    #[error("{0}")]
    CommandError(String),

    #[error("Veronymous client error.")]
    VeronymousClientError(VeronymousClientError),
}
