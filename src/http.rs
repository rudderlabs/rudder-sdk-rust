//! Low-level HTTP bindings to the Rudderstack tracking API.

use crate::client::Client;
use crate::message::Message;
use failure::Error;
use std::time::Duration;

/// A client which synchronously sends single messages to the Rudderstack API.
pub struct HttpClient {
    client: reqwest::Client,
    host: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient {
            client: reqwest::Client::builder()
                .connect_timeout(Some(Duration::new(10, 0)))
                .build()
                .unwrap(),
            host: "https://hosted.rudderlabs.com".to_owned(),
        }
    }
}

impl HttpClient {
    /// Construct a new `HttpClient` from a `reqwest::Client` and a Rudderstack API
    /// scheme and host.
    ///
    /// If you don't care to re-use an existing `reqwest::Client`, you can use
    /// the `Default::default` value, which will send events to
    /// `https://hosted.rudderlabs.com`.
    pub fn new(client: reqwest::Client, host: String) -> HttpClient {
        HttpClient { client, host }
    }
}

impl Client for HttpClient {

    fn send(&self, write_key: &str, msg: &Message) -> Result<(), Error> {
        println!("Printing debug info: step-2");

        // println!("Printing debug info...{:?}",msg);
        let path = match msg {
            Message::Identify(_) => "/v1/identify",
            Message::Track(_) => "/v1/track",
            Message::Page(_) => "/v1/page",
            Message::Screen(_) => "/v1/screen",
            Message::Group(_) => "/v1/group",
            Message::Alias(_) => "/v1/alias",
            Message::Batch(_) => "/v1/batch",
        };

        self.client
            .post(&format!("{}{}", self.host, path))
            .basic_auth(write_key, Some(""))
            .json(msg)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}
