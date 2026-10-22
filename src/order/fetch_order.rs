use reqwest::Method;

use crate::{
    RazorpayClient,
    collection::{Collection, ListOptions},
    error::Error,
    order::{ORDER_URL, Order},
    payment::Payment,
};

impl RazorpayClient {
    pub async fn fetch_order(&self, order_id: &str) -> Result<Order, Error> {
        let request = self.request(Method::GET, &format!("{ORDER_URL}/{order_id}"))?;

        Self::send(request, format!("Order_id: {order_id}")).await
    }

    /// Payments made against an order, including failed attempts.
    pub async fn fetch_order_payments(&self, order_id: &str) -> Result<Collection<Payment>, Error> {
        let request = self.request(Method::GET, &format!("{ORDER_URL}/{order_id}/payments"))?;

        Self::send(request, format!("Order_id: {order_id}")).await
    }

    /// Lists orders; use `ListOptions` to page through them.
    pub async fn fetch_orders(&self, options: &ListOptions) -> Result<Collection<Order>, Error> {
        let request = self.request(Method::GET, ORDER_URL)?.query(options);

        Self::send(request, "Fetch orders").await
    }
}
