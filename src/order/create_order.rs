use crate::{RazorpayClient, currency::Currency, error::Error, order::ORDER_URL};

impl RazorpayClient {
    pub async fn create_order(
        &self,
        amount: u32,
        currency: Currency,
    ) -> Result<CreateOrderResponse, Error> {
        let request_url = self.url.join(ORDER_URL)?;

        let order = CreateOrderRequest::new(amount, currency.to_string());

        let response = self
            .client()
            .post(request_url)
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .json(&order)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "Amount: {}, currency: {}, StatusCode: {:?}, Error: {:?}",
                amount,
                currency,
                response.status(),
                response.text().await?
            )));
        }

        let response = response.json::<CreateOrderResponse>().await?;

        Ok(response)
    }
}

#[derive(serde::Serialize, Debug, Clone)]
struct CreateOrderRequest {
    amount: u32,
    currency: String,
}

impl CreateOrderRequest {
    fn new(amount: u32, currency: String) -> Self {
        Self { amount, currency }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateOrderResponse {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub status: String,
}
