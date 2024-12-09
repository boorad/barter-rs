use barter_integration::error::SocketError;
use serde::{Deserialize, Serialize};



/// ### System Response
/// ```json
/// {
///   "topic":"system",
///   "success":"some_username",
///   "isFT":false,
///   "isPaper":false
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbkrSystemResponse {
    #[serde(rename = "success")]
    pub username: String,
    #[serde(rename = "isFT")]
    pub is_ft: bool,
    #[serde(rename = "isPaper")]
    pub is_paper: bool,
}

impl IbkrSystemResponse {
    pub fn validate(self) -> Result<IbkrSystemResponse, SocketError> {
        // TODO: not sure if a zero-length string is indicator of error
        //       (i.e. not successful, because no username string)
        if self.username.clone().len() > 0 {
            Ok(self)
        } else {
            Err(SocketError::Subscribe(format!(
                "received failed system response success value"
            )))
        }
    }
}
