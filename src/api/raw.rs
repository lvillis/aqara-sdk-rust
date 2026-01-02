use serde_json::Value;

use crate::error::Result;
use crate::types::AqaraValueResponse;

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Raw APIs (unstable, async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct RawService {
    client: Client,
}

#[cfg(feature = "async")]
impl RawService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Call an arbitrary Aqara intent.
    pub async fn call(
        &self,
        intent: &str,
        data: Value,
        include_access_token: bool,
        idempotent: bool,
    ) -> Result<AqaraValueResponse> {
        self.client
            .call_json(intent, data, include_access_token, idempotent)
            .await
    }
}

/// Raw APIs (unstable, blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingRawService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingRawService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// Call an arbitrary Aqara intent.
    pub fn call(
        &self,
        intent: &str,
        data: Value,
        include_access_token: bool,
        idempotent: bool,
    ) -> Result<AqaraValueResponse> {
        self.client
            .call_json(intent, data, include_access_token, idempotent)
    }
}
