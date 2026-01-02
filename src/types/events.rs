//! Event set (condition set) related request types.

/// A condition parameter entry.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct EventConditionParam {
    /// Parameter id.
    pub param_id: Option<String>,
    /// Parameter value.
    pub value: Option<String>,
    /// Optional parameter type.
    pub param_type: Option<String>,
    /// Optional parameter unit.
    pub param_unit: Option<String>,
}

impl EventConditionParam {
    /// Create param with id and value.
    pub fn new(param_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            param_id: Some(param_id.into()),
            value: Some(value.into()),
            param_type: None,
            param_unit: None,
        }
    }

    /// Set parameter type.
    pub fn with_param_type(mut self, param_type: impl Into<String>) -> Self {
        self.param_type = Some(param_type.into());
        self
    }

    /// Set parameter unit.
    pub fn with_param_unit(mut self, param_unit: impl Into<String>) -> Self {
        self.param_unit = Some(param_unit.into());
        self
    }
}

/// A single event (condition set) condition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EventCondition {
    /// Optional subject id (e.g. device id / scene id). At least one of subject_id/model must be present.
    pub subject_id: Option<String>,
    /// Optional model.
    pub model: Option<String>,
    /// Trigger definition id.
    pub trigger_definition_id: String,
    /// Optional begin time.
    pub begin_time: Option<String>,
    /// Optional end time.
    pub end_time: Option<String>,
    /// Optional parameter list.
    pub params: Option<Vec<EventConditionParam>>,
}

impl EventCondition {
    /// Create params with required fields.
    pub fn new(trigger_definition_id: impl Into<String>) -> Self {
        Self {
            subject_id: None,
            model: None,
            trigger_definition_id: trigger_definition_id.into(),
            begin_time: None,
            end_time: None,
            params: None,
        }
    }

    /// Set subject id.
    pub fn with_subject_id(mut self, subject_id: impl Into<String>) -> Self {
        self.subject_id = Some(subject_id.into());
        self
    }

    /// Set model.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set begin time.
    pub fn with_begin_time(mut self, begin_time: impl Into<String>) -> Self {
        self.begin_time = Some(begin_time.into());
        self
    }

    /// Set end time.
    pub fn with_end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// Set params.
    pub fn with_params(mut self, params: impl Into<Vec<EventConditionParam>>) -> Self {
        self.params = Some(params.into());
        self
    }
}

/// Parameters for `config.event.create`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreateEventParams {
    /// Event set name.
    pub name: String,
    /// Optional position id. Empty means default position.
    pub position_id: Option<String>,
    /// Relation (0: AND, 1: OR).
    pub relation: i32,
    /// Condition list.
    pub condition: Vec<EventCondition>,
}

impl CreateEventParams {
    /// Create params with required fields.
    pub fn new(
        name: impl Into<String>,
        relation: i32,
        condition: impl Into<Vec<EventCondition>>,
    ) -> Self {
        Self {
            name: name.into(),
            position_id: None,
            relation,
            condition: condition.into(),
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.event.update`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdateEventParams {
    /// Event set id.
    pub event_id: String,
    /// Enable flag (`0` or `1`).
    pub enable: i32,
    /// Event set name.
    pub name: String,
    /// Optional position id.
    pub position_id: Option<String>,
    /// Relation (0: AND, 1: OR).
    pub relation: i32,
    /// Condition list.
    pub condition: Vec<EventCondition>,
}

impl UpdateEventParams {
    /// Create params with required fields.
    pub fn new(
        event_id: impl Into<String>,
        enable: i32,
        name: impl Into<String>,
        relation: i32,
        condition: impl Into<Vec<EventCondition>>,
    ) -> Self {
        Self {
            event_id: event_id.into(),
            enable,
            name: name.into(),
            position_id: None,
            relation,
            condition: condition.into(),
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.event.delete`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DeleteEventParams {
    /// Event set id.
    pub event_id: String,
}

impl DeleteEventParams {
    /// Create params.
    pub fn new(event_id: impl Into<String>) -> Self {
        Self {
            event_id: event_id.into(),
        }
    }
}

/// Parameters for `query.event.detail`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryEventDetailParams {
    /// Event set id.
    pub event_id: String,
}

impl QueryEventDetailParams {
    /// Create params.
    pub fn new(event_id: impl Into<String>) -> Self {
        Self {
            event_id: event_id.into(),
        }
    }
}

/// Parameters for `query.event.listBySubjectId`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryEventsBySubjectIdParams {
    /// Subject id.
    pub subject_id: String,
}

impl QueryEventsBySubjectIdParams {
    /// Create params.
    pub fn new(subject_id: impl Into<String>) -> Self {
        Self {
            subject_id: subject_id.into(),
        }
    }
}

/// Parameters for `query.event.listByPositionId`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryEventsByPositionIdParams {
    /// Optional position id (empty for all).
    pub position_id: Option<String>,
    /// Page number (1-based).
    pub page_num: u32,
    /// Page size.
    pub page_size: u32,
}

impl Default for QueryEventsByPositionIdParams {
    fn default() -> Self {
        Self {
            position_id: None,
            page_num: 1,
            page_size: 50,
        }
    }
}

impl QueryEventsByPositionIdParams {
    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }

    /// Set page number.
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
