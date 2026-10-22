use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    error::Error,
    order::create_order::MAX_NOTES,
    payment_link::{PAYMENT_LINK_URL, PaymentLink},
};

impl RazorpayClient {
    /// Updates a link that is not yet paid, expired or cancelled.
    pub async fn update_payment_link(
        &self,
        payment_link_id: &str,
        payment_link: UpdatePaymentLinkRequest,
    ) -> Result<PaymentLink, Error> {
        let request = self
            .request(
                Method::PATCH,
                &format!("{PAYMENT_LINK_URL}/{payment_link_id}"),
            )?
            .json(&payment_link);

        Self::send(request, format!("PaymentLinkId: {payment_link_id}")).await
    }

    pub async fn cancel_payment_link(&self, payment_link_id: &str) -> Result<PaymentLink, Error> {
        let request = self.request(
            Method::POST,
            &format!("{PAYMENT_LINK_URL}/{payment_link_id}/cancel"),
        )?;

        Self::send(request, format!("PaymentLinkId: {payment_link_id}")).await
    }

    /// Re-sends the link to the customer.
    pub async fn notify_payment_link(
        &self,
        payment_link_id: &str,
        medium: NotifyMedium,
    ) -> Result<(), Error> {
        #[derive(serde::Deserialize)]
        struct NotifyResponse {
            success: bool,
        }

        let request = self.request(
            Method::POST,
            &format!("{PAYMENT_LINK_URL}/{payment_link_id}/notify_by/{medium}"),
        )?;

        let response: NotifyResponse = Self::send(
            request,
            format!("PaymentLinkId: {payment_link_id}, medium: {medium}"),
        )
        .await?;

        if !response.success {
            return Err(Error::ApiError(format!(
                "PaymentLinkId: {payment_link_id}, medium: {medium}, Error: notification was not sent"
            )));
        }

        Ok(())
    }
}

#[derive(strum::Display, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyMedium {
    #[strum(serialize = "sms")]
    Sms,
    #[strum(serialize = "email")]
    Email,
}

#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct UpdatePaymentLinkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_partial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reminder_enable: Option<bool>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    notes: HashMap<String, String>,
}

impl UpdatePaymentLinkRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reference_id(&mut self, reference_id: impl Into<String>) {
        self.reference_id = Some(reference_id.into());
    }

    pub fn with_expire_by(&mut self, expire_by: u64) {
        self.expire_by = Some(expire_by);
    }

    pub fn with_accept_partial(&mut self, accept_partial: bool) {
        self.accept_partial = Some(accept_partial);
    }

    pub fn with_reminder_enable(&mut self, reminder_enable: bool) {
        self.reminder_enable = Some(reminder_enable);
    }

    pub fn with_note(&mut self, key: String, value: impl Into<String>) {
        if self.notes.len() >= MAX_NOTES && !self.notes.contains_key(&key) {
            return;
        }

        self.notes.insert(key, value.into());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn notify_medium_should_display_as_path_segment() {
        assert_eq!(NotifyMedium::Sms.to_string(), "sms");
        assert_eq!(NotifyMedium::Email.to_string(), "email");
    }

    #[test]
    fn update_request_should_only_serialize_set_fields() {
        let mut request = UpdatePaymentLinkRequest::new();
        request.with_accept_partial(false);

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "accept_partial": false })
        );
    }
}
