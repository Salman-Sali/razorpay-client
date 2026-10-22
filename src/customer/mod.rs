pub mod create_customer;
pub mod fetch_customer;

use std::collections::HashMap;

pub const CUSTOMER_URL: &str = "v1/customers";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Customer {
    pub id: String,
    pub entity: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub gstin: Option<String>,
    #[serde(default, deserialize_with = "crate::notes::deserialize")]
    pub notes: HashMap<String, String>,
    pub created_at: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn customer_should_deserialize() {
        let customer = serde_json::from_value::<Customer>(serde_json::json!({
            "id": "cust_1",
            "entity": "customer",
            "name": "Gaurav Kumar",
            "email": "gaurav.kumar@example.com",
            "contact": "9123456780",
            "gstin": null,
            "notes": { "notes_key_1": "Tea, Earl Grey, Hot" },
            "created_at": 1700000000
        }))
        .unwrap();

        assert_eq!(customer.name.as_deref(), Some("Gaurav Kumar"));
        assert_eq!(customer.notes.len(), 1);
    }
}
