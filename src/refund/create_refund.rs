use std::collections::HashMap;

use crate::{
    RazorpayClient,
    error::Error,
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
        let request_url = self
            .url
            .join(PAYMENT_URL)?
            .join(&format!("{payment_id}/refund"))?;

        let response = self
            .client()
            .post(request_url)
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "PaymentId: {}, Amount: {}, StatusCode: {:?}, Error: {:?}",
                payment_id,
                request.amount,
                response.status(),
                response.text().await?
            )));
        }

        let response = response.json::<Refund>().await?;

        Ok(response)
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
        if self.notes.len() > 15 {
            return;
        }

        self.notes.insert(key, value.into());
    }

    pub fn with_receipt(&mut self, receipt: impl Into<String>) {
        self.receipt = Some(receipt.into());
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
