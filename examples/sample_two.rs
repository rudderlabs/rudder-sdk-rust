//! An example showing how to do an ETL-like operation loading events into
//! Segment.

use testrudder::batcher::Batcher;
use testrudder::client::RudderAnalytics;
use testrudder::message::{BatchMessage, Track};
use serde_json::json;

fn main() {
    let write_key = "YOUR_WRITE_KEY";
    let data_plane_url = "YOUR_DATA_PLANE_URL";

    let client = RudderAnalytics::load(write_key.to_string(), data_plane_url.to_string());
    let mut batcher = Batcher::new(None);

    // Pretend this is reading off of a queue, a file, or some other data
    // source.
    for i in 0..100 {
        let msg = BatchMessage::Track(Track {
            user_id: Some("sample_user_id".to_string()),
            event: "Example Event".to_owned(),
            properties: Some(json!({
                "foo": format!("bar-{}", i),
            })),
            ..Default::default()
        });

        // An error here indicates a message is too large. In real life, you
        // would probably want to put this message in a deadletter queue or some
        // equivalent.
        if let Some(msg) = batcher.push(msg).unwrap() {
            client.send(&batcher.into_message()).unwrap();

            batcher = Batcher::new(None);
            batcher.push(msg).unwrap(); // Same error condition as above.
        }
    }
}
