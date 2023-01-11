use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use crate::utils;
use failure::Error;
use log::debug;
use std::time::Duration;
use crate::config::Config;


pub trait Client {
    fn send_compat(&self, write_key: &str, message: &Message) -> Result<(), Error>;
    fn send(&self, message: &Message) -> Result<(), Error>;
    fn flush(&self) -> Result<(), Error>;
}


#[deprecated(since = "2.0.0", note = "Use `http::RudderHttpClient` instead")]
pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
}
#[deprecated(since = "2.0.0", note = "Use `http::RudderHttpClient` instead")]
impl RudderAnalytics {
    // Function to initialize the Rudderanalytics client with write-key and data-plane-url
    #[deprecated(since = "2.0.0", note = "Use `http::RudderHttpClient.load` instead")]
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        RudderAnalytics {
            write_key,
            data_plane_url,
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::new(10, 0))
                .build()
                .unwrap(),
        }
    }
    #[deprecated(since = "2.0.0", note = "Use `http::RudderHttpClient.send` instead")]
    fn send(&self, msg: &Message) -> Result<(), failure::Error> {
        todo!()

    }
}
