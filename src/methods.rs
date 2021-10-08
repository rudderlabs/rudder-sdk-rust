//! Interfaces to the RudderStack APIs.

use crate::message::{Identify, Track, Page, Screen, Group, Alias, Batch};
use failure::Error;

/// `Methods` is a trait representing the HTTP transport layer of the analytics library.
pub trait Methods {
    /// Send a single message to RudderStack using the given write key.
    fn identify(&self, msg: &Identify) -> Result<(), Error>;

    fn track(&self, msg: &Track) -> Result<(), Error>;

    fn page(&self, msg: &Page) -> Result<(), Error>;

    fn screen(&self, msg: &Screen) -> Result<(), Error>;

    fn group(&self, msg: &Group) -> Result<(), Error>;

    fn alias(&self, msg: &Alias) -> Result<(), Error>;

    fn batch(&self, msg: &Batch) -> Result<(), Error>;
}
