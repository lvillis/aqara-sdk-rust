use http::HeaderMap;
use url::Url;

use crate::error::{Error, Result};
use crate::types::{
    BodySnippetConfig, Credentials, Endpoint, RetryConfig, SecretString, TimeoutConfig,
};

/// SDK client builder.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    credentials: Credentials,
    endpoint: Endpoint,
    access_token: Option<SecretString>,
    lang: String,
    user_agent: String,
    timeouts: TimeoutConfig,
    retry: RetryConfig,
    body_snippet: BodySnippetConfig,
    extra_headers: HeaderMap,
}

impl ClientBuilder {
    /// Create a new builder with required credentials.
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            endpoint: Endpoint::China,
            access_token: None,
            lang: "en".to_string(),
            user_agent: format!("aqara-sdk-rust/{}", env!("CARGO_PKG_VERSION")),
            timeouts: TimeoutConfig::default(),
            retry: RetryConfig::default(),
            body_snippet: BodySnippetConfig::default(),
            extra_headers: HeaderMap::new(),
        }
    }

    /// Select Aqara endpoint.
    pub fn endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = endpoint;
        self
    }

    /// Override base URL.
    pub fn base_url(mut self, base_url: Url) -> Self {
        self.endpoint = Endpoint::Custom(base_url);
        self
    }

    /// Set an access token for endpoints that require it.
    pub fn access_token(mut self, access_token: impl Into<String>) -> Self {
        self.access_token = Some(SecretString::new(access_token));
        self
    }

    /// Set `Lang` header value (default: `"en"`).
    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = lang.into();
        self
    }

    /// Set `User-Agent` header value.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Set timeout configuration.
    pub fn timeouts(mut self, timeouts: TimeoutConfig) -> Self {
        self.timeouts = timeouts;
        self
    }

    /// Set retry configuration.
    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.retry = retry;
        self
    }

    /// Configure response snippet capture for diagnostics.
    pub fn body_snippet(mut self, config: BodySnippetConfig) -> Self {
        self.body_snippet = config;
        self
    }

    /// Add an extra header sent with every request.
    pub fn extra_header(
        mut self,
        name: http::header::HeaderName,
        value: http::header::HeaderValue,
    ) -> Self {
        self.extra_headers.insert(name, value);
        self
    }

    #[cfg(feature = "async")]
    /// Build an async client.
    pub fn build(self) -> Result<super::Client> {
        super::async_client::Client::from_builder(self)
    }

    #[cfg(feature = "blocking")]
    /// Build a blocking client.
    pub fn build_blocking(self) -> Result<super::BlockingClient> {
        super::blocking_client::BlockingClient::from_builder(self)
    }

    pub(crate) fn into_config(self) -> Result<ClientConfig> {
        let base_url = normalize_base_url(endpoint_to_url(&self.endpoint)?)?;

        if base_url.scheme() == "https"
            && !(cfg!(feature = "rustls") || cfg!(feature = "native-tls"))
        {
            return Err(Error::InvalidConfig {
                message: "https base_url requires enabling one of: rustls, native-tls".to_string(),
            });
        }

        Ok(ClientConfig {
            base_url,
            credentials: self.credentials,
            access_token: self.access_token,
            lang: self.lang,
            user_agent: self.user_agent,
            timeouts: self.timeouts,
            retry: self.retry,
            body_snippet: self.body_snippet,
            extra_headers: self.extra_headers,
        })
    }
}

pub(crate) struct ClientConfig {
    pub(crate) base_url: Url,
    pub(crate) credentials: Credentials,
    pub(crate) access_token: Option<SecretString>,
    pub(crate) lang: String,
    pub(crate) user_agent: String,
    pub(crate) timeouts: TimeoutConfig,
    pub(crate) retry: RetryConfig,
    pub(crate) body_snippet: BodySnippetConfig,
    pub(crate) extra_headers: HeaderMap,
}

fn endpoint_to_url(endpoint: &Endpoint) -> Result<Url> {
    if let Some(url) = endpoint.custom_url() {
        return Ok(url.clone());
    }

    let url = endpoint
        .base_url_str()
        .ok_or_else(|| Error::InvalidConfig {
            message: "invalid endpoint".to_string(),
        })?;

    Url::parse(url).map_err(|e| Error::InvalidConfig {
        message: format!("invalid base_url: {e}"),
    })
}

fn normalize_base_url(mut url: Url) -> Result<Url> {
    url.set_fragment(None);
    url.set_query(None);

    let path = url.path().trim_end_matches('/').to_string();
    if path.is_empty() {
        return Err(Error::InvalidConfig {
            message: "base_url path cannot be empty".to_string(),
        });
    }
    url.set_path(&path);

    Ok(url)
}
