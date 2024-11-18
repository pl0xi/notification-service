mod middlewares;
mod routes;
mod services;

use axum::{middleware, routing::post, Extension, Router};
use dotenv::dotenv;
use handlebars::Handlebars;
use middlewares::verify_shopify_origin;
use routes::webhooks::handlers::order_created;
use services::db::queries::email_template::{find_all, partials::find_all_partials};
use services::{db::client::DbClient, email::client::EmailClient, template::client::TemplateClient};
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
        env::var("origin_email").unwrap(),
    );

    // Get templates from database and persist in memory with the template client
    let mut templates = Handlebars::new();
    let templates_from_db = find_all(&db_client.get_client().await.unwrap()).await.unwrap();
    for template in templates_from_db {
        let name: &str = template.get("name");
        let content: &str = template.get("content");

        if templates.register_template_string(name, content).is_ok() {
            println!("Template registered and ready: {}", name);
        } else {
            println!("Error registering template: {}", name);
        }
    }

    // Get partials from database and persist in memory with the template client
    let partials_templates_from_db = find_all_partials(&db_client.get_client().await.unwrap()).await.unwrap();
    for partial in partials_templates_from_db {
        let name: &str = partial.get("name");
        let content: &str = partial.get("content");

        if templates.register_partial(name, content).is_ok() {
            println!("Partial template registered and ready: {}", name);
        } else {
            println!("Error registering partial template: {}", name);
        }
    }

    // Create a template client
    let template_client = TemplateClient::new(templates);

    // Create the app
    let app = Router::new()
        .route("/api/order/create", post(order_created))
        .layer(ServiceBuilder::new().layer(Extension(email_client)).layer(Extension(template_client)))
        .route_layer(middleware::from_fn_with_state(db_client, verify_shopify_origin));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
