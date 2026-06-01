use crate::errors::Error as AnalyticsError;
use crate::message::Message;
use crate::retry::{is_error_retryable, is_status_retryable, retry_delay, RetryConfig};
use crate::ruddermessage::Ruddermessage;
use crate::utils;
use log::debug;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde_json::Value;
use std::thread;
use std::time::Duration;

// Rudderanalytics client
pub struct RudderAnalytics {
    pub write_key: String,
    pub data_plane_url: String,
    pub client: reqwest::blocking::Client,
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

    // Function that will receive user event data
    // and after validation
    // modify it to Ruddermessage format and send the event to data plane url
    pub fn send(&self, msg: &Message) -> Result<(), AnalyticsError> {
        self.send_with_retry_config(msg, &RetryConfig::default())
    }

    /// Send a message once, without retrying transient failures.
    pub fn send_once(&self, msg: &Message) -> Result<(), AnalyticsError> {
        self.send_with_retry_config(msg, &RetryConfig::disabled())
    }

    /// Send a message with the supplied retry configuration.
    pub fn send_with_retry_config(
        &self,
        msg: &Message,
        retry_config: &RetryConfig,
    ) -> Result<(), AnalyticsError> {
        let path = validate_and_path(msg)?;
        let rudder_message = parse_rudder_message(msg);
        let mut retries = 0;

        debug!("rudder_message: {:#?}", rudder_message);

        loop {
            let attempt = retries + 1;
            match self.post(path, &rudder_message) {
                Ok(res) => {
                    let status = res.status();
                    if status.is_success() {
                        return Ok(());
                    }

                    let can_retry = retry_config.enabled
                        && is_status_retryable(status)
                        && retries < retry_config.max_retries;

                    if !can_retry {
                        return Err(AnalyticsError::InvalidRequest(invalid_request_message(
                            status,
                            attempt,
                            retry_config.enabled
                                && is_status_retryable(status)
                                && retries >= retry_config.max_retries,
                        )));
                    }

                    retries += 1;
                    let delay = retry_delay(retry_config, retries, Some(res.headers()));
                    debug!(
                        "retrying RudderStack request after status {} in {:?} (attempt {} of {})",
                        status,
                        delay,
                        retries + 1,
                        retry_config.max_retries + 1
                    );
                    sleep_retry_delay(delay);
                }
                Err(err) => {
                    let can_retry = retry_config.enabled
                        && is_error_retryable(&err)
                        && retries < retry_config.max_retries;

                    if !can_retry {
                        return Err(AnalyticsError::SendRequestError(err));
                    }

                    retries += 1;
                    let delay = retry_delay(retry_config, retries, None);
                    debug!(
                        "retrying RudderStack request after transport error in {:?} (attempt {} of {})",
                        delay,
                        retries + 1,
                        retry_config.max_retries + 1
                    );
                    sleep_retry_delay(delay);
                }
            }
        }
    }

    fn post(&self, path: &str, rudder_message: &Ruddermessage) -> Result<Response, reqwest::Error> {
        self.client
            .post(&format!("{}{}", self.data_plane_url, path))
            .basic_auth(self.write_key.to_string(), Some(""))
            .json(rudder_message)
            .send()
    }
}

fn validate_and_path(msg: &Message) -> Result<&'static str, AnalyticsError> {
    match msg {
        Message::Identify(b_) => {
            validate_user_or_anonymous_id(&b_.user_id, &b_.anonymous_id)?;
            validate_context(&b_.context)?;
            Ok("/v1/identify")
        }
        Message::Track(b_) => {
            validate_user_or_anonymous_id(&b_.user_id, &b_.anonymous_id)?;
            validate_context(&b_.context)?;
            Ok("/v1/track")
        }
        Message::Page(b_) => {
            validate_user_or_anonymous_id(&b_.user_id, &b_.anonymous_id)?;
            validate_context(&b_.context)?;
            Ok("/v1/page")
        }
        Message::Screen(b_) => {
            validate_user_or_anonymous_id(&b_.user_id, &b_.anonymous_id)?;
            validate_context(&b_.context)?;
            Ok("/v1/screen")
        }
        Message::Group(b_) => {
            validate_user_or_anonymous_id(&b_.user_id, &b_.anonymous_id)?;
            validate_context(&b_.context)?;
            Ok("/v1/group")
        }
        Message::Alias(b_) => {
            validate_context(&b_.context)?;
            Ok("/v1/alias")
        }
        Message::Batch(b_) => {
            validate_context(&b_.context)?;
            Ok("/v1/batch")
        }
    }
}

fn validate_user_or_anonymous_id(
    user_id: &Option<String>,
    anonymous_id: &Option<String>,
) -> Result<(), AnalyticsError> {
    if user_id.is_none() && anonymous_id.is_none() {
        Err(AnalyticsError::InvalidRequest(String::from(
            "Either of user_id or anonymous_id is required",
        )))
    } else {
        Ok(())
    }
}

fn validate_context(context: &Option<Value>) -> Result<(), AnalyticsError> {
    if context
        .as_ref()
        .map(|context| utils::check_reserved_keywords_conflict(context.clone()))
        .unwrap_or(false)
    {
        Err(AnalyticsError::InvalidRequest(String::from(
            "Reserve keyword present in context",
        )))
    } else {
        Ok(())
    }
}

fn parse_rudder_message(msg: &Message) -> Ruddermessage {
    match msg {
        Message::Identify(b_) => utils::parse_identify(b_),
        Message::Track(b_) => utils::parse_track(b_),
        Message::Page(b_) => utils::parse_page(b_),
        Message::Screen(b_) => utils::parse_screen(b_),
        Message::Group(b_) => utils::parse_group(b_),
        Message::Alias(b_) => utils::parse_alias(b_),
        Message::Batch(b_) => utils::parse_batch(b_),
    }
}

fn invalid_request_message(status: StatusCode, attempts: u32, retries_exhausted: bool) -> String {
    if retries_exhausted {
        format!(
            "status code: {}, attempts: {}, message: retries exhausted",
            status, attempts
        )
    } else {
        format!(
            "status code: {}, attempts: {}, message: Invalid request",
            status, attempts
        )
    }
}

fn sleep_retry_delay(delay: Duration) {
    if delay > Duration::from_secs(0) {
        thread::sleep(delay);
    }
}
