use reqwest::Method;

use crate::{
    RazorpayClient,
    collection::{Collection, ListOptions},
    error::Error,
    payment::{PAYMENT_URL, Payment},
};

impl RazorpayClient {
    pub async fn fetch_payment(&self, payment_id: impl AsRef<str>) -> Result<Payment, Error> {
        let payment_id = payment_id.as_ref();
        let request = self.request(Method::GET, &format!("{PAYMENT_URL}{payment_id}"))?;

        Self::send(request, format!("PaymentId: {payment_id}")).await
    }

    /// Lists payments; use `ListOptions` to page through them.
    pub async fn fetch_payments(
        &self,
        options: &ListOptions,
    ) -> Result<Collection<Payment>, Error> {
        let request = self
            .request(Method::GET, PAYMENT_URL.trim_end_matches('/'))?
            .query(options);

        Self::send(request, "Fetch payments").await
    }
}
