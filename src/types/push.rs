//! Push subscription related request types.

/// A resource subscription entry.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ResourceSubscription {
    /// Subject id.
    pub subject_id: String,
    /// Resource id list.
    pub resource_ids: Vec<String>,
    /// Optional attach string echoed in push payload.
    pub attach: Option<String>,
}

impl ResourceSubscription {
    /// Create params with required fields.
    pub fn new(subject_id: impl Into<String>, resource_ids: impl Into<Vec<String>>) -> Self {
        Self {
            subject_id: subject_id.into(),
            resource_ids: resource_ids.into(),
            attach: None,
        }
    }

    /// Set attach string echoed in push payload.
    pub fn with_attach(mut self, attach: impl Into<String>) -> Self {
        self.attach = Some(attach.into());
        self
    }
}

/// Parameters for `config.resource.subscribe`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SubscribeResourceParams {
    /// Resource subscriptions.
    pub resources: Vec<ResourceSubscription>,
}

impl SubscribeResourceParams {
    /// Create params.
    pub fn new(resources: impl Into<Vec<ResourceSubscription>>) -> Self {
        Self {
            resources: resources.into(),
        }
    }
}

/// Parameters for `config.resource.unsubscribe`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UnsubscribeResourceParams {
    /// Resource subscriptions to remove.
    pub resources: Vec<ResourceSubscription>,
}

impl UnsubscribeResourceParams {
    /// Create params.
    pub fn new(resources: impl Into<Vec<ResourceSubscription>>) -> Self {
        Self {
            resources: resources.into(),
        }
    }
}

/// Parameters for `query.push.errorMsg`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryPushErrorMsgParams {
    /// App id.
    pub app_id: String,
    /// Optional user open id.
    pub open_id: Option<String>,
    /// Optional message type.
    pub msg_type: Option<String>,
    /// Optional start timestamp millis.
    pub start_time: Option<i64>,
    /// Optional end timestamp millis.
    pub end_time: Option<i64>,
    /// Optional size.
    pub size: Option<u32>,
    /// Optional scan id.
    pub scan_id: Option<String>,
}

impl QueryPushErrorMsgParams {
    /// Create params with required fields.
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            open_id: None,
            msg_type: None,
            start_time: None,
            end_time: None,
            size: None,
            scan_id: None,
        }
    }

    /// Set open id.
    pub fn with_open_id(mut self, open_id: impl Into<String>) -> Self {
        self.open_id = Some(open_id.into());
        self
    }

    /// Set message type.
    pub fn with_msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.msg_type = Some(msg_type.into());
        self
    }

    /// Set start timestamp millis.
    pub fn with_start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set end timestamp millis.
    pub fn with_end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Set size.
    pub fn with_size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    /// Set scan id.
    pub fn with_scan_id(mut self, scan_id: impl Into<String>) -> Self {
        self.scan_id = Some(scan_id.into());
        self
    }
}

/// A trait subscription entry.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TraitSubscription {
    /// Target device id.
    pub subject_id: String,
    /// Code paths (`endpointId.functionCode.traitCode`).
    pub code_paths: Vec<String>,
    /// Optional attach string echoed in push payload.
    pub attach: Option<String>,
}

impl TraitSubscription {
    /// Create params.
    pub fn new(subject_id: impl Into<String>, code_paths: impl Into<Vec<String>>) -> Self {
        Self {
            subject_id: subject_id.into(),
            code_paths: code_paths.into(),
            attach: None,
        }
    }

    /// Set attach string echoed in push payload.
    pub fn with_attach(mut self, attach: impl Into<String>) -> Self {
        self.attach = Some(attach.into());
        self
    }
}

/// Parameters for `spec.config.trait.subscribe`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TraitSubscribeParams {
    /// Trait subscriptions.
    pub traits: Vec<TraitSubscription>,
}

impl TraitSubscribeParams {
    /// Create params.
    pub fn new(traits: impl Into<Vec<TraitSubscription>>) -> Self {
        Self {
            traits: traits.into(),
        }
    }
}

/// Parameters for `spec.config.trait.unsubscribe`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct TraitUnsubscribeParams {
    /// Trait subscriptions to remove.
    pub traits: Vec<TraitSubscription>,
}

impl TraitUnsubscribeParams {
    /// Create params.
    pub fn new(traits: impl Into<Vec<TraitSubscription>>) -> Self {
        Self {
            traits: traits.into(),
        }
    }
}
