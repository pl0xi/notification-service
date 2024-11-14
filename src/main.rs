mod middlewares;
mod routes;
mod services;

use axum::{middleware, routing::post, Extension, Router};
use dotenv::dotenv;
use middlewares::verify_shopify_origin;
use routes::webhooks::handlers::order_created;
use services::db::client::DbClient;
use services::email::client::EmailClient;
use std::env;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Create a database client
    let db_client = DbClient::new();

    // Create an email client
    let email_client = EmailClient::new(
        env::var("smtp_username").unwrap(),
        env::var("smtp_password").unwrap(),
        env::var("smtp_host").unwrap().as_str(),
    );

    // Create the app
    let app = Router::new()
        .route("/api/order/create", post(order_created))
        .layer(ServiceBuilder::new().layer(Extension(email_client)).layer(Extension(db_client.clone())))
        .route_layer(middleware::from_fn_with_state(db_client.clone(), verify_shopify_origin));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
