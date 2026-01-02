use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::positions::{
    CreatePositionParams, DeletePositionParams, ListPositionsParams, PositionDetailParams,
    SetPositionTimeZoneParams, UpdatePositionParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Position-related APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PositionService {
    client: Client,
}

#[cfg(feature = "async")]
impl PositionService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `config.position.create`.
    pub async fn create(&self, params: CreatePositionParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "positionName": params.position_name,
        });
        if let Some(description) = params.description {
            data["description"] = json!(description);
        }
        if let Some(parent_position_id) = params.parent_position_id {
            data["parentPositionId"] = json!(parent_position_id);
        }
        self.client
            .call_json("config.position.create", data, true, false)
            .await
    }

    /// `config.position.update`.
    pub async fn update(&self, params: UpdatePositionParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "positionId": params.position_id,
            "positionName": params.position_name,
        });
        if let Some(description) = params.description {
            data["description"] = json!(description);
        }
        self.client
            .call_json("config.position.update", data, true, false)
            .await
    }

    /// `config.position.delete`.
    pub async fn delete(&self, params: DeletePositionParams) -> Result<AqaraValueResponse> {
        let data = json!({ "positionId": params.position_id });
        self.client
            .call_json("config.position.delete", data, true, false)
            .await
    }

    /// `config.position.timeZone`.
    pub async fn set_time_zone(
        &self,
        params: SetPositionTimeZoneParams,
    ) -> Result<AqaraValueResponse> {
        let mut data = json!({ "positionId": params.position_id });
        if let Some(time_zone) = params.time_zone {
            data["timeZone"] = json!(time_zone);
        }
        self.client
            .call_json("config.position.timeZone", data, true, false)
            .await
    }

    /// `query.position.info`.
    pub async fn list(&self, params: ListPositionsParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "parentPositionId": params.parent_position_id.unwrap_or_default(),
            "pageNum": params.page_num,
            "pageSize": params.page_size,
        });
        self.client
            .call_json("query.position.info", data, true, true)
            .await
    }

    /// `query.position.detail`.
    pub async fn detail(&self, params: PositionDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "positionIds": params.position_ids });
        self.client
            .call_json("query.position.detail", data, true, true)
            .await
    }
}

/// Position-related APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingPositionService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPositionService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `config.position.create`.
    pub fn create(&self, params: CreatePositionParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "positionName": params.position_name,
        });
        if let Some(description) = params.description {
            data["description"] = json!(description);
        }
        if let Some(parent_position_id) = params.parent_position_id {
            data["parentPositionId"] = json!(parent_position_id);
        }
        self.client
            .call_json("config.position.create", data, true, false)
    }

    /// `config.position.update`.
    pub fn update(&self, params: UpdatePositionParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "positionId": params.position_id,
            "positionName": params.position_name,
        });
        if let Some(description) = params.description {
            data["description"] = json!(description);
        }
        self.client
            .call_json("config.position.update", data, true, false)
    }

    /// `config.position.delete`.
    pub fn delete(&self, params: DeletePositionParams) -> Result<AqaraValueResponse> {
        let data = json!({ "positionId": params.position_id });
        self.client
            .call_json("config.position.delete", data, true, false)
    }

    /// `config.position.timeZone`.
    pub fn set_time_zone(&self, params: SetPositionTimeZoneParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "positionId": params.position_id });
        if let Some(time_zone) = params.time_zone {
            data["timeZone"] = json!(time_zone);
        }
        self.client
            .call_json("config.position.timeZone", data, true, false)
    }

    /// `query.position.info`.
    pub fn list(&self, params: ListPositionsParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "parentPositionId": params.parent_position_id.unwrap_or_default(),
            "pageNum": params.page_num,
            "pageSize": params.page_size,
        });
        self.client
            .call_json("query.position.info", data, true, true)
    }

    /// `query.position.detail`.
    pub fn detail(&self, params: PositionDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "positionIds": params.position_ids });
        self.client
            .call_json("query.position.detail", data, true, true)
    }
}
