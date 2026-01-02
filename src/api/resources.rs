use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::resources::{
    CommandDeviceResourceParams, ConfigResourceInfoParams, FetchResourceHistoryParams,
    FetchResourceStatisticsParams, QueryResourceNameParams, QueryResourceValueParams,
    ResourceInfoParams, WriteResourceDeviceParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Resource-related APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct ResourceService {
    client: Client,
}

#[cfg(feature = "async")]
impl ResourceService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `query.resource.info`.
    pub async fn info(&self, params: ResourceInfoParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "model": params.model });
        if let Some(resource_id) = params.resource_id {
            data["resourceId"] = json!(resource_id);
        }
        self.client
            .call_json("query.resource.info", data, true, true)
            .await
    }

    /// `query.resource.name`.
    pub async fn name(&self, params: QueryResourceNameParams) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectIds": params.subject_ids });
        self.client
            .call_json("query.resource.name", data, true, true)
            .await
    }

    /// `config.resource.info`.
    pub async fn set_info(&self, params: ConfigResourceInfoParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "subjectId": params.subject_id,
            "resourceId": params.resource_id,
            "name": params.name,
        });
        self.client
            .call_json("config.resource.info", data, true, false)
            .await
    }

    /// `query.resource.value`.
    pub async fn value(&self, params: QueryResourceValueParams) -> Result<AqaraValueResponse> {
        let resources = params
            .resources
            .into_iter()
            .map(|r| {
                let mut v = json!({ "subjectId": r.subject_id });
                if let Some(resource_ids) = r.resource_ids {
                    v["resourceIds"] = json!(resource_ids);
                }
                v
            })
            .collect::<Vec<_>>();
        let data = json!({ "resources": resources });
        self.client
            .call_json("query.resource.value", data, true, true)
            .await
    }

    /// `write.resource.device`.
    pub async fn write_device(
        &self,
        params: WriteResourceDeviceParams,
    ) -> Result<AqaraValueResponse> {
        let data = params
            .data
            .into_iter()
            .map(|d| {
                let resources = d
                    .resources
                    .into_iter()
                    .map(|r| json!({ "resourceId": r.resource_id, "value": r.value }))
                    .collect::<Vec<_>>();
                json!({ "subjectId": d.subject_id, "resources": resources })
            })
            .collect::<Vec<_>>();
        self.client
            .call_json("write.resource.device", json!(data), true, false)
            .await
    }

    /// `fetch.resource.history`.
    pub async fn history(&self, params: FetchResourceHistoryParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "subjectId": params.subject_id,
            "resourceIds": params.resource_ids,
            "startTime": params.start_time,
        });
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
            .call_json("fetch.resource.history", data, true, true)
            .await
    }

    /// `fetch.resource.statistics`.
    pub async fn statistics(
        &self,
        params: FetchResourceStatisticsParams,
    ) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "resources": {
                "subjectId": params.resources.subject_id,
                "aggrTypes": params.resources.aggr_types,
                "resourceIds": params.resources.resource_ids,
            },
            "startTime": params.start_time,
            "dimension": params.dimension,
        });
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
            .call_json("fetch.resource.statistics", data, true, true)
            .await
    }

    /// `command.device.resource`.
    pub async fn command_device_resource(
        &self,
        params: CommandDeviceResourceParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id,
            "queryText": params.query_text,
        });
        self.client
            .call_json("command.device.resource", data, true, false)
            .await
    }
}

/// Resource-related APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingResourceService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingResourceService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `query.resource.info`.
    pub fn info(&self, params: ResourceInfoParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "model": params.model });
        if let Some(resource_id) = params.resource_id {
            data["resourceId"] = json!(resource_id);
        }
        self.client
            .call_json("query.resource.info", data, true, true)
    }

    /// `query.resource.name`.
    pub fn name(&self, params: QueryResourceNameParams) -> Result<AqaraValueResponse> {
        let data = json!({ "subjectIds": params.subject_ids });
        self.client
            .call_json("query.resource.name", data, true, true)
    }

    /// `config.resource.info`.
    pub fn set_info(&self, params: ConfigResourceInfoParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "subjectId": params.subject_id,
            "resourceId": params.resource_id,
            "name": params.name,
        });
        self.client
            .call_json("config.resource.info", data, true, false)
    }

    /// `query.resource.value`.
    pub fn value(&self, params: QueryResourceValueParams) -> Result<AqaraValueResponse> {
        let resources = params
            .resources
            .into_iter()
            .map(|r| {
                let mut v = json!({ "subjectId": r.subject_id });
                if let Some(resource_ids) = r.resource_ids {
                    v["resourceIds"] = json!(resource_ids);
                }
                v
            })
            .collect::<Vec<_>>();
        let data = json!({ "resources": resources });
        self.client
            .call_json("query.resource.value", data, true, true)
    }

    /// `write.resource.device`.
    pub fn write_device(&self, params: WriteResourceDeviceParams) -> Result<AqaraValueResponse> {
        let data = params
            .data
            .into_iter()
            .map(|d| {
                let resources = d
                    .resources
                    .into_iter()
                    .map(|r| json!({ "resourceId": r.resource_id, "value": r.value }))
                    .collect::<Vec<_>>();
                json!({ "subjectId": d.subject_id, "resources": resources })
            })
            .collect::<Vec<_>>();
        self.client
            .call_json("write.resource.device", json!(data), true, false)
    }

    /// `fetch.resource.history`.
    pub fn history(&self, params: FetchResourceHistoryParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "subjectId": params.subject_id,
            "resourceIds": params.resource_ids,
            "startTime": params.start_time,
        });
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
            .call_json("fetch.resource.history", data, true, true)
    }

    /// `fetch.resource.statistics`.
    pub fn statistics(&self, params: FetchResourceStatisticsParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "resources": {
                "subjectId": params.resources.subject_id,
                "aggrTypes": params.resources.aggr_types,
                "resourceIds": params.resources.resource_ids,
            },
            "startTime": params.start_time,
            "dimension": params.dimension,
        });
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
            .call_json("fetch.resource.statistics", data, true, true)
    }

    /// `command.device.resource`.
    pub fn command_device_resource(
        &self,
        params: CommandDeviceResourceParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id,
            "queryText": params.query_text,
        });
        self.client
            .call_json("command.device.resource", data, true, false)
    }
}
