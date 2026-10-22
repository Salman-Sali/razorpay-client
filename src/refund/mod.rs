pub mod create_refund;
pub mod fetch_refund;
pub mod update_refund;

use std::collections::HashMap;

use crate::currency::Currency;

pub const REFUND_URL: &str = "v1/refunds/";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Refund {
    pub id: String,
    pub entity: String,
    /// In the smallest currency unit (paise for INR).
    pub amount: u32,
    pub currency: Currency,
    pub payment_id: String,
    #[serde(default, deserialize_with = "crate::notes::deserialize")]
    pub notes: HashMap<String, String>,
    pub receipt: Option<String>,
    pub batch_id: Option<String>,
    pub status: RefundStatus,
    pub speed_requested: Option<RefundSpeed>,
    pub speed_processed: Option<RefundSpeed>,
    pub created_at: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RefundStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RefundSpeed {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "optimum")]
    Optimum,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn refund_should_deserialize_with_empty_notes_array() {
        let refund = serde_json::from_value::<Refund>(serde_json::json!({
            "id": "rfnd_1",
            "entity": "refund",
            "amount": 5000,
            "currency": "INR",
            "payment_id": "pay_1",
            "notes": [],
            "receipt": null,
            "acquirer_data": { "arn": null },
            "created_at": 1700000000,
            "batch_id": null,
            "status": "pending",
            "speed_processed": "normal",
            "speed_requested": "normal"
        }))
        .unwrap();

        assert_eq!(refund.id, "rfnd_1");
        assert_eq!(refund.amount, 5000);
        assert_eq!(refund.status, RefundStatus::Pending);
        assert_eq!(refund.speed_requested, Some(RefundSpeed::Normal));
    }

    #[test]
    fn refund_should_deserialize_with_notes_object() {
        let refund = serde_json::from_value::<Refund>(serde_json::json!({
            "id": "rfnd_1",
            "entity": "refund",
            "amount": 100,
            "currency": "INR",
            "payment_id": "pay_1",
            "notes": { "k": "v" },
            "status": "processed"
        }))
        .unwrap();

        assert_eq!(refund.status, RefundStatus::Processed);
        assert_eq!(refund.receipt, None);
        assert_eq!(refund.notes["k"], "v");
    }
}
