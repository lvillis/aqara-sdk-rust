//! Auth-related request types.

use crate::types::SecretString;

/// Parameters for `config.auth.createAccount`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CreateAccountParams {
    /// Developer-defined virtual account id (must be unique within the app).
    pub account_id: String,
    /// Optional remark.
    pub remark: Option<String>,
    /// Whether access/refresh tokens should be returned.
    pub need_access_token: Option<bool>,
    /// Access token validity duration string (e.g. `"7d"`).
    pub access_token_validity: Option<String>,
}

impl CreateAccountParams {
    /// Create params with required fields.
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            remark: None,
            need_access_token: None,
            access_token_validity: None,
        }
    }

    /// Set remark.
    pub fn with_remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    /// Set whether access/refresh tokens should be returned.
    pub fn with_need_access_token(mut self, need_access_token: bool) -> Self {
        self.need_access_token = Some(need_access_token);
        self
    }

    /// Set access token validity duration string (e.g. `"7d"`).
    pub fn with_access_token_validity(mut self, validity: impl Into<String>) -> Self {
        self.access_token_validity = Some(validity.into());
        self
    }
}

/// Parameters for `config.auth.getAuthCode`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GetAuthCodeParams {
    /// User account.
    pub account: String,
    /// Account type, as defined by Aqara.
    pub account_type: i32,
    /// Access token validity duration string (e.g. `"7d"`).
    pub access_token_validity: Option<String>,
}

impl GetAuthCodeParams {
    /// Create params with required fields.
    pub fn new(account: impl Into<String>, account_type: i32) -> Self {
        Self {
            account: account.into(),
            account_type,
            access_token_validity: None,
        }
    }

    /// Set access token validity duration string (e.g. `"7d"`).
    pub fn with_access_token_validity(mut self, validity: impl Into<String>) -> Self {
        self.access_token_validity = Some(validity.into());
        self
    }
}

/// Parameters for `config.auth.getToken`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct GetTokenParams {
    /// Authorization code (secret).
    pub auth_code: SecretString,
    /// User account.
    pub account: String,
    /// Account type, as defined by Aqara.
    pub account_type: i32,
}

impl GetTokenParams {
    /// Create params with required fields.
    pub fn new(
        auth_code: impl Into<String>,
        account: impl Into<String>,
        account_type: i32,
    ) -> Self {
        Self {
            auth_code: SecretString::new(auth_code),
            account: account.into(),
            account_type,
        }
    }
}

/// Parameters for `config.auth.refreshToken`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RefreshTokenParams {
    /// Refresh token (secret).
    pub refresh_token: SecretString,
}

impl RefreshTokenParams {
    /// Create params.
    pub fn new(refresh_token: impl Into<String>) -> Self {
        Self {
            refresh_token: SecretString::new(refresh_token),
        }
    }
}
