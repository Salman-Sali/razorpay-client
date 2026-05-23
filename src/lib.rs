pub mod currency;
pub mod error;
pub mod order;
pub mod payment;
pub mod webhook;

use std::str::FromStr;

use hmac::{Hmac, Mac};
use once_cell::sync::OnceCell;
use reqwest::Client;
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

    fn client(&self) -> &Client {
        self.client.get_or_init(|| Client::new())
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

    async fn verify_signature(key: &str, body: &[u8], signature: &str) -> Result<bool, Error> {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).map_err(|e| {
            Error::SomethingWentWrong(format!("Error while reading razorpay webhook secret {e:?}"))
        })?;
        mac.update(body);
        let expected = hex::encode(mac.finalize().into_bytes());
        Ok(expected == signature)
    }
}
