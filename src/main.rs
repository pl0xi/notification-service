mod db;
mod middlewares;
mod routes;

use axum::{middleware, routing::post, Router};
use db::client::DbClient;
use dotenv::dotenv;
use middlewares::verify_shopify_origin;
use routes::webhooks::handlers::order_created;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_client = DbClient::new();

    let app = Router::new()
        .route("/api/order/create", post(order_created))
        .route_layer(middleware::from_fn_with_state(db_client, verify_shopify_origin));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
