//! Device-related request types.

/// Parameters for `query.device.info`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryDeviceInfoParams {
    /// Optional device id list (max 100).
    pub dids: Option<Vec<String>>,
    /// Optional position id. Empty means querying all devices in the account/project.
    pub position_id: Option<String>,
    /// Page number (1-based).
    pub page_num: u32,
    /// Page size.
    pub page_size: u32,
}

impl Default for QueryDeviceInfoParams {
    fn default() -> Self {
        Self {
            dids: None,
            position_id: None,
            page_num: 1,
            page_size: 50,
        }
    }
}

impl QueryDeviceInfoParams {
    /// Set device id list (max 100).
    pub fn with_dids(mut self, dids: impl Into<Vec<String>>) -> Self {
        self.dids = Some(dids.into());
        self
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }

    /// Set page number (1-based).
    pub fn with_page_num(mut self, page_num: u32) -> Self {
        self.page_num = page_num;
        self
    }

    /// Set page size.
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }
}

/// Parameters for `query.device.subInfo`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QuerySubDevicesParams {
    /// Gateway DID.
    pub gateway_did: String,
}

impl QuerySubDevicesParams {
    /// Create params.
    pub fn new(gateway_did: impl Into<String>) -> Self {
        Self {
            gateway_did: gateway_did.into(),
        }
    }
}

/// Parameters for `config.device.name`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdateDeviceNameParams {
    /// Device DID.
    pub did: String,
    /// New device name.
    pub name: String,
}

impl UpdateDeviceNameParams {
    /// Create params.
    pub fn new(did: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            name: name.into(),
        }
    }
}

/// Parameters for `config.device.position`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdateDevicePositionParams {
    /// Device DID list.
    pub dids: Vec<String>,
    /// Target position id.
    pub position_id: String,
}

impl UpdateDevicePositionParams {
    /// Create params.
    pub fn new(dids: impl Into<Vec<String>>, position_id: impl Into<String>) -> Self {
        Self {
            dids: dids.into(),
            position_id: position_id.into(),
        }
    }
}

/// Parameters for `write.device.unbind`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UnbindDeviceParams {
    /// Gateway DID or sub-device DID.
    pub did: String,
}

impl UnbindDeviceParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}
