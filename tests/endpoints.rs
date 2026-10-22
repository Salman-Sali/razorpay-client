//! Runs each endpoint against a one-shot local server and checks the request it sends.

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread::{self, JoinHandle},
};

use razorpay_client::{
    RazorpayClient, collection::ListOptions, currency::Currency, error::Error,
    order::create_order::CreateOrderRequest, payment_link::update_payment_link::NotifyMedium,
    refund::create_refund::CreateRefundRequest,
};

struct Captured {
    request_line: String,
    body: String,
}

/// Serves one request with `status` and `body`, returning the client and the captured request.
fn serve(status: u16, body: serde_json::Value) -> (RazorpayClient, JoinHandle<Captured>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let mut content_length = 0;
        loop {
            let mut header = String::new();
            reader.read_line(&mut header).unwrap();
            if header == "\r\n" {
                break;
            }
            if let Some((name, value)) = header.split_once(':')
                && name.eq_ignore_ascii_case("content-length")
            {
                content_length = value.trim().parse().unwrap();
            }
        }
        let mut request_body = vec![0; content_length];
        reader.read_exact(&mut request_body).unwrap();

        let body = body.to_string();
        write!(
            stream,
            "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();

        Captured {
            request_line: request_line.trim().to_string(),
            body: String::from_utf8(request_body).unwrap(),
        }
    });

    let client = RazorpayClient::new("key".into(), "secret".into(), "webhook".into())
        .unwrap()
        .with_url(&url)
        .unwrap();

    (client, handle)
}

fn payment() -> serde_json::Value {
    serde_json::json!({
        "id": "pay_1", "entity": "payment", "amount": 1000, "currency": "INR",
        "status": "captured", "order_id": "order_1", "international": false, "method": "upi",
        "amount_refunded": 0, "captured": true, "notes": { "k": "v" }
    })
}

fn refund() -> serde_json::Value {
    serde_json::json!({
        "id": "rfnd_1", "entity": "refund", "amount": 500, "currency": "INR",
        "payment_id": "pay_1", "notes": [], "status": "pending"
    })
}

fn collection(item: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "entity": "collection", "count": 1, "items": [item] })
}

#[tokio::test]
async fn create_order() {
    let (client, server) = serve(
        200,
        serde_json::json!({ "id": "order_1", "amount": 1000, "currency": "INR", "status": "created" }),
    );
    let mut request = CreateOrderRequest::new(1000, Currency::INR);
    request.with_receipt("r1");

    let order = client.create_order(request).await.unwrap();
    let captured = server.join().unwrap();

    assert_eq!(order.id, "order_1");
    assert_eq!(captured.request_line, "POST /v1/orders HTTP/1.1");
    assert!(captured.body.contains(r#""receipt":"r1""#));
}

#[tokio::test]
async fn fetch_orders_sends_list_options() {
    let (client, server) = serve(
        200,
        serde_json::json!({ "entity": "collection", "count": 0, "items": [] }),
    );
    let mut options = ListOptions::new();
    options.with_count(250);
    options.with_skip(10);

    let orders = client.fetch_orders(&options).await.unwrap();

    assert_eq!(orders.count, 0);
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/orders?count=100&skip=10 HTTP/1.1"
    );
}

#[tokio::test]
async fn fetch_order_payments() {
    let (client, server) = serve(200, collection(payment()));

    let payments = client.fetch_order_payments("order_1").await.unwrap();

    assert_eq!(payments.items[0].id, "pay_1");
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/orders/order_1/payments HTTP/1.1"
    );
}

#[tokio::test]
async fn fetch_payment_accepts_str_and_string() {
    let (client, server) = serve(200, payment());
    client.fetch_payment("pay_1").await.unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/payments/pay_1 HTTP/1.1"
    );

    let (client, server) = serve(200, payment());
    client.fetch_payment("pay_1".to_string()).await.unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/payments/pay_1 HTTP/1.1"
    );
}

#[tokio::test]
async fn fetch_payments() {
    let (client, server) = serve(200, collection(payment()));

    client.fetch_payments(&ListOptions::new()).await.unwrap();

    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/payments HTTP/1.1"
    );
}

#[tokio::test]
async fn capture_payment() {
    let (client, server) = serve(200, payment());

    client
        .capture_payment("pay_1", 1000, Currency::INR)
        .await
        .unwrap();
    let captured = server.join().unwrap();

    assert_eq!(
        captured.request_line,
        "POST /v1/payments/pay_1/capture HTTP/1.1"
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&captured.body).unwrap(),
        serde_json::json!({ "amount": 1000, "currency": "INR" })
    );
}

#[tokio::test]
async fn update_payment() {
    let (client, server) = serve(200, payment());

    let payment = client
        .update_payment("pay_1", HashMap::from([("k".to_string(), "v".to_string())]))
        .await
        .unwrap();
    let captured = server.join().unwrap();

    assert_eq!(payment.notes["k"], "v");
    assert_eq!(captured.request_line, "PATCH /v1/payments/pay_1 HTTP/1.1");
    assert_eq!(captured.body, r#"{"notes":{"k":"v"}}"#);
}

#[tokio::test]
async fn create_refund() {
    let (client, server) = serve(200, refund());

    client
        .create_refund("pay_1", CreateRefundRequest::new(500))
        .await
        .unwrap();

    assert_eq!(
        server.join().unwrap().request_line,
        "POST /v1/payments/pay_1/refund HTTP/1.1"
    );
}

#[tokio::test]
async fn refund_reads() {
    let (client, server) = serve(200, refund());
    client.fetch_refund("rfnd_1").await.unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/refunds/rfnd_1 HTTP/1.1"
    );

    let (client, server) = serve(200, collection(refund()));
    client.fetch_refunds(&ListOptions::new()).await.unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/refunds HTTP/1.1"
    );

    let (client, server) = serve(200, collection(refund()));
    client
        .fetch_payment_refunds("pay_1", &ListOptions::new())
        .await
        .unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/payments/pay_1/refunds HTTP/1.1"
    );

    let (client, server) = serve(200, refund());
    client
        .fetch_payment_refund("pay_1", "rfnd_1")
        .await
        .unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/payments/pay_1/refunds/rfnd_1 HTTP/1.1"
    );

    let (client, server) = serve(200, refund());
    client
        .update_refund("rfnd_1", HashMap::new())
        .await
        .unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "PATCH /v1/refunds/rfnd_1 HTTP/1.1"
    );
}

#[tokio::test]
async fn customers() {
    let customer =
        serde_json::json!({ "id": "cust_1", "entity": "customer", "name": "A", "notes": [] });

    let (client, server) = serve(200, customer.clone());
    client
        .create_customer(
            razorpay_client::customer::create_customer::CreateCustomerRequest::new("A"),
        )
        .await
        .unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "POST /v1/customers HTTP/1.1"
    );

    let (client, server) = serve(200, customer.clone());
    client
        .edit_customer("cust_1", Default::default())
        .await
        .unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "PUT /v1/customers/cust_1 HTTP/1.1"
    );

    let (client, server) = serve(200, collection(customer));
    client.fetch_customers(&ListOptions::new()).await.unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/customers HTTP/1.1"
    );
}

#[tokio::test]
async fn payment_links() {
    let link = serde_json::json!({
        "id": "plink_1", "amount": 1000, "currency": "INR", "short_url": "https://rzp.io/i/a",
        "status": "cancelled", "notes": null
    });

    let (client, server) = serve(200, link.clone());
    client.cancel_payment_link("plink_1").await.unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "POST /v1/payment_links/plink_1/cancel HTTP/1.1"
    );

    let (client, server) = serve(200, serde_json::json!({ "payment_links": [link] }));
    let mut options =
        razorpay_client::payment_link::fetch_payment_link::FetchPaymentLinksOptions::new();
    options.with_reference_id("ref 1");
    let links = client.fetch_payment_links(&options).await.unwrap();
    assert_eq!(links.len(), 1);
    assert_eq!(
        server.join().unwrap().request_line,
        "GET /v1/payment_links?reference_id=ref+1 HTTP/1.1"
    );

    let (client, server) = serve(200, serde_json::json!({ "success": true }));
    client
        .notify_payment_link("plink_1", NotifyMedium::Sms)
        .await
        .unwrap();
    assert_eq!(
        server.join().unwrap().request_line,
        "POST /v1/payment_links/plink_1/notify_by/sms HTTP/1.1"
    );
}

#[tokio::test]
async fn api_errors_are_parsed() {
    let (client, server) = serve(
        400,
        serde_json::json!({ "error": {
            "code": "BAD_REQUEST_ERROR",
            "description": "The id provided does not exist",
            "source": "business", "step": "payment_initiation",
            "reason": "input_validation_failed", "metadata": {}
        } }),
    );

    let error = client.fetch_order("order_missing").await.unwrap_err();
    server.join().unwrap();

    let Error::Razorpay {
        status,
        context,
        error,
    } = error
    else {
        panic!("expected Razorpay error, got {error:?}");
    };
    assert_eq!(status, 400);
    assert_eq!(context, "Order_id: order_missing");
    assert_eq!(error.code, "BAD_REQUEST_ERROR");
}
