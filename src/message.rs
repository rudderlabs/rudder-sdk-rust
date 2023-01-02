use crate::errors::Error as AnalyticsError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

macro_rules! is_user_id_or_anonymous_id_present {
    ($msg:ident) => {
        $msg.user_id.is_some() || $msg.anonymous_id.is_some()
    };
}

macro_rules! is_msg_context_valid {
    ($msg:ident) => {
        $msg.context.is_none
            || utils::check_reserved_keywords_conflict($msg.context.clone().unwrap())
    };
}
macro_rules! assert_valid_user_id_or_anonymous_id {
    ($msg:ident) => {{
        if is_user_id_or_anonymous_id_present!($msg) {
            Ok(())
        } else {
            Result::Err(AnalyticsError::InvalidRequest(
                "Either of user_id or anonymous_id is required".to_string(),
            ))
        }
    }};
}

macro_rules! assert_valid_context {
    ($msg:ident) => {
        if is_msg_context_valid!(msg) {
            Ok(())
        } else {
            Err(errors::AnalyticsError::InvalidRequest(
                "Reserve keyword present in context".to_string(),
            ))
        }
    };
}

macro_rules! self_match_blocks_for_message_types {
    ($apply_on : expr,$value: ident, $($($msg: ident::$msg_type:ident );*, $code_block: block);*, $default_block: block)=> {
        match $apply_on{
            $($($msg::$msg_type($value) => $code_block),* )*
           _ => $default_block,
        }
       }
}
macro_rules! self_match_blocks_for_message_types_with_no_default {
    ($apply_on : expr,$value: ident, $($($msg: ident::$msg_type:ident );*, $code_block: block);*)=> {
        match $apply_on{
            $($($msg::$msg_type($value) => $code_block),* )*
        }
       }
}

/// An enum containing all values which may be sent to RudderStack's API.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Identify(Identify),
    Track(Track),
    Page(Page),
    Screen(Screen),
    Group(Group),
    Alias(Alias),
    Batch(Batch),
}

/// An identify event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Identify {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The traits to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A track event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Track {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The name of the event being tracked.
    pub event: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A page event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Page {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The name of the page being tracked.
    pub name: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A screen event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Screen {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The name of the screen being tracked.
    pub name: String,

    /// The properties associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A group event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    /// The user id associated with this message.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// The anonymous user id associated with this message.
    #[serde(rename = "anonymousId", skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,

    /// The group the user is being associated with.
    #[serde(rename = "groupId")]
    pub group_id: String,

    /// The traits to assign to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// An alias event.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Alias {
    /// The user id associated with this message.
    #[serde(rename = "userId")]
    pub user_id: String,

    /// The user's previous ID.
    #[serde(rename = "previousId")]
    pub previous_id: String,

    /// The traits to assign to the alias.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,

    /// Context associated with this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,
}

/// A batch of events.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Batch {
    /// The batch of messages to send.
    pub batch: Vec<BatchMessage>,

    /// Context associated with this message.
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,

    /// Integrations to route this message to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Value>,

    /// The timestamp associated with this message.
    #[serde(rename = "originalTimestamp", skip_serializing_if = "Option::is_none")]
    pub original_timestamp: Option<DateTime<Utc>>,
}

impl Message {
    fn assert_valid_user_id_or_anonymous_id(&self) -> Result<(), AnalyticsError> {
        self_match_blocks_for_message_types!(self, msg,
            Message::Track;
            Message::Identify;
            Message::Page;
            Message::Screen;
            Message::Group,{
                assert_valid_user_id_or_anonymous_id!(msg)
            }, {Ok(())} )
    }
}

/// An enum containing all messages which may be placed inside a batch.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BatchMessage {
    #[serde(rename = "identify")]
    Identify(Identify),
    #[serde(rename = "track")]
    Track(Track),
    #[serde(rename = "page")]
    Page(Page),
    #[serde(rename = "screen")]
    Screen(Screen),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "alias")]
    Alias(Alias),
}

impl BatchMessage {
    pub fn update_context_with(&mut self, context: Value) {
        self_match_blocks_for_message_types_with_no_default!(self, msg,
            BatchMessage::Track;
            BatchMessage::Identify;
            BatchMessage::Page;
            BatchMessage::Screen;
            BatchMessage::Alias;
            BatchMessage::Group,{
                msg.context = BatchMessage::get_merged_context(&msg.context, &context)
            } )
    }

    fn get_merged_context(old_context: &Option<Value>, new_context: &Value) -> Option<Value> {
        let original_context = match old_context {
            Some(value) => value,
            None => &Value::Null,
        };
        return Some(BatchMessage::merge_values(original_context, new_context));
    }
    fn merge_values(original: &Value, updated_values: &Value) -> Value {
        match original {
            Value::Object(map) => {
                let mut map = map.clone();
                BatchMessage::update_map_with_value(&mut map, updated_values);
                Value::Object(map)
            }
            _ => updated_values.clone(),
        }
    }
    fn update_map_with_value(map: &mut Map<String, Value>, value: &Value) {
        match value {
            Value::Object(updated_value_map) => {
                for (k, v) in updated_value_map {
                    map.insert(k.clone(), v.clone());
                }
            }

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_return_self_match_blocks_for_message_types() {
        let test_message = BatchMessage::Track(Track {
            user_id: Some("user_id".to_string()),
            anonymous_id: None,
            event: "event".to_string(),
            ..Default::default()
        });
        let f = self_match_blocks_for_message_types!(test_message, msg, BatchMessage::Track;BatchMessage::Identify, {
            msg.user_id.unwrap()
        }, { "none".to_string() } );
        assert_eq!(f, "user_id".to_string());
        assert_ne!(f, "wrong_id".to_string());
    }
}
