use std::collections::HashMap;

use reqwest::Method;

use crate::{
    RazorpayClient,
    customer::{CUSTOMER_URL, Customer},
    error::Error,
    order::create_order::MAX_NOTES,
};

impl RazorpayClient {
    pub async fn create_customer(
        &self,
        customer: CreateCustomerRequest,
    ) -> Result<Customer, Error> {
        let request = self.request(Method::POST, CUSTOMER_URL)?.json(&customer);

        Self::send(request, format!("Customer: {}", customer.name)).await
    }

    /// Updates the given details of a customer; fields left unset are not changed.
    pub async fn edit_customer(
        &self,
        customer_id: &str,
        customer: EditCustomerRequest,
    ) -> Result<Customer, Error> {
        let request = self
            .request(Method::PUT, &format!("{CUSTOMER_URL}/{customer_id}"))?
            .json(&customer);

        Self::send(request, format!("CustomerId: {customer_id}")).await
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct CreateCustomerRequest {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gstin: Option<String>,
    /// `"0"` returns the existing customer when the email and contact match one, instead of
    /// failing.
    fail_existing: &'static str,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    notes: HashMap<String, String>,
}

impl CreateCustomerRequest {
    /// By default an existing customer with the same email and contact is returned rather than
    /// the request failing; see `fail_if_exists`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: None,
            contact: None,
            gstin: None,
            fail_existing: "0",
            notes: HashMap::new(),
        }
    }

    pub fn with_email(&mut self, email: impl Into<String>) {
        self.email = Some(email.into());
    }

    pub fn with_contact(&mut self, contact: impl Into<String>) {
        self.contact = Some(contact.into());
    }

    pub fn with_gstin(&mut self, gstin: impl Into<String>) {
        self.gstin = Some(gstin.into());
    }

    pub fn with_note(&mut self, key: String, value: impl Into<String>) {
        if self.notes.len() >= MAX_NOTES && !self.notes.contains_key(&key) {
            return;
        }

        self.notes.insert(key, value.into());
    }

    /// Makes Razorpay reject the request when the customer already exists.
    pub fn fail_if_exists(&mut self) {
        self.fail_existing = "1";
    }
}

#[derive(serde::Serialize, Debug, Clone, Default)]
pub struct EditCustomerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<String>,
}

impl EditCustomerRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(&mut self, name: impl Into<String>) {
        self.name = Some(name.into());
    }

    pub fn with_email(&mut self, email: impl Into<String>) {
        self.email = Some(email.into());
    }

    pub fn with_contact(&mut self, contact: impl Into<String>) {
        self.contact = Some(contact.into());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_should_serialize() {
        let mut request = CreateCustomerRequest::new("Gaurav Kumar");
        request.with_email("gaurav.kumar@example.com");
        request.with_contact("9123456780");

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "name": "Gaurav Kumar",
                "email": "gaurav.kumar@example.com",
                "contact": "9123456780",
                "fail_existing": "0"
            })
        );
    }

    #[test]
    fn edit_request_should_only_serialize_set_fields() {
        let mut request = EditCustomerRequest::new();
        request.with_contact("9000000000");

        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "contact": "9000000000" })
        );
    }
}
