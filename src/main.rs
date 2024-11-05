mod middlewares;
mod routes;

use axum::{middleware, routing::post, Router};
use dotenv::dotenv;
use middlewares::verify_shopify_origin;
use routes::webhooks::handlers::order_created;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/api/order/create", post(order_created))
        .layer(middleware::from_fn(verify_shopify_origin));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
