mod routes;

use axum::{routing::post, Router};
use routes::webhooks::handlers::order_created::order_created;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/order/create", post(order_created));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
