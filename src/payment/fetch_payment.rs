use crate::{
    RazorpayClient,
    error::Error,
    payment::{PAYMENT_URL, Payment},
};

impl RazorpayClient {
    pub async fn fetch_payment(&self, payment_id: String) -> Result<Payment, Error> {
        let request_url = self.url.join(PAYMENT_URL)?.join(&payment_id)?;

        let response = self
            .client()
            .get(request_url)
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "PaymentId: {}, StatusCode: {:?}, Error: {:?}",
                payment_id,
                response.status(),
                response.text().await?
            )));
        }

        let response = response.json::<Payment>().await?;

        Ok(response)
    }
}
