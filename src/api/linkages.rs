use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::linkages::{
    CreateLinkageParams, DeleteLinkageParams, EnableLinkageParams, QueryLinkageDetailParams,
    QueryLinkagesByPositionIdParams, QueryLinkagesBySubjectIdParams, UpdateLinkageParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Automation (linkage) APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct LinkageService {
    client: Client,
}

#[cfg(feature = "async")]
impl LinkageService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `config.linkage.create`.
    pub async fn create(&self, params: CreateLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "conditions": params.conditions,
            "actions": params.actions,
        });
        self.client
            .call_json("config.linkage.create", data, true, false)
            .await
    }

    /// `config.linkage.update`.
    pub async fn update(&self, params: UpdateLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "linkageId": params.linkage_id,
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "conditions": params.conditions,
            "actions": params.actions,
        });
        self.client
            .call_json("config.linkage.update", data, true, false)
            .await
    }

    /// `config.linkage.delete`.
    pub async fn delete(&self, params: DeleteLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({ "linkageId": params.linkage_id });
        self.client
            .call_json("config.linkage.delete", data, true, false)
            .await
    }

    /// `config.linkage.enable`.
    pub async fn enable(&self, params: EnableLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "linkageId": params.linkage_id,
            "enable": params.enable,
        });
        self.client
            .call_json("config.linkage.enable", data, true, false)
            .await
    }

    /// `query.linkage.detail`.
    pub async fn detail(&self, params: QueryLinkageDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "linkageId": params.linkage_id });
        self.client
            .call_json("query.linkage.detail", data, true, true)
            .await
    }

    /// `query.linkage.listBySubjectId`.
    pub async fn list_by_subject_id(
        &self,
        params: QueryLinkagesBySubjectIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectId": params.subject_id });
        self.client
            .call_json("query.linkage.listBySubjectId", data, true, true)
            .await
    }

    /// `query.linkage.listByPositionId`.
    pub async fn list_by_position_id(
        &self,
        params: QueryLinkagesByPositionIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageNum": params.page_num,
            "pageSize": params.page_size,
        });
        self.client
            .call_json("query.linkage.listByPositionId", data, true, true)
            .await
    }
}

/// Automation (linkage) APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingLinkageService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingLinkageService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `config.linkage.create`.
    pub fn create(&self, params: CreateLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "conditions": params.conditions,
            "actions": params.actions,
        });
        self.client
            .call_json("config.linkage.create", data, true, false)
    }

    /// `config.linkage.update`.
    pub fn update(&self, params: UpdateLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "linkageId": params.linkage_id,
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "conditions": params.conditions,
            "actions": params.actions,
        });
        self.client
            .call_json("config.linkage.update", data, true, false)
    }

    /// `config.linkage.delete`.
    pub fn delete(&self, params: DeleteLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({ "linkageId": params.linkage_id });
        self.client
            .call_json("config.linkage.delete", data, true, false)
    }

    /// `config.linkage.enable`.
    pub fn enable(&self, params: EnableLinkageParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "linkageId": params.linkage_id,
            "enable": params.enable,
        });
        self.client
            .call_json("config.linkage.enable", data, true, false)
    }

    /// `query.linkage.detail`.
    pub fn detail(&self, params: QueryLinkageDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "linkageId": params.linkage_id });
        self.client
            .call_json("query.linkage.detail", data, true, true)
    }

    /// `query.linkage.listBySubjectId`.
    pub fn list_by_subject_id(
        &self,
        params: QueryLinkagesBySubjectIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectId": params.subject_id });
        self.client
            .call_json("query.linkage.listBySubjectId", data, true, true)
    }

    /// `query.linkage.listByPositionId`.
    pub fn list_by_position_id(
        &self,
        params: QueryLinkagesByPositionIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageNum": params.page_num,
            "pageSize": params.page_size,
        });
        self.client
            .call_json("query.linkage.listByPositionId", data, true, true)
    }
}
