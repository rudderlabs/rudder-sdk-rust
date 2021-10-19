<p align="center"><a href="https://rudderstack.com"><img src="https://user-images.githubusercontent.com/59817155/126267034-ae9870b7-9137-4f45-be65-d621b055a972.png" alt="RudderStack - Customer Data Platform for Developers" height="50"/></a></p>
<h1 align="center"></h1>
<p align="center"><b>Customer Data Platform for Developers</b></p>
<br/>

# About RudderStack

[**RudderStack**](https://rudderstack.com/) is a customer data platform for developers. Our tooling makes it easy to deploy pipelines that collect customer data from every app, website and SaaS platform, then activate it in your warehouse and business tools.

# RudderStack Rust SDK

| The RudderStack Rust SDK is an analytics client to track events from your Rust application. Once enabled, the event requests hit the RudderStack servers. RudderStack then transforms and routes these events to your specified destination platforms. |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |

> Questions? Start a conversation on our [**Slack channel**][slack].

> For detailed documentation on the RudderStack Rust SDK, click [**here**](https://docs.rudderstack.com/stream-sources/rudderstack-sdk-integration-guides/rudderstack-rust-sdk).

## Getting Started with Rust SDK

Include `rudderanalytics` as a dependency in your Rust application `Cargo.toml` file:

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

## Contribute

We would love to see you contribute to RudderStack. Get more information on how to contribute [**here**](CONTRIBUTING.md).

## Contact Us

For more information on any of the sections covered in this readme, you can [**contact us**](mailto:%20docs@rudderstack.com) or start a conversation on our [**Slack**](https://resources.rudderstack.com/join-rudderstack-slack) channel.

## Follow Us

- [**RudderStack blog**][rudderstack-blog]
- [**Slack**][slack]
- [**Twitter**][twitter]
- [**LinkedIn**][linkedin]
- [**dev.to**][devto]
- [**Medium**][medium]
- [**YouTube**][youtube]
- [**HackerNews**][hackernews]
- [**Product Hunt**][producthunt]

<!----variables---->

[slack]: https://rudderstack.com/join-rudderstack-slack-community
[twitter]: https://twitter.com/rudderstack
[linkedin]: https://www.linkedin.com/company/rudderlabs/
[devto]: https://dev.to/rudderstack
[medium]: https://rudderstack.medium.com/
[youtube]: https://www.youtube.com/channel/UCgV-B77bV_-LOmKYHw8jvBw
[rudderstack-blog]: https://rudderstack.com/blog/
[hackernews]: https://news.ycombinator.com/item?id=21081756
[producthunt]: https://www.producthunt.com/posts/rudderstack
[mit_license]: https://opensource.org/licenses/MIT
[agplv3_license]: https://www.gnu.org/licenses/agpl-3.0-standalone.html
[sspl_license]: https://www.mongodb.com/licensing/server-side-public-license
[config-generator]: https://github.com/rudderlabs/config-generator
[config-generator-section]: https://github.com/rudderlabs/rudder-server/blob/master/README.md#rudderstack-config-generator
[rudder-logo]: https://repository-images.githubusercontent.com/197743848/b352c900-dbc8-11e9-9d45-4deb9274101f
