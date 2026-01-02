//! Scene-related request types.

/// A single scene action parameter.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SceneActionParam {
    /// Parameter id.
    pub param_id: String,
    /// Parameter value.
    pub value: String,
    /// Optional parameter type.
    pub param_type: Option<String>,
    /// Optional parameter unit.
    pub param_unit: Option<String>,
}

impl SceneActionParam {
    /// Create params with required fields.
    pub fn new(param_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            param_id: param_id.into(),
            value: value.into(),
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

/// A single scene action.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SceneAction {
    /// Subject id.
    pub subject_id: String,
    /// Action definition id.
    pub action_definition_id: String,
    /// Action parameter list.
    pub params: Vec<SceneActionParam>,
    /// Optional delay time.
    pub delay_time: Option<String>,
    /// Optional delay time unit.
    pub delay_time_unit: Option<String>,
}

impl SceneAction {
    /// Create params with required fields.
    pub fn new(
        subject_id: impl Into<String>,
        action_definition_id: impl Into<String>,
        params: impl Into<Vec<SceneActionParam>>,
    ) -> Self {
        Self {
            subject_id: subject_id.into(),
            action_definition_id: action_definition_id.into(),
            params: params.into(),
            delay_time: None,
            delay_time_unit: None,
        }
    }

    /// Set delay time.
    pub fn with_delay_time(mut self, delay_time: impl Into<String>) -> Self {
        self.delay_time = Some(delay_time.into());
        self
    }

    /// Set delay time unit.
    pub fn with_delay_time_unit(mut self, delay_time_unit: impl Into<String>) -> Self {
        self.delay_time_unit = Some(delay_time_unit.into());
        self
    }
}

/// Parameters for `config.scene.create`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreateSceneParams {
    /// Scene name.
    pub name: String,
    /// Optional position id. Empty means default position.
    pub position_id: Option<String>,
    /// Scene action list.
    pub action: Vec<SceneAction>,
}

impl CreateSceneParams {
    /// Create params with required fields.
    pub fn new(name: impl Into<String>, action: impl Into<Vec<SceneAction>>) -> Self {
        Self {
            name: name.into(),
            position_id: None,
            action: action.into(),
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.scene.update`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdateSceneParams {
    /// Scene id.
    pub scene_id: String,
    /// Scene name.
    pub name: String,
    /// Optional position id. Empty means default position.
    pub position_id: Option<String>,
    /// Scene action list.
    pub action: Vec<SceneAction>,
}

impl UpdateSceneParams {
    /// Create params with required fields.
    pub fn new(
        scene_id: impl Into<String>,
        name: impl Into<String>,
        action: impl Into<Vec<SceneAction>>,
    ) -> Self {
        Self {
            scene_id: scene_id.into(),
            name: name.into(),
            position_id: None,
            action: action.into(),
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.scene.delete`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DeleteSceneParams {
    /// Scene id.
    pub scene_id: String,
}

impl DeleteSceneParams {
    /// Create params.
    pub fn new(scene_id: impl Into<String>) -> Self {
        Self {
            scene_id: scene_id.into(),
        }
    }
}

/// Parameters for `config.scene.run`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RunSceneParams {
    /// Scene id.
    pub scene_id: String,
}

impl RunSceneParams {
    /// Create params.
    pub fn new(scene_id: impl Into<String>) -> Self {
        Self {
            scene_id: scene_id.into(),
        }
    }
}

/// Parameters for `query.scene.detail`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QuerySceneDetailParams {
    /// Scene id.
    pub scene_id: String,
}

impl QuerySceneDetailParams {
    /// Create params.
    pub fn new(scene_id: impl Into<String>) -> Self {
        Self {
            scene_id: scene_id.into(),
        }
    }
}

/// Parameters for `query.scene.listBySubjectId`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryScenesBySubjectIdParams {
    /// Subject id (e.g. device id / event id).
    pub subject_id: String,
}

impl QueryScenesBySubjectIdParams {
    /// Create params.
    pub fn new(subject_id: impl Into<String>) -> Self {
        Self {
            subject_id: subject_id.into(),
        }
    }
}

/// Parameters for `query.scene.listByPositionId`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryScenesByPositionIdParams {
    /// Optional position id (empty for all).
    pub position_id: Option<String>,
    /// Page number (1-based).
    pub page_num: u32,
    /// Page size.
    pub page_size: u32,
}

impl Default for QueryScenesByPositionIdParams {
    fn default() -> Self {
        Self {
            position_id: None,
            page_num: 1,
            page_size: 50,
        }
    }
}

impl QueryScenesByPositionIdParams {
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
