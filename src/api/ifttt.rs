use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::ifttt::IftttModelsParams;

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// IFTTT metadata query APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct IftttService {
    client: Client,
}

#[cfg(feature = "async")]
impl IftttService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `query.ifttt.trigger`.
    pub async fn trigger(&self, params: IftttModelsParams) -> Result<AqaraValueResponse> {
        let data = json!({ "models": params.models });
        self.client
            .call_json("query.ifttt.trigger", data, true, true)
            .await
    }

    /// `query.ifttt.action`.
    pub async fn action(&self, params: IftttModelsParams) -> Result<AqaraValueResponse> {
        let data = json!({ "models": params.models });
        self.client
            .call_json("query.ifttt.action", data, true, true)
            .await
    }
}

/// IFTTT metadata query APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingIftttService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingIftttService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `query.ifttt.trigger`.
    pub fn trigger(&self, params: IftttModelsParams) -> Result<AqaraValueResponse> {
        let data = json!({ "models": params.models });
        self.client
            .call_json("query.ifttt.trigger", data, true, true)
    }

    /// `query.ifttt.action`.
    pub fn action(&self, params: IftttModelsParams) -> Result<AqaraValueResponse> {
        let data = json!({ "models": params.models });
        self.client
            .call_json("query.ifttt.action", data, true, true)
    }
}
