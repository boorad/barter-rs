use barter_integration::error::SocketError;
use serde::{Deserialize, Serialize};


/// ### System Heartbeat
/// ```json
/// {
///   "topic":"system",
///   "hb":1729601500848
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbkrSystemHeartbeat {
    #[serde(rename = "hb")]
    pub hb: u64,
}

impl IbkrSystemHeartbeat {
    pub fn validate(self) -> Result<IbkrSystemHeartbeat, SocketError> {
        if self.hb > 0 {
            Ok(self)
        } else {
            Err(SocketError::Subscribe(format!(
                "received failed system response success value"
            )))
        }
    }
}
