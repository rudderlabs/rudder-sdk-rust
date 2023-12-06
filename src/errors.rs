//! Errors which may arise from this crate.

use thiserror::Error;

/// An enum of errors this crate may produce. These are compatible with
/// `failure` errors.
#[derive(Debug, Error)]
pub enum Error {
    /// The given message is too large to be sent to RudderStack's API.
    #[error("message too large")]
    MessageTooLarge(String),

    #[error("Invalid request")]
    InvalidRequest(String),

    #[error("Error sending request")]
    SendRequestError(#[from] reqwest::Error),
}
