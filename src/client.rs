//! Interfaces to the Rudderstack tracking API.

use crate::message::Message;
use failure::Error;

/// `Client` is a trait representing the HTTP transport layer of the analytics library.
pub trait Client {
    /// Send a single message to Rudderstack using the given write key.
    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error>;
}
