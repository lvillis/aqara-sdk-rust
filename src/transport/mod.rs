use std::error::Error as StdError;

use http::{HeaderMap, StatusCode};
use url::Url;

#[cfg(feature = "async")]
pub(crate) mod async_transport;

#[cfg(feature = "blocking")]
pub(crate) mod blocking_transport;

pub(crate) struct TransportRequest {
    pub(crate) url: Url,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Vec<u8>,
}

pub(crate) struct TransportResponse {
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Vec<u8>,
}

type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TransportErrorKind {
    Timeout,
    Connect,
    Other,
}

pub(crate) struct TransportError {
    pub(crate) kind: TransportErrorKind,
    pub(crate) message: String,
    pub(crate) source: BoxError,
}

impl TransportError {
    pub(crate) fn retryable(&self) -> bool {
        matches!(
            self.kind,
            TransportErrorKind::Timeout | TransportErrorKind::Connect
        )
    }
}

pub(crate) fn ensure_rustls_provider_installed() {
    #[cfg(feature = "rustls")]
    {
        if rustls::crypto::CryptoProvider::get_default().is_none() {
            let _ = rustls::crypto::ring::default_provider().install_default();
        }
    }
}
