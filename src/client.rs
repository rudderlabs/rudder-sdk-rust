use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use crate::utils;
use failure::Error;
use log::debug;
use std::time::Duration;

// Rudderanalytics client
pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
}

impl RudderAnalytics {
    /// # Panics
    // Function to initialize the Rudderanalytics client with write-key and data-plane-url
    #[must_use]
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

    // Function that will receive user event data
    // and after validation
    // modify it to Ruddermessage format and send the event to data plane url
    /// # Errors
    /// # Panics
    #[allow(clippy::too_many_lines)]
    pub fn send(&self, message: &Message) -> Result<(), Error> {
        let id_error_message = String::from("Either of user_id or anonymous_id is required");
        let reserve_key_error_message = String::from("Reserve keyword present in context");
        let empty_message = String::new();
        let mut error_message: String = String::new();

        // match the type of event and fetch the proper API path
        let path = match message {
            Message::Identify(identify_message) => {
                // Checking for userId and anonymousId
                if identify_message.user_id.is_none() && identify_message.anonymous_id.is_none() {
                    error_message = id_error_message;
                } else {
                    error_message = empty_message;
                    // Checking conflicts with reserved keywords
                    if identify_message.context.is_some()
                        && utils::check_reserved_keywords_conflict(&identify_message.context.clone().unwrap())
                    {
                        error_message = reserve_key_error_message;
                    }
                }
                "/v1/identify"
            }
            Message::Track(track_message) => {
                // Checking for userId and anonymousId
                if track_message.user_id.is_none() && track_message.anonymous_id.is_none() {
                    error_message = id_error_message;
                } else {
                    error_message = empty_message;
                    // Checking conflicts with reserved keywords
                    if track_message.context.is_some()
                        && utils::check_reserved_keywords_conflict(&track_message.context.clone().unwrap())
                    {
                        error_message = reserve_key_error_message;
                    }
                }
                "/v1/track"
            }
            Message::Page(page_message) => {
                // Checking for userId and anonymousId
                if page_message.user_id.is_none() && page_message.anonymous_id.is_none() {
                    error_message = id_error_message;
                } else {
                    error_message = empty_message;
                    // Checking conflicts with reserved keywords
                    if page_message.context.is_some()
                        && utils::check_reserved_keywords_conflict(&page_message.context.clone().unwrap())
                    {
                        error_message = reserve_key_error_message;
                    }
                }
                "/v1/page"
            }
            Message::Screen(screen_message) => {
                // Checking for userId and anonymousId
                if screen_message.user_id.is_none() && screen_message.anonymous_id.is_none() {
                    error_message = id_error_message;
                } else {
                    error_message = empty_message;
                    // Checking conflicts with reserved keywords
                    if screen_message.context.is_some()
                        && utils::check_reserved_keywords_conflict(&screen_message.context.clone().unwrap())
                    {
                        error_message = reserve_key_error_message;
                    }
                }
                "/v1/screen"
            }
            Message::Group(group_message) => {
                // Checking for userId and anonymousId
                if group_message.user_id.is_none() && group_message.anonymous_id.is_none() {
                    error_message = id_error_message;
                } else {
                    error_message = empty_message;
                    // Checking conflicts with reserved keywords
                    if group_message.context.is_some()
                        && utils::check_reserved_keywords_conflict(&group_message.context.clone().unwrap())
                    {
                        error_message = reserve_key_error_message;
                    }
                }
                "/v1/group"
            }
            Message::Alias(alias_message) => {
                // Checking conflicts with reserved keywords
                if alias_message.context.is_some()
                    && utils::check_reserved_keywords_conflict(&alias_message.context.clone().unwrap())
                {
                    error_message = reserve_key_error_message;
                }
                "/v1/alias"
            }
            Message::Batch(batch_message) => {
                // Checking conflicts with reserved keywords
                if batch_message.context.is_some()
                    && utils::check_reserved_keywords_conflict(&batch_message.context.clone().unwrap())
                {
                    error_message = reserve_key_error_message;
                }
                "/v1/batch"
            }
        };

        if error_message == String::new() {
            // match the type of event and manipulate the payload to rudder format
            let rudder_message = match message {
                Message::Identify(identify_message) => utils::parse_identify(identify_message),
                Message::Track(track_message) => utils::parse_track(track_message),
                Message::Page(page_message) => utils::parse_page(page_message),
                Message::Screen(screen_message) => utils::parse_screen(screen_message),
                Message::Group(group_message) => utils::parse_group(group_message),
                Message::Alias(alias_message) => utils::parse_alias(alias_message),
                Message::Batch(batch_message) => utils::parse_batch(batch_message),
            };

            // final payload
            debug!("rudder_message: {:#?}", rudder_message);

            let request = self
                .client
                .post(format!("{}{}", self.data_plane_url, path))
                .basic_auth(self.write_key.to_string(), Some(""))
                .json(&rudder_message);

            std::thread::spawn(|| {
                let _: Result<_, _> = request.send();
            });

            Ok(())

            // Send the payload to the data plane url
        } else {
            Err(AnalyticsError::InvalidRequest(error_message).into())
        }
    }
}
