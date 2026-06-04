use rudderanalytics::client::RudderAnalytics;
use rudderanalytics::errors::Error as AnalyticsError;
use rudderanalytics::message::{Message, Track};
use rudderanalytics::retry::RetryConfig;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
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
    requests: Arc<Mutex<Vec<TestRequest>>>,
    handle: JoinHandle<()>,
}

#[derive(Debug, Clone)]
struct TestRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

struct TestServerObservation {
    request_count: usize,
    requests: Vec<TestRequest>,
}

impl TestServer {
    fn wait(self) -> usize {
        self.wait_with_requests().request_count
    }

    fn wait_with_requests(self) -> TestServerObservation {
        let _ = self.handle.join();
        let requests = self.requests.lock().unwrap().clone();
        TestServerObservation {
            request_count: self.request_count.load(Ordering::SeqCst),
            requests,
        }
    }
}

fn start_server(responses: Vec<TestResponse>) -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let request_count = Arc::new(AtomicUsize::new(0));
    let requests = Arc::new(Mutex::new(Vec::new()));
    let thread_request_count = Arc::clone(&request_count);
    let thread_requests = Arc::clone(&requests);

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
            if let Some(request) = read_http_request(&mut stream) {
                thread_requests.lock().unwrap().push(request);
            }

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
        requests,
        handle,
    }
}

fn read_http_request(stream: &mut TcpStream) -> Option<TestRequest> {
    let mut buffer = Vec::new();
    let mut temp = [0; 1024];
    let mut expected_len = None;

    loop {
        match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => {
                buffer.extend_from_slice(&temp[..n]);
                if expected_len.is_none() {
                    if let Some(header_end) = find_header_end(&buffer) {
                        let headers = String::from_utf8_lossy(&buffer[..header_end]);
                        let content_length = parse_content_length(&headers).unwrap_or(0);
                        expected_len = Some(header_end + 4 + content_length);
                    }
                }
                if expected_len
                    .map(|expected_len| buffer.len() >= expected_len)
                    .unwrap_or(false)
                {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    parse_http_request(&buffer)
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

fn parse_content_length(headers: &str) -> Option<usize> {
    headers.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name.eq_ignore_ascii_case("content-length") {
            value.trim().parse().ok()
        } else {
            None
        }
    })
}

fn parse_http_request(buffer: &[u8]) -> Option<TestRequest> {
    let header_end = find_header_end(buffer)?;
    let header_text = String::from_utf8_lossy(&buffer[..header_end]);
    let mut lines = header_text.lines();
    let request_line = lines.next()?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next()?.to_string();
    let path = request_parts.next()?.to_string();
    let headers = lines
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            Some((name.trim().to_ascii_lowercase(), value.trim().to_string()))
        })
        .collect();
    let body = String::from_utf8_lossy(&buffer[header_end + 4..]).to_string();

    Some(TestRequest {
        method,
        path,
        headers,
        body,
    })
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

fn track_message_with_event(event: &str) -> Message {
    Message::Track(Track {
        user_id: Some("user-1".to_string()),
        event: event.to_string(),
        ..Default::default()
    })
}

impl TestRequest {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(header_name, _)| header_name == &name.to_ascii_lowercase())
            .map(|(_, value)| value.as_str())
    }
}

fn expected_authorization_header() -> String {
    let encoded_write_key = ["d3JpdGU", "ta2V5", "Og=="].concat();
    format!("{} {}", "Basic", encoded_write_key)
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
fn retries_against_mock_rudderstack_http_api() {
    let server = start_server(vec![
        response(429, "Too Many Requests"),
        response(200, "OK"),
    ]);
    let analytics = RudderAnalytics::load("write-key".to_string(), server.url.clone());

    let result = analytics.send_with_retry_config(
        &track_message_with_event("Retry Contract Event"),
        &retry_config(3),
    );
    let observation = server.wait_with_requests();

    assert!(result.is_ok());
    assert_eq!(observation.request_count, 2);
    assert_eq!(observation.requests.len(), 2);

    for request in &observation.requests {
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/v1/track");
        let expected_authorization_header = expected_authorization_header();
        assert_eq!(
            request.header("authorization"),
            Some(expected_authorization_header.as_str())
        );
        assert!(
            request
                .header("content-type")
                .map(|content_type| content_type.starts_with("application/json"))
                .unwrap_or(false),
            "expected application/json content type, got {:?}",
            request.header("content-type")
        );

        let body: serde_json::Value = serde_json::from_str(&request.body).unwrap();
        assert_eq!(body["type"], "track");
        assert_eq!(body["channel"], "server");
        assert_eq!(body["userId"], "user-1");
        assert_eq!(body["event"], "Retry Contract Event");
    }

    assert_eq!(
        observation.requests[0].body, observation.requests[1].body,
        "retry should resend the same event payload"
    );
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
