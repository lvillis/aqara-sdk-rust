use std::sync::{Arc, RwLock};

use http::{HeaderMap, HeaderValue, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use url::Url;

use crate::api;
use crate::auth::{SignatureParts, sign_headers};
use crate::client::builder::ClientBuilder;
use crate::error::{ApiError, Error, Result};
use crate::transport::blocking_transport::BlockingTransport;
use crate::transport::{TransportRequest, TransportResponse};
use crate::types::{
    AqaraEnvelope, AqaraResponse, AqaraValueResponse, BodySnippetConfig, CallOptions, Credentials,
    RetryConfig, SecretString,
};
use crate::util::redact;
use crate::util::retry;

#[derive(Serialize)]
struct AqaraIntentRequest<'a, T: ?Sized> {
    intent: &'a str,
    data: &'a T,
}

/// Blocking Aqara client.
#[derive(Clone)]
pub struct BlockingClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    base_url: Url,
    credentials: Credentials,
    access_token: RwLock<Option<SecretString>>,
    lang: String,
    user_agent: String,
    retry: RetryConfig,
    body_snippet: BodySnippetConfig,
    extra_headers: HeaderMap,
    transport: BlockingTransport,
}

impl BlockingClient {
    /// Create a new builder.
    pub fn builder(credentials: Credentials) -> ClientBuilder {
        ClientBuilder::new(credentials)
    }

    pub(crate) fn from_builder(builder: ClientBuilder) -> Result<Self> {
        let cfg = builder.into_config()?;
        let transport = BlockingTransport::new(cfg.timeouts, &cfg.user_agent);

        Ok(Self {
            inner: Arc::new(ClientInner {
                base_url: cfg.base_url,
                credentials: cfg.credentials,
                access_token: RwLock::new(cfg.access_token),
                lang: cfg.lang,
                user_agent: cfg.user_agent,
                retry: cfg.retry,
                body_snippet: cfg.body_snippet,
                extra_headers: cfg.extra_headers,
                transport,
            }),
        })
    }

    /// Base URL used by this client.
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    /// Update access token used for endpoints that require it.
    pub fn set_access_token(&self, access_token: impl Into<String>) {
        let mut guard = match self.inner.access_token.write() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        *guard = Some(SecretString::new(access_token));
    }

    /// Clear access token.
    pub fn clear_access_token(&self) {
        let mut guard = match self.inner.access_token.write() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        *guard = None;
    }

    /// Auth service.
    pub fn auth(&self) -> api::auth::BlockingAuthService {
        api::auth::BlockingAuthService::new(self.clone())
    }

    /// Devices service.
    pub fn devices(&self) -> api::devices::BlockingDeviceService {
        api::devices::BlockingDeviceService::new(self.clone())
    }

    /// Resources service.
    pub fn resources(&self) -> api::resources::BlockingResourceService {
        api::resources::BlockingResourceService::new(self.clone())
    }

    /// Positions service.
    pub fn positions(&self) -> api::positions::BlockingPositionService {
        api::positions::BlockingPositionService::new(self.clone())
    }

    /// OTA service.
    pub fn ota(&self) -> api::ota::BlockingOtaService {
        api::ota::BlockingOtaService::new(self.clone())
    }

    /// Device networking / pairing service.
    pub fn networking(&self) -> api::networking::BlockingNetworkingService {
        api::networking::BlockingNetworkingService::new(self.clone())
    }

    /// IFTTT metadata query service.
    pub fn ifttt(&self) -> api::ifttt::BlockingIftttService {
        api::ifttt::BlockingIftttService::new(self.clone())
    }

    /// Automation (linkage) service.
    pub fn linkages(&self) -> api::linkages::BlockingLinkageService {
        api::linkages::BlockingLinkageService::new(self.clone())
    }

    /// Scene service.
    pub fn scenes(&self) -> api::scenes::BlockingSceneService {
        api::scenes::BlockingSceneService::new(self.clone())
    }

    /// Condition set (event) service.
    pub fn events(&self) -> api::events::BlockingEventService {
        api::events::BlockingEventService::new(self.clone())
    }

    /// Infrared device service.
    pub fn ir(&self) -> api::ir::BlockingIrService {
        api::ir::BlockingIrService::new(self.clone())
    }

    /// Push subscription service.
    pub fn push(&self) -> api::push::BlockingPushService {
        api::push::BlockingPushService::new(self.clone())
    }

    /// Voice command service.
    pub fn voice(&self) -> api::voice::BlockingVoiceService {
        api::voice::BlockingVoiceService::new(self.clone())
    }

    #[cfg(feature = "unstable-raw")]
    /// Raw (unstable) service for calling arbitrary intents.
    pub fn raw(&self) -> api::raw::BlockingRawService {
        api::raw::BlockingRawService::new(self.clone())
    }

    /// Call an Aqara intent and deserialize `result` into `Res`.
    pub fn call<Req, Res>(
        &self,
        intent: &str,
        data: &Req,
        options: CallOptions,
    ) -> Result<AqaraResponse<Res>>
    where
        Req: Serialize + ?Sized,
        Res: DeserializeOwned,
    {
        let resp = self.call_value(intent, data, options)?;
        let AqaraEnvelope {
            code,
            request_id,
            message,
            result,
        } = resp.envelope;

        let decoded_result = match result {
            Some(value) => {
                let snippet = self.snippet_json_if_enabled(&value);
                let parsed = serde_json::from_value(value).map_err(|e| Error::Decode {
                    message: "failed to decode response result".to_string(),
                    source: Box::new(e),
                    status: Some(resp.status),
                    request_id: Some(request_id.clone()),
                    body_snippet: snippet,
                })?;
                Some(parsed)
            }
            None => None,
        };

        Ok(AqaraResponse {
            status: resp.status,
            envelope: AqaraEnvelope {
                code,
                request_id,
                message,
                result: decoded_result,
            },
        })
    }

    /// Call an Aqara intent and return raw JSON `result`.
    pub fn call_value<Req>(
        &self,
        intent: &str,
        data: &Req,
        options: CallOptions,
    ) -> Result<AqaraValueResponse>
    where
        Req: Serialize + ?Sized,
    {
        let body = AqaraIntentRequest { intent, data };
        let body_bytes = serde_json::to_vec(&body).map_err(|e| Error::Decode {
            message: "failed to encode request body".to_string(),
            source: Box::new(e),
            status: None,
            request_id: None,
            body_snippet: None,
        })?;

        let mut headers = self.inner.extra_headers.clone();
        insert_required_headers(&mut headers, &self.inner.user_agent, &self.inner.lang)?;

        let access_token = self.read_access_token();
        let signature = sign_headers(
            &self.inner.credentials,
            access_token.as_ref(),
            options.include_access_token,
        )?;
        insert_signature_headers(
            &mut headers,
            &self.inner.credentials,
            &signature,
            access_token.as_ref(),
            options.include_access_token,
        )?;

        let req = TransportRequest {
            url: self.inner.base_url.clone(),
            headers,
            body: body_bytes,
        };

        #[cfg(feature = "tracing")]
        let _span = tracing::debug_span!(
            "aqara.call",
            intent = %intent,
            idempotent = options.idempotent,
            include_access_token = options.include_access_token
        )
        .entered();

        self.execute_with_retry(intent, req, options.idempotent)
    }

    pub(crate) fn call_json(
        &self,
        intent: &str,
        data: Value,
        include_access_token: bool,
        idempotent: bool,
    ) -> Result<AqaraValueResponse> {
        let options = if include_access_token {
            CallOptions::with_access_token()
        } else {
            CallOptions::without_access_token()
        }
        .idempotent(idempotent);

        self.call_value(intent, &data, options)
    }

    fn execute_with_retry(
        &self,
        intent: &str,
        req: TransportRequest,
        idempotent: bool,
    ) -> Result<AqaraValueResponse> {
        let max_attempts = if idempotent {
            self.inner.retry.max_retries.saturating_add(1)
        } else {
            1
        };

        #[cfg(not(any(feature = "metrics", feature = "tracing")))]
        let _ = intent;

        let mut attempt: u32 = 0;
        loop {
            attempt = attempt.saturating_add(1);

            #[cfg(feature = "metrics")]
            metrics::counter!("aqara_sdk.requests_total", "intent" => intent.to_string())
                .increment(1);

            #[cfg(feature = "tracing")]
            tracing::trace!(attempt, "sending request");
            let resp = self.inner.transport.send(&req);

            match resp {
                Ok(resp) => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(attempt, status = %resp.status, "received response");

                    if should_retry_status(resp.status, idempotent, attempt, max_attempts) {
                        let delay = retry_delay_for_status(&resp, attempt, self.inner.retry);
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            attempt,
                            status = %resp.status,
                            delay_ms = delay.as_millis(),
                            "retrying due to http status"
                        );
                        sleep(delay);
                        continue;
                    }

                    let parsed = self.parse_response(resp);
                    match parsed {
                        Ok(ok) => return Ok(ok),
                        Err(e) => {
                            if idempotent && attempt < max_attempts && should_retry_error(&e) {
                                let delay = retry_delay_for_error(&e, attempt, self.inner.retry);
                                #[cfg(feature = "tracing")]
                                tracing::debug!(
                                    attempt,
                                    delay_ms = delay.as_millis(),
                                    error_kind = ?e.kind(),
                                    "retrying due to api error"
                                );
                                sleep(delay);
                                continue;
                            }
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    if idempotent && attempt < max_attempts && e.retryable() {
                        let delay = retry::compute_backoff_with_jitter(attempt, self.inner.retry);
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            attempt,
                            delay_ms = delay.as_millis(),
                            "retrying due to transport error"
                        );
                        sleep(delay);
                        continue;
                    }
                    return Err(Error::Transport {
                        message: e.message,
                        source: Some(e.source),
                    });
                }
            }
        }
    }

    fn parse_response(&self, resp: TransportResponse) -> Result<AqaraValueResponse> {
        let request_id = extract_request_id(&resp.headers, &resp.body);
        let snippet = self.snippet_if_enabled(&resp.body);

        if resp.status == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::RateLimited {
                retry_after: retry::parse_retry_after(&resp.headers),
                request_id,
                body_snippet: snippet,
            });
        }

        let envelope: AqaraEnvelope<Value> = match serde_json::from_slice(&resp.body) {
            Ok(envelope) => envelope,
            Err(e) => {
                if resp.status.is_success() {
                    return Err(Error::Decode {
                        message: "failed to decode response body".to_string(),
                        source: Box::new(e),
                        status: Some(resp.status),
                        request_id,
                        body_snippet: snippet,
                    });
                }
                return Err(Error::Http {
                    status: resp.status,
                    request_id,
                    body_snippet: snippet,
                });
            }
        };

        let request_id = Some(envelope.request_id.clone());

        if envelope.code == 429 {
            return Err(Error::RateLimited {
                retry_after: retry::parse_retry_after(&resp.headers),
                request_id,
                body_snippet: snippet,
            });
        }

        if envelope.code != 0 {
            return Err(Error::Api {
                error: ApiError {
                    status: Some(resp.status),
                    code: Some(envelope.code),
                    message: Some(envelope.message.clone()),
                    request_id,
                    body_snippet: snippet,
                },
            });
        }

        if !resp.status.is_success() {
            return Err(Error::Http {
                status: resp.status,
                request_id,
                body_snippet: snippet,
            });
        }

        Ok(AqaraResponse {
            status: resp.status,
            envelope,
        })
    }

    fn read_access_token(&self) -> Option<SecretString> {
        let guard = match self.inner.access_token.read() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.clone()
    }

    fn snippet_if_enabled(&self, body: &[u8]) -> Option<String> {
        if !self.inner.body_snippet.enabled {
            return None;
        }
        Some(redact::snippet_from_bytes(
            body,
            self.inner.body_snippet.max_len,
        ))
    }

    fn snippet_json_if_enabled(&self, value: &Value) -> Option<String> {
        if !self.inner.body_snippet.enabled {
            return None;
        }
        let bytes = serde_json::to_vec(value).ok()?;
        Some(redact::snippet_from_bytes(
            &bytes,
            self.inner.body_snippet.max_len,
        ))
    }
}

fn insert_required_headers(headers: &mut HeaderMap, user_agent: &str, lang: &str) -> Result<()> {
    headers.insert(
        http::header::USER_AGENT,
        HeaderValue::from_str(user_agent).map_err(|e| Error::InvalidConfig {
            message: format!("invalid user-agent header value: {e}"),
        })?,
    );
    headers.insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        http::header::HeaderName::from_static("lang"),
        HeaderValue::from_str(lang).map_err(|e| Error::InvalidConfig {
            message: format!("invalid lang header value: {e}"),
        })?,
    );
    Ok(())
}

fn insert_signature_headers(
    headers: &mut HeaderMap,
    credentials: &Credentials,
    signature: &SignatureParts,
    access_token: Option<&SecretString>,
    include_access_token: bool,
) -> Result<()> {
    insert_header_str(headers, "appid", credentials.app_id())?;
    insert_header_str(headers, "keyid", credentials.key_id())?;
    insert_header_str(headers, "nonce", &signature.nonce)?;
    insert_header_str(headers, "time", &signature.time_millis)?;
    insert_header_str(headers, "sign", &signature.sign)?;

    if include_access_token && let Some(token) = access_token {
        insert_header_str(headers, "accesstoken", token.expose())?;
    }

    Ok(())
}

fn insert_header_str(headers: &mut HeaderMap, name: &'static str, value: &str) -> Result<()> {
    let name = http::header::HeaderName::from_static(name);
    let value = HeaderValue::from_str(value).map_err(|e| Error::InvalidConfig {
        message: format!("invalid header value for {name}: {e}"),
    })?;
    headers.insert(name, value);
    Ok(())
}

fn extract_request_id(headers: &HeaderMap, body: &[u8]) -> Option<String> {
    let from_headers = headers
        .get("x-request-id")
        .or_else(|| headers.get("request-id"))
        .or_else(|| headers.get("x-correlation-id"))
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    if from_headers.is_some() {
        return from_headers;
    }

    let body: Value = serde_json::from_slice(body).ok()?;
    body.get("requestId")
        .or_else(|| body.get("request_id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

fn should_retry_status(
    status: StatusCode,
    idempotent: bool,
    attempt: u32,
    max_attempts: u32,
) -> bool {
    if !idempotent || attempt >= max_attempts {
        return false;
    }
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn retry_delay_for_status(
    resp: &TransportResponse,
    attempt: u32,
    retry_cfg: RetryConfig,
) -> std::time::Duration {
    retry::parse_retry_after(&resp.headers)
        .unwrap_or_else(|| retry::compute_backoff_with_jitter(attempt, retry_cfg))
}

fn should_retry_error(err: &Error) -> bool {
    match err {
        Error::RateLimited { .. } => true,
        Error::Api { error } => error.code.is_some_and(is_retryable_api_code),
        _ => false,
    }
}

fn is_retryable_api_code(code: i64) -> bool {
    matches!(code, 100 | 104 | 429 | 500 | 501)
}

fn retry_delay_for_error(err: &Error, attempt: u32, retry_cfg: RetryConfig) -> std::time::Duration {
    match err {
        Error::RateLimited { retry_after, .. } => {
            retry_after.unwrap_or_else(|| retry::compute_backoff_with_jitter(attempt, retry_cfg))
        }
        _ => retry::compute_backoff_with_jitter(attempt, retry_cfg),
    }
}

fn sleep(delay: std::time::Duration) {
    if delay == std::time::Duration::from_secs(0) {
        return;
    }
    std::thread::sleep(delay);
}
