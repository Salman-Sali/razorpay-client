use reqwest::Method;

use crate::{
    RazorpayClient,
    collection::{Collection, ListOptions},
    customer::{CUSTOMER_URL, Customer},
    error::Error,
};

impl RazorpayClient {
    pub async fn fetch_customer(&self, customer_id: &str) -> Result<Customer, Error> {
        let request = self.request(Method::GET, &format!("{CUSTOMER_URL}/{customer_id}"))?;

        Self::send(request, format!("CustomerId: {customer_id}")).await
    }

    /// Lists customers; use `ListOptions` to page through them.
    pub async fn fetch_customers(
        &self,
        options: &ListOptions,
    ) -> Result<Collection<Customer>, Error> {
        let request = self.request(Method::GET, CUSTOMER_URL)?.query(options);

        Self::send(request, "Fetch customers").await
    }
}
