//! Utilities for batching up messages.

use crate::{errors, utils};
use crate::errors::Error as AnalyticsError;
use crate::message::{Batch, BatchMessage, Message};
use chrono::prelude::*;
use failure::Error;
use serde_json::Value;
use std::io;

const MAX_MESSAGE_SIZE: usize = 1024 * 32;
const MAX_BATCH_SIZE: usize = 1024 * 512;

// / A batcher can accept messages into an internal buffer, and report when
// / messages must be flushed.
// /
// / The recommended usage pattern looks something like this:
// /
// / ```
// / use rudderanalytics::batcher::Batcher;
// / use rudderanalytics::client::RudderAnalytics;
// / use rudderanalytics::message::{BatchMessage, Track};
// / use serde_json::json;
// /
// / let mut batcher = Batcher::new(None);
// / let rudderAnalytics = RudderAnalytics::load("WRITE-KEY","DATA-PLANE-URL");
// /
// / for i in 0..100 {
// /     let msg = BatchMessage::Track(Track {
// /         user_id: Some(format!("user-{}", i)),
// /         event: "Example".to_owned(),
// /         properties: Some(json!({ "foo": "bar" })),
// /         ..Default::default()
// /     });
// /
// /     // Batcher returns back ownership of a message if the internal buffer
// /     // would overflow.
// /     //
// /     // When this occurs, we flush the batcher, create a new batcher, and add
// /     // the message into the new batcher.
// /     if let Some(msg) = batcher.push(msg).unwrap() {
// /         rudderAnalytics.send(&batcher.into_message()).unwrap();
// /         batcher = Batcher::new(None);
// /         batcher.push(msg).unwrap();
// /     }
// / }
/// ```
///
// / Batcher will attempt to fit messages into maximally-sized batches, thus
// / reducing the number of round trips required with RudderStack's tracking API.
// / However, if you produce messages infrequently, this may significantly delay
// / the sending of messages to RudderStack.
// /
// / If this delay is a concern, it is recommended that you periodically flush
// / the batcher on your own by calling `into_message`.
pub struct Batcher {
    buf: Vec<BatchMessage>,
    byte_count: usize,
    context: Option<Value>,
}

impl Batcher {
    /// Construct a new, empty batcher.
    ///
    /// Optionally, you may specify a `context` that should be set on every
    /// batch returned by `into_message`.
    pub fn new(context: Option<Value>) -> Self {
        Self {
            buf: Vec::new(),
            byte_count: 0,
            context,
        }
    }

    /// Push a message into the batcher.
    ///
    /// Returns `Ok(None)` if the message was accepted and is now owned by the
    /// batcher.
    ///
    /// Returns `Ok(Some(msg))` if the message was rejected because the current
    /// batch would be oversized if this message were accepted. The given
    /// message is returned back, and it is recommended that you flush the
    /// current batch before attempting to push `msg` in again.
    ///
    /// Returns an error if the message is too large to be sent to RudderStack's
    /// API.
    pub fn push(&mut self, msg: BatchMessage) -> Result<Option<BatchMessage>, Error> {
        Batcher::assert_no_reserved_keywords_in_context(&self.context)?;
        let msg = Batcher::updated_message_with_common_context(self.context.clone(), msg);
        let size = Batcher::get_size_of_message(&msg)?;
        Batcher::assert_message_size_before_push(size)?;
        self.update_byte_count_with_next_msg_size(size);
        Batcher::assert_batch_size_before_push(self)?;

        self.buf.push(msg);
        Ok(None)
    }
    fn updated_message_with_common_context(
        context: Option<Value>,
        msg: BatchMessage,
    ) -> BatchMessage {
        match context {
            Some(context) => {
                let mut msg = msg.clone();
                msg.update_context_with(context);
                msg
            }
            None => msg,
        }
    }
    fn get_size_of_message(msg: &BatchMessage) -> Result<usize, errors::Error> {
        match serde_json::to_vec(&msg).map_err(|e| {
            Err(AnalyticsError::InvalidRequest(String::from(format!(
                "status code: 400, message: {}",
                e
            ))))
        }) {
            Ok(v) => Ok(v.len()),
            Err(e) => {
                return e;
            }
        }
    }
    fn assert_message_size_before_push(size: usize) -> Result<(), errors::Error> {
        if size > MAX_MESSAGE_SIZE {
            Err(AnalyticsError::MessageTooLarge(String::from(
                "status code: 400, message: Message too large",
            )))
        } else {
            Ok(())
        }
    }

    fn update_byte_count_with_next_msg_size(&mut self, next_msg_size: usize) {
        self.byte_count += next_msg_size + 1; // +1 to account for Serialized data's extra commas
    }
    fn assert_batch_size_before_push(&mut self) -> Result<(), errors::Error> {
        if self.byte_count > MAX_BATCH_SIZE {
            Err(AnalyticsError::BatchTooLarge(String::from(
                "status code: 400, message: Batch size too large",
            )))
        } else {
            Ok(())
        }
    }
    fn assert_no_reserved_keywords_in_context(context : &Option<Value>)-> Result<(), errors::Error>{
        // Checking conflicts with reserved keywords
        let reserve_key_err_msg = String::from("Reserve keyword present in context");
        return if *context != Option::None
            && utils::check_reserved_keywords_conflict(context.clone().unwrap())
        {
            Err(AnalyticsError::BatchTooLarge(String::from(
                format!("status code: 400, message: {}", reserve_key_err_msg)
            )))
        }else {
            Ok(())
        }
    }

    /// Consumes this batcher and converts it into a message that can be sent to
    /// RudderStack.
    pub fn into_message(self) -> Message {
        Message::Batch(Batch {
            batch: self.buf,
            integrations: None,
            context: self.context,
            original_timestamp: Some(Utc::now()),
        })
    }
}
