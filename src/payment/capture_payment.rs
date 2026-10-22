use reqwest::Method;

use crate::{
    RazorpayClient,
    currency::Currency,
    error::Error,
    payment::{PAYMENT_URL, Payment},
};

impl RazorpayClient {
    /// Captures an `authorized` payment. `amount` must equal the authorized amount, in the
    /// smallest currency unit. Payments left uncaptured are refunded by Razorpay.
    pub async fn capture_payment(
        &self,
        payment_id: &str,
        amount: u32,
        currency: Currency,
    ) -> Result<Payment, Error> {
        let request = self
            .request(Method::POST, &format!("{PAYMENT_URL}{payment_id}/capture"))?
            .json(&serde_json::json!({
                "amount": amount,
                "currency": currency.to_string(),
            }));

        Self::send(
            request,
            format!("PaymentId: {payment_id}, Amount: {amount}, currency: {currency}"),
        )
        .await
    }
}
