//! IR device related request types.

/// Parameters for `query.ir.brands`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrBrandsParams {
    /// Category id.
    pub category_id: u32,
}

impl QueryIrBrandsParams {
    /// Create params.
    pub fn new(category_id: u32) -> Self {
        Self { category_id }
    }
}

/// Parameters for `query.ir.match`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrMatchParams {
    /// Query type (e.g. `1`).
    pub r#type: u32,
    /// Category id.
    pub category_id: u32,
    /// Brand id.
    pub brand_id: u32,
}

impl QueryIrMatchParams {
    /// Create params.
    pub fn new(r#type: u32, category_id: u32, brand_id: u32) -> Self {
        Self {
            r#type,
            category_id,
            brand_id,
        }
    }
}

/// Parameters for `config.ir.create`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreateIrControllerParams {
    /// Gateway DID.
    pub parent_did: String,
    /// Optional position id.
    pub position_id: Option<String>,
    /// IR category id.
    pub category_id: u32,
    /// IR brand id.
    pub brand_id: u32,
    /// Controller id (from match tree).
    pub controller_id: u32,
    /// Controller name.
    pub name: String,
}

impl CreateIrControllerParams {
    /// Create params with required fields.
    pub fn new(
        parent_did: impl Into<String>,
        category_id: u32,
        brand_id: u32,
        controller_id: u32,
        name: impl Into<String>,
    ) -> Self {
        Self {
            parent_did: parent_did.into(),
            position_id: None,
            category_id,
            brand_id,
            controller_id,
            name: name.into(),
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}

/// Parameters for `config.ir.delete`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DeleteIrControllerParams {
    /// IR controller device id.
    pub did: String,
}

impl DeleteIrControllerParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}

/// Parameters for `config.ir.update`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UpdateIrControllerParams {
    /// IR controller device id.
    pub did: String,
    /// New name.
    pub name: String,
}

impl UpdateIrControllerParams {
    /// Create params.
    pub fn new(did: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            name: name.into(),
        }
    }
}

/// Parameters for `query.ir.info`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrInfoParams {
    /// IR controller device id.
    pub did: String,
}

impl QueryIrInfoParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}

/// Parameters for `query.ir.list`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrListParams {
    /// Gateway DID.
    pub parent_did: String,
}

impl QueryIrListParams {
    /// Create params.
    pub fn new(parent_did: impl Into<String>) -> Self {
        Self {
            parent_did: parent_did.into(),
        }
    }
}

/// Parameters for `write.ir.click`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WriteIrClickParams {
    /// Gateway DID or IR controller device id.
    pub did: String,
    /// Optional brand id (required for AC matching).
    pub brand_id: Option<u32>,
    /// Optional controller id.
    pub controller_id: Option<u32>,
    /// Optional key id (for non-AC / stateless AC).
    pub key_id: Option<String>,
    /// Optional AC match state (`0` matched, `1` matching).
    pub is_ac_match: Option<String>,
    /// Optional AC key.
    pub ac_key: Option<String>,
}

impl WriteIrClickParams {
    /// Create params with required fields.
    pub fn new(did: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            brand_id: None,
            controller_id: None,
            key_id: None,
            is_ac_match: None,
            ac_key: None,
        }
    }

    /// Set brand id.
    pub fn with_brand_id(mut self, brand_id: u32) -> Self {
        self.brand_id = Some(brand_id);
        self
    }

    /// Set controller id.
    pub fn with_controller_id(mut self, controller_id: u32) -> Self {
        self.controller_id = Some(controller_id);
        self
    }

    /// Set key id.
    pub fn with_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }

    /// Set AC match state.
    pub fn with_is_ac_match(mut self, is_ac_match: impl Into<String>) -> Self {
        self.is_ac_match = Some(is_ac_match.into());
        self
    }

    /// Set AC key.
    pub fn with_ac_key(mut self, ac_key: impl Into<String>) -> Self {
        self.ac_key = Some(ac_key.into());
        self
    }
}

/// Parameters for `query.ir.acState`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrAcStateParams {
    /// IR controller device id.
    pub did: String,
}

impl QueryIrAcStateParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}

/// Parameters for `query.ir.functions`.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct QueryIrFunctionsParams {
    /// Optional IR controller device id.
    pub did: Option<String>,
    /// Optional controller id.
    pub controller_id: Option<u32>,
}

impl QueryIrFunctionsParams {
    /// Set IR controller device id.
    pub fn with_did(mut self, did: impl Into<String>) -> Self {
        self.did = Some(did.into());
        self
    }

    /// Set controller id.
    pub fn with_controller_id(mut self, controller_id: u32) -> Self {
        self.controller_id = Some(controller_id);
        self
    }
}

/// Parameters for `query.ir.keys`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrKeysParams {
    /// IR controller device id.
    pub did: String,
}

impl QueryIrKeysParams {
    /// Create params.
    pub fn new(did: impl Into<String>) -> Self {
        Self { did: did.into() }
    }
}

/// Parameters for `write.ir.startLearn`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WriteIrStartLearnParams {
    /// Gateway/IR device id.
    pub did: String,
    /// Optional learning time length (seconds).
    pub time_length: Option<u32>,
}

impl WriteIrStartLearnParams {
    /// Create params with required fields.
    pub fn new(did: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            time_length: None,
        }
    }

    /// Set time length (seconds).
    pub fn with_time_length(mut self, time_length: u32) -> Self {
        self.time_length = Some(time_length);
        self
    }
}

/// Parameters for `write.ir.cancelLearn`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WriteIrCancelLearnParams {
    /// Gateway/IR device id.
    pub did: String,
    /// Optional learning key id.
    pub key_id: Option<String>,
}

impl WriteIrCancelLearnParams {
    /// Create params with required fields.
    pub fn new(did: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            key_id: None,
        }
    }

    /// Set learning key id.
    pub fn with_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }
}

/// Parameters for `query.ir.learnResult`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct QueryIrLearnResultParams {
    /// Gateway/IR device id.
    pub did: String,
    /// Optional learning key id.
    pub key_id: Option<String>,
}

impl QueryIrLearnResultParams {
    /// Create params with required fields.
    pub fn new(did: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            key_id: None,
        }
    }

    /// Set learning key id.
    pub fn with_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.key_id = Some(key_id.into());
        self
    }
}

/// A custom IR code entry for `config.ir.custom`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct IrCodeInfo {
    /// Key name.
    pub key_name: String,
    /// Key id.
    pub key_id: String,
    /// IR code value.
    pub ircode: String,
    /// Optional frequency.
    pub freq: Option<String>,
}

impl IrCodeInfo {
    /// Create params with required fields.
    pub fn new(
        key_id: impl Into<String>,
        key_name: impl Into<String>,
        ircode: impl Into<String>,
    ) -> Self {
        Self {
            key_name: key_name.into(),
            key_id: key_id.into(),
            ircode: ircode.into(),
            freq: None,
        }
    }

    /// Set frequency.
    pub fn with_freq(mut self, freq: impl Into<String>) -> Self {
        self.freq = Some(freq.into());
        self
    }
}

/// Parameters for `config.ir.custom`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ConfigIrCustomParams {
    /// Gateway DID.
    pub parent_did: String,
    /// Controller name.
    pub name: String,
    /// Optional position id.
    pub position_id: Option<String>,
    /// IR code list.
    pub ir_code_infos: Vec<IrCodeInfo>,
}

impl ConfigIrCustomParams {
    /// Create params with required fields.
    pub fn new(
        parent_did: impl Into<String>,
        name: impl Into<String>,
        ir_code_infos: impl Into<Vec<IrCodeInfo>>,
    ) -> Self {
        Self {
            parent_did: parent_did.into(),
            name: name.into(),
            position_id: None,
            ir_code_infos: ir_code_infos.into(),
        }
    }

    /// Set position id.
    pub fn with_position_id(mut self, position_id: impl Into<String>) -> Self {
        self.position_id = Some(position_id.into());
        self
    }
}
