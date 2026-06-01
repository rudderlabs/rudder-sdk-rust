use rudderanalytics::client::RudderAnalytics;
use rudderanalytics::errors::Error as AnalyticsError;
use rudderanalytics::message::{Message, Track};
use rudderanalytics::retry::RetryConfig;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

struct TestResponse {
    status: u16,
    reason: &'static str,
    headers: Vec<(&'static str, &'static str)>,
}

struct TestServer {
    url: String,
    request_count: Arc<AtomicUsize>,
    handle: JoinHandle<()>,
}

impl TestServer {
    fn wait(self) -> usize {
        let _ = self.handle.join();
        self.request_count.load(Ordering::SeqCst)
    }
}

fn start_server(responses: Vec<TestResponse>) -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let request_count = Arc::new(AtomicUsize::new(0));
    let thread_request_count = Arc::clone(&request_count);

    let handle = thread::spawn(move || {
        for response in responses {
            let deadline = Instant::now() + Duration::from_secs(2);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(conn) => break conn,
                    Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            return;
                        }
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(_) => return,
                }
            };

            thread_request_count.fetch_add(1, Ordering::SeqCst);
            let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
            let mut buffer = [0; 8192];
            let _ = stream.read(&mut buffer);

            let mut raw_response = format!(
                "HTTP/1.1 {} {}\r\nContent-Length: 0\r\nConnection: close\r\n",
                response.status, response.reason
            );
            for (name, value) in response.headers {
                raw_response.push_str(&format!("{}: {}\r\n", name, value));
            }
            raw_response.push_str("\r\n");

            let _ = stream.write_all(raw_response.as_bytes());
            let _ = stream.flush();
        }
    });

    TestServer {
        url,
        request_count,
        handle,
    }
}

fn response(status: u16, reason: &'static str) -> TestResponse {
    TestResponse {
        status,
        reason,
        headers: Vec::new(),
    }
}

fn response_with_header(
    status: u16,
    reason: &'static str,
    name: &'static str,
    value: &'static str,
) -> TestResponse {
    TestResponse {
        status,
        reason,
        headers: vec![(name, value)],
    }
}

fn retry_config(max_retries: u32) -> RetryConfig {
    RetryConfig {
        max_retries,
        base_delay: Duration::from_millis(0),
        max_backoff_delay: Duration::from_millis(0),
        jitter_ratio: 0.0,
        ..Default::default()
    }
}

fn track_message() -> Message {
    Message::Track(Track {
        user_id: Some("user-1".to_string()),
        event: "Test Event".to_string(),
        ..Default::default()
    })
}

#[test]
fn retries_429_until_success() {
    let server = start_server(vec![
        response(429, "Too Many Requests"),
        response(200, "OK"),
    ]);
    let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

    let result = analytics.send_with_retry_config(&track_message(), &retry_config(3));
    let request_count = server.wait();

    assert!(result.is_ok());
    assert_eq!(request_count, 2);
}

#[test]
fn send_once_does_not_retry_429() {
    let server = start_server(vec![response(429, "Too Many Requests")]);
    let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

    let result = analytics.send_once(&track_message());
    let request_count = server.wait();

    assert!(result.is_err());
    assert_eq!(request_count, 1);
}

#[test]
fn does_not_retry_non_retryable_400() {
    let server = start_server(vec![response(400, "Bad Request")]);
    let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

    let result = analytics.send_with_retry_config(&track_message(), &retry_config(3));
    let request_count = server.wait();

    assert!(result.is_err());
    assert_eq!(request_count, 1);
}

#[test]
fn does_not_retry_terminal_4xx_statuses() {
    for status in [401, 403, 404, 413, 422] {
        let server = start_server(vec![response(status, "Terminal Client Error")]);
        let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

        let result = analytics.send_with_retry_config(&track_message(), &retry_config(3));
        let request_count = server.wait();

        assert!(result.is_err(), "expected status {} to fail", status);
        assert_eq!(request_count, 1, "expected status {} not to retry", status);
    }
}

#[test]
fn returns_error_after_retry_budget_is_exhausted() {
    let server = start_server(vec![
        response(429, "Too Many Requests"),
        response(429, "Too Many Requests"),
    ]);
    let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

    let result = analytics.send_with_retry_config(&track_message(), &retry_config(1));
    let request_count = server.wait();

    match result {
        Err(AnalyticsError::InvalidRequest(message)) => {
            assert!(message.contains("retries exhausted"));
        }
        other => panic!("expected retries exhausted error, got {:?}", other),
    }
    assert_eq!(request_count, 2);
}

#[test]
fn retries_common_5xx_until_success() {
    for status in [500, 502, 503, 504] {
        let server = start_server(vec![response(status, "Server Error"), response(200, "OK")]);
        let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

        let result = analytics.send_with_retry_config(&track_message(), &retry_config(3));
        let request_count = server.wait();

        assert!(result.is_ok(), "expected status {} to retry", status);
        assert_eq!(request_count, 2, "expected status {} to retry once", status);
    }
}

#[test]
fn honors_retry_after_delay_seconds() {
    let server = start_server(vec![
        response_with_header(429, "Too Many Requests", "Retry-After", "1"),
        response(200, "OK"),
    ]);
    let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

    let start = Instant::now();
    let result = analytics.send_with_retry_config(&track_message(), &retry_config(3));
    let elapsed = start.elapsed();
    let request_count = server.wait();

    assert!(result.is_ok());
    assert_eq!(request_count, 2);
    assert!(
        elapsed >= Duration::from_secs(1),
        "expected Retry-After delay to be honored, elapsed {:?}",
        elapsed
    );
}
