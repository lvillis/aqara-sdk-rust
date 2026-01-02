use std::time::Duration;

use http::HeaderMap;

use crate::transport::{
    TransportError, TransportErrorKind, TransportRequest, TransportResponse,
    ensure_rustls_provider_installed,
};
use crate::types::TimeoutConfig;

pub(crate) struct AsyncTransport {
    client: reqwest::Client,
}

impl AsyncTransport {
    pub(crate) fn new(connect_timeout: Option<Duration>) -> Result<Self, reqwest::Error> {
        ensure_rustls_provider_installed();
        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = connect_timeout {
            builder = builder.connect_timeout(timeout);
        }
        Ok(Self {
            client: builder.build()?,
        })
    }

    pub(crate) async fn send(
        &self,
        req: &TransportRequest,
        timeouts: TimeoutConfig,
    ) -> Result<TransportResponse, TransportError> {
        let mut builder = self
            .client
            .post(req.url.clone())
            .headers(req.headers.clone());
        builder = builder.body(req.body.clone());

        let send_fut = builder.send();
        let response = match timeouts.request {
            Some(d) => match tokio::time::timeout(d, send_fut).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(TransportError {
                        kind: TransportErrorKind::Timeout,
                        message: "request timed out".to_string(),
                        source: Box::new(e),
                    });
                }
            },
            None => send_fut.await,
        }
        .map_err(map_reqwest_error)?;

        let status = response.status();
        let headers: HeaderMap = response.headers().clone();

        let bytes_fut = response.bytes();
        let body = match timeouts.read {
            Some(d) => match tokio::time::timeout(d, bytes_fut).await {
                Ok(r) => r,
                Err(e) => {
                    return Err(TransportError {
                        kind: TransportErrorKind::Timeout,
                        message: "response read timed out".to_string(),
                        source: Box::new(e),
                    });
                }
            },
            None => bytes_fut.await,
        }
        .map_err(map_reqwest_error)?
        .to_vec();

        Ok(TransportResponse {
            status,
            headers,
            body,
        })
    }
}

fn map_reqwest_error(err: reqwest::Error) -> TransportError {
    let kind = if err.is_timeout() {
        TransportErrorKind::Timeout
    } else if err.is_connect() {
        TransportErrorKind::Connect
    } else {
        TransportErrorKind::Other
    };

    TransportError {
        kind,
        message: err.to_string(),
        source: Box::new(err),
    }
}
