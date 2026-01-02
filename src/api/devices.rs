use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::devices::{
    QueryDeviceInfoParams, QuerySubDevicesParams, UnbindDeviceParams, UpdateDeviceNameParams,
    UpdateDevicePositionParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Device-related APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct DeviceService {
    client: Client,
}

#[cfg(feature = "async")]
impl DeviceService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `query.device.info`.
    pub async fn info(&self, params: QueryDeviceInfoParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageNum": params.page_num,
            "pageSize": params.page_size,
        });
        if let Some(dids) = params.dids {
            data["dids"] = json!(dids);
        }
        self.client
            .call_json("query.device.info", data, true, true)
            .await
    }

    /// `query.device.subInfo`.
    pub async fn sub_info(&self, params: QuerySubDevicesParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.gateway_did });
        self.client
            .call_json("query.device.subInfo", data, true, true)
            .await
    }

    /// `config.device.name`.
    pub async fn update_name(&self, params: UpdateDeviceNameParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "did": params.did,
            "name": params.name,
        });
        self.client
            .call_json("config.device.name", data, true, false)
            .await
    }

    /// `config.device.position`.
    pub async fn update_position(
        &self,
        params: UpdateDevicePositionParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "dids": params.dids,
            "positionId": params.position_id,
        });
        self.client
            .call_json("config.device.position", data, true, false)
            .await
    }

    /// `write.device.unbind`.
    pub async fn unbind(&self, params: UnbindDeviceParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("write.device.unbind", data, true, false)
            .await
    }
}

/// Device-related APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingDeviceService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingDeviceService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `query.device.info`.
    pub fn info(&self, params: QueryDeviceInfoParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageNum": params.page_num,
            "pageSize": params.page_size,
        });
        if let Some(dids) = params.dids {
            data["dids"] = json!(dids);
        }
        self.client.call_json("query.device.info", data, true, true)
    }

    /// `query.device.subInfo`.
    pub fn sub_info(&self, params: QuerySubDevicesParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.gateway_did });
        self.client
            .call_json("query.device.subInfo", data, true, true)
    }

    /// `config.device.name`.
    pub fn update_name(&self, params: UpdateDeviceNameParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "did": params.did,
            "name": params.name,
        });
        self.client
            .call_json("config.device.name", data, true, false)
    }

    /// `config.device.position`.
    pub fn update_position(
        &self,
        params: UpdateDevicePositionParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "dids": params.dids,
            "positionId": params.position_id,
        });
        self.client
            .call_json("config.device.position", data, true, false)
    }

    /// `write.device.unbind`.
    pub fn unbind(&self, params: UnbindDeviceParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("write.device.unbind", data, true, false)
    }
}
