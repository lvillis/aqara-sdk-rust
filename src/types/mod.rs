//! Public types shared across async/blocking clients.

use std::fmt;
use std::time::Duration;

use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use url::Url;

pub mod auth;
pub mod devices;
pub mod events;
pub mod ifttt;
pub mod ir;
pub mod linkages;
pub mod networking;
pub mod ota;
pub mod positions;
pub mod push;
pub mod resources;
pub mod scenes;
pub mod voice;

/// A string that should not be logged.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SecretString(String);

impl SecretString {
    /// Create a new secret string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Expose the underlying secret value.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

/// Credentials required by Aqara Open API.
#[derive(Clone)]
pub struct Credentials {
    app_id: String,
    key_id: String,
    app_key: SecretString,
}

impl Credentials {
    /// Create credentials.
    pub fn new(
        app_id: impl Into<String>,
        key_id: impl Into<String>,
        app_key: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            key_id: key_id.into(),
            app_key: SecretString::new(app_key),
        }
    }

    /// App id (public).
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Key id (public).
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// App key (secret).
    pub fn app_key(&self) -> &SecretString {
        &self.app_key
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("app_id", &self.app_id)
            .field("key_id", &self.key_id)
            .field("app_key", &self.app_key)
            .finish()
    }
}

/// Aqara cloud endpoint selection.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Endpoint {
    /// Mainland China.
    China,
    /// United States.
    Usa,
    /// Europe (Germany).
    Europe,
    /// Korea.
    Korea,
    /// Russia.
    Russia,
    /// Singapore.
    Singapore,
    /// Custom base URL.
    Custom(Url),
}

impl Endpoint {
    pub(crate) fn base_url_str(&self) -> Option<&'static str> {
        match self {
            Self::China => Some("https://open-cn.aqara.com/v3.0/open/api"),
            Self::Usa => Some("https://open-usa.aqara.com/v3.0/open/api"),
            Self::Europe => Some("https://open-ger.aqara.com/v3.0/open/api"),
            Self::Korea => Some("https://open-kr.aqara.com/v3.0/open/api"),
            Self::Russia => Some("https://open-ru.aqara.com/v3.0/open/api"),
            Self::Singapore => Some("https://open-sg.aqara.com/v3.0/open/api"),
            Self::Custom(_) => None,
        }
    }

    pub(crate) fn custom_url(&self) -> Option<&Url> {
        match self {
            Self::Custom(url) => Some(url),
            _ => None,
        }
    }
}

/// Timeout configuration.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct TimeoutConfig {
    /// Connection / TLS handshake timeout.
    pub connect: Option<Duration>,
    /// Timeout for sending the request (excluding reading the full response body).
    pub request: Option<Duration>,
    /// Timeout for reading the response body.
    pub read: Option<Duration>,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect: Some(Duration::from_secs(10)),
            request: Some(Duration::from_secs(30)),
            read: Some(Duration::from_secs(30)),
        }
    }
}

impl TimeoutConfig {
    /// Create a timeout configuration.
    pub fn new(
        connect: Option<Duration>,
        request: Option<Duration>,
        read: Option<Duration>,
    ) -> Self {
        Self {
            connect,
            request,
            read,
        }
    }
}

/// Retry configuration (applies only to idempotent operations).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct RetryConfig {
    /// Maximum number of retries (excluding the first attempt).
    pub max_retries: u32,
    /// Base delay for exponential backoff.
    pub base_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(2),
        }
    }
}

impl RetryConfig {
    /// Create a retry configuration.
    pub fn new(max_retries: u32, base_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay,
            max_delay,
        }
    }
}

/// Response body snippet capture configuration (used for diagnostics on errors).
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct BodySnippetConfig {
    /// Enable capturing a truncated, redacted snippet on errors.
    pub enabled: bool,
    /// Max snippet length in bytes.
    pub max_len: usize,
}

impl Default for BodySnippetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_len: 2048,
        }
    }
}

impl BodySnippetConfig {
    /// Create a body snippet configuration.
    pub fn new(enabled: bool, max_len: usize) -> Self {
        Self { enabled, max_len }
    }
}

/// Options for calling an Aqara intent.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CallOptions {
    /// Include `Accesstoken` header and participate in signing.
    pub include_access_token: bool,
    /// Whether the operation is idempotent (enables retries).
    pub idempotent: bool,
}

impl CallOptions {
    /// Create options for an endpoint requiring an access token.
    pub fn with_access_token() -> Self {
        Self {
            include_access_token: true,
            idempotent: false,
        }
    }

    /// Create options for an endpoint that does not use an access token.
    pub fn without_access_token() -> Self {
        Self {
            include_access_token: false,
            idempotent: false,
        }
    }

    /// Set whether the operation is idempotent (enables retries).
    pub fn idempotent(mut self, idempotent: bool) -> Self {
        self.idempotent = idempotent;
        self
    }
}

/// Aqara response envelope described in the Open API docs.
#[derive(Debug, Deserialize)]
#[non_exhaustive]
pub struct AqaraEnvelope<T> {
    /// Business status code (`0` means success).
    pub code: i64,
    /// Request id.
    #[serde(rename = "requestId", alias = "request_id")]
    pub request_id: String,
    /// Human-readable message.
    #[serde(alias = "msg")]
    pub message: String,
    /// Result payload (optional).
    #[serde(default)]
    pub result: Option<T>,
}

/// A successful Aqara response.
#[derive(Debug)]
#[non_exhaustive]
pub struct AqaraResponse<T> {
    /// HTTP status.
    pub status: StatusCode,
    /// Response envelope.
    pub envelope: AqaraEnvelope<T>,
}

impl<T> AqaraResponse<T> {
    /// Convenience accessor for `requestId`.
    pub fn request_id(&self) -> &str {
        &self.envelope.request_id
    }

    /// Convenience accessor for `message`.
    pub fn message(&self) -> &str {
        &self.envelope.message
    }

    /// Convenience accessor for `result`.
    pub fn result(&self) -> Option<&T> {
        self.envelope.result.as_ref()
    }
}

/// A successful Aqara response whose result is raw JSON.
pub type AqaraValueResponse = AqaraResponse<Value>;
