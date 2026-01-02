//! Automation (linkage) related request types.

use serde_json::Value;

/// Parameters for `config.linkage.create`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreateLinkageParams {
    /// Automation name.
    pub name: String,
    /// Optional position id. Empty means default position.
    pub position_id: Option<String>,
    /// Conditions object (see Aqara docs for structure).
    pub conditions: Value,
    /// Actions object (see Aqara docs for structure).
    pub actions: Value,
}

impl CreateLinkageParams {
    /// Create params with required fields.
    pub fn new(name: impl Into<String>, conditions: Value, actions: Value) -> Self {
        Self {
            name: name.into(),
            position_id: None,
            conditions,
            actions,
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.linkage.update`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdateLinkageParams {
    /// Automation id.
    pub linkage_id: String,
    /// Automation name.
    pub name: String,
    /// Optional position id.
    pub position_id: Option<String>,
    /// Conditions object (see Aqara docs for structure).
    pub conditions: Value,
    /// Actions object (see Aqara docs for structure).
    pub actions: Value,
}

impl UpdateLinkageParams {
    /// Create params with required fields.
    pub fn new(
        linkage_id: impl Into<String>,
        name: impl Into<String>,
        conditions: Value,
        actions: Value,
    ) -> Self {
        Self {
            linkage_id: linkage_id.into(),
            name: name.into(),
            position_id: None,
            conditions,
            actions,
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.linkage.delete`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DeleteLinkageParams {
    /// Automation id.
    pub linkage_id: String,
}

impl DeleteLinkageParams {
    /// Create params.
    pub fn new(linkage_id: impl Into<String>) -> Self {
        Self {
            linkage_id: linkage_id.into(),
        }
    }
}

/// Parameters for `config.linkage.enable`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EnableLinkageParams {
    /// Automation id.
    pub linkage_id: String,
    /// Enable flag (`0` or `1`).
    pub enable: i32,
}

impl EnableLinkageParams {
    /// Create params.
    pub fn new(linkage_id: impl Into<String>, enable: i32) -> Self {
        Self {
            linkage_id: linkage_id.into(),
            enable,
        }
    }
}

/// Parameters for `query.linkage.detail`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryLinkageDetailParams {
    /// Automation id.
    pub linkage_id: String,
}

impl QueryLinkageDetailParams {
    /// Create params.
    pub fn new(linkage_id: impl Into<String>) -> Self {
        Self {
            linkage_id: linkage_id.into(),
        }
    }
}

/// Parameters for `query.linkage.listBySubjectId`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryLinkagesBySubjectIdParams {
    /// Subject id.
    pub subject_id: String,
}

impl QueryLinkagesBySubjectIdParams {
    /// Create params.
    pub fn new(subject_id: impl Into<String>) -> Self {
        Self {
            subject_id: subject_id.into(),
        }
    }
}

/// Parameters for `query.linkage.listByPositionId`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryLinkagesByPositionIdParams {
    /// Optional position id (empty for all).
    pub position_id: Option<String>,
    /// Page number (1-based).
    pub page_num: u32,
    /// Page size.
    pub page_size: u32,
}

impl Default for QueryLinkagesByPositionIdParams {
    fn default() -> Self {
        Self {
            position_id: None,
            page_num: 1,
            page_size: 50,
        }
    }
}

impl QueryLinkagesByPositionIdParams {
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
