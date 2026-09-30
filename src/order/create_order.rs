use std::collections::HashMap;

use crate::{RazorpayClient, currency::Currency, error::Error, order::ORDER_URL};

impl RazorpayClient {
    pub async fn create_order(
        &self,
        order: CreateOrderRequest,
    ) -> Result<CreateOrderResponse, Error> {
        let request_url = self.url.join(ORDER_URL)?;

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
                order.amount,
                order.currency,
                response.status(),
                response.text().await?
            )));
        }

        let response = response.json::<CreateOrderResponse>().await?;

        Ok(response)
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreateOrderRequest {
    amount: u32,
    currency: String,
    notes: HashMap<String, String>,
}

impl CreateOrderRequest {
    pub fn new(amount: u32, currency: Currency) -> Self {
        Self {
            amount,
            currency: currency.to_string(),
            notes: HashMap::new(),
        }
    }

    pub fn with_note(&mut self, key: String, value: impl Into<String>) {
        if self.notes.len() > 15 {
            return;
        }

        self.notes.insert(key, value.into());
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateOrderResponse {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub status: String,
}
