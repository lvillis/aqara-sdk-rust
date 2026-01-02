use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

use http::StatusCode;

/// SDK result type.
pub type Result<T> = std::result::Result<T, Error>;

type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// High-level error categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Authentication or authorization failure.
    Auth,
    /// Resource not found.
    NotFound,
    /// Conflict (usually idempotency or version conflict).
    Conflict,
    /// Rate limited.
    RateLimited,
    /// Network / HTTP transport failures.
    Transport,
    /// Response decoding failures.
    Decode,
    /// Aqara API returned a structured error.
    Api,
    /// Invalid client configuration.
    InvalidConfig,
}

/// A structured API error.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ApiError {
    /// HTTP status code, if available.
    pub status: Option<StatusCode>,
    /// Aqara business error code, if available.
    pub code: Option<i64>,
    /// Error message, if available.
    pub message: Option<String>,
    /// Provider request id, if available.
    pub request_id: Option<String>,
    /// Redacted response snippet, if enabled.
    pub body_snippet: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.status, self.code.as_ref(), self.message.as_ref()) {
            (Some(status), Some(code), Some(message)) => {
                write!(f, "status={status}, code={code}, message={message}")
            }
            (Some(status), Some(code), None) => write!(f, "status={status}, code={code}"),
            (Some(status), None, Some(message)) => write!(f, "status={status}, message={message}"),
            (Some(status), None, None) => write!(f, "status={status}"),
            (None, Some(code), Some(message)) => write!(f, "code={code}, message={message}"),
            (None, Some(code), None) => write!(f, "code={code}"),
            (None, None, Some(message)) => write!(f, "{message}"),
            (None, None, None) => write!(f, "unknown api error"),
        }
    }
}

impl StdError for ApiError {}

/// SDK error type.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Invalid client configuration.
    #[error("invalid config: {message}")]
    InvalidConfig {
        /// Human-readable message.
        message: String,
    },

    /// Network/transport level failures.
    #[error("transport error: {message}")]
    Transport {
        /// Human-readable message.
        message: String,
        /// Underlying error (not part of the stable public API).
        #[source]
        source: Option<BoxError>,
    },

    /// HTTP returned a non-success status.
    #[error("http error: status={status}")]
    Http {
        /// HTTP status.
        status: StatusCode,
        /// Provider request id, if available.
        request_id: Option<String>,
        /// Redacted response snippet, if enabled.
        body_snippet: Option<String>,
    },

    /// The request was rate limited.
    #[error("rate limited")]
    RateLimited {
        /// Suggested delay before retrying.
        retry_after: Option<Duration>,
        /// Provider request id, if available.
        request_id: Option<String>,
        /// Redacted response snippet, if enabled.
        body_snippet: Option<String>,
    },

    /// Aqara API returned a structured business error.
    #[error("api error: {error}")]
    Api {
        /// Structured details.
        error: ApiError,
    },

    /// Failed to decode response body.
    #[error("decode error: {message}")]
    Decode {
        /// Human-readable message.
        message: String,
        /// Underlying decode error.
        #[source]
        source: BoxError,
        /// HTTP status, if available.
        status: Option<StatusCode>,
        /// Provider request id, if available.
        request_id: Option<String>,
        /// Redacted response snippet, if enabled.
        body_snippet: Option<String>,
    },
}

impl Error {
    /// Get the high-level category.
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::InvalidConfig { .. } => ErrorKind::InvalidConfig,
            Self::Transport { .. } => ErrorKind::Transport,
            Self::Decode { .. } => ErrorKind::Decode,
            Self::RateLimited { .. } => ErrorKind::RateLimited,
            Self::Api { error } => error
                .code
                .and_then(code_to_kind)
                .or_else(|| status_to_kind(error.status))
                .unwrap_or(ErrorKind::Api),
            Self::Http { status, .. } => {
                status_to_kind(Some(*status)).unwrap_or(ErrorKind::Transport)
            }
        }
    }

    /// HTTP status code, if any.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Http { status, .. } => Some(*status),
            Self::Api { error } => error.status,
            Self::Decode { status, .. } => *status,
            Self::RateLimited { .. } => Some(StatusCode::TOO_MANY_REQUESTS),
            Self::InvalidConfig { .. } | Self::Transport { .. } => None,
        }
    }

    /// Provider request id, if any.
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::Http { request_id, .. } => request_id.as_deref(),
            Self::Api { error } => error.request_id.as_deref(),
            Self::Decode { request_id, .. } => request_id.as_deref(),
            Self::RateLimited { request_id, .. } => request_id.as_deref(),
            Self::InvalidConfig { .. } | Self::Transport { .. } => None,
        }
    }

    /// Redacted response body snippet, if enabled.
    pub fn body_snippet(&self) -> Option<&str> {
        match self {
            Self::Http { body_snippet, .. } => body_snippet.as_deref(),
            Self::Api { error } => error.body_snippet.as_deref(),
            Self::Decode { body_snippet, .. } => body_snippet.as_deref(),
            Self::RateLimited { body_snippet, .. } => body_snippet.as_deref(),
            Self::InvalidConfig { .. } | Self::Transport { .. } => None,
        }
    }

    /// Suggested delay before retrying, if rate limited.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::RateLimited { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

fn status_to_kind(status: Option<StatusCode>) -> Option<ErrorKind> {
    match status? {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Some(ErrorKind::Auth),
        StatusCode::NOT_FOUND => Some(ErrorKind::NotFound),
        StatusCode::CONFLICT => Some(ErrorKind::Conflict),
        StatusCode::TOO_MANY_REQUESTS => Some(ErrorKind::RateLimited),
        _ => None,
    }
}

fn code_to_kind(code: i64) -> Option<ErrorKind> {
    match code {
        106 | 107 | 108 | 109 | 403 => Some(ErrorKind::Auth),
        429 => Some(ErrorKind::RateLimited),
        _ => None,
    }
}
