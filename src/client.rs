use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use failure::Error;
use std::time::Duration;

pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::Client,
}

impl RudderAnalytics {
    pub fn load(write_key: String, data_plane_url: String) -> RudderAnalytics {
        RudderAnalytics {
            write_key,
            data_plane_url,
            client: reqwest::Client::builder()
                .connect_timeout(Duration::new(10, 0))
                .build()
                .unwrap(),
        }
    }

    pub fn send(&self, msg: &Message) -> Result<(), Error> {
        let path = match msg {
            Message::Identify(_) => "/v1/identify",
            Message::Track(_) => "/v1/track",
            Message::Page(_) => "/v1/page",
            Message::Screen(_) => "/v1/screen",
            Message::Group(_) => "/v1/group",
            Message::Alias(_) => "/v1/alias",
            Message::Batch(_) => "/v1/batch",
        };

        let id_err_msg = String::from("Either of user_id or anonymous_id is required");
        let empty_msg = String::from("");
        let mut error_msg: String = String::from("");

        match msg {
            Message::Identify(b_) => {
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                }
            }
            Message::Track(b_) => {
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                }
            }
            Message::Page(b_) => {
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                }
            }
            Message::Screen(b_) => {
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                }
            }
            Message::Group(b_) => {
                if b_.user_id == Option::None && b_.anonymous_id == Option::None {
                    error_msg = id_err_msg;
                } else {
                    error_msg = empty_msg;
                }
            }
            Message::Alias(_) => {}
            Message::Batch(_) => {}
        };

        if error_msg == String::from("") {
            let res = self
                .client
                .post(&format!("{}{}", self.data_plane_url, path))
                .basic_auth(self.write_key.to_string(), Some(""))
                .json(&msg)
                .send()?;

            if res.status() == 200 {
                return Ok(());
            } else {
                return Err(AnalyticsError::InvalidRequest(String::from(format!(
                    "status code: {}, message: Invalid request",
                    res.status()
                )))
                .into());
            }
        } else {
            return Err(AnalyticsError::InvalidRequest(error_msg).into());
        }
    }
}
