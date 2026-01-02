use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::push::{
    QueryPushErrorMsgParams, SubscribeResourceParams, TraitSubscribeParams, TraitUnsubscribeParams,
    UnsubscribeResourceParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Push subscription APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct PushService {
    client: Client,
}

#[cfg(feature = "async")]
impl PushService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `config.resource.subscribe`.
    pub async fn subscribe(&self, params: SubscribeResourceParams) -> Result<AqaraValueResponse> {
        let resources = params
            .resources
            .into_iter()
            .map(|r| {
                let mut v = json!({
                    "subjectId": r.subject_id,
                    "resourceIds": r.resource_ids,
                });
                if let Some(attach) = r.attach {
                    v["attach"] = json!(attach);
                }
                v
            })
            .collect::<Vec<_>>();
        let data = json!({ "resources": resources });
        self.client
            .call_json("config.resource.subscribe", data, true, false)
            .await
    }

    /// `config.resource.unsubscribe`.
    pub async fn unsubscribe(
        &self,
        params: UnsubscribeResourceParams,
    ) -> Result<AqaraValueResponse> {
        let resources = params
            .resources
            .into_iter()
            .map(|r| json!({ "subjectId": r.subject_id, "resourceIds": r.resource_ids }))
            .collect::<Vec<_>>();
        let data = json!({ "resources": resources });
        self.client
            .call_json("config.resource.unsubscribe", data, true, false)
            .await
    }

    /// `query.push.errorMsg`.
    pub async fn error_msg(&self, params: QueryPushErrorMsgParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "appId": params.app_id,
        });
        if let Some(open_id) = params.open_id {
            data["openId"] = json!(open_id);
        }
        if let Some(msg_type) = params.msg_type {
            data["msgType"] = json!(msg_type);
        }
        if let Some(start_time) = params.start_time {
            data["startTime"] = json!(start_time);
        }
        if let Some(end_time) = params.end_time {
            data["endTime"] = json!(end_time);
        }
        if let Some(size) = params.size {
            data["size"] = json!(size);
        }
        if let Some(scan_id) = params.scan_id {
            data["scanId"] = json!(scan_id);
        }
        self.client
            .call_json("query.push.errorMsg", data, true, true)
            .await
    }

    /// `spec.config.trait.subscribe`.
    pub async fn subscribe_traits(
        &self,
        params: TraitSubscribeParams,
    ) -> Result<AqaraValueResponse> {
        let traits = params
            .traits
            .into_iter()
            .map(|t| {
                let mut v = json!({
                    "subjectId": t.subject_id,
                    "codePaths": t.code_paths,
                });
                if let Some(attach) = t.attach {
                    v["attach"] = json!(attach);
                }
                v
            })
            .collect::<Vec<_>>();
        let data = json!({ "traits": traits });
        self.client
            .call_json("spec.config.trait.subscribe", data, true, false)
            .await
    }

    /// `spec.config.trait.unsubscribe`.
    pub async fn unsubscribe_traits(
        &self,
        params: TraitUnsubscribeParams,
    ) -> Result<AqaraValueResponse> {
        let traits = params
            .traits
            .into_iter()
            .map(|t| json!({ "subjectId": t.subject_id, "codePaths": t.code_paths }))
            .collect::<Vec<_>>();
        let data = json!({ "traits": traits });
        self.client
            .call_json("spec.config.trait.unsubscribe", data, true, false)
            .await
    }
}

/// Push subscription APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingPushService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingPushService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `config.resource.subscribe`.
    pub fn subscribe(&self, params: SubscribeResourceParams) -> Result<AqaraValueResponse> {
        let resources = params
            .resources
            .into_iter()
            .map(|r| {
                let mut v = json!({
                    "subjectId": r.subject_id,
                    "resourceIds": r.resource_ids,
                });
                if let Some(attach) = r.attach {
                    v["attach"] = json!(attach);
                }
                v
            })
            .collect::<Vec<_>>();
        let data = json!({ "resources": resources });
        self.client
            .call_json("config.resource.subscribe", data, true, false)
    }

    /// `config.resource.unsubscribe`.
    pub fn unsubscribe(&self, params: UnsubscribeResourceParams) -> Result<AqaraValueResponse> {
        let resources = params
            .resources
            .into_iter()
            .map(|r| json!({ "subjectId": r.subject_id, "resourceIds": r.resource_ids }))
            .collect::<Vec<_>>();
        let data = json!({ "resources": resources });
        self.client
            .call_json("config.resource.unsubscribe", data, true, false)
    }

    /// `query.push.errorMsg`.
    pub fn error_msg(&self, params: QueryPushErrorMsgParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "appId": params.app_id,
        });
        if let Some(open_id) = params.open_id {
            data["openId"] = json!(open_id);
        }
        if let Some(msg_type) = params.msg_type {
            data["msgType"] = json!(msg_type);
        }
        if let Some(start_time) = params.start_time {
            data["startTime"] = json!(start_time);
        }
        if let Some(end_time) = params.end_time {
            data["endTime"] = json!(end_time);
        }
        if let Some(size) = params.size {
            data["size"] = json!(size);
        }
        if let Some(scan_id) = params.scan_id {
            data["scanId"] = json!(scan_id);
        }
        self.client
            .call_json("query.push.errorMsg", data, true, true)
    }

    /// `spec.config.trait.subscribe`.
    pub fn subscribe_traits(&self, params: TraitSubscribeParams) -> Result<AqaraValueResponse> {
        let traits = params
            .traits
            .into_iter()
            .map(|t| {
                let mut v = json!({
                    "subjectId": t.subject_id,
                    "codePaths": t.code_paths,
                });
                if let Some(attach) = t.attach {
                    v["attach"] = json!(attach);
                }
                v
            })
            .collect::<Vec<_>>();
        let data = json!({ "traits": traits });
        self.client
            .call_json("spec.config.trait.subscribe", data, true, false)
    }

    /// `spec.config.trait.unsubscribe`.
    pub fn unsubscribe_traits(&self, params: TraitUnsubscribeParams) -> Result<AqaraValueResponse> {
        let traits = params
            .traits
            .into_iter()
            .map(|t| json!({ "subjectId": t.subject_id, "codePaths": t.code_paths }))
            .collect::<Vec<_>>();
        let data = json!({ "traits": traits });
        self.client
            .call_json("spec.config.trait.unsubscribe", data, true, false)
    }
}
