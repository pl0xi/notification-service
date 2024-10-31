mod routes;

use axum::{routing::get, Router};
use routes::webhooks::subscriptions::shopify::{initiate_shopify_subscriptions, Status};

#[tokio::main]
async fn main() {
    if initiate_shopify_subscriptions() == Status::Active {
        println!("Shopify webhook is active");
    } else {
        println!("Shopify webhook failed to activate");
    }
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
