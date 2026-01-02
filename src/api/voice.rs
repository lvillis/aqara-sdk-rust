use serde_json::json;

use crate::error::Result;
use crate::types::AqaraValueResponse;
use crate::types::voice::CommandDeviceResourceParams;

#[cfg(feature = "async")]
use crate::Client;

#[cfg(feature = "blocking")]
use crate::BlockingClient;

/// Voice command APIs (async).
#[cfg(feature = "async")]
#[derive(Clone)]
pub struct VoiceService {
    client: Client,
}

#[cfg(feature = "async")]
impl VoiceService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `command.device.resource`.
    pub async fn command_device_resource(
        &self,
        params: CommandDeviceResourceParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id,
            "queryText": params.query_text,
        });
        self.client
            .call_json("command.device.resource", data, true, false)
            .await
    }
}

/// Voice command APIs (blocking).
#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingVoiceService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingVoiceService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    /// `command.device.resource`.
    pub fn command_device_resource(
        &self,
        params: CommandDeviceResourceParams,
    ) -> Result<AqaraValueResponse> {
        let data = json!({
            "positionId": params.position_id,
            "queryText": params.query_text,
        });
        self.client
            .call_json("command.device.resource", data, true, false)
    }
}
