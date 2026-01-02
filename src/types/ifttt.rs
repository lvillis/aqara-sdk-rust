//! IFTTT metadata query request types.

/// Parameters for `query.ifttt.trigger` / `query.ifttt.action`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IftttModelsParams {
    /// Model list.
    pub models: Vec<String>,
}

impl IftttModelsParams {
    /// Create params.
    pub fn new(models: impl Into<Vec<String>>) -> Self {
        Self {
            models: models.into(),
        }
    }
}
