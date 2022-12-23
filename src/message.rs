use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
        match self {
            BatchMessage::Identify(message) => {
                message.context = BatchMessage::get_merged_context(&message.context, &context)
            }
            BatchMessage::Track(message) => {
                message.context = BatchMessage::get_merged_context(&message.context, &context);
            }
            BatchMessage::Page(message) => {
                message.context = BatchMessage::get_merged_context(&message.context, &context);
            }
            BatchMessage::Screen(message) => {
                message.context = BatchMessage::get_merged_context(&message.context, &context);
            }
            BatchMessage::Group(message) => {
                message.context = BatchMessage::get_merged_context(&message.context, &context);
            }
            BatchMessage::Alias(message) => {
                message.context = BatchMessage::get_merged_context(&message.context, &context);
            }
        }
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
