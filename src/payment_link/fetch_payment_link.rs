use reqwest::Method;

use crate::{
    RazorpayClient,
    error::Error,
    payment_link::{PAYMENT_LINK_URL, PaymentLink},
};

impl RazorpayClient {
    pub async fn fetch_payment_link(&self, payment_link_id: &str) -> Result<PaymentLink, Error> {
        let request = self.request(
            Method::GET,
            &format!("{PAYMENT_LINK_URL}/{payment_link_id}"),
        )?;

        Self::send(request, format!("PaymentLinkId: {payment_link_id}")).await
    }

    /// Lists payment links, optionally only those with the given payment or reference id.
    pub async fn fetch_payment_links(
        &self,
        options: &FetchPaymentLinksOptions,
    ) -> Result<Vec<PaymentLink>, Error> {
        #[derive(serde::Deserialize)]
        struct PaymentLinks {
            payment_links: Vec<PaymentLink>,
        }

        let request = self.request(Method::GET, PAYMENT_LINK_URL)?.query(options);

        let response: PaymentLinks = Self::send(request, "Fetch payment links").await?;

        Ok(response.payment_links)
    }
}

#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct FetchPaymentLinksOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
}

impl FetchPaymentLinksOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_payment_id(&mut self, payment_id: impl Into<String>) {
        self.payment_id = Some(payment_id.into());
    }

    pub fn with_reference_id(&mut self, reference_id: impl Into<String>) {
        self.reference_id = Some(reference_id.into());
    }
}
