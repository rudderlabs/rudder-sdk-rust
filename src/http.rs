use crate::client::Client;
use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use log::debug;
use std::time::Duration;
use failure::Error;
use crate::config::Config;

use crate::utils;

pub struct RudderHttpClient {
    client: reqwest::blocking::Client,
    data_plane_url: String,
    write_key: String,
    config: Config

}
const DEFAULT_TIMEOUT_IN_SECS: u64 = 10;
const DEFAULT_DATA_PLANE_URL: &str = "https://app.rudderstack.com";

impl Default for RudderHttpClient {
    fn default() -> Self {
        RudderHttpClient {
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::new(DEFAULT_TIMEOUT_IN_SECS, 0))
                .build()
                .unwrap(),
            data_plane_url: DEFAULT_DATA_PLANE_URL.to_string(),
            write_key: String::new(),
            config: Config::default()
        }
    }
}

impl Client for RudderHttpClient {

    // Function that will receive user event data
    // and after validation
    // modify it to Ruddermessage format and send the event to data plane url
    fn send_compat(&self, write_key: &str, msg: &Message) -> Result<(), failure::Error> {
        msg.assert_valid_user_id_or_anonymous_id()?;
        msg.assert_valid_context()?;
        // match the type of event and fetch the proper API path
        let path = match msg {
            Message::Identify(b_) => "/v1/identify",
            Message::Track(b_) => "/v1/track",
            Message::Page(b_) => "/v1/page",
            Message::Screen(b_) => "/v1/screen",
            Message::Group(b_) => "/v1/group",
            Message::Alias(b_) => "/v1/alias",
            Message::Batch(b_) => "/v1/batch",
        };

            // match the type of event and manipulate the payload to rudder format
            let rudder_message = match msg {
                Message::Identify(b_) => utils::parse_identify(b_),
                Message::Track(b_) => utils::parse_track(b_),
                Message::Page(b_) => utils::parse_page(b_),
                Message::Screen(b_) => utils::parse_screen(b_),
                Message::Group(b_) => utils::parse_group(b_),
                Message::Alias(b_) => utils::parse_alias(b_),
                Message::Batch(b_) => utils::parse_batch(b_),
            };

            // final payload
            debug!("rudder_message: {:#?}", rudder_message);
            // Send the payload to the data plane url
            let res = self
                .client
                .post(&format!("{}{}", self.data_plane_url, path))
                .basic_auth(write_key.to_string(), Some(""))
                .json(&rudder_message)
                .send()?;

            // handle error and send response
            if res.status() == 200 {
                Ok(())
            } else {
                Err(AnalyticsError::InvalidRequest(String::from(format!(
                    "status code: {}, message: Invalid request",
                    res.status()
                )))
                .into())
            }

    }

    fn send(&self, message: &Message) -> Result<(), Error> {
        todo!()
    }

    fn flush(&self,) -> Result<(), Error> {
        todo!()
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Mock};
    # [test]
    fn test_initialisation() {
        let rudder_client = RudderHttpClient{
            ..Default::default()
        };
        assert_eq!(rudder_client.write_key, "".to_string());
        assert_eq!(rudder_client.data_plane_url, DEFAULT_DATA_PLANE_URL.to_string());
        assert_eq!(rudder_client.config, Config{..Default::default()});

        let rudder_client = RudderHttpClient{
            write_key: "write_key".to_string(),
            ..Default::default()
        };
        assert_eq!(rudder_client.write_key, "write_key".to_string())
    }
    #[test]
    fn test_send_success() {
        let rudder_client = RudderHttpClient{client: reqwest::blocking::Client::builder()
            .build()
            .unwrap(),
            data_plane_url: mockito::server_url(),
            ..Default::default()};
        let _m = mock_rudder_server("POST", "/batch", 200, "");
    }


    fn mock_rudder_server(http_method : &str, path : &str, expected_status : usize, response_body: &str) -> Mock {
         mock(http_method, path)
            .with_status(expected_status)
            .with_header("content-type", "application/json")
            // .with_header("x-api-key", "1234")
            .with_body(response_body)
            .create()

        // Any calls to GET /hello beyond this line will respond with 201, the
        // `content-type: text/plain` header and the body "world".
    }



}
