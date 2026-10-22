pub mod collection;
pub mod currency;
pub mod customer;
pub mod error;
mod notes;
pub mod order;
pub mod payment;
pub mod payment_link;
pub mod refund;
pub mod webhook;

use std::{fmt::Display, str::FromStr};

use hmac::{Hmac, Mac};
use once_cell::sync::OnceCell;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use sha2::Sha256;
use url::Url;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct RazorpayClient {
    key_id: String,
    key_secret: String,
    url: Url,
    webhook_secret: String,
    client: OnceCell<Client>,
}

pub const RAZORPAY_DEFAULT_URL: &str = "https://api.razorpay.com/";

impl RazorpayClient {
    pub fn new(key_id: String, key_secret: String, webhook_secret: String) -> Result<Self, Error> {
        Ok(Self {
            key_id,
            key_secret,
            url: Url::from_str(RAZORPAY_DEFAULT_URL)
                .map_err(|e| Error::SomethingWentWrong(e.to_string()))?,
            webhook_secret,
            client: OnceCell::new(),
        })
    }

    /// Points the client at a different base url, e.g. a mock server in tests.
    pub fn with_url(mut self, url: &str) -> Result<Self, Error> {
        self.url = Url::from_str(url)?;
        Ok(self)
    }

    fn client(&self) -> &Client {
        self.client.get_or_init(Client::new)
    }

    /// An authenticated request to `path`, relative to the base url.
    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let request_url = self.url.join(path)?;

        Ok(self
            .client()
            .request(method, request_url)
            .basic_auth(&self.key_id, Some(&self.key_secret)))
    }

    /// Sends `request` and parses the response; `context` is included in the error on failure.
    async fn send<T: DeserializeOwned>(
        request: RequestBuilder,
        context: impl Display,
    ) -> Result<T, Error> {
        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await?;
            return Err(Error::from_response(
                status.as_u16(),
                context.to_string(),
                body,
            ));
        }

        let response = response.json::<T>().await?;

        Ok(response)
    }

    pub async fn verify_webhook_signature(
        &self,
        body: &[u8],
        signature: &str,
    ) -> Result<bool, Error> {
        Self::verify_signature(&self.webhook_secret, body, signature).await
    }

    pub async fn verify_payment_signature(
        &self,
        order_id: &str,
        payment_id: &str,
        signature: &str,
    ) -> Result<bool, Error> {
        let payload = format!("{}|{}", order_id, payment_id);
        Self::verify_signature(&self.key_secret, payload.as_bytes(), signature).await
    }

    /// Verifies the `razorpay_signature` Razorpay appends to a payment link's callback url.
    pub async fn verify_payment_link_signature(
        &self,
        payment_link_id: &str,
        payment_link_reference_id: &str,
        payment_link_status: &str,
        payment_id: &str,
        signature: &str,
    ) -> Result<bool, Error> {
        let payload = format!(
            "{}|{}|{}|{}",
            payment_link_id, payment_link_reference_id, payment_link_status, payment_id
        );
        Self::verify_signature(&self.key_secret, payload.as_bytes(), signature).await
    }

    async fn verify_signature(key: &str, body: &[u8], signature: &str) -> Result<bool, Error> {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).map_err(|e| {
            Error::SomethingWentWrong(format!("Error while reading razorpay webhook secret {e:?}"))
        })?;
        mac.update(body);

        let Ok(signature) = hex::decode(signature) else {
            return Ok(false);
        };

        // Constant time comparison.
        Ok(mac.verify_slice(&signature).is_ok())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sign(key: &str, body: &[u8]) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
        mac.update(body);
        hex::encode(mac.finalize().into_bytes())
    }

    fn client() -> RazorpayClient {
        RazorpayClient::new("key".into(), "secret".into(), "webhook".into()).unwrap()
    }

    #[tokio::test]
    async fn should_verify_webhook_signature() {
        let client = client();
        let body = br#"{"event":"payment.captured"}"#;

        assert!(
            client
                .verify_webhook_signature(body, &sign("webhook", body))
                .await
                .unwrap()
        );
        assert!(
            !client
                .verify_webhook_signature(body, &sign("secret", body))
                .await
                .unwrap()
        );
        assert!(
            !client
                .verify_webhook_signature(body, "not-hex")
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn should_verify_payment_signatures() {
        let client = client();

        assert!(
            client
                .verify_payment_signature("order_1", "pay_1", &sign("secret", b"order_1|pay_1"))
                .await
                .unwrap()
        );
        assert!(
            client
                .verify_payment_link_signature(
                    "plink_1",
                    "ref_1",
                    "paid",
                    "pay_1",
                    &sign("secret", b"plink_1|ref_1|paid|pay_1")
                )
                .await
                .unwrap()
        );
    }

    #[test]
    fn request_should_join_path_and_authenticate() {
        let request = client()
            .with_url("http://localhost:8080/")
            .unwrap()
            .request(Method::GET, "v1/orders/order_1")
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            request.url().as_str(),
            "http://localhost:8080/v1/orders/order_1"
        );
        assert!(request.headers().contains_key("authorization"));
    }
}
