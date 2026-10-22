pub mod create_payment_link;
pub mod fetch_payment_link;
pub mod update_payment_link;

use std::collections::HashMap;

use crate::currency::Currency;

pub const PAYMENT_LINK_URL: &str = "v1/payment_links";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PaymentLink {
    pub id: String,
    /// In the smallest currency unit (paise for INR).
    pub amount: u32,
    #[serde(default)]
    pub amount_paid: u32,
    pub currency: Currency,
    #[serde(default)]
    pub accept_partial: bool,
    pub first_min_partial_amount: Option<u32>,
    pub description: Option<String>,
    pub reference_id: Option<String>,
    /// The url to share with the customer.
    pub short_url: String,
    pub status: PaymentLinkStatus,
    pub order_id: Option<String>,
    pub customer: Option<PaymentLinkCustomer>,
    pub notify: Option<PaymentLinkNotify>,
    #[serde(default)]
    pub reminder_enable: bool,
    #[serde(default, deserialize_with = "crate::notes::deserialize")]
    pub notes: HashMap<String, String>,
    pub callback_url: Option<String>,
    pub callback_method: Option<String>,
    /// Payments made through the link; `None` until one is made.
    pub payments: Option<Vec<PaymentLinkPayment>>,
    pub expire_by: Option<u64>,
    pub expired_at: Option<u64>,
    pub cancelled_at: Option<u64>,
    pub created_at: Option<u64>,
    pub updated_at: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PaymentLinkStatus {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "partially_paid")]
    PartiallyPaid,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct PaymentLinkCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

/// Whether Razorpay sends the link to the customer itself.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct PaymentLinkNotify {
    #[serde(default)]
    pub sms: bool,
    #[serde(default)]
    pub email: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub whatsapp: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PaymentLinkPayment {
    pub payment_id: String,
    pub plink_id: Option<String>,
    pub amount: u32,
    pub method: Option<String>,
    pub status: String,
    pub created_at: Option<u64>,
    pub updated_at: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn payment_link_should_deserialize() {
        let link = serde_json::from_value::<PaymentLink>(serde_json::json!({
            "accept_partial": true,
            "amount": 1000,
            "amount_paid": 500,
            "callback_method": "",
            "callback_url": "",
            "cancelled_at": 0,
            "created_at": 1700000000,
            "currency": "INR",
            "customer": {
                "contact": "+919000090000",
                "email": "gaurav.kumar@example.com",
                "name": "Gaurav Kumar"
            },
            "description": "Payment for policy no #23456",
            "expire_by": 1791097057,
            "expired_at": 0,
            "first_min_partial_amount": 100,
            "id": "plink_1",
            "notes": null,
            "notify": { "email": true, "sms": true },
            "order_id": "order_1",
            "payments": [{
                "amount": 500,
                "created_at": 1700000100,
                "method": "upi",
                "payment_id": "pay_1",
                "plink_id": "plink_1",
                "status": "captured",
                "updated_at": 1700000100
            }],
            "reference_id": "TS1989",
            "reminder_enable": true,
            "reminders": [],
            "short_url": "https://rzp.io/i/abc",
            "status": "partially_paid",
            "updated_at": 1700000100,
            "user_id": ""
        }))
        .unwrap();

        assert_eq!(link.status, PaymentLinkStatus::PartiallyPaid);
        assert_eq!(link.payments.unwrap()[0].payment_id, "pay_1");
        assert!(link.notes.is_empty());
    }
}
