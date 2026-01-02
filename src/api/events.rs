use serde_json::{Value, json};

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::events::{
    CreateEventParams, DeleteEventParams, EventCondition, EventConditionParam,
    QueryEventDetailParams, QueryEventsByPositionIdParams, QueryEventsBySubjectIdParams,
    UpdateEventParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

fn event_condition_params(params: &[EventConditionParam]) -> Vec<Value> {
    params
        .iter()
        .map(|p| {
            let mut v = json!({});
            if let Some(param_id) = &p.param_id {
                v["paramId"] = json!(param_id);
            }
            if let Some(value) = &p.value {
                v["value"] = json!(value);
            }
            if let Some(param_type) = &p.param_type {
                v["paramType"] = json!(param_type);
            }
            if let Some(param_unit) = &p.param_unit {
                v["paramUnit"] = json!(param_unit);
            }
            v
        })
        .collect()
}

fn event_conditions(conditions: &[EventCondition]) -> Vec<Value> {
    conditions
        .iter()
        .map(|c| {
            let mut v = json!({
                "triggerDefinitionId": c.trigger_definition_id,
            });
            if let Some(subject_id) = &c.subject_id {
                v["subjectId"] = json!(subject_id);
            }
            if let Some(model) = &c.model {
                v["model"] = json!(model);
            }
            if let Some(begin_time) = &c.begin_time {
                v["beginTime"] = json!(begin_time);
            }
            if let Some(end_time) = &c.end_time {
                v["endTime"] = json!(end_time);
            }
            if let Some(params) = &c.params {
                v["params"] = json!(event_condition_params(params));
            }
            v
        })
        .collect()
}

/// Event set (condition set) APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct EventService {
    client: Client,
}

#[cfg(feature = "async")]
impl EventService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `config.event.create`.
    pub async fn create(&self, params: CreateEventParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "name": params.name,
            "relation": params.relation,
            "condition": event_conditions(&params.condition),
        });
        self.client
            .call_json("config.event.create", data, true, false)
            .await
    }

    /// `config.event.update`.
    pub async fn update(&self, params: UpdateEventParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "eventId": params.event_id,
            "enable": params.enable,
            "positionId": params.position_id.unwrap_or_default(),
            "name": params.name,
            "relation": params.relation,
            "condition": event_conditions(&params.condition),
        });
        self.client
            .call_json("config.event.update", data, true, false)
            .await
    }

    /// `config.event.delete`.
    pub async fn delete(&self, params: DeleteEventParams) -> Result<AqaraValueResponse> {
        let data = json!({ "eventId": params.event_id });
        self.client
            .call_json("config.event.delete", data, true, false)
            .await
    }

    /// `query.event.detail`.
    pub async fn detail(&self, params: QueryEventDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "eventId": params.event_id });
        self.client
            .call_json("query.event.detail", data, true, true)
            .await
    }

    /// `query.event.listBySubjectId`.
    pub async fn list_by_subject_id(
        &self,
        params: QueryEventsBySubjectIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectId": params.subject_id });
        self.client
            .call_json("query.event.listBySubjectId", data, true, true)
            .await
    }

    /// `query.event.listByPositionId`.
    pub async fn list_by_position_id(
        &self,
        params: QueryEventsByPositionIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageSize": params.page_size,
            "pageNum": params.page_num,
        });
        self.client
            .call_json("query.event.listByPositionId", data, true, true)
            .await
    }
}

/// Event set (condition set) APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingEventService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingEventService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `config.event.create`.
    pub fn create(&self, params: CreateEventParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "name": params.name,
            "relation": params.relation,
            "condition": event_conditions(&params.condition),
        });
        self.client
            .call_json("config.event.create", data, true, false)
    }

    /// `config.event.update`.
    pub fn update(&self, params: UpdateEventParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "eventId": params.event_id,
            "enable": params.enable,
            "positionId": params.position_id.unwrap_or_default(),
            "name": params.name,
            "relation": params.relation,
            "condition": event_conditions(&params.condition),
        });
        self.client
            .call_json("config.event.update", data, true, false)
    }

    /// `config.event.delete`.
    pub fn delete(&self, params: DeleteEventParams) -> Result<AqaraValueResponse> {
        let data = json!({ "eventId": params.event_id });
        self.client
            .call_json("config.event.delete", data, true, false)
    }

    /// `query.event.detail`.
    pub fn detail(&self, params: QueryEventDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "eventId": params.event_id });
        self.client
            .call_json("query.event.detail", data, true, true)
    }

    /// `query.event.listBySubjectId`.
    pub fn list_by_subject_id(
        &self,
        params: QueryEventsBySubjectIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectId": params.subject_id });
        self.client
            .call_json("query.event.listBySubjectId", data, true, true)
    }

    /// `query.event.listByPositionId`.
    pub fn list_by_position_id(
        &self,
        params: QueryEventsByPositionIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageSize": params.page_size,
            "pageNum": params.page_num,
        });
        self.client
            .call_json("query.event.listByPositionId", data, true, true)
    }
}
