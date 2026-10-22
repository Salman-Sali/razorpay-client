use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    error::Error,
    payment::{PAYMENT_URL, Payment},
};

impl RazorpayClient {
    /// Replaces the notes of a payment; notes are the only field Razorpay lets you update.
    pub async fn update_payment(
        &self,
        payment_id: &str,
        notes: HashMap<String, String>,
    ) -> Result<Payment, Error> {
        let request = self
            .request(Method::PATCH, &format!("{PAYMENT_URL}{payment_id}"))?
            .json(&serde_json::json!({ "notes": notes }));

        Self::send(request, format!("PaymentId: {payment_id}")).await
    }
}
