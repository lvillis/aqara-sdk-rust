use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::auth::{
    CreateAccountParams, GetAuthCodeParams, GetTokenParams, RefreshTokenParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Auth-related APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct AuthService {
    client: Client,
}

#[cfg(feature = "async")]
impl AuthService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `config.auth.createAccount`.
    pub async fn create_account(&self, params: CreateAccountParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "accountId": params.account_id,
        });
        if let Some(remark) = params.remark {
            data["remark"] = json!(remark);
        }
        if let Some(need_access_token) = params.need_access_token {
            data["needAccessToken"] = json!(need_access_token);
        }
        if let Some(access_token_validity) = params.access_token_validity {
            data["accessTokenValidity"] = json!(access_token_validity);
        }
        self.client
            .call_json("config.auth.createAccount", data, false, false)
            .await
    }

    /// `config.auth.getAuthCode`.
    pub async fn get_auth_code(&self, params: GetAuthCodeParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "account": params.account,
            "accountType": params.account_type,
            "accessTokenValidity": params.access_token_validity.as_deref().unwrap_or("7d"),
        });
        self.client
            .call_json("config.auth.getAuthCode", data, false, false)
            .await
    }

    /// `config.auth.getToken`.
    pub async fn get_token(&self, params: GetTokenParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "authCode": params.auth_code.expose(),
            "account": params.account,
            "accountType": params.account_type,
        });
        self.client
            .call_json("config.auth.getToken", data, false, false)
            .await
    }

    /// `config.auth.refreshToken`.
    pub async fn refresh_token(&self, params: RefreshTokenParams) -> Result<AqaraValueResponse> {
        let data = json!({ "refreshToken": params.refresh_token.expose() });
        self.client
            .call_json("config.auth.refreshToken", data, false, false)
            .await
    }
}

/// Auth-related APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingAuthService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingAuthService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `config.auth.createAccount`.
    pub fn create_account(&self, params: CreateAccountParams) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "accountId": params.account_id,
        });
        if let Some(remark) = params.remark {
            data["remark"] = json!(remark);
        }
        if let Some(need_access_token) = params.need_access_token {
            data["needAccessToken"] = json!(need_access_token);
        }
        if let Some(access_token_validity) = params.access_token_validity {
            data["accessTokenValidity"] = json!(access_token_validity);
        }
        self.client
            .call_json("config.auth.createAccount", data, false, false)
    }

    /// `config.auth.getAuthCode`.
    pub fn get_auth_code(&self, params: GetAuthCodeParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "account": params.account,
            "accountType": params.account_type,
            "accessTokenValidity": params.access_token_validity.as_deref().unwrap_or("7d"),
        });
        self.client
            .call_json("config.auth.getAuthCode", data, false, false)
    }

    /// `config.auth.getToken`.
    pub fn get_token(&self, params: GetTokenParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "authCode": params.auth_code.expose(),
            "account": params.account,
            "accountType": params.account_type,
        });
        self.client
            .call_json("config.auth.getToken", data, false, false)
    }

    /// `config.auth.refreshToken`.
    pub fn refresh_token(&self, params: RefreshTokenParams) -> Result<AqaraValueResponse> {
        let data = json!({ "refreshToken": params.refresh_token.expose() });
        self.client
            .call_json("config.auth.refreshToken", data, false, false)
    }
}
