use axum::{middleware, routing::post, Extension, Router};
use dotenv::dotenv;
use handlebars::Handlebars;
use notification_service::{
    middlewares::verify_shopify_origin,
    routes::webhooks::handlers::{order_cancelled, order_created},
    services::{
        database::Pool,
        email::Mailer,
        queries::{email_template, partial},
        template::Manager,
    },
};
use std::env;
use tower::ServiceBuilder;

/// Main application entry point
/// # Panics
/// This function may panic if:
/// - Required environment variables are missing
/// - Database connection fails
/// - TCP listener fails to bind
#[tokio::main]
pub async fn main() {
    dotenv().ok();

    // Create a database client
    let db_client = Pool::new(
        env::var("postgres_db").unwrap(),
        env::var("postgres_url").unwrap(),
        env::var("postgres_user").unwrap(),
        env::var("postgres_password").unwrap(),
    );

    // Create an email client
    let mailer = Mailer::new(
        env::var("smtp_username").unwrap(),
        env::var("smtp_password").unwrap(),
        env::var("smtp_host").unwrap().as_str(),
        env::var("origin_email").unwrap(),
    );

    // Get templates from database and persist in memory with the template client
    let mut templates = Handlebars::new();
    let templates_from_db = email_template::get_all(&db_client.get_client().await.unwrap()).await.unwrap();
    for template in templates_from_db {
        let name: &str = template.get("name");
        let content: &str = template.get("content");

        if templates.register_template_string(name, content).is_ok() {
            println!("Template registered and ready: {name}");
        } else {
            println!("Error registering template: {name}");
        }
    }

    // Get partials from database and persist in memory with the template client
    let partials_templates_from_db = partial::get_all(&db_client.get_client().await.unwrap()).await.unwrap();
    for partial in partials_templates_from_db {
        let name: &str = partial.get("name");
        let content: &str = partial.get("content");

        if templates.register_partial(name, content).is_ok() {
            println!("Partial template registered and ready: {name}");
        } else {
            println!("Error registering partial template: {name}");
        }
    }

    // Create a template client
    let template_manager = Manager::new(templates);

    // Create the app
    let app = Router::new()
        .route("/api/order/create", post(order_created))
        .route("/api/order/cancel", post(order_cancelled))
        .layer(ServiceBuilder::new().layer(Extension(mailer)).layer(Extension(template_manager)))
        .route_layer(middleware::from_fn_with_state(db_client, verify_shopify_origin));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
