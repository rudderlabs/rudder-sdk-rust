use crate::message::{Alias, Batch, BatchMessage, Group, Identify, Page, Screen, Track};
use crate::ruddermessage::{
    Alias as Rudderalias, Batch as Rudderbatch, BatchMessage as Rudderbatchmessage,
    Group as Ruddergroup, Identify as Rudderidentify, Page as Rudderpage, Ruddermessage,
    Screen as Rudderscreen, Track as Ruddertrack,
};
use chrono::prelude::*;
use serde_json::{json, Value};

// constants and reserved keywords
const NAME: &str = "RudderStack Rust SDK";
const VERSION: &str = "1.0.0";
static RESERVED_KEYS: [&str; 1] = ["library"];
const CHANNEL: &str = "server";

// function to merge two objects
fn merge(left: &mut Value, right: Value) {
    match (left, right) {
        (left @ &mut Value::Object(_), Value::Object(right)) => {
            let left = left.as_object_mut().unwrap();
            for (key, value) in right {
                merge(left.entry(key).or_insert(Value::Null), value);
            }
        }
        (left, right) => *left = right,
    }
}

// function to check if any reserve keyword is present in a given object or not
// returns true/false
pub fn check_reserved_keywords_conflict(context: &Value) -> bool {
    let mut result = false;
    for (key, _value) in context.as_object().unwrap().iter() {
        let key: String = key.clone();
        if RESERVED_KEYS.contains(&&key[..]) {
            result = true;
            break;
        }
    }
    result
}

// Build and return static context fields
fn get_default_context() -> Value {
    let default_context = json!({
        "library":{
            "name": NAME,
            "version": VERSION
        }
    });
    default_context
}

// modify identify payload to rudder format
pub fn parse_identify(message: &Identify) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        message.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if message.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        message.original_timestamp
    };

    Ruddermessage::Identify(Rudderidentify {
        user_id: message.user_id.clone(),
        anonymous_id: message.anonymous_id.clone(),
        traits: message.traits.clone(),
        original_timestamp,
        sent_at: Some(sent_at),
        integrations: message.integrations.clone(),
        context: Some(modified_context),
        r#type: String::from("identify"),
        channel: CHANNEL.to_string(),
    })
}

// modify track payload to rudder format
pub fn parse_track(message: &Track) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        message.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if message.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        message.original_timestamp
    };

    Ruddermessage::Track(Ruddertrack {
        user_id: message.user_id.clone(),
        anonymous_id: message.anonymous_id.clone(),
        event: message.event.clone(),
        properties: message.properties.clone(),
        original_timestamp,
        sent_at: Some(sent_at),
        integrations: message.integrations.clone(),
        context: Some(modified_context),
        r#type: String::from("track"),
        channel: CHANNEL.to_string(),
    })
}

// modify page payload to rudder format
pub fn parse_page(message: &Page) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        message.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if message.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        message.original_timestamp
    };

    Ruddermessage::Page(Rudderpage {
        user_id: message.user_id.clone(),
        anonymous_id: message.anonymous_id.clone(),
        name: message.name.clone(),
        properties: message.properties.clone(),
        original_timestamp,
        sent_at: Some(sent_at),
        integrations: message.integrations.clone(),
        context: Some(modified_context),
        r#type: String::from("page"),
        channel: CHANNEL.to_string(),
    })
}

// modify screen payload to rudder format
pub fn parse_screen(message: &Screen) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        message.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if message.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        message.original_timestamp
    };

    Ruddermessage::Screen(Rudderscreen {
        user_id: message.user_id.clone(),
        anonymous_id: message.anonymous_id.clone(),
        name: message.name.clone(),
        properties: message.properties.clone(),
        original_timestamp,
        sent_at: Some(sent_at),
        integrations: message.integrations.clone(),
        context: Some(modified_context),
        r#type: String::from("screen"),
        channel: CHANNEL.to_string(),
    })
}

// modify group payload to rudder format
pub fn parse_group(message: &Group) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        message.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if message.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        message.original_timestamp
    };

    Ruddermessage::Group(Ruddergroup {
        user_id: message.user_id.clone(),
        anonymous_id: message.anonymous_id.clone(),
        group_id: message.group_id.clone(),
        traits: message.traits.clone(),
        original_timestamp,
        sent_at: Some(sent_at),
        integrations: message.integrations.clone(),
        context: Some(modified_context),
        r#type: String::from("group"),
        channel: CHANNEL.to_string(),
    })
}

// modify alias payload to rudder format
pub fn parse_alias(message: &Alias) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        message.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if message.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        message.original_timestamp
    };

    Ruddermessage::Alias(Rudderalias {
        user_id: message.user_id.clone(),
        previous_id: message.previous_id.clone(),
        traits: message.traits.clone(),
        original_timestamp,
        sent_at: Some(sent_at),
        integrations: message.integrations.clone(),
        context: Some(modified_context),
        r#type: String::from("alias"),
        channel: CHANNEL.to_string(),
    })
}

#[allow(clippy::too_many_lines)]
// modify batch payload to rudder format
pub fn parse_batch(batch: &Batch) -> Ruddermessage {
    let mut modified_context = get_default_context();
    merge(
        &mut modified_context,
        batch.context.clone().unwrap_or(json!({})),
    );
    let sent_at = Utc::now();
    let original_timestamp = if batch.original_timestamp.is_none() {
        Some(sent_at)
    } else {
        batch.original_timestamp
    };

    let integrations = batch.integrations.clone();

    let batch = batch
        .messages
        .iter()
        .map(|message| match message {
            BatchMessage::Identify(identify_message) => {
                Rudderbatchmessage::Identify(Rudderidentify {
                    user_id: identify_message.user_id.clone(),
                    anonymous_id: identify_message.anonymous_id.clone(),
                    traits: identify_message.traits.clone(),
                    original_timestamp,
                    sent_at: Some(sent_at),
                    integrations: identify_message.integrations.clone(),
                    context: Some(modified_context.clone()),
                    r#type: String::from("identify"),
                    channel: CHANNEL.to_string(),
                })
            }
            BatchMessage::Track(track_message) => Rudderbatchmessage::Track(Ruddertrack {
                user_id: track_message.user_id.clone(),
                anonymous_id: track_message.anonymous_id.clone(),
                event: track_message.event.clone(),
                properties: track_message.properties.clone(),
                original_timestamp,
                sent_at: Some(sent_at),
                integrations: track_message.integrations.clone(),
                context: Some(modified_context.clone()),
                r#type: String::from("track"),
                channel: CHANNEL.to_string(),
            }),
            BatchMessage::Page(page_message) => Rudderbatchmessage::Page(Rudderpage {
                user_id: page_message.user_id.clone(),
                anonymous_id: page_message.anonymous_id.clone(),
                name: page_message.name.clone(),
                properties: page_message.properties.clone(),
                original_timestamp,
                sent_at: Some(sent_at),
                integrations: page_message.integrations.clone(),
                context: Some(modified_context.clone()),
                r#type: String::from("page"),
                channel: CHANNEL.to_string(),
            }),
            BatchMessage::Screen(screen_message) => Rudderbatchmessage::Screen(Rudderscreen {
                user_id: screen_message.user_id.clone(),
                anonymous_id: screen_message.anonymous_id.clone(),
                name: screen_message.name.clone(),
                properties: screen_message.properties.clone(),
                original_timestamp,
                sent_at: Some(sent_at),
                integrations: screen_message.integrations.clone(),
                context: Some(modified_context.clone()),
                r#type: String::from("screen"),
                channel: CHANNEL.to_string(),
            }),
            BatchMessage::Group(group_message) => Rudderbatchmessage::Group(Ruddergroup {
                user_id: group_message.user_id.clone(),
                anonymous_id: group_message.anonymous_id.clone(),
                group_id: group_message.group_id.clone(),
                traits: group_message.traits.clone(),
                original_timestamp,
                sent_at: Some(sent_at),
                integrations: group_message.integrations.clone(),
                context: Some(modified_context.clone()),
                r#type: String::from("group"),
                channel: CHANNEL.to_string(),
            }),
            BatchMessage::Alias(alias_message) => Rudderbatchmessage::Alias(Rudderalias {
                user_id: alias_message.user_id.clone(),
                previous_id: alias_message.previous_id.clone(),
                traits: alias_message.traits.clone(),
                original_timestamp,
                sent_at: Some(sent_at),
                integrations: alias_message.integrations.clone(),
                context: Some(modified_context.clone()),
                r#type: String::from("alias"),
                channel: CHANNEL.to_string(),
            }),
        })
        .collect();

    Ruddermessage::Batch(Rudderbatch {
        batch,
        integrations,
        context: Some(modified_context),
        r#type: String::from("batch"),
        original_timestamp,
        sent_at: Some(sent_at),
    })
}
