use serde::{Deserialize, Serialize};
use barter_integration::subscription::SubscriptionId;
use crate::Identifier;


/// Tickle Payload
/// ```json
/// {
///   "topic": "tic",
///   "alive": true,
///   "id": "XXXXX",
///   "lastAccessed": 1731507334085
/// }
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct IbkrTickleResponse {
  pub alive: bool,
  pub id: String,
  pub last_accessed: u64,
}

impl Identifier<Option<SubscriptionId>> for IbkrTickleResponse {
    fn id(&self) -> Option<SubscriptionId> {
        Some(SubscriptionId::from(self.id.clone()))
    }
}
