pub mod capture_payment;
pub mod fetch_payment;
pub mod update_payment;

use std::collections::HashMap;

use crate::currency::Currency;

pub const PAYMENT_URL: &str = "/v1/payments/";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Payment {
    pub id: String,
    pub entity: String,
    /// In the smallest currency unit (paise for INR).
    pub amount: u32,
    pub currency: Currency,
    pub status: PaymentStatus,
    /// `None` for payments not made against an order, e.g. through a QR code.
    pub order_id: Option<String>,
    pub invoice_id: Option<String>,
    pub international: bool,
    pub method: PaymentMethod,
    pub amount_refunded: u32,
    /// `partial` or `full` once a refund has been made.
    pub refund_status: Option<String>,
    pub captured: bool,
    pub description: Option<String>,
    pub card_id: Option<String>,
    pub bank: Option<String>,
    pub wallet: Option<String>,
    pub vpa: Option<String>,
    pub email: Option<String>,
    pub contact: Option<String>,
    pub customer_id: Option<String>,
    #[serde(default, deserialize_with = "crate::notes::deserialize")]
    pub notes: HashMap<String, String>,
    pub fee: Option<u32>,
    pub tax: Option<u32>,
    pub error_code: Option<String>,
    pub error_description: Option<String>,
    pub error_source: Option<String>,
    pub error_step: Option<String>,
    pub error_reason: Option<String>,
    pub created_at: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
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
    #[serde(rename = "cardless_emi")]
    CardlessEMI,
    #[serde(rename = "bank_transfer")]
    BankTransfer,
    #[serde(rename = "emandate")]
    EMandate,
    #[serde(rename = "nach")]
    Nach,
    #[serde(rename = "app")]
    App,
    /// A method this crate does not know about yet.
    #[serde(other)]
    Other,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn payment_should_deserialize_without_order_and_with_unknown_method() {
        let payment = serde_json::from_value::<Payment>(serde_json::json!({
            "id": "pay_1",
            "entity": "payment",
            "amount": 10000,
            "currency": "INR",
            "status": "captured",
            "order_id": null,
            "invoice_id": null,
            "international": false,
            "method": "some_new_method",
            "amount_refunded": 0,
            "refund_status": null,
            "captured": true,
            "description": null,
            "email": "gaurav.kumar@example.com",
            "contact": "+919000090000",
            "notes": [],
            "fee": 236,
            "tax": 36,
            "acquirer_data": { "rrn": "123" },
            "created_at": 1700000000
        }))
        .unwrap();

        assert_eq!(payment.order_id, None);
        assert_eq!(payment.method, PaymentMethod::Other);
        assert_eq!(payment.contact.as_deref(), Some("+919000090000"));
    }
}
