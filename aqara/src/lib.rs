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
        // Use compile-time features to select different interface addresses
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

    pub fn generate_signature(
        &self,
        nonce: &str,
        time: &str,
        include_access_token: bool,
    ) -> String {
        let mut sign_str = String::new();

        // Decide whether to add an Accesstoken as needed
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

    pub async fn send_api_request(
        &self,
        intent: &str,
        data: Value,
        include_access_token: bool,
    ) -> Result<String, Error> {
        let nonce = self.generate_nonce();
        let time = format!("{}", chrono::Utc::now().timestamp_millis());

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
        debug!(
            "  Sign: {}",
            self.generate_signature(&nonce, &time, include_access_token)
        );
        debug!("Request Body: {}", request_body.to_string());

        let mut request = self
            .client
            .post(&self.base_url)
            .header("Appid", &self.config.app_id)
            .header("Keyid", &self.config.key_id)
            .header("Nonce", &nonce)
            .header("Time", &time)
            .header(
                "Sign",
                &self.generate_signature(&nonce, &time, include_access_token),
            )
            .header("Lang", "en")
            .header("Content-Type", "application/json")
            .header("User-Agent", "AqaraSDK/1.0");

        // If you need to include an access_token in the header
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

    /// config.auth.getAuthCode
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

    /// config.auth.refreshToken
    pub async fn config_auth_refresh_token(&self, refresh_token: &str) -> Result<String, Error> {
        let data = json!({
            "refreshToken": refresh_token
        });
        self.send_api_request("config.auth.refreshToken", data, false)
            .await
    }

    /// query.device.subInfo
    pub async fn query_device_sub_info(&self, gateway_did: &str) -> Result<String, Error> {
        let data = json!({
            "did": gateway_did
        });
        self.send_api_request("query.device.subInfo", data, true)
            .await
    }

    /// query.resource.info
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

    /// command.device.resource
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

    /// query.position.info
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

    /// query.position.detail
    /// Queries detailed information for specified positions.
    ///
    /// This interface allows querying detailed information for up to 50 specified positions simultaneously.
    ///
    /// # Parameters
    ///
    /// - `position_ids`: A slice of position IDs to query. Maximum of 50 IDs.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response body as a `String` if successful, or an `AqaraClientError` otherwise.
    pub async fn query_position_detail(
        &self,
        position_ids: &[&str],
    ) -> Result<String, Error> {

        let data = json!({
            "positionIds": position_ids
        });
        self.send_api_request("query.position.detail", data, true).await
    }

}
