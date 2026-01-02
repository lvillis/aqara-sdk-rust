use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error;

pub(crate) fn unix_timestamp_millis() -> Result<String, Error> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::Transport {
            message: "system clock is before unix epoch".to_string(),
            source: Some(Box::new(e)),
        })?;
    Ok(duration.as_millis().to_string())
}
