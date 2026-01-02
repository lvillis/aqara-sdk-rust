use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::networking::{
    CloseConnectParams, OpenConnectParams, QueryBindKeyParams, QueryBindParams,
    QueryDeviceSupportGatewayParams, QueryPositionSupportGatewayParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Device networking / pairing related APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct NetworkingService {
    client: Client,
}

#[cfg(feature = "async")]
impl NetworkingService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `query.device.bindKey`.
    pub async fn bind_key(&self, params: QueryBindKeyParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "connectType": params.connect_type.unwrap_or_else(|| "lumi".to_string()),
        });
        self.client
            .call_json("query.device.bindKey", data, true, true)
            .await
    }

    /// `query.device.bind`.
    pub async fn bind(&self, params: QueryBindParams) -> Result<AqaraValueResponse> {
        let data = json!({ "bindKey": params.bind_key.expose() });
        self.client
            .call_json("query.device.bind", data, true, true)
            .await
    }

    /// `write.device.openConnect`.
    pub async fn open_connect(&self, params: OpenConnectParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("write.device.openConnect", data, true, false)
            .await
    }

    /// `write.device.closeConnect`.
    pub async fn close_connect(&self, params: CloseConnectParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("write.device.closeConnect", data, true, false)
            .await
    }

    /// `query.device.supportGateway`.
    pub async fn support_gateway(
        &self,
        params: QueryDeviceSupportGatewayParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "model": params.model });
        self.client
            .call_json("query.device.supportGateway", data, true, true)
            .await
    }

    /// `query.position.supportGateway`.
    pub async fn support_gateway_by_position(
        &self,
        params: QueryPositionSupportGatewayParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "model": params.model,
            "pageNum": params.page_num.to_string(),
            "pageSize": params.page_size.to_string(),
        });
        self.client
            .call_json("query.position.supportGateway", data, true, true)
            .await
    }
}

/// Device networking / pairing related APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingNetworkingService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingNetworkingService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `query.device.bindKey`.
    pub fn bind_key(&self, params: QueryBindKeyParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "connectType": params.connect_type.unwrap_or_else(|| "lumi".to_string()),
        });
        self.client
            .call_json("query.device.bindKey", data, true, true)
    }

    /// `query.device.bind`.
    pub fn bind(&self, params: QueryBindParams) -> Result<AqaraValueResponse> {
        let data = json!({ "bindKey": params.bind_key.expose() });
        self.client.call_json("query.device.bind", data, true, true)
    }

    /// `write.device.openConnect`.
    pub fn open_connect(&self, params: OpenConnectParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("write.device.openConnect", data, true, false)
    }

    /// `write.device.closeConnect`.
    pub fn close_connect(&self, params: CloseConnectParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("write.device.closeConnect", data, true, false)
    }

    /// `query.device.supportGateway`.
    pub fn support_gateway(
        &self,
        params: QueryDeviceSupportGatewayParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "model": params.model });
        self.client
            .call_json("query.device.supportGateway", data, true, true)
    }

    /// `query.position.supportGateway`.
    pub fn support_gateway_by_position(
        &self,
        params: QueryPositionSupportGatewayParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "model": params.model,
            "pageNum": params.page_num.to_string(),
            "pageSize": params.page_size.to_string(),
        });
        self.client
            .call_json("query.position.supportGateway", data, true, true)
    }
}
