use rudderanalytics::batcher::Batcher;
use rudderanalytics::errors::Error as AnalyticsError;
use rudderanalytics::message::{
    Alias, Batch, BatchMessage, Group, Identify, Message, Page, Screen, Track,
};
// / To test all the cases run the command `cargo test --all`
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_string(&Message::Identify(Identify {
                user_id: Some("foo".to_string()),
                traits: Some(json!({
                    "foo": "bar",
                    "baz": "quux",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","traits":{"baz":"quux","foo":"bar"}}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Track(Track {
                anonymous_id: Some("foo".to_string()),
                event: "Foo".to_owned(),
                properties: Some(json!({
                    "foo": "bar",
                    "baz": "quux",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"anonymousId":"foo","event":"Foo","properties":{"baz":"quux","foo":"bar"}}"#
                .to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Page(Page {
                user_id: Some("foo".to_string()),
                name: "Foo".to_owned(),
                properties: Some(json!({
                    "foo": "bar",
                    "baz": "quux",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","name":"Foo","properties":{"baz":"quux","foo":"bar"}}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Screen(Screen {
                user_id: Some("foo".to_string()),
                name: "Foo".to_owned(),
                properties: Some(json!({
                    "foo": "bar",
                    "baz": "quux",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","name":"Foo","properties":{"baz":"quux","foo":"bar"}}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Group(Group {
                user_id: Some("foo".to_string()),
                group_id: "bar".to_owned(),
                traits: Some(json!({
                    "foo": "bar",
                    "baz": "quux",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","groupId":"bar","traits":{"baz":"quux","foo":"bar"}}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Alias(Alias {
                user_id: "foo".to_owned(),
                previous_id: "bar".to_owned(),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"userId":"foo","previousId":"bar"}"#.to_owned(),
        );

        assert_eq!(
            serde_json::to_string(&Message::Batch(Batch {
                batch: vec![
                    BatchMessage::Track(Track {
                        user_id: Some("foo".to_string()),
                        event: "Foo".to_owned(),
                        properties: Some(json!({})),
                        ..Default::default()
                    }),
                    BatchMessage::Track(Track {
                        user_id: Some("bar".to_string()),
                        event: "Bar".to_owned(),
                        properties: Some(json!({})),
                        ..Default::default()
                    }),
                    BatchMessage::Track(Track {
                        user_id: Some("baz".to_string()),
                        event: "Baz".to_owned(),
                        properties: Some(json!({})),
                        ..Default::default()
                    })
                ],
                context: Some(json!({
                    "foo": "bar",
                })),
                ..Default::default()
            }))
            .unwrap(),
            r#"{"batch":[{"type":"track","userId":"foo","event":"Foo","properties":{}},{"type":"track","userId":"bar","event":"Bar","properties":{}},{"type":"track","userId":"baz","event":"Baz","properties":{}}],"context":{"foo":"bar"}}"#
                .to_owned(),
        );
    }

    #[test]
    fn test_push_and_into() {
        let batch_msg = BatchMessage::Track(Track {
            ..Default::default()
        });

        let context = json!({
            "foo": "bar",
        });

        let mut batcher = Batcher::new(Some(context.clone()));
        let result = batcher.push(batch_msg.clone());
        assert_eq!(None, result.ok().unwrap());

        let batch = batcher.into_message();
        let inner_batch = match batch {
            Message::Batch(b) => b,
            _ => panic!("invalid message type"),
        };
        assert_eq!(context, inner_batch.context.unwrap());
        assert_eq!(1, inner_batch.batch.len());

        assert_eq!(inner_batch.batch, vec![batch_msg]);
    }

    #[test]
    fn test_bad_message_size() {
        let batch_msg = BatchMessage::Track(Track {
            user_id: Some(String::from_utf8(vec![b'a'; 1024 * 33]).unwrap()),
            ..Default::default()
        });

        let mut batcher = Batcher::new(None);
        let result = batcher.push(batch_msg.into());

        let err = result.err().unwrap();
        let err: &AnalyticsError = err.as_fail().downcast_ref().unwrap();

        match err {
            AnalyticsError::MessageTooLarge(_str) => {}
            AnalyticsError::InvalidRequest(_str) => {}
        }
    }

    #[test]
    fn test_max_buffer() {
        let batch_msg = BatchMessage::Track(Track {
            user_id: Some(String::from_utf8(vec![b'a'; 1024 * 30]).unwrap()),
            ..Default::default()
        });

        let mut batcher = Batcher::new(None);
        let mut result = Ok(None);
        for _i in 0..20 {
            result = batcher.push(batch_msg.clone().into());
            dbg!(&result);
            if result.is_ok() && result.as_ref().ok().unwrap().is_some() {
                break;
            }
        }

        let msg = result.ok().unwrap();
        assert_eq!(batch_msg, msg.unwrap());
    }
}
