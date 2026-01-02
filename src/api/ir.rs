use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::ir::{
    ConfigIrCustomParams, CreateIrControllerParams, DeleteIrControllerParams, QueryIrAcStateParams,
    QueryIrBrandsParams, QueryIrFunctionsParams, QueryIrInfoParams, QueryIrKeysParams,
    QueryIrLearnResultParams, QueryIrListParams, QueryIrMatchParams, UpdateIrControllerParams,
    WriteIrCancelLearnParams, WriteIrClickParams, WriteIrStartLearnParams,
};

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// IR device APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct IrService {
    client: Client,
}

#[cfg(feature = "async")]
impl IrService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `query.ir.categories`.
    pub async fn categories(&self) -> Result<AqaraValueResponse> {
        self.client
            .call_json("query.ir.categories", json!({}), true, true)
            .await
    }

    /// `query.ir.brands`.
    pub async fn brands(&self, params: QueryIrBrandsParams) -> Result<AqaraValueResponse> {
        let data = json!({ "categoryId": params.category_id });
        self.client
            .call_json("query.ir.brands", data, true, true)
            .await
    }

    /// `query.ir.match`.
    pub async fn match_tree(&self, params: QueryIrMatchParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "type": params.r#type,
            "categoryId": params.category_id,
            "brandId": params.brand_id,
        });
        self.client
            .call_json("query.ir.match", data, true, true)
            .await
    }

    /// `config.ir.create`.
    pub async fn create_controller(
        &self,
        params: CreateIrControllerParams,
    ) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "brandId": params.brand_id,
            "categoryId": params.category_id,
            "name": params.name,
            "parentDid": params.parent_did,
            "controllerId": params.controller_id,
        });
        if let Some(position_id) = params.position_id {
            data["positionId"] = json!(position_id);
        }
        self.client
            .call_json("config.ir.create", data, true, false)
            .await
    }

    /// `config.ir.delete`.
    pub async fn delete_controller(
        &self,
        params: DeleteIrControllerParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("config.ir.delete", data, true, false)
            .await
    }

    /// `config.ir.update`.
    pub async fn update_controller(
        &self,
        params: UpdateIrControllerParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "did": params.did,
            "name": params.name,
        });
        self.client
            .call_json("config.ir.update", data, true, false)
            .await
    }

    /// `query.ir.info`.
    pub async fn info(&self, params: QueryIrInfoParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("query.ir.info", data, true, true)
            .await
    }

    /// `query.ir.list`.
    pub async fn list(&self, params: QueryIrListParams) -> Result<AqaraValueResponse> {
        let data = json!({ "parentDid": params.parent_did });
        self.client
            .call_json("query.ir.list", data, true, true)
            .await
    }

    /// `write.ir.click`.
    pub async fn click(&self, params: WriteIrClickParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(brand_id) = params.brand_id {
            data["brandId"] = json!(brand_id);
        }
        if let Some(controller_id) = params.controller_id {
            data["controllerId"] = json!(controller_id);
        }
        if let Some(key_id) = params.key_id {
            data["keyId"] = json!(key_id);
        }
        if let Some(is_ac_match) = params.is_ac_match {
            data["isAcMatch"] = json!(is_ac_match);
        }
        if let Some(ac_key) = params.ac_key {
            data["acKey"] = json!(ac_key);
        }
        self.client
            .call_json("write.ir.click", data, true, false)
            .await
    }

    /// `query.ir.acState`.
    pub async fn ac_state(&self, params: QueryIrAcStateParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("query.ir.acState", data, true, true)
            .await
    }

    /// `query.ir.functions`.
    pub async fn functions(&self, params: QueryIrFunctionsParams) -> Result<AqaraValueResponse> {
        let mut data = json!({});
        if let Some(did) = params.did {
            data["did"] = json!(did);
        }
        if let Some(controller_id) = params.controller_id {
            data["controllerId"] = json!(controller_id);
        }
        self.client
            .call_json("query.ir.functions", data, true, true)
            .await
    }

    /// `query.ir.keys`.
    pub async fn keys(&self, params: QueryIrKeysParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client
            .call_json("query.ir.keys", data, true, true)
            .await
    }

    /// `write.ir.startLearn`.
    pub async fn start_learn(&self, params: WriteIrStartLearnParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(time_length) = params.time_length {
            data["timeLength"] = json!(time_length);
        }
        self.client
            .call_json("write.ir.startLearn", data, true, false)
            .await
    }

    /// `write.ir.cancelLearn`.
    pub async fn cancel_learn(
        &self,
        params: WriteIrCancelLearnParams,
    ) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(key_id) = params.key_id {
            data["keyId"] = json!(key_id);
        }
        self.client
            .call_json("write.ir.cancelLearn", data, true, false)
            .await
    }

    /// `query.ir.learnResult`.
    pub async fn learn_result(
        &self,
        params: QueryIrLearnResultParams,
    ) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(key_id) = params.key_id {
            data["keyId"] = json!(key_id);
        }
        self.client
            .call_json("query.ir.learnResult", data, true, true)
            .await
    }

    /// `config.ir.custom`.
    pub async fn custom_controller(
        &self,
        params: ConfigIrCustomParams,
    ) -> Result<AqaraValueResponse> {
        let ir_code_infos = params
            .ir_code_infos
            .into_iter()
            .map(|i| {
                let mut v = json!({
                    "keyId": i.key_id,
                    "keyName": i.key_name,
                    "ircode": i.ircode,
                });
                if let Some(freq) = i.freq {
                    v["freq"] = json!(freq);
                }
                v
            })
            .collect::<Vec<_>>();

        let mut data = json!({
            "parentDid": params.parent_did,
            "name": params.name,
            "irCodeInfos": ir_code_infos,
        });
        if let Some(position_id) = params.position_id {
            data["positionId"] = json!(position_id);
        }
        self.client
            .call_json("config.ir.custom", data, true, false)
            .await
    }
}

/// IR device APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingIrService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingIrService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `query.ir.categories`.
    pub fn categories(&self) -> Result<AqaraValueResponse> {
        self.client
            .call_json("query.ir.categories", json!({}), true, true)
    }

    /// `query.ir.brands`.
    pub fn brands(&self, params: QueryIrBrandsParams) -> Result<AqaraValueResponse> {
        let data = json!({ "categoryId": params.category_id });
        self.client.call_json("query.ir.brands", data, true, true)
    }

    /// `query.ir.match`.
    pub fn match_tree(&self, params: QueryIrMatchParams) -> Result<AqaraValueResponse> {
        let data = json!({
            "type": params.r#type,
            "categoryId": params.category_id,
            "brandId": params.brand_id,
        });
        self.client.call_json("query.ir.match", data, true, true)
    }

    /// `config.ir.create`.
    pub fn create_controller(
        &self,
        params: CreateIrControllerParams,
    ) -> Result<AqaraValueResponse> {
        let mut data = json!({
            "brandId": params.brand_id,
            "categoryId": params.category_id,
            "name": params.name,
            "parentDid": params.parent_did,
            "controllerId": params.controller_id,
        });
        if let Some(position_id) = params.position_id {
            data["positionId"] = json!(position_id);
        }
        self.client.call_json("config.ir.create", data, true, false)
    }

    /// `config.ir.delete`.
    pub fn delete_controller(
        &self,
        params: DeleteIrControllerParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client.call_json("config.ir.delete", data, true, false)
    }

    /// `config.ir.update`.
    pub fn update_controller(
        &self,
        params: UpdateIrControllerParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "did": params.did,
            "name": params.name,
        });
        self.client.call_json("config.ir.update", data, true, false)
    }

    /// `query.ir.info`.
    pub fn info(&self, params: QueryIrInfoParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client.call_json("query.ir.info", data, true, true)
    }

    /// `query.ir.list`.
    pub fn list(&self, params: QueryIrListParams) -> Result<AqaraValueResponse> {
        let data = json!({ "parentDid": params.parent_did });
        self.client.call_json("query.ir.list", data, true, true)
    }

    /// `write.ir.click`.
    pub fn click(&self, params: WriteIrClickParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(brand_id) = params.brand_id {
            data["brandId"] = json!(brand_id);
        }
        if let Some(controller_id) = params.controller_id {
            data["controllerId"] = json!(controller_id);
        }
        if let Some(key_id) = params.key_id {
            data["keyId"] = json!(key_id);
        }
        if let Some(is_ac_match) = params.is_ac_match {
            data["isAcMatch"] = json!(is_ac_match);
        }
        if let Some(ac_key) = params.ac_key {
            data["acKey"] = json!(ac_key);
        }
        self.client.call_json("write.ir.click", data, true, false)
    }

    /// `query.ir.acState`.
    pub fn ac_state(&self, params: QueryIrAcStateParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client.call_json("query.ir.acState", data, true, true)
    }

    /// `query.ir.functions`.
    pub fn functions(&self, params: QueryIrFunctionsParams) -> Result<AqaraValueResponse> {
        let mut data = json!({});
        if let Some(did) = params.did {
            data["did"] = json!(did);
        }
        if let Some(controller_id) = params.controller_id {
            data["controllerId"] = json!(controller_id);
        }
        self.client
            .call_json("query.ir.functions", data, true, true)
    }

    /// `query.ir.keys`.
    pub fn keys(&self, params: QueryIrKeysParams) -> Result<AqaraValueResponse> {
        let data = json!({ "did": params.did });
        self.client.call_json("query.ir.keys", data, true, true)
    }

    /// `write.ir.startLearn`.
    pub fn start_learn(&self, params: WriteIrStartLearnParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(time_length) = params.time_length {
            data["timeLength"] = json!(time_length);
        }
        self.client
            .call_json("write.ir.startLearn", data, true, false)
    }

    /// `write.ir.cancelLearn`.
    pub fn cancel_learn(&self, params: WriteIrCancelLearnParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(key_id) = params.key_id {
            data["keyId"] = json!(key_id);
        }
        self.client
            .call_json("write.ir.cancelLearn", data, true, false)
    }

    /// `query.ir.learnResult`.
    pub fn learn_result(&self, params: QueryIrLearnResultParams) -> Result<AqaraValueResponse> {
        let mut data = json!({ "did": params.did });
        if let Some(key_id) = params.key_id {
            data["keyId"] = json!(key_id);
        }
        self.client
            .call_json("query.ir.learnResult", data, true, true)
    }

    /// `config.ir.custom`.
    pub fn custom_controller(&self, params: ConfigIrCustomParams) -> Result<AqaraValueResponse> {
        let ir_code_infos = params
            .ir_code_infos
            .into_iter()
            .map(|i| {
                let mut v = json!({
                    "keyId": i.key_id,
                    "keyName": i.key_name,
                    "ircode": i.ircode,
                });
                if let Some(freq) = i.freq {
                    v["freq"] = json!(freq);
                }
                v
            })
            .collect::<Vec<_>>();

        let mut data = json!({
            "parentDid": params.parent_did,
            "name": params.name,
            "irCodeInfos": ir_code_infos,
        });
        if let Some(position_id) = params.position_id {
            data["positionId"] = json!(position_id);
        }
        self.client.call_json("config.ir.custom", data, true, false)
    }
}
