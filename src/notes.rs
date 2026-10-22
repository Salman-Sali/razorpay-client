use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

/// Razorpay sends `notes` as an empty array (or null) when there are none, and as an object
/// otherwise. Non-string values are kept as their JSON text.
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Notes {
        Map(HashMap<String, serde_json::Value>),
        #[allow(dead_code)]
        List(Vec<serde::de::IgnoredAny>),
    }

    Ok(match Option::<Notes>::deserialize(deserializer)? {
        Some(Notes::Map(map)) => map
            .into_iter()
            .map(|(key, value)| match value {
                serde_json::Value::String(value) => (key, value),
                value => (key, value.to_string()),
            })
            .collect(),
        Some(Notes::List(_)) | None => HashMap::new(),
    })
}
