use crate::{error::Error, payment::Payment, refund::Refund};

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

/// The entities a webhook carries; refund events contain both the `refund` and its `payment`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct WebhookPayload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<Entity<Payment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refund: Option<Entity<Refund>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Entity<T> {
    pub entity: T,
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum WebhookEvent {
    Payment(PaymentEvent),
    Refund(RefundEvent),
    Other(String),
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum PaymentEvent {
    Authorized,
    Captured,
    Failed,
}

#[derive(serde::Serialize, Debug, Clone)]
pub enum RefundEvent {
    Created,
    Processed,
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
            "refund.created" => WebhookEvent::Refund(RefundEvent::Created),
            "refund.processed" => WebhookEvent::Refund(RefundEvent::Processed),
            "refund.failed" => WebhookEvent::Refund(RefundEvent::Failed),
            other => WebhookEvent::Other(other.to_string()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::refund::RefundStatus;

    fn payment_json(status: &str) -> serde_json::Value {
        serde_json::json!({
            "id": "pay_1",
            "entity": "payment",
            "amount": 10000,
            "currency": "INR",
            "status": status,
            "order_id": "order_1",
            "international": false,
            "method": "upi",
            "amount_refunded": 5000,
            "captured": true,
            "description": null,
            "fee": null,
            "tax": null,
            "error_code": null,
            "error_description": null,
            "error_source": null,
            "error_step": null,
            "error_reason": null
        })
    }

    fn refund_webhook(event: &str, refund_status: &str) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "entity": "event",
            "account_id": "acc_1",
            "event": event,
            "contains": ["refund", "payment"],
            "payload": {
                "refund": { "entity": {
                    "id": "rfnd_1",
                    "entity": "refund",
                    "amount": 5000,
                    "currency": "INR",
                    "payment_id": "pay_1",
                    "notes": [],
                    "receipt": null,
                    "status": refund_status
                } },
                "payment": { "entity": payment_json("captured") }
            },
            "created_at": 1700000000
        }))
        .unwrap()
    }

    #[test]
    fn webhook_should_parse_refund_processed() {
        let webhook =
            Webhook::try_from(refund_webhook("refund.processed", "processed").as_slice()).unwrap();

        assert!(matches!(
            webhook.event,
            WebhookEvent::Refund(RefundEvent::Processed)
        ));
        let refund = webhook.payload.refund.unwrap().entity;
        assert_eq!(refund.id, "rfnd_1");
        assert_eq!(refund.status, RefundStatus::Processed);
        assert_eq!(
            webhook.payload.payment.unwrap().entity.amount_refunded,
            5000
        );
    }

    #[test]
    fn webhook_should_parse_refund_failed_and_created() {
        let failed =
            Webhook::try_from(refund_webhook("refund.failed", "failed").as_slice()).unwrap();
        let created =
            Webhook::try_from(refund_webhook("refund.created", "pending").as_slice()).unwrap();

        assert!(matches!(
            failed.event,
            WebhookEvent::Refund(RefundEvent::Failed)
        ));
        assert!(matches!(
            created.event,
            WebhookEvent::Refund(RefundEvent::Created)
        ));
    }

    #[test]
    fn webhook_should_parse_payment_only_payload() {
        let body = serde_json::to_vec(&serde_json::json!({
            "entity": "event",
            "account_id": "acc_1",
            "event": "payment.captured",
            "contains": ["payment"],
            "payload": { "payment": { "entity": payment_json("captured") } },
            "created_at": 1700000000
        }))
        .unwrap();

        let webhook = Webhook::try_from(body.as_slice()).unwrap();

        assert!(matches!(
            webhook.event,
            WebhookEvent::Payment(PaymentEvent::Captured)
        ));
        assert!(webhook.payload.payment.is_some());
        assert!(webhook.payload.refund.is_none());
    }

    #[test]
    fn webhook_should_keep_unknown_events() {
        let body = serde_json::to_vec(&serde_json::json!({
            "entity": "event",
            "account_id": "acc_1",
            "event": "order.paid",
            "contains": ["order"],
            "payload": { "order": { "entity": {} } },
            "created_at": 1700000000
        }))
        .unwrap();

        let webhook = Webhook::try_from(body.as_slice()).unwrap();

        assert!(matches!(webhook.event, WebhookEvent::Other(e) if e == "order.paid"));
    }
}
