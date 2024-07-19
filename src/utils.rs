use crate::message::{Identify, Track, Page, Screen, Group, Alias, Batch, BatchMessage};
use crate::ruddermessage::{
    Ruddermessage,
    Identify as Rudderidentify,
    Track as Ruddertrack,
    Page as Rudderpage,
    Screen as Rudderscreen,
    Group as Ruddergroup,
    Alias as Rudderalias,
    Batch as Rudderbatch,
    BatchMessage as Rudderbatchmessage
};
use serde_json::{json, Value};
use chrono::prelude::*;

// constants and reserved keywords
const NAME: &str = "RudderStack Rust SDK";
const VERSION: &str = "1.1.3";
static RESERVED_KEYS : [&str;1] = ["library"];
const CHANNEL :&str = "server";

// function to merge two objects
fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

// function to check if any reserve keyword is present in a given object or not
// returns true/false
pub fn check_reserved_keywords_conflict(context: Value)->bool{
    let mut result = false;
    for (k, _v) in context.as_object().unwrap().iter(){
        let s: String = k.to_owned();
        if RESERVED_KEYS.contains(&&s[..]) {
            result = true;
            break;
        } 
    }
    result
}

// Build and return static context fields
fn get_default_context()->Value{
    let default_context = json!({
        "library":{
            "name": NAME,
            "version": VERSION
        }
    });
    default_context
}

// modify identify payload to rudder format
pub fn parse_identify(msg:&Identify)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }
    
    let new_message = Ruddermessage::Identify(
        Rudderidentify {
            user_id: msg.user_id.clone(),
            anonymous_id: msg.anonymous_id.clone(),
            traits: msg.traits.clone(),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("identify"),
            channel: CHANNEL.to_string()
        } 
    );
    new_message
}

// modify track payload to rudder format
pub fn parse_track(msg:&Track)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }

    let new_message= Ruddermessage::Track(
        Ruddertrack {
            user_id: msg.user_id.clone(),
            anonymous_id: msg.anonymous_id.clone(),
            event: msg.event.clone(),
            properties: msg.properties.clone(),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("track"),
            channel: CHANNEL.to_string()
        }
    );
    new_message
}

// modify page payload to rudder format
pub fn parse_page(msg:&Page)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }

    let new_message= Ruddermessage::Page(
        Rudderpage {
            user_id: msg.user_id.clone(),
            anonymous_id: msg.anonymous_id.clone(),
            name: msg.name.clone(),
            properties: msg.properties.clone(),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("page"),
            channel: CHANNEL.to_string()
        }
    );
    new_message
}

// modify screen payload to rudder format
pub fn parse_screen(msg:&Screen)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }

    let new_message= Ruddermessage::Screen(
        Rudderscreen {
            user_id: msg.user_id.clone(),
            anonymous_id: msg.anonymous_id.clone(),
            name: msg.name.clone(),
            properties: msg.properties.clone(),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("screen"),
            channel: CHANNEL.to_string()
        }
    );
    new_message
}

// modify group payload to rudder format
pub fn parse_group(msg:&Group)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }

    let new_message= Ruddermessage::Group(
        Ruddergroup {
            user_id: msg.user_id.clone(),
            anonymous_id: msg.anonymous_id.clone(),
            group_id: msg.group_id.clone(),
            traits: msg.traits.clone(),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("group"),
            channel: CHANNEL.to_string()
        }
    );
    new_message
}

// modify alias payload to rudder format
pub fn parse_alias(msg:&Alias)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }

    let new_message= Ruddermessage::Alias(
        Rudderalias {
            user_id: msg.user_id.clone(),
            previous_id: msg.previous_id.clone(),
            traits: msg.traits.clone(),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("alias"),
            channel: CHANNEL.to_string()
        }
    );
    new_message
}

// modify batch payload to rudder format
pub fn parse_batch(msg:&Batch)-> Ruddermessage{
    let mut modified_context = get_default_context();
    merge(&mut modified_context, msg.context.clone().unwrap_or(json!({})));

    let original_timestamp;
    let sent_at = Utc::now();
    if msg.original_timestamp==Option::None {
        original_timestamp = Some(sent_at);
    }else {
        original_timestamp = msg.original_timestamp;
    }

    let mut batch:Vec<Rudderbatchmessage> = Vec::new();

    for i in &msg.batch {
        match i {
            BatchMessage::Identify(a_) =>{
                batch.push(Rudderbatchmessage::Identify(Rudderidentify 
                {
                    user_id: a_.user_id.clone(),
                    anonymous_id: a_.anonymous_id.clone(),
                    traits: a_.traits.clone(),
                    original_timestamp: original_timestamp,
                    sent_at: Some(sent_at),
                    integrations: a_.integrations.clone(),
                    context: Some(modified_context.clone()),
                    r#type: String::from("identify"),
                    channel: CHANNEL.to_string()
                }));
            },           
            BatchMessage::Track(a_) =>{
                batch.push(Rudderbatchmessage::Track(
                    Ruddertrack {
                        user_id: a_.user_id.clone(),
                        anonymous_id: a_.anonymous_id.clone(),
                        event: a_.event.clone(),
                        properties: a_.properties.clone(),
                        original_timestamp: original_timestamp,
                        sent_at: Some(sent_at),
                        integrations: a_.integrations.clone(),
                        context: Some(modified_context.clone()),
                        r#type: String::from("track"),
                        channel: CHANNEL.to_string()
                    }
                ));
            },           
            BatchMessage::Page(a_) =>{
                batch.push(Rudderbatchmessage::Page(
                    Rudderpage {
                        user_id: a_.user_id.clone(),
                        anonymous_id: a_.anonymous_id.clone(),
                        name: a_.name.clone(),
                        properties: a_.properties.clone(),
                        original_timestamp: original_timestamp,
                        sent_at: Some(sent_at),
                        integrations: a_.integrations.clone(),
                        context: Some(modified_context.clone()),
                        r#type: String::from("page"),
                        channel: CHANNEL.to_string()
                    }
                ));
            },           
            BatchMessage::Screen(a_) =>{
                batch.push(Rudderbatchmessage::Screen(
                    Rudderscreen {
                        user_id: a_.user_id.clone(),
                        anonymous_id: a_.anonymous_id.clone(),
                        name: a_.name.clone(),
                        properties: a_.properties.clone(),
                        original_timestamp: original_timestamp,
                        sent_at: Some(sent_at),
                        integrations: a_.integrations.clone(),
                        context: Some(modified_context.clone()),
                        r#type: String::from("screen"),
                        channel: CHANNEL.to_string()
                    }
                ));
            },           
            BatchMessage::Group(a_) =>{
                batch.push(Rudderbatchmessage::Group(
                    Ruddergroup {
                        user_id: a_.user_id.clone(),
                        anonymous_id: a_.anonymous_id.clone(),
                        group_id: a_.group_id.clone(),
                        traits: a_.traits.clone(),
                        original_timestamp: original_timestamp,
                        sent_at: Some(sent_at),
                        integrations: a_.integrations.clone(),
                        context: Some(modified_context.clone()),
                        r#type: String::from("group"),
                        channel: CHANNEL.to_string()
                    }
                ));
            },           
            BatchMessage::Alias(a_) =>{
                batch.push(Rudderbatchmessage::Alias(
                    Rudderalias {
                        user_id: a_.user_id.clone(),
                        previous_id: a_.previous_id.clone(),
                        traits: a_.traits.clone(),
                        original_timestamp: original_timestamp,
                        sent_at: Some(sent_at),
                        integrations: a_.integrations.clone(),
                        context: Some(modified_context.clone()),
                        r#type: String::from("alias"),
                        channel: CHANNEL.to_string()
                    }
                ));
            },           
        }
    }

    let new_message= Ruddermessage::Batch(
        Rudderbatch {
            batch: batch,
            integrations: msg.integrations.clone(),
            context: Some(modified_context),
            r#type: String::from("batch"),
            original_timestamp: original_timestamp,
            sent_at: Some(sent_at),
        }
    );
    new_message
}
