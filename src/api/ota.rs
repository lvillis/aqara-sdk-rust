use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::ota::{OtaFirmwareParams, OtaUpgradeParams, OtaUpgradeStatusParams};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// OTA-related APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct OtaService {
    client: Client,
}

#[cfg(feature = "async")]
impl OtaService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `query.ota.firmware`.
    pub async fn firmware(&self, params: OtaFirmwareParams) -> Result<AqaraValueResponse> {
        let data = json!({ "model": params.model });
        self.client
            .call_json("query.ota.firmware", data, true, true)
            .await
    }

    /// `write.ota.upgrade`.
    pub async fn upgrade(&self, params: OtaUpgradeParams) -> Result<AqaraValueResponse> {
        let data = json!({ "dids": params.dids });
        self.client
            .call_json("write.ota.upgrade", data, true, false)
            .await
    }

    /// `query.ota.upgrade`.
    pub async fn upgrade_status(
        &self,
        params: OtaUpgradeStatusParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "dids": params.dids });
        self.client
            .call_json("query.ota.upgrade", data, true, true)
            .await
    }
}

/// OTA-related APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingOtaService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingOtaService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `query.ota.firmware`.
    pub fn firmware(&self, params: OtaFirmwareParams) -> Result<AqaraValueResponse> {
        let data = json!({ "model": params.model });
        self.client
            .call_json("query.ota.firmware", data, true, true)
    }

    /// `write.ota.upgrade`.
    pub fn upgrade(&self, params: OtaUpgradeParams) -> Result<AqaraValueResponse> {
        let data = json!({ "dids": params.dids });
        self.client
            .call_json("write.ota.upgrade", data, true, false)
    }

    /// `query.ota.upgrade`.
    pub fn upgrade_status(&self, params: OtaUpgradeStatusParams) -> Result<AqaraValueResponse> {
        let data = json!({ "dids": params.dids });
        self.client.call_json("query.ota.upgrade", data, true, true)
    }
}
