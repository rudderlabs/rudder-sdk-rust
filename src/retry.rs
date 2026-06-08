//! Retry configuration and helpers for transient delivery failures.

use rand::Rng;
use reqwest::header::{HeaderMap, RETRY_AFTER};
use reqwest::StatusCode;
use std::time::{Duration, SystemTime};

/// Configuration for retrying transient delivery failures.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Whether retries are enabled.
    pub enabled: bool,
    /// Number of retries after the initial request.
    pub max_retries: u32,
    /// Base delay for the first retry before jitter.
    pub base_delay: Duration,
    /// Maximum delay from the SDK's exponential backoff calculation.
    pub max_backoff_delay: Duration,
    /// Jitter ratio added to the selected delay. `0.2` means up to 20%.
    pub jitter_ratio: f64,
    /// Whether to honor the standard Retry-After response header.
    pub respect_retry_after: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_backoff_delay: Duration::from_secs(30),
            jitter_ratio: 0.2,
            respect_retry_after: true,
        }
    }
}

impl RetryConfig {
    /// Return a configuration that preserves one-attempt delivery behavior.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            max_retries: 0,
            base_delay: Duration::from_millis(100),
            max_backoff_delay: Duration::from_secs(30),
            jitter_ratio: 0.0,
            respect_retry_after: false,
        }
    }
}

pub(crate) fn is_status_retryable(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

pub(crate) fn is_error_retryable(err: &reqwest::Error) -> bool {
    err.is_connect() || err.is_timeout()
}

pub(crate) fn retry_delay(
    config: &RetryConfig,
    retry_number: u32,
    headers: Option<&HeaderMap>,
) -> Duration {
    let backoff_delay = exponential_backoff(config, retry_number);
    let retry_after_delay = if config.respect_retry_after {
        headers
            .and_then(parse_retry_after)
            .unwrap_or_else(|| Duration::from_secs(0))
    } else {
        Duration::from_secs(0)
    };

    add_jitter(
        std::cmp::max(backoff_delay, retry_after_delay),
        config.jitter_ratio,
    )
}

fn exponential_backoff(config: &RetryConfig, retry_number: u32) -> Duration {
    if config.base_delay == Duration::from_secs(0) {
        return Duration::from_secs(0);
    }

    let attempt_index = retry_number.saturating_sub(1);
    let multiplier = 1u32.checked_shl(attempt_index).unwrap_or(u32::MAX);
    config
        .base_delay
        .checked_mul(multiplier)
        .unwrap_or(config.max_backoff_delay)
        .min(config.max_backoff_delay)
}

fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?.to_str().ok()?;
    let delay = parse_retry_after_value(value, SystemTime::now());
    if delay.is_none() {
        log::debug!("ignoring invalid Retry-After header: {}", value);
    }
    delay
}

fn parse_retry_after_value(value: &str, now: SystemTime) -> Option<Duration> {
    let value = value.trim();

    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let retry_at = httpdate::parse_http_date(value).ok()?;
    Some(
        retry_at
            .duration_since(now)
            .unwrap_or_else(|_| Duration::from_secs(0)),
    )
}

fn add_jitter(delay: Duration, jitter_ratio: f64) -> Duration {
    if delay == Duration::from_secs(0) || jitter_ratio <= 0.0 || !jitter_ratio.is_finite() {
        return delay;
    }

    let jitter_ratio = rand::thread_rng().gen_range(0.0..=jitter_ratio.min(1.0));
    let jitter = duration_mul_f64_saturating(delay, jitter_ratio);
    delay.checked_add(jitter).unwrap_or_else(max_duration)
}

fn duration_mul_f64_saturating(duration: Duration, factor: f64) -> Duration {
    if factor <= 0.0 {
        return Duration::from_secs(0);
    }

    let nanos = duration.as_nanos() as f64 * factor;
    if !nanos.is_finite() || nanos >= u128::MAX as f64 {
        return max_duration();
    }

    duration_from_nanos_saturating(nanos as u128)
}

fn duration_from_nanos_saturating(nanos: u128) -> Duration {
    let secs = nanos / 1_000_000_000;
    if secs > u64::MAX as u128 {
        return max_duration();
    }

    Duration::new(secs as u64, (nanos % 1_000_000_000) as u32)
}

fn max_duration() -> Duration {
    Duration::new(u64::MAX, 999_999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpdate::fmt_http_date;

    #[test]
    fn parses_retry_after_delay_seconds() {
        let delay = parse_retry_after_value("120", SystemTime::UNIX_EPOCH).unwrap();
        assert_eq!(delay, Duration::from_secs(120));
    }

    #[test]
    fn parses_retry_after_http_date() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let retry_at = now + Duration::from_secs(60);
        let delay = parse_retry_after_value(&fmt_http_date(retry_at), now).unwrap();

        assert_eq!(delay, Duration::from_secs(60));
    }

    #[test]
    fn treats_past_retry_after_date_as_zero() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let retry_at = SystemTime::UNIX_EPOCH + Duration::from_secs(60);
        let delay = parse_retry_after_value(&fmt_http_date(retry_at), now).unwrap();

        assert_eq!(delay, Duration::from_secs(0));
    }

    #[test]
    fn ignores_invalid_retry_after_value() {
        assert_eq!(
            parse_retry_after_value("not a retry header", SystemTime::UNIX_EPOCH),
            None
        );
    }

    #[test]
    fn first_retry_uses_base_delay() {
        let config = RetryConfig {
            base_delay: Duration::from_millis(100),
            max_backoff_delay: Duration::from_secs(30),
            jitter_ratio: 0.0,
            ..Default::default()
        };

        assert_eq!(retry_delay(&config, 1, None), Duration::from_millis(100));
        assert_eq!(retry_delay(&config, 2, None), Duration::from_millis(200));
        assert_eq!(retry_delay(&config, 3, None), Duration::from_millis(400));
    }

    #[test]
    fn retry_after_is_floor_on_backoff() {
        let config = RetryConfig {
            base_delay: Duration::from_millis(100),
            max_backoff_delay: Duration::from_secs(30),
            jitter_ratio: 0.0,
            ..Default::default()
        };
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, "5".parse().unwrap());

        assert_eq!(
            retry_delay(&config, 1, Some(&headers)),
            Duration::from_secs(5)
        );
    }

    #[test]
    fn retry_after_does_not_shorten_backoff() {
        let config = RetryConfig {
            base_delay: Duration::from_secs(10),
            max_backoff_delay: Duration::from_secs(30),
            jitter_ratio: 0.0,
            ..Default::default()
        };
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, "1".parse().unwrap());

        assert_eq!(
            retry_delay(&config, 1, Some(&headers)),
            Duration::from_secs(10)
        );
    }

    #[test]
    fn jitter_does_not_overflow_large_durations() {
        assert_eq!(add_jitter(max_duration(), 1.0), max_duration());
    }
}
