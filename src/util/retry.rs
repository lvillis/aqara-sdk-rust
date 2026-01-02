use std::cmp;
use std::time::{Duration, SystemTime};

use http::HeaderMap;

use crate::types::RetryConfig;
use rand::Rng;

pub(crate) fn compute_backoff_with_jitter(attempt: u32, retry: RetryConfig) -> Duration {
    let base_ms = duration_to_millis_u64(retry.base_delay);
    if base_ms == 0 {
        return Duration::from_millis(0);
    }

    let exp = cmp::min(attempt.saturating_sub(1), 30);
    let factor = 1_u64 << exp;
    let exp_ms = base_ms.saturating_mul(factor);

    let capped = cmp::min(exp_ms, duration_to_millis_u64(retry.max_delay));
    if capped == 0 {
        return Duration::from_millis(0);
    }

    let mut rng = rand::rng();
    let jitter_ms = rng.random_range(0..=capped);
    Duration::from_millis(jitter_ms)
}

pub(crate) fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(http::header::RETRY_AFTER)?;
    let value = value.to_str().ok()?.trim();

    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let retry_at = httpdate::parse_http_date(value).ok()?;
    let now = SystemTime::now();
    match retry_at.duration_since(now) {
        Ok(d) => Some(d),
        Err(_) => Some(Duration::from_secs(0)),
    }
}

fn duration_to_millis_u64(d: Duration) -> u64 {
    cmp::min(d.as_millis(), u128::from(u64::MAX)) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_after_seconds_parses() {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::RETRY_AFTER,
            http::HeaderValue::from_static("5"),
        );
        assert_eq!(parse_retry_after(&headers), Some(Duration::from_secs(5)));
    }

    #[test]
    fn backoff_respects_max_delay() {
        let cfg = RetryConfig {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(150),
        };
        let delay = compute_backoff_with_jitter(10, cfg);
        assert!(delay <= cfg.max_delay);
    }
}
