# [](https://github.com/rudderlabs/rudder-sdk-rust/blob/master/README.md#rudderstack-rust-sdk) RudderStack Rust SDK

The [**RudderStack**](https://rudderstack.com/) Rust SDK is an analytics client to track events from your Rust application. Once enabled, the event requests hit the RudderStack servers. RudderStack then transforms and routes these events to your specified destination platforms.

> For detailed documentation on the RudderStack Rust SDK, click [**here**](https://docs.rudderstack.com/stream-sources/rudderstack-sdk-integration-guides/rudderstack-rust-sdk).

## Getting Started with Rust SDK

Include `rudderanalytics` as a dependency in your Rust application:

```toml
[dependencies]
rudderanalytics = "1.0.0"
```

## Initializing the RudderStack Client

```rust
use rudderanalytics::client::RudderAnalytics;
use rudderanalytics::message::{Identify, Track, Page, Group, Screen, Alias, Batch, Message, BatchMessage};

let rudder_analytics = RudderAnalytics::load("YOUR_WRITE_KEY".to_string(), "YOUR_DATA_PLANE_URL".to_string());
```

## Sending Events

Once the RudderStack client is initialized, you can use it to send your events. A sample call for track event is shown below:

```rust
use serde_json::json;

rudder_analytics.send(&Message::Track(Track {
        user_id: Some("sample_user_id".to_string()),
            event: "Test Event".to_owned(),
            properties: Some(json!({
                "some property": "some value",
                "some other property": "some other value",
            })),
            ..Default::default()
        })).expect("Failed to send data to Rudderstack");
```

For more information on the supported calls, refer to the [**documentation**](https://docs.rudderstack.com/stream-sources/rudderstack-sdk-integration-guides/rudderstack-rust-sdk#sending-events-from-rudderstack).

## About RudderStack

[**RudderStack**](https://rudderstack.com/) is a customer data platform for developers. Our tooling makes it easy to deploy pipelines that collect customer data from every app, website and SaaS platform, then activate it in your warehouse and business tools.

More information on RudderStack can be found [**here**](https://github.com/rudderlabs/rudder-server).

## Contact Us

For more information on any of the sections covered in this readme, you can [**contact us**](mailto:%20docs@rudderstack.com) or start a conversation on our [**Slack**](https://resources.rudderstack.com/join-rudderstack-slack) channel.
