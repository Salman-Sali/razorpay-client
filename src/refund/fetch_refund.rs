use reqwest::Method;

use crate::{
    RazorpayClient,
    collection::{Collection, ListOptions},
    error::Error,
    payment::PAYMENT_URL,
    refund::{REFUND_URL, Refund},
};

impl RazorpayClient {
    pub async fn fetch_refund(&self, refund_id: &str) -> Result<Refund, Error> {
        let request = self.request(Method::GET, &format!("{REFUND_URL}{refund_id}"))?;

        Self::send(request, format!("RefundId: {refund_id}")).await
    }

    /// Lists refunds across all payments; use `ListOptions` to page through them.
    pub async fn fetch_refunds(&self, options: &ListOptions) -> Result<Collection<Refund>, Error> {
        let request = self
            .request(Method::GET, REFUND_URL.trim_end_matches('/'))?
            .query(options);

        Self::send(request, "Fetch refunds").await
    }

    /// Lists the refunds of one payment.
    pub async fn fetch_payment_refunds(
        &self,
        payment_id: &str,
        options: &ListOptions,
    ) -> Result<Collection<Refund>, Error> {
        let request = self
            .request(Method::GET, &format!("{PAYMENT_URL}{payment_id}/refunds"))?
            .query(options);

        Self::send(request, format!("PaymentId: {payment_id}")).await
    }

    /// Fetches a refund, checking it belongs to `payment_id`.
    pub async fn fetch_payment_refund(
        &self,
        payment_id: &str,
        refund_id: &str,
    ) -> Result<Refund, Error> {
        let request = self.request(
            Method::GET,
            &format!("{PAYMENT_URL}{payment_id}/refunds/{refund_id}"),
        )?;

        Self::send(
            request,
            format!("PaymentId: {payment_id}, RefundId: {refund_id}"),
        )
        .await
    }
}
