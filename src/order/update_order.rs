use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    error::Error,
    order::{ORDER_URL, Order},
};

impl RazorpayClient {
    /// Replaces the notes of an order; notes are the only field Razorpay lets you update.
    pub async fn update_order(
        &self,
        order_id: &str,
        notes: HashMap<String, String>,
    ) -> Result<Order, Error> {
        let request = self
            .request(Method::PATCH, &format!("{ORDER_URL}/{order_id}"))?
            .json(&serde_json::json!({ "notes": notes }));

        Self::send(request, format!("Order_id: {order_id}")).await
    }
}
