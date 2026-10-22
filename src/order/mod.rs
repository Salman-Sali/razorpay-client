use std::collections::HashMap;

pub mod create_order;
pub mod fetch_order;
pub mod update_order;

pub const ORDER_URL: &str = "v1/orders";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Order {
    pub id: String,
    pub entity: String,
    pub amount: u32,
    pub amount_paid: u32,
    pub amount_due: u32,
    pub currency: String,
    pub receipt: Option<String>,
    pub offer_id: Option<String>,
    pub status: OrderStatus,
    pub attempts: Option<u32>,
    #[serde(default, deserialize_with = "crate::notes::deserialize")]
    pub notes: HashMap<String, String>,
    pub created_at: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "attempted")]
    Attempted,
    #[serde(rename = "paid")]
    Paid,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn order_should_deserialize_with_empty_notes_array() {
        let order = serde_json::from_value::<Order>(serde_json::json!({
            "id": "order_1",
            "entity": "order",
            "amount": 50000,
            "amount_paid": 0,
            "amount_due": 50000,
            "currency": "INR",
            "receipt": "receipt#1",
            "offer_id": null,
            "status": "created",
            "attempts": 0,
            "notes": [],
            "created_at": 1700000000
        }))
        .unwrap();

        assert_eq!(order.status, OrderStatus::Created);
        assert!(order.notes.is_empty());
    }

    #[test]
    fn order_should_deserialize_with_notes_object() {
        let order = serde_json::from_value::<Order>(serde_json::json!({
            "id": "order_1",
            "entity": "order",
            "amount": 50000,
            "amount_paid": 50000,
            "amount_due": 0,
            "currency": "INR",
            "receipt": null,
            "status": "paid",
            "notes": { "key": "value", "count": 2 }
        }))
        .unwrap();

        assert_eq!(order.notes["key"], "value");
        assert_eq!(order.notes["count"], "2");
    }
}
