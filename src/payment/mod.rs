pub mod fetch_payment;

use crate::currency::Currency;

pub const PAYMENT_URL: &str = "/v1/payments/";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Payment {
    pub id: String,
    pub entity: String,
    pub amount: u32,
    pub currency: Currency,
    pub status: PaymentStatus,
    pub order_id: String,
    pub international: bool,
    pub method: PaymentMethod,
    pub amount_refunded: u32,
    pub captured: bool,
    pub description: Option<String>,
    pub fee: Option<u32>,
    pub tax: Option<u32>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_step: Option<String>,
    pub error_reason: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "captured")]
    Captured,
    #[serde(rename = "refunded")]
    Refunded,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum PaymentMethod {
    #[serde(rename = "card")]
    Card,
    #[serde(rename = "netbanking")]
    NetBanking,
    #[serde(rename = "wallet")]
    Wallet,
    #[serde(rename = "emi")]
    EMI,
    #[serde(rename = "upi")]
    UPI,
    #[serde(rename = "paylater")]
    Paylater,
}
