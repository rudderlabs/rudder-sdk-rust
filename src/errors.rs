//! Errors which may arise from this crate.

use failure::Fail;

/// An enum of errors this crate may produce. These are compatible with
/// `failure` errors.
#[derive(Debug, Fail)]
pub enum Error {
    /// The given message is too large to be sent to RudderStack's API.
    #[fail(display = "message too large")]
    MessageTooLarge(String),

    /// The given batch is too large to be sent to RudderStack's API.
    #[fail(display = "batch too large")]
    BatchTooLarge(String),

    #[fail(display = "Invalid request")]
    InvalidRequest(String),
}
