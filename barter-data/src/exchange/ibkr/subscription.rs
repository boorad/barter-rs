use barter_integration::{error::SocketError, Validator};
use serde::Deserialize;

/// [`Ibkr`](super::Ibkr) WebSocket subscription response.
///
/// ### Raw Payload Examples
/// #### Subscription Market Data Ok Response
/// ```json
/// {
///     "server_id": "server_id",
///     "conidEx": "conidEx",
///     "conid": "conid",
///     "_updated": "_updated",
///     "6119": "serverId",
///     "field_1": "field_1",
///     "field_2": "field_2",
///     "field_n": "field_n",
///     "field_n+1": "field_n+1",
///     "6509": "RB",
///     "topic": "smd+conid"
/// }
/// ```
///
/// See docs: https://www.interactivebrokers.com/campus/ibkr-api-page/cpapi-v1/
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// pub struct IbkrRawSubResponse {
//     #[serde(rename = "topic")]
//     topic: String,
// }

// #[serde(tag = "topic")]
pub enum IbkrSubResponse {
    Subscribed,
    Error,
}

impl<'de> Deserialize<'de> for IbkrSubResponse {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(IbkrSubResponse::Subscribed)
    }
}
//         struct SubVisitor;
//         impl<'de> Visitor<'de> for SubVisitor {
//             type Value = IbkrSubResponse;
//             fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 formatter.write_str("IbkrSubResponse struct from the Ibkr WebSocket API")
//             }
//             fn visit_str<E>(self, payload: &str) -> Result<Self::Value, E>
//             where
//                 E: serde::de::Error,
//             {
//                 let parsed = SubType::from_str(payload);
//                 match &parsed.topic.as_str()[1..1] {
//                     "s" => Ok(IbkrSubResponse::Subscribed),
//                     _ => Ok(IbkrSubResponse::Error),
//                 }
//             }
//         }
//         deserializer.deserialize_str(SubVisitor)
//     }
// }
//
// struct SubType {
//     pub topic: String,
// }
//
// impl SubType {
//     fn from_str(payload: &str) -> Result<Self, SocketError> {
//         let topic = <&str as Deserialize>::deserialize(payload)?;
//         Ok(Self {
//             topic: topic.to_string(),
//         })
//     }
// }
// /// extract subscription indicator from topic (ex: "smd+conid+..." => "s")
// pub fn de_sub_topic<'de, D>(deserializer: D) -> Result<IbkrSubResponse, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     <&str as Deserialize>::deserialize(deserializer)
//         .map(|topic| match &topic[1..1] {
//             "s" => IbkrSubResponse::Subscribed,
//             _ => IbkrSubResponse::Error,
//         })
// }

impl Validator for IbkrSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        match self {
            Self::Subscribed => Ok(self),
            Self::Error => Err(SocketError::Subscribe(format!(
                "received failure for subscription",
            ))),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     mod de {
//         use super::*;

//         #[test]
//         fn test_ibkr_subscription_response() {
//             struct TestCase {
//                 input: &'static str,
//                 expected: Result<IbkrSubResponse, SocketError>,
//             }

//             let cases = vec![
//                 TestCase {
//                     // TC0: input response is subscription success
//                     input: r#"
//                     {
//                         "server_id": "server_id",
//                         "conidEx": "1234",
//                         "conid": 1234,
//                         "_updated": 1630048897897,
//                         "6119": "serverId",
//                         "31": "100.99",
//                         "84": "100.90",
//                         "88": "1200",
//                         "86": "101.90",
//                         "85": "1500",
//                         "6509":"RB",
//                         "topic":"smd+1234"
//                     }
//                     "#,
//                     expected: Ok(IbkrSubResponse::Subscribed),
//                 },
//             ];

//             for (index, test) in cases.into_iter().enumerate() {
//                 let actual = IbkrSubResponse::deserialize(test.input);
//                 match (actual, test.expected) {
//                     (Ok(actual), Ok(expected)) => {
//                         assert_eq!(actual, expected, "TC{} failed", index)
//                     }
//                     (Err(_), Err(_)) => {
//                         // Test passed
//                     }
//                     (actual, expected) => {
//                         // Test failed
//                         panic!("TC{index} failed because actual != expected. \nActual: {actual:?}\nExpected: {expected:?}\n");
//                     }
//                 }
//             }
//         }
//     }

//     #[test]
//     fn test_validate_ibkr_sub_response() {
//         struct TestCase {
//             input_response: IbkrSubResponse,
//             is_valid: bool,
//         }

//         let cases = vec![
//             TestCase {
//                 // TC0: input response is subscription success
//                 input_response: IbkrSubResponse::Subscribed,
//                 is_valid: true,
//             },
//         ];

//         for (index, test) in cases.into_iter().enumerate() {
//             let actual = test.input_response.validate().is_ok();
//             assert_eq!(actual, test.is_valid, "TestCase {} failed", index);
//         }
//     }
// }
