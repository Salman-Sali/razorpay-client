# Razorpay Client

A Rust client library for interacting with the [Razorpay API](https://razorpay.com/docs/api/).

This crate provides a simple and intuitive interface for integrating Razorpay payments into your Rust applications. It covers orders, payments, refunds, customers, payment links, webhooks and signature verification.

## Features

- Orders: create, fetch, list, update notes, fetch an order's payments
- Payments: fetch, list, capture, update notes
- Refunds: create (full, partial, instant), fetch, list, list per payment, update notes
- Customers: create, edit, fetch, list
- Payment links: create, fetch, list, update, cancel, resend notification
- Webhooks: typed payment, order, refund and payment link events
- Webhook, payment and payment link signature verification (constant time)
- Structured Razorpay API errors
- Async/await support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
razorpay-client = "0.4"
```

## Usage

### Initialize the Client

```rust
use razorpay_client::RazorpayClient;

let client = RazorpayClient::new(
    "your_key_id".to_string(),
    "your_key_secret".to_string(),
    "your_webhook_secret".to_string()
)?;
```

### Orders

```rust
use razorpay_client::{currency::Currency, order::create_order::CreateOrderRequest};

let mut request = CreateOrderRequest::new(50000, Currency::INR); // ₹500.00, in paise
request.with_receipt("receipt#1");
request.with_note("customer".to_string(), "42");

let order = client.create_order(request).await?;
let order = client.fetch_order(&order.id).await?;
let payments = client.fetch_order_payments(&order.id).await?;
```

### Payments

```rust
let payment = client.fetch_payment("pay_29QQoUBi66xm2f").await?;

// Only needed when payments are not captured automatically.
let payment = client.capture_payment(&payment.id, payment.amount, Currency::INR).await?;
```

### Refunds

```rust
use razorpay_client::refund::{RefundSpeed, create_refund::CreateRefundRequest};

let mut request = CreateRefundRequest::new(10000);
request.with_speed(RefundSpeed::Optimum);

let refund = client.create_refund("pay_29QQoUBi66xm2f", request).await?;
let refund = client.fetch_refund(&refund.id).await?;
```

### Listing

List endpoints take `ListOptions` and return a `Collection`:

```rust
use razorpay_client::collection::ListOptions;

let mut options = ListOptions::new();
options.with_count(100);
options.with_skip(100);

let orders = client.fetch_orders(&options).await?;
for order in orders.items { /* ... */ }
```

### Customers

```rust
use razorpay_client::customer::create_customer::CreateCustomerRequest;

let mut request = CreateCustomerRequest::new("Gaurav Kumar");
request.with_email("gaurav.kumar@example.com");
request.with_contact("+919000090000");

let customer = client.create_customer(request).await?;
```

### Payment Links

```rust
use razorpay_client::payment_link::{
    PaymentLinkCustomer, create_payment_link::CreatePaymentLinkRequest,
};

let mut request = CreatePaymentLinkRequest::new(100000, Currency::INR);
request.with_reference_id("invoice-1001");
request.with_description("Invoice #1001");
request.with_customer(PaymentLinkCustomer {
    name: Some("Gaurav Kumar".into()),
    contact: Some("+919000090000".into()),
    email: None,
});
request.with_notify(true, true);
request.with_callback_url("https://example.com/payment/callback");

let link = client.create_payment_link(request).await?;
println!("Share: {}", link.short_url);
```

On the callback, verify the query parameters Razorpay sends:

```rust
let is_valid = client
    .verify_payment_link_signature(&plink_id, &reference_id, &status, &payment_id, &signature)
    .await?;
```

### Webhooks

```rust
use razorpay_client::webhook::{Webhook, WebhookEvent, PaymentEvent, RefundEvent};

if !client.verify_webhook_signature(&body, &signature).await? {
    return Err(/* reject */);
}

let webhook = Webhook::try_from(body.as_slice())?;
match webhook.event {
    WebhookEvent::Payment(PaymentEvent::Captured) => {
        let payment = webhook.payload.payment.unwrap().entity;
    }
    WebhookEvent::Refund(RefundEvent::Processed) => {
        let refund = webhook.payload.refund.unwrap().entity;
    }
    _ => {}
}
```

### Verify Payment Signature (Checkout)

```rust
let is_valid = client.verify_payment_signature(&order_id, &payment_id, &signature).await?;
```

## Error Handling

All operations return a `Result` with the crate's `Error` type:

- `Razorpay { status, context, error }` - Razorpay rejected the request; `error.code` is e.g. `BAD_REQUEST_ERROR`
- `ApiError` - Network errors and non-JSON error responses
- `UrlParseError` - URL parsing errors
- `SerdeJsonError` - JSON serialization/deserialization errors
- `SomethingWentWrong` - General errors

## Supported Currencies

The crate includes an extensive `Currency` enum with support for numerous currencies including:

- INR (Indian Rupee)
- USD (US Dollar)
- EUR (Euro)
- GBP (British Pound)
- And many more...


## Acknowledgments

- This crate is not officially affiliated with Razorpay
- Inspired by the official Razorpay API documentation
