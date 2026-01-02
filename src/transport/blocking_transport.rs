use crate::transport::{
    TransportError, TransportErrorKind, TransportRequest, TransportResponse,
    ensure_rustls_provider_installed,
};
use crate::types::TimeoutConfig;

pub(crate) struct BlockingTransport {
    agent: ureq::Agent,
}

impl BlockingTransport {
    pub(crate) fn new(timeouts: TimeoutConfig, user_agent: &str) -> Self {
        ensure_rustls_provider_installed();
        let mut builder = ureq::config::Config::builder()
            .http_status_as_error(false)
            .user_agent(user_agent);

        builder = builder.timeout_connect(timeouts.connect);
        builder = builder.timeout_send_request(timeouts.request);
        builder = builder.timeout_recv_response(timeouts.request);
        builder = builder.timeout_recv_body(timeouts.read);

        let config = builder.build();
        Self {
            agent: config.new_agent(),
        }
    }

    pub(crate) fn send(&self, req: &TransportRequest) -> Result<TransportResponse, TransportError> {
        let mut builder = self.agent.post(req.url.as_str());

        for (name, value) in req.headers.iter() {
            let Ok(value) = value.to_str() else {
                continue;
            };
            builder = builder.header(name.as_str(), value);
        }

        let resp = builder.send(req.body.as_slice()).map_err(map_ureq_error)?;
        let (parts, mut body) = resp.into_parts();
        let body = body.read_to_vec().map_err(map_ureq_error)?;

        Ok(TransportResponse {
            status: parts.status,
            headers: parts.headers,
            body,
        })
    }
}

fn map_ureq_error(err: ureq::Error) -> TransportError {
    let kind = match &err {
        ureq::Error::Timeout(_) => TransportErrorKind::Timeout,
        ureq::Error::Io(_)
        | ureq::Error::HostNotFound
        | ureq::Error::ConnectionFailed
        | ureq::Error::BadUri(_) => TransportErrorKind::Connect,
        _ => TransportErrorKind::Other,
    };

    TransportError {
        kind,
        message: err.to_string(),
        source: Box::new(err),
    }
}
