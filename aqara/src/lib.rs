use md5;
use rand::distr::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct AqaraConfig {
    pub access_token: String,
    pub app_id: String,
    pub key_id: String,
    pub app_key: String,
}

pub struct AqaraClient {
    config: AqaraConfig,
    client: Client,
    base_url: String,
}

impl AqaraClient {
    pub fn new(config: AqaraConfig) -> Self {
        // 根据编译特性选择不同的接口地址
        // Select different API endpoints based on compilation features
        let base_url = if cfg!(feature = "china") {
            "https://open-cn.aqara.com/v3.0/open/api"
        } else if cfg!(feature = "usa") {
            "https://open-usa.aqara.com/v3.0/open/api"
        } else if cfg!(feature = "europe") {
            "https://open-ger.aqara.com/v3.0/open/api"
        } else if cfg!(feature = "korea") {
            "https://open-kr.aqara.com/v3.0/open/api"
        } else if cfg!(feature = "russia") {
            "https://open-ru.aqara.com/v3.0/open/api"
        } else if cfg!(feature = "singapore") {
            "https://open-sg.aqara.com/v3.0/open/api"
        } else {
            "https://open-cn.aqara.com/v3.0/open/api"
        };

        AqaraClient {
            client: Client::new(),
            config,
            base_url: base_url.to_string(),
        }
    }

    fn generate_nonce(&self) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    pub fn generate_signature(&self, nonce: &str, time: &str, include_access_token: bool) -> String {
        let mut sign_str = String::new();

        // 决定是否加入Accesstoken / Decide whether to include Accesstoken
        if include_access_token && !self.config.access_token.is_empty() {
            sign_str.push_str(&format!("Accesstoken={}&", self.config.access_token));
        }
        sign_str.push_str(&format!(
            "Appid={}&Keyid={}&Nonce={}&Time={}",
            self.config.app_id, self.config.key_id, nonce, time
        ));
        sign_str.push_str(&self.config.app_key);
        let sign_str = sign_str.to_lowercase();
        let digest = md5::compute(sign_str.as_bytes());
        format!("{:x}", digest)
    }

    async fn send_api_request(
        &self,
        intent: &str,
        data: Value,
        include_access_token: bool,
    ) -> Result<String, Error> {
        let nonce = self.generate_nonce();
        let time = format!("{}", chrono::Utc::now().timestamp_millis());
        let sign = self.generate_signature(&nonce, &time, include_access_token);

        let request_body = json!({
            "intent": intent,
            "data": data
        });

        debug!("Request URL: {}", self.base_url);
        debug!("Request Headers:");
        debug!("  Appid: {}", &self.config.app_id);
        debug!("  Keyid: {}", &self.config.key_id);
        debug!("  Nonce: {}", &nonce);
        debug!("  Time: {}", &time);
        debug!("  Sign: {}", &sign);
        debug!("Request Body: {}", request_body.to_string());

        let mut request = self.client
            .post(&self.base_url)
            .header("Appid", &self.config.app_id)
            .header("Keyid", &self.config.key_id)
            .header("Nonce", &nonce)
            .header("Time", &time)
            .header("Sign", &sign)
            .header("Lang", "en")
            .header("Content-Type", "application/json")
            .header("User-Agent", "AqaraSDK/1.0");

        if include_access_token {
            request = request.header("Accesstoken", &self.config.access_token);
        }

        let response = request.json(&request_body).send().await?;

        if response.status().is_success() {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }

    /// 获取授权码 (Get auth code)
    ///
    /// intent: config.auth.getAuthCode
    ///
    /// # Parameters 参数
    /// - `account`: 用户账户 / User account
    /// - `account_type`: 账户类型 / Account type
    /// - `access_token_validity`: AccessToken有效期 / Validity of the access token (e.g. "7d")
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn config_auth_get_auth_code(
        &self,
        account: &str,
        account_type: i32,
        access_token_validity: Option<&str>,
    ) -> Result<String, Error> {
        let data = json!({
            "account": account,
            "accountType": account_type,
            "accessTokenValidity": access_token_validity.unwrap_or("7d")
        });
        self.send_api_request("config.auth.getAuthCode", data, true)
            .await
    }

    /// 刷新Token (Refresh token)
    ///
    /// intent: config.auth.refreshToken
    ///
    /// # Parameters 参数
    /// - `refresh_token`: 需要刷新的RefreshToken / The refresh token to be used
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn config_auth_refresh_token(&self, refresh_token: &str) -> Result<String, Error> {
        let data = json!({
            "refreshToken": refresh_token
        });
        self.send_api_request("config.auth.refreshToken", data, false)
            .await
    }

    /// 查询子设备信息 (Query sub device info)
    ///
    /// intent: query.device.subInfo
    ///
    /// # Parameters 参数
    /// - `gateway_did`: 网关ID / Gateway DID
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn query_device_sub_info(&self, gateway_did: &str) -> Result<String, Error> {
        let data = json!({
            "did": gateway_did
        });
        self.send_api_request("query.device.subInfo", data, true)
            .await
    }

    /// 查询资源信息 (Query resource info)
    ///
    /// intent: query.resource.info
    ///
    /// # Parameters 参数
    /// - `model`: 设备型号 / Device model
    /// - `resource_id`: 资源ID (可选) / Resource ID (optional)
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn query_resource_info(
        &self,
        model: &str,
        resource_id: Option<&str>,
    ) -> Result<String, Error> {
        let mut data = json!({
            "model": model,
        });
        if let Some(resource_id) = resource_id {
            data["resourceId"] = json!(resource_id);
        }
        self.send_api_request("query.resource.info", data, true)
            .await
    }

    /// 命令设备资源 (Command device resource)
    ///
    /// intent: command.device.resource
    ///
    /// # Parameters 参数
    /// - `position_id`: 位置ID / Position ID
    /// - `query_text`: 命令内容 / Query text
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn command_device_resource(
        &self,
        position_id: &str,
        query_text: &str,
    ) -> Result<String, Error> {
        let data = json!({
            "positionId": position_id,
            "queryText": query_text
        });
        self.send_api_request("command.device.resource", data, true)
            .await
    }

    /// 查询位置信息 (Query position info)
    ///
    /// intent: query.position.info
    ///
    /// # Parameters 参数
    /// - `parent_position_id`: 父位置ID (可选) / Parent position ID (optional)
    /// - `page_num`: 页码 (可选) / Page number (optional)
    /// - `page_size`: 每页数量 (可选) / Page size (optional)
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn query_position_info(
        &self,
        parent_position_id: Option<&str>,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<String, Error> {
        let data = json!({
            "parentPositionId": parent_position_id.unwrap_or(""),
            "pageNum": page_num.unwrap_or(1),
            "pageSize": page_size.unwrap_or(30)
        });
        self.send_api_request("query.position.info", data, true).await
    }

    /// 查询指定位置的详细信息 (Query detailed position info)
    ///
    /// intent: query.position.detail
    ///
    /// # Parameters 参数
    /// - `position_ids`: 位置ID列表 (最多50个) / A slice of up to 50 position IDs
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn query_position_detail(
        &self,
        position_ids: &[&str],
    ) -> Result<String, Error> {
        let data = json!({
            "positionIds": position_ids
        });
        self.send_api_request("query.position.detail", data, true).await
    }

    /// 查询固件版本信息 (Query OTA firmware versions)
    ///
    /// intent: query.ota.firmware
    ///
    /// # Parameters 参数
    /// - `model`: 设备型号 / Device model
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn query_ota_firmware(&self, model: &str) -> Result<String, Error> {
        let data = json!({
            "model": model
        });
        self.send_api_request("query.ota.firmware", data, true).await
    }

    /// 升级固件 (Upgrade firmware)
    ///
    /// intent: write.ota.upgrade
    ///
    /// # Parameters 参数
    /// - `dids`: 设备ID数组 / Array of device IDs to upgrade
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn write_ota_upgrade(&self, dids: &[&str]) -> Result<String, Error> {
        let data = json!({
            "dids": dids
        });
        self.send_api_request("write.ota.upgrade", data, true).await
    }

    /// 查询设备升级状态 (Query device upgrade status)
    ///
    /// intent: query.ota.upgrade
    ///
    /// # Parameters 参数
    /// - `dids`: 设备ID数组 / Array of device IDs
    ///
    /// # Returns
    /// 成功返回字符串 / Returns response string on success
    pub async fn query_ota_upgrade(&self, dids: &[&str]) -> Result<String, Error> {
        let data = json!({
            "dids": dids
        });
        self.send_api_request("query.ota.upgrade", data, true).await
    }
}
