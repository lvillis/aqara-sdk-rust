//! Position-related request types.

/// Parameters for `config.position.create`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreatePositionParams {
    /// Position name.
    pub position_name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Optional parent position id (empty for top-level).
    pub parent_position_id: Option<String>,
}

impl CreatePositionParams {
    /// Create params with required fields.
    pub fn new(position_name: impl Into<String>) -> Self {
        Self {
            position_name: position_name.into(),
            description: None,
            parent_position_id: None,
        }
    }

    /// Set description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set parent position id.
    pub fn with_parent_position_id(mut self, parent_position_id: impl Into<String>) -> Self {
        self.parent_position_id = Some(parent_position_id.into());
        self
    }
}

/// Parameters for `config.position.update`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdatePositionParams {
    /// Position id.
    pub position_id: String,
    /// Updated position name.
    pub position_name: String,
    /// Optional updated description.
    pub description: Option<String>,
}

impl UpdatePositionParams {
    /// Create params with required fields.
    pub fn new(position_id: impl Into<String>, position_name: impl Into<String>) -> Self {
        Self {
            position_id: position_id.into(),
            position_name: position_name.into(),
            description: None,
        }
    }

    /// Set updated description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Parameters for `config.position.delete`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DeletePositionParams {
    /// Position id.
    pub position_id: String,
}

impl DeletePositionParams {
    /// Create params.
    pub fn new(position_id: impl Into<String>) -> Self {
        Self {
            position_id: position_id.into(),
        }
    }
}

/// Parameters for `config.position.timeZone`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SetPositionTimeZoneParams {
    /// Top-level position id.
    pub position_id: String,
    /// Optional timezone string (e.g. `"GMT+08:00"`).
    pub time_zone: Option<String>,
}

impl SetPositionTimeZoneParams {
    /// Create params with required fields.
    pub fn new(position_id: impl Into<String>) -> Self {
        Self {
            position_id: position_id.into(),
            time_zone: None,
        }
    }

    /// Set timezone string (e.g. `"GMT+08:00"`).
    pub fn with_time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }
}

/// Parameters for `query.position.info`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ListPositionsParams {
    /// Optional parent position id.
    pub parent_position_id: Option<String>,
    /// Page number (1-based).
    pub page_num: u32,
    /// Page size.
    pub page_size: u32,
}

impl Default for ListPositionsParams {
    fn default() -> Self {
        Self {
            parent_position_id: None,
            page_num: 1,
            page_size: 30,
        }
    }
}

impl ListPositionsParams {
    /// Set parent position id.
    pub fn with_parent_position_id(mut self, parent_position_id: impl Into<String>) -> Self {
        self.parent_position_id = Some(parent_position_id.into());
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

/// Parameters for `query.position.detail`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PositionDetailParams {
    /// Position id list (max 50).
    pub position_ids: Vec<String>,
}

impl PositionDetailParams {
    /// Create params.
    pub fn new(position_ids: impl Into<Vec<String>>) -> Self {
        Self {
            position_ids: position_ids.into(),
        }
    }
}
