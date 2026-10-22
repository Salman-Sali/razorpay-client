use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    error::Error,
    refund::{REFUND_URL, Refund},
};

impl RazorpayClient {
    /// Replaces the notes of a refund; notes are the only field Razorpay lets you update.
    pub async fn update_refund(
        &self,
        refund_id: &str,
        notes: HashMap<String, String>,
    ) -> Result<Refund, Error> {
        let request = self
            .request(Method::PATCH, &format!("{REFUND_URL}{refund_id}"))?
            .json(&serde_json::json!({ "notes": notes }));

        Self::send(request, format!("RefundId: {refund_id}")).await
    }
}
