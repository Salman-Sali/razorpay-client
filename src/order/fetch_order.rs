use crate::{
    RazorpayClient,
    error::Error,
    order::{ORDER_URL, Order},
};

impl RazorpayClient {
    pub async fn fetch_order(&self, order_id: &str) -> Result<Order, Error> {
        let request_url = self.url.join(ORDER_URL)?.join(order_id)?;

        let response = self
            .client()
            .get(request_url)
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "Order_id: {}, StatusCode: {:?}, Error: {:?}",
                order_id,
                response.status(),
                response.text().await?
            )));
        }
        let response = response.json::<Order>().await?;

        Ok(response)
    }
}
