//! Error types for p2p-messenger.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessengerError {
    #[error("network error: {0}")]
    Network(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, MessengerError>;
