//! Resource-related request types.

/// Parameters for `query.resource.info`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ResourceInfoParams {
    /// Device model.
    pub model: String,
    /// Optional resource id.
    pub resource_id: Option<String>,
}

impl ResourceInfoParams {
    /// Create params with required fields.
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            resource_id: None,
        }
    }

    /// Set optional resource id.
    pub fn with_resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }
}

/// Parameters for `query.resource.name`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryResourceNameParams {
    /// Device id list (max 50).
    pub subject_ids: Vec<String>,
}

impl QueryResourceNameParams {
    /// Create params.
    pub fn new(subject_ids: impl Into<Vec<String>>) -> Self {
        Self {
            subject_ids: subject_ids.into(),
        }
    }
}

/// Parameters for `config.resource.info`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ConfigResourceInfoParams {
    /// Device id.
    pub subject_id: String,
    /// Resource id.
    pub resource_id: String,
    /// Custom resource name.
    pub name: String,
}

impl ConfigResourceInfoParams {
    /// Create params.
    pub fn new(
        subject_id: impl Into<String>,
        resource_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            subject_id: subject_id.into(),
            resource_id: resource_id.into(),
            name: name.into(),
        }
    }
}

/// A single device resource query entry for `query.resource.value`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ResourceValueQuery {
    /// Device id.
    pub subject_id: String,
    /// Optional resource id list. Empty means querying all open resources.
    pub resource_ids: Option<Vec<String>>,
}

impl ResourceValueQuery {
    /// Create params.
    pub fn new(subject_id: impl Into<String>) -> Self {
        Self {
            subject_id: subject_id.into(),
            resource_ids: None,
        }
    }

    /// Set resource id list.
    pub fn with_resource_ids(mut self, resource_ids: impl Into<Vec<String>>) -> Self {
        self.resource_ids = Some(resource_ids.into());
        self
    }
}

/// Parameters for `query.resource.value`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryResourceValueParams {
    /// Resource query list.
    pub resources: Vec<ResourceValueQuery>,
}

impl QueryResourceValueParams {
    /// Create params.
    pub fn new(resources: impl Into<Vec<ResourceValueQuery>>) -> Self {
        Self {
            resources: resources.into(),
        }
    }
}

/// A single resource write entry for `write.resource.device`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WriteResource {
    /// Resource id.
    pub resource_id: String,
    /// Resource value.
    pub value: String,
}

impl WriteResource {
    /// Create params.
    pub fn new(resource_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            resource_id: resource_id.into(),
            value: value.into(),
        }
    }
}

/// A single device entry for `write.resource.device`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WriteResourceDeviceItem {
    /// Device id.
    pub subject_id: String,
    /// Resource list.
    pub resources: Vec<WriteResource>,
}

impl WriteResourceDeviceItem {
    /// Create params.
    pub fn new(subject_id: impl Into<String>, resources: impl Into<Vec<WriteResource>>) -> Self {
        Self {
            subject_id: subject_id.into(),
            resources: resources.into(),
        }
    }
}

/// Parameters for `write.resource.device`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WriteResourceDeviceParams {
    /// Data array sent as request `data`.
    pub data: Vec<WriteResourceDeviceItem>,
}

impl WriteResourceDeviceParams {
    /// Create params.
    pub fn new(data: impl Into<Vec<WriteResourceDeviceItem>>) -> Self {
        Self { data: data.into() }
    }
}

/// Parameters for `fetch.resource.history`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FetchResourceHistoryParams {
    /// Device id.
    pub subject_id: String,
    /// Resource id list (max 100).
    pub resource_ids: Vec<String>,
    /// Start time (timestamp millis as string).
    pub start_time: String,
    /// Optional end time (timestamp millis as string). Default is now.
    pub end_time: Option<String>,
    /// Optional pull size (default 30, max 300).
    pub size: Option<u32>,
    /// Optional scan id for pagination.
    pub scan_id: Option<String>,
}

impl FetchResourceHistoryParams {
    /// Create params with required fields.
    pub fn new(
        subject_id: impl Into<String>,
        resource_ids: impl Into<Vec<String>>,
        start_time: impl Into<String>,
    ) -> Self {
        Self {
            subject_id: subject_id.into(),
            resource_ids: resource_ids.into(),
            start_time: start_time.into(),
            end_time: None,
            size: None,
            scan_id: None,
        }
    }

    /// Set end time (timestamp millis as string).
    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// Set pull size.
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    /// Set scan id for pagination.
    pub fn with_scan_id(mut self, scan_id: impl Into<String>) -> Self {
        self.scan_id = Some(scan_id.into());
        self
    }
}

/// Resource selection for `fetch.resource.statistics`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ResourceStatisticsQuery {
    /// Subject id.
    pub subject_id: String,
    /// Resource id list (max 50).
    pub resource_ids: Vec<String>,
    /// Aggregation types (`0..=4`).
    pub aggr_types: Vec<i32>,
}

impl ResourceStatisticsQuery {
    /// Create params.
    pub fn new(
        subject_id: impl Into<String>,
        resource_ids: impl Into<Vec<String>>,
        aggr_types: impl Into<Vec<i32>>,
    ) -> Self {
        Self {
            subject_id: subject_id.into(),
            resource_ids: resource_ids.into(),
            aggr_types: aggr_types.into(),
        }
    }
}

/// Parameters for `fetch.resource.statistics`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FetchResourceStatisticsParams {
    /// Resource selection.
    pub resources: ResourceStatisticsQuery,
    /// Start time (timestamp millis as string).
    pub start_time: String,
    /// Optional end time (timestamp millis as string). Default is now.
    pub end_time: Option<String>,
    /// Aggregation dimension (e.g. `"30m"`, `"1h"`, `"1d"`).
    pub dimension: String,
    /// Optional pull size (default 100, min 10, max 300).
    pub size: Option<u32>,
    /// Optional scan id for pagination.
    pub scan_id: Option<String>,
}

impl FetchResourceStatisticsParams {
    /// Create params with required fields.
    pub fn new(
        resources: ResourceStatisticsQuery,
        start_time: impl Into<String>,
        dimension: impl Into<String>,
    ) -> Self {
        Self {
            resources,
            start_time: start_time.into(),
            end_time: None,
            dimension: dimension.into(),
            size: None,
            scan_id: None,
        }
    }

    /// Set end time (timestamp millis as string).
    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// Set pull size.
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    /// Set scan id for pagination.
    pub fn with_scan_id(mut self, scan_id: impl Into<String>) -> Self {
        self.scan_id = Some(scan_id.into());
        self
    }
}

/// Parameters for `command.device.resource`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CommandDeviceResourceParams {
    /// Position id.
    pub position_id: String,
    /// Command content.
    pub query_text: String,
}

impl CommandDeviceResourceParams {
    /// Create params.
    pub fn new(position_id: impl Into<String>, query_text: impl Into<String>) -> Self {
        Self {
            position_id: position_id.into(),
            query_text: query_text.into(),
        }
    }
}
