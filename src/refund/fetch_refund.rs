use crate::{
    RazorpayClient,
    error::Error,
    refund::{REFUND_URL, Refund},
};

impl RazorpayClient {
    pub async fn fetch_refund(&self, refund_id: &str) -> Result<Refund, Error> {
        let request_url = self.url.join(REFUND_URL)?.join(refund_id)?;

        let response = self
            .client()
            .get(request_url)
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "RefundId: {}, StatusCode: {:?}, Error: {:?}",
                refund_id,
                response.status(),
                response.text().await?
            )));
        }

        let response = response.json::<Refund>().await?;

        Ok(response)
    }
}
