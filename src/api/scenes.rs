use serde_json::{Value, json};

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::scenes::{
    CreateSceneParams, DeleteSceneParams, QuerySceneDetailParams, QueryScenesByPositionIdParams,
    QueryScenesBySubjectIdParams, RunSceneParams, SceneAction, SceneActionParam, UpdateSceneParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

fn scene_action_params(params: &[SceneActionParam]) -> Vec<Value> {
    params
        .iter()
        .map(|p| {
            let mut v = json!({ "paramId": p.param_id, "value": p.value });
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

fn scene_actions(actions: &[SceneAction]) -> Vec<Value> {
    actions
        .iter()
        .map(|a| {
            let mut v = json!({
                "subjectId": a.subject_id,
                "actionDefinitionId": a.action_definition_id,
                "params": scene_action_params(&a.params),
            });
            if let Some(delay_time) = &a.delay_time {
                v["delayTime"] = json!(delay_time);
            }
            if let Some(delay_time_unit) = &a.delay_time_unit {
                v["delayTimeUnit"] = json!(delay_time_unit);
            }
            v
        })
        .collect()
}

/// Scene APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct SceneService {
    client: Client,
}

#[cfg(feature = "async")]
impl SceneService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `config.scene.create`.
    pub async fn create(&self, params: CreateSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "action": scene_actions(&params.action),
        });
        self.client
            .call_json("config.scene.create", data, true, false)
            .await
    }

    /// `config.scene.update`.
    pub async fn update(&self, params: UpdateSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "sceneId": params.scene_id,
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "action": scene_actions(&params.action),
        });
        self.client
            .call_json("config.scene.update", data, true, false)
            .await
    }

    /// `config.scene.delete`.
    pub async fn delete(&self, params: DeleteSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({ "sceneId": params.scene_id });
        self.client
            .call_json("config.scene.delete", data, true, false)
            .await
    }

    /// `config.scene.run`.
    pub async fn run(&self, params: RunSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({ "sceneId": params.scene_id });
        self.client
            .call_json("config.scene.run", data, true, false)
            .await
    }

    #[cfg(feature = "unstable-raw")]
    /// `config.scene.try` (undocumented).
    pub async fn try_run_raw(&self, data: Value) -> Result<AqaraValueResponse> {
        self.client
            .call_json("config.scene.try", data, true, false)
            .await
    }

    /// `query.scene.detail`.
    pub async fn detail(&self, params: QuerySceneDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "sceneId": params.scene_id });
        self.client
            .call_json("query.scene.detail", data, true, true)
            .await
    }

    /// `query.scene.listBySubjectId`.
    pub async fn list_by_subject_id(
        &self,
        params: QueryScenesBySubjectIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectId": params.subject_id });
        self.client
            .call_json("query.scene.listBySubjectId", data, true, true)
            .await
    }

    /// `query.scene.listByPositionId`.
    pub async fn list_by_position_id(
        &self,
        params: QueryScenesByPositionIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageSize": params.page_size,
            "pageNum": params.page_num,
        });
        self.client
            .call_json("query.scene.listByPositionId", data, true, true)
            .await
    }
}

/// Scene APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingSceneService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingSceneService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `config.scene.create`.
    pub fn create(&self, params: CreateSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "action": scene_actions(&params.action),
        });
        self.client
            .call_json("config.scene.create", data, true, false)
    }

    /// `config.scene.update`.
    pub fn update(&self, params: UpdateSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "sceneId": params.scene_id,
            "name": params.name,
            "positionId": params.position_id.unwrap_or_default(),
            "action": scene_actions(&params.action),
        });
        self.client
            .call_json("config.scene.update", data, true, false)
    }

    /// `config.scene.delete`.
    pub fn delete(&self, params: DeleteSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({ "sceneId": params.scene_id });
        self.client
            .call_json("config.scene.delete", data, true, false)
    }

    /// `config.scene.run`.
    pub fn run(&self, params: RunSceneParams) -> Result<AqaraValueResponse> {
        let data = json!({ "sceneId": params.scene_id });
        self.client.call_json("config.scene.run", data, true, false)
    }

    #[cfg(feature = "unstable-raw")]
    /// `config.scene.try` (undocumented).
    pub fn try_run_raw(&self, data: Value) -> Result<AqaraValueResponse> {
        self.client.call_json("config.scene.try", data, true, false)
    }

    /// `query.scene.detail`.
    pub fn detail(&self, params: QuerySceneDetailParams) -> Result<AqaraValueResponse> {
        let data = json!({ "sceneId": params.scene_id });
        self.client
            .call_json("query.scene.detail", data, true, true)
    }

    /// `query.scene.listBySubjectId`.
    pub fn list_by_subject_id(
        &self,
        params: QueryScenesBySubjectIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectId": params.subject_id });
        self.client
            .call_json("query.scene.listBySubjectId", data, true, true)
    }

    /// `query.scene.listByPositionId`.
    pub fn list_by_position_id(
        &self,
        params: QueryScenesByPositionIdParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id.unwrap_or_default(),
            "pageSize": params.page_size,
            "pageNum": params.page_num,
        });
        self.client
            .call_json("query.scene.listByPositionId", data, true, true)
    }
}
