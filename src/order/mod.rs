pub mod create_order;
pub mod fetch_order;

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
    pub status: OrderStatus,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum OrderStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "attempted")]
    Attempted,
    #[serde(rename = "paid")]
    Paid,
}
