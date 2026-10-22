use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    error::Error,
    order::create_order::MAX_NOTES,
    payment::PAYMENT_URL,
    refund::{Refund, RefundSpeed},
};

impl RazorpayClient {
    /// Refunds `request.amount` (paise) of a captured payment. Call it repeatedly for several
    /// partial refunds; Razorpay marks the payment `refunded` only once it is fully refunded.
    pub async fn create_refund(
        &self,
        payment_id: &str,
        request: CreateRefundRequest,
    ) -> Result<Refund, Error> {
        let http_request = self
            .request(Method::POST, &format!("{PAYMENT_URL}{payment_id}/refund"))?
            .json(&request);

        Self::send(
            http_request,
            format!("PaymentId: {}, Amount: {}", payment_id, request.amount),
        )
        .await
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreateRefundRequest {
    amount: u32,
    speed: RefundSpeed,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    notes: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt: Option<String>,
}

impl CreateRefundRequest {
    /// `amount` is in paise; speed defaults to `normal`.
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            speed: RefundSpeed::Normal,
            notes: HashMap::new(),
            receipt: None,
        }
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn receipt(&self) -> Option<&str> {
        self.receipt.as_deref()
    }

    pub fn with_note(&mut self, key: String, value: impl Into<String>) {
        if self.notes.len() >= MAX_NOTES && !self.notes.contains_key(&key) {
            return;
        }

        self.notes.insert(key, value.into());
    }

    pub fn with_receipt(&mut self, receipt: impl Into<String>) {
        self.receipt = Some(receipt.into());
    }

    /// `optimum` refunds instantly where the bank supports it, at an extra fee.
    pub fn with_speed(&mut self, speed: RefundSpeed) {
        self.speed = speed;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_should_serialize_minimal_body() {
        let request = CreateRefundRequest::new(5000);

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "amount": 5000, "speed": "normal" })
        );
    }

    #[test]
    fn request_should_serialize_notes_and_receipt() {
        let mut request = CreateRefundRequest::new(100);
        request.with_note("order".to_string(), "100001");
        request.with_receipt("refund-1");

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "amount": 100,
                "speed": "normal",
                "notes": { "order": "100001" },
                "receipt": "refund-1"
            })
        );
    }
}
