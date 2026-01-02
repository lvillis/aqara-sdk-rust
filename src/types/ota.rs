//! OTA-related request types.

/// Parameters for `query.ota.firmware`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct OtaFirmwareParams {
    /// Device model.
    pub model: String,
}

impl OtaFirmwareParams {
    /// Create params.
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }
}

/// Parameters for `write.ota.upgrade`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct OtaUpgradeParams {
    /// Device DID list.
    pub dids: Vec<String>,
}

impl OtaUpgradeParams {
    /// Create params.
    pub fn new(dids: impl Into<Vec<String>>) -> Self {
        Self { dids: dids.into() }
    }
}

/// Parameters for `query.ota.upgrade`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct OtaUpgradeStatusParams {
    /// Device DID list.
    pub dids: Vec<String>,
}

impl OtaUpgradeStatusParams {
    /// Create params.
    pub fn new(dids: impl Into<Vec<String>>) -> Self {
        Self { dids: dids.into() }
    }
}
