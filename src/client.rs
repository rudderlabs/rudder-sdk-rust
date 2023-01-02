use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use crate::utils;
use failure::Error;
use log::debug;
use std::time::Duration;

pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
}
pub trait Client {
    fn send(&self, write_key: &str, message: &Message) -> Result<(), Error>;
}
impl RudderAnalytics {
    // Function to initialize the Rudderanalytics client with write-key and data-plane-url
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
    fn send(&self, msg: &Message) -> Result<(), failure::Error> {
        //TODO: implement this method
        return Ok(());
    }
}
