use axum::http::HeaderMap;
use notification_service::app::app;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use testcontainers_modules::{
    postgres,
    testcontainers::{core::IntoContainerPort, runners::AsyncRunner, GenericImage, ImageExt},
};

// Must run with e2e custom profile
#[tokio::test]
async fn test_e2e() {
    // Start postgres container
    let db_container = postgres::Postgres::default()
        .with_init_sql(include_str!("../../db/core.sql").to_string().into_bytes())
        .with_tag("17.0-alpine")
        .start()
        .await
        .unwrap();
    let db_host_port = db_container.get_host_port_ipv4(5432).await.unwrap();

    // Setup fake SMTP server
    let smtp_container = GenericImage::new("axllent/mailpit", "v1.21.8")
        .with_exposed_port(1025.tcp())
        .with_exposed_port(8025.tcp())
        .start()
        .await
        .unwrap();

    let smtp_host_port = smtp_container.get_host_port_ipv4(1025).await.unwrap();
    let smtp_web_port = smtp_container.get_host_port_ipv4(8025).await.unwrap();
    let shopify_shop_url = "test.myshopify.com";
    let shopify_api_version = "2024-10";
    let shopify_webhook_secret = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2";

    env::set_var("postgres_url", format!("postgres://postgres:postgres@localhost:{db_host_port}"));
    env::set_var("postgres_user", "postgres");
    env::set_var("postgres_password", "postgres");
    env::set_var("postgres_db", "postgres");
    env::set_var("shopify_shop_url", shopify_shop_url);
    env::set_var("shopify_webhook_secret", shopify_webhook_secret);
    env::set_var("shopify_api_version", shopify_api_version);
    env::set_var("smtp_username", "user");
    env::set_var("smtp_password", "password");
    env::set_var("smtp_host", "localhost");
    env::set_var("smtp_port", smtp_host_port.to_string());
    env::set_var("origin_email", "Notifcation Service <noreply@test.com>");

    tokio::spawn(async move {
        let app = app().await;

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        eprintln!("Server is listening on port 3000");
        axum::serve(listener, app).await.unwrap();
    });

    let app_address = "localhost:3000";
    let client = reqwest::Client::new();

    wait_for_server_start(app_address).await;

    let mut headers = HeaderMap::new();
    headers.insert("X-Shopify-Topic", "orders/create".parse().unwrap());
    headers.insert("X-Shopify-Webhook-Id", "1234567890".parse().unwrap());
    headers.insert("X-Shopify-Event-Id", "1234567890".parse().unwrap());
    headers.insert("X-Shopify-Shop-Domain", shopify_shop_url.parse().unwrap());
    headers.insert("X-Shopify-Hmac-Sha256", shopify_webhook_secret.parse().unwrap());
    headers.insert("X-Shopify-Api-Version", shopify_api_version.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    create_order_route_test(client, &app_address, headers.clone(), smtp_web_port).await;
}

async fn wait_for_server_start(address: &str) {
    let client = reqwest::Client::new();
    let mut response = client.get(format!("http://{address}/health")).send().await;

    while response.is_err() {
        let err = response.unwrap_err();
        let err_message = err.to_string();

        assert!(
            !err_message.contains("tcp connect error"),
            "Unexpected error while waiting for server: {err:?}"
        );

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        response = client.get(format!("http://{address}/health")).send().await;
    }
}

// Tests
async fn create_order_route_test(client: Client, address: &str, headers: HeaderMap, smtp_host_port: u16) {
    let body = r#"{"order_number": "1", "customer": {"email": "test@test.com", "first_name": "John", "last_name": "Doe"}}"#;

    let response = client
        .post(format!("http://{address}/api/order/create"))
        .headers(headers)
        .body(body)
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success(), "Response status is not success {response:?}");

    check_email_sent(client, "%231: We have received your order", smtp_host_port).await;
}

#[allow(non_snake_case, dead_code)]
#[derive(Serialize, Deserialize)]
struct Message {
    ID: String,
    MessageID: String,
}

#[derive(Serialize, Deserialize)]
struct EmailSearchResponse {
    messages: Vec<Message>,
}

// Function for checking email sent
async fn check_email_sent(client: Client, subject: &str, smtp_port: u16) {
    let response = client
        .get(format!("http://localhost:{smtp_port}/api/v1/search?query=subject:\"{subject}\""))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success(), "Response status is not success {response:?}");
    assert!(
        !response.json::<EmailSearchResponse>().await.unwrap().messages.is_empty(),
        "No emails found"
    );
}
