//! Device networking / pairing related request types.

use crate::types::SecretString;

/// Parameters for `query.device.bindKey`.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct QueryBindKeyParams {
    /// Optional position id. Empty means default position.
    pub position_id: Option<String>,
    /// Optional connect type (default: `"lumi"`).
    pub connect_type: Option<String>,
}

impl QueryBindKeyParams {
    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }

    /// Set connect type.
    pub fn with_connect_type(mut self, connect_type: impl Into<String>) -> Self {
        self.connect_type = Some(connect_type.into());
        self
    }
}

/// Parameters for `query.device.bind`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryBindParams {
    /// Bind key (secret).
    pub bind_key: SecretString,
}

impl QueryBindParams {
    /// Create params.
    pub fn new(bind_key: impl Into<String>) -> Self {
        Self {
            bind_key: SecretString::new(bind_key),
        }
    }
}

/// Parameters for `write.device.openConnect`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct OpenConnectParams {
    /// Gateway DID.
    pub did: String,
}

impl OpenConnectParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}

/// Parameters for `write.device.closeConnect`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CloseConnectParams {
    /// Gateway DID.
    pub did: String,
}

impl CloseConnectParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}

/// Parameters for `query.device.supportGateway`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryDeviceSupportGatewayParams {
    /// Sub-device model.
    pub model: String,
}

impl QueryDeviceSupportGatewayParams {
    /// Create params.
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}

/// Parameters for `query.position.supportGateway`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryPositionSupportGatewayParams {
    /// Optional position id. Empty means default position.
    pub position_id: Option<String>,
    /// Sub-device model.
    pub model: String,
    /// Page number (1-based).
    pub page_num: u32,
    /// Page size.
    pub page_size: u32,
}

impl QueryPositionSupportGatewayParams {
    /// Create params with required fields.
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            position_id: None,
            model: model.into(),
            page_num: 1,
            page_size: 30,
        }
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
