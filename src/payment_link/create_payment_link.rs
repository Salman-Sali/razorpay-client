use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    currency::Currency,
    error::Error,
    order::create_order::MAX_NOTES,
    payment_link::{PAYMENT_LINK_URL, PaymentLink, PaymentLinkCustomer, PaymentLinkNotify},
};

impl RazorpayClient {
    pub async fn create_payment_link(
        &self,
        payment_link: CreatePaymentLinkRequest,
    ) -> Result<PaymentLink, Error> {
        let request = self
            .request(Method::POST, PAYMENT_LINK_URL)?
            .json(&payment_link);

        Self::send(
            request,
            format!(
                "Amount: {}, currency: {}",
                payment_link.amount, payment_link.currency
            ),
        )
        .await
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreatePaymentLinkRequest {
    amount: u32,
    currency: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    accept_partial: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_min_partial_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_by: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<PaymentLinkCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify: Option<PaymentLinkNotify>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    reminder_enable: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    notes: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_method: Option<&'static str>,
}

impl CreatePaymentLinkRequest {
    /// `amount` is in the smallest currency unit (paise for INR).
    pub fn new(amount: u32, currency: Currency) -> Self {
        Self {
            amount,
            currency: currency.to_string(),
            accept_partial: false,
            first_min_partial_amount: None,
            description: None,
            reference_id: None,
            expire_by: None,
            customer: None,
            notify: None,
            reminder_enable: false,
            notes: HashMap::new(),
            callback_url: None,
            callback_method: None,
        }
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    /// Lets the customer pay in parts, the first being at least `first_min_partial_amount`
    /// when given.
    pub fn with_partial_payment(&mut self, first_min_partial_amount: Option<u32>) {
        self.accept_partial = true;
        self.first_min_partial_amount = first_min_partial_amount;
    }

    pub fn with_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
    }

    /// Your own unique reference for the link, at most 40 characters.
    pub fn with_reference_id(&mut self, reference_id: impl Into<String>) {
        self.reference_id = Some(reference_id.into());
    }

    /// Unix timestamp (seconds) at which the link expires; at least 15 minutes from now.
    pub fn with_expire_by(&mut self, expire_by: u64) {
        self.expire_by = Some(expire_by);
    }

    pub fn with_customer(&mut self, customer: PaymentLinkCustomer) {
        self.customer = Some(customer);
    }

    /// Has Razorpay send the link to the customer by sms and/or email.
    pub fn with_notify(&mut self, sms: bool, email: bool) {
        self.notify = Some(PaymentLinkNotify {
            sms,
            email,
            whatsapp: false,
        });
    }

    pub fn with_reminders(&mut self) {
        self.reminder_enable = true;
    }

    pub fn with_note(&mut self, key: String, value: impl Into<String>) {
        if self.notes.len() >= MAX_NOTES && !self.notes.contains_key(&key) {
            return;
        }

        self.notes.insert(key, value.into());
    }

    /// Redirects the customer to `callback_url` with a GET request after payment. Verify the
    /// query parameters with `RazorpayClient::verify_payment_link_signature`.
    pub fn with_callback_url(&mut self, callback_url: impl Into<String>) {
        self.callback_url = Some(callback_url.into());
        self.callback_method = Some("get");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_should_serialize_minimal_body() {
        let request = CreatePaymentLinkRequest::new(1000, Currency::INR);

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "amount": 1000, "currency": "INR" })
        );
    }

    #[test]
    fn request_should_serialize_all_fields() {
        let mut request = CreatePaymentLinkRequest::new(1000, Currency::INR);
        request.with_partial_payment(Some(100));
        request.with_description("Policy #23456");
        request.with_reference_id("TS1989");
        request.with_expire_by(1791097057);
        request.with_customer(PaymentLinkCustomer {
            name: Some("Gaurav Kumar".into()),
            contact: Some("+919000090000".into()),
            email: None,
        });
        request.with_notify(true, false);
        request.with_reminders();
        request.with_note("policy".into(), "23456");
        request.with_callback_url("https://example.com/callback");

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "amount": 1000,
                "currency": "INR",
                "accept_partial": true,
                "first_min_partial_amount": 100,
                "description": "Policy #23456",
                "reference_id": "TS1989",
                "expire_by": 1791097057,
                "customer": { "name": "Gaurav Kumar", "contact": "+919000090000" },
                "notify": { "sms": true, "email": false },
                "reminder_enable": true,
                "notes": { "policy": "23456" },
                "callback_url": "https://example.com/callback",
                "callback_method": "get"
            })
        );
    }
}
