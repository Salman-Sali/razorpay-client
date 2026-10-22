use std::collections::HashMap;

use reqwest::Method;

use crate::{RazorpayClient, currency::Currency, error::Error, order::ORDER_URL};

/// Razorpay allows at most 15 key-value pairs in `notes`.
pub(crate) const MAX_NOTES: usize = 15;

impl RazorpayClient {
    pub async fn create_order(
        &self,
        order: CreateOrderRequest,
    ) -> Result<CreateOrderResponse, Error> {
        let request = self.request(Method::POST, ORDER_URL)?.json(&order);

        Self::send(
            request,
            format!("Amount: {}, currency: {}", order.amount, order.currency),
        )
        .await
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreateOrderRequest {
    amount: u32,
    currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    partial_payment: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_payment_min_amount: Option<u32>,
    notes: HashMap<String, String>,
}

impl CreateOrderRequest {
    /// `amount` is in the smallest currency unit (paise for INR).
    pub fn new(amount: u32, currency: Currency) -> Self {
        Self {
            amount,
            currency: currency.to_string(),
            receipt: None,
            partial_payment: false,
            first_payment_min_amount: None,
            notes: HashMap::new(),
        }
    }

    pub fn with_note(&mut self, key: String, value: impl Into<String>) {
        if self.notes.len() >= MAX_NOTES && !self.notes.contains_key(&key) {
            return;
        }

        self.notes.insert(key, value.into());
    }

    /// Your own reference for the order, at most 40 characters.
    pub fn with_receipt(&mut self, receipt: impl Into<String>) {
        self.receipt = Some(receipt.into());
    }

    /// Lets the customer pay the order in parts, the first being at least
    /// `first_payment_min_amount` when given.
    pub fn with_partial_payment(&mut self, first_payment_min_amount: Option<u32>) {
        self.partial_payment = true;
        self.first_payment_min_amount = first_payment_min_amount;
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateOrderResponse {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub status: String,
    pub receipt: Option<String>,
    pub created_at: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_should_serialize_minimal_body() {
        let request = CreateOrderRequest::new(5000, Currency::INR);

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "amount": 5000, "currency": "INR", "notes": {} })
        );
    }

    #[test]
    fn request_should_serialize_receipt_and_partial_payment() {
        let mut request = CreateOrderRequest::new(5000, Currency::INR);
        request.with_receipt("receipt#1");
        request.with_partial_payment(Some(1000));

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "amount": 5000,
                "currency": "INR",
                "receipt": "receipt#1",
                "partial_payment": true,
                "first_payment_min_amount": 1000,
                "notes": {}
            })
        );
    }

    #[test]
    fn request_should_cap_notes_at_fifteen() {
        let mut request = CreateOrderRequest::new(5000, Currency::INR);
        for i in 0..20 {
            request.with_note(format!("k{i}"), "v");
        }
        request.with_note("k0".to_string(), "updated");

        assert_eq!(request.notes.len(), MAX_NOTES);
        assert_eq!(request.notes["k0"], "updated");
    }
}
