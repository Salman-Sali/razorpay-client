# Razorpay Client

A Rust client library for interacting with the [Razorpay API](https://razorpay.com/docs/api/).

This crate provides a simple and intuitive interface for integrating Razorpay payments into your Rust applications. It supports order creation, payment processing, webhook verification, and more.

## Features

- Create and fetch orders
- Fetch payment details
- Verify webhook signatures
- Verify payment signatures
- Strong typing with comprehensive error handling
- Async/await support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
razorpay-client = { path = "./razorpay-client" }
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

### Create an Order

```rust
use razorpay_client::currency::Currency;

let order = client.create_order(100, Currency::INR).await?;
println!("Order ID: {}", order.id);
```

### Fetch an Order

```rust
let order = client.fetch_order("order_id").await?;
println!("Order status: {:?}", order.status);
```

### Fetch a Payment

```rust
let payment = client.fetch_payment("payment_id".to_string()).await?;
println!("Payment status: {:?}", payment.status);
```

### Verify Webhook Signature

```rust
let is_valid = client.verify_webhook_signature(&body, &signature).await?;
if is_valid {
    println!("Webhook signature is valid");
}
```

### Verify Payment Signature

```rust
let is_valid = client.verify_payment_signature(&order_id, &payment_id, &signature).await?;
if is_valid {
    println!("Payment signature is valid");
}
```

## Modules

### Order Management

The `order` module provides functionality for creating and fetching orders:

- `create_order(amount, currency)` - Creates a new order
- `fetch_order(order_id)` - Retrieves an existing order by ID

### Payment Processing

The `payment` module handles payment-related operations:

- `fetch_payment(payment_id)` - Retrieves payment details by ID

### Webhook Handling

The `webhook` module provides structures for parsing and verifying Razorpay webhooks:

- Webhook event parsing for payment events (authorized, captured, failed)
- Type-safe webhook payload handling

## Error Handling

All operations return a `Result` with the crate's custom `Error` type, which includes:

- `ApiError` - Errors from the Razorpay API
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
