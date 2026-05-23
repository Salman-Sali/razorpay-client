use crate::{error::Error, payment::Payment};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Webhook {
    pub entity: String,
    pub account_id: String,
    pub event: WebhookEvent,
    pub contains: Vec<String>,
    pub payload: WebhookPayload,
    pub created_at: u32,
}

impl TryFrom<&[u8]> for Webhook {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value).map_err(|e| Error::SerdeJsonError(e))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WebhookPayload {
    Payment(Entity<Payment>),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Entity<T> {
    pub entity: T,
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum WebhookEvent {
    Payment(PaymentEvent),
    Other(String),
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum PaymentEvent {
    Authorized,
    Captured,
    Failed,
}

impl<'de> serde::Deserialize<'de> for WebhookEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "payment.authorized" => WebhookEvent::Payment(PaymentEvent::Authorized),
            "payment.captured" => WebhookEvent::Payment(PaymentEvent::Captured),
            "payment.failed" => WebhookEvent::Payment(PaymentEvent::Failed),
            other => WebhookEvent::Other(other.to_string()),
        })
    }
}
