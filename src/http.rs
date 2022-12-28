use std::time::Duration;
use log::debug;
use crate::client::Client;
use crate::message::Message;
use crate::errors::Error as AnalyticsError;

use crate::utils;



pub struct RudderHttpClient{
    client: reqwest::blocking::Client,
    data_plane_url: String,
}
const DEFAULT_TIMEOUT_IN_SECS : u64 = 10;
const DEFAULT_DATA_PLANE_URL : &str = "https://app.rudderstack.com";

impl Default for RudderHttpClient {
    fn default() -> Self {
        RudderHttpClient {
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::new(DEFAULT_TIMEOUT_IN_SECS, 0))
                .build()
                .unwrap(),
            data_plane_url: DEFAULT_DATA_PLANE_URL.to_string(),
        }
    }
}

impl Client for RudderHttpClient {
    // Function that will receive user event data
// and after validation
// modify it to Ruddermessage format and send the event to data plane url
    fn send(&self,write_key: &str, msg: &Message) -> Result<(), failure::Error> {
        let reserve_key_err_msg = String::from("Reserve keyword present in context");
        let id_err_msg = String::from("Either of user_id or anonymous_id is required");
        let empty_msg = String::from("");
        let mut error_msg: String = String::from("");

        // match the type of event and fetch the proper API path
        let path = match msg {
            Message::Identify(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context != Option::None
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/identify"
            }
            Message::Track(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context != Option::None
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/track"
            }
            Message::Page(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context != Option::None
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/page"
            }
            Message::Screen(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context != Option::None
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/screen"
            }
            Message::Group(b_) => {
                // Checking for userId and anonymousId
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                    // Checking conflicts with reserved keywords
                    if b_.context != Option::None
                        && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                    {
                        error_msg = reserve_key_err_msg;
                    }
                }
                "/v1/group"
            }
            Message::Alias(b_) => {
                // Checking conflicts with reserved keywords
                if b_.context != Option::None
                    && utils::check_reserved_keywords_conflict(b_.context.clone().unwrap())
                {
                    error_msg = reserve_key_err_msg;
                }
                "/v1/alias"
            }
            Message::Batch(b_) => "/v1/batch",
        };

        return if error_msg == String::from("") {
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
        } else {
            Err(AnalyticsError::InvalidRequest(error_msg).into())
        };
    }


}

