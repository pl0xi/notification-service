use axum::{body::Body, extract::Request, http::StatusCode, middleware, routing::post, Extension, Router};
use handlebars::Handlebars;
use notification_service::middlewares::verify_shopify_origin;
use notification_service::routes::webhooks::handlers::{order_cancelled, order_created};
use notification_service::services::database::Pool;
use notification_service::services::email::MailerError;
use notification_service::services::queries::{email_template, partial};
use notification_service::services::template::Manager;
use tower::{ServiceBuilder, ServiceExt};

#[derive(Clone)]
pub struct MockMailer {}

impl MockMailer {
    /// Simulates sending an email to a recipient
    ///
    /// # Errors
    ///
    /// Currently this mock implementation never returns an error
    pub fn send_email(&self, recipient_email: String, subject: String, body: String) -> Result<(), MailerError> {
        let _ = (recipient_email, subject, body);
        Ok(())
    }
}

/// Setup the app for testing
///
/// # Returns
///
/// A `Router` instance that can be used to test the routes
///
/// # Errors
///
/// This function will return an error if the database connection fails
///
/// # Panics
///
/// This function will panic if the template registration fails
pub async fn setup_app() -> Result<Router, Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.test").ok();
    let mailer = MockMailer {};

    let db_client = Pool::new(
        std::env::var("postgres_db").unwrap(),
        std::env::var("postgres_url").unwrap(),
        std::env::var("postgres_user").unwrap(),
        std::env::var("postgres_password").unwrap(),
    );

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

    Ok(Router::new()
        .route("/api/order/create", post(order_created))
        .route("/api/order/cancel", post(order_cancelled))
        .layer(ServiceBuilder::new().layer(Extension(mailer)).layer(Extension(template_manager)))
        .route_layer(middleware::from_fn_with_state(db_client, verify_shopify_origin)))
}

#[tokio::test]
async fn test_order_no_headers() {
    let app = setup_app().await.unwrap();
    let response = app
        .oneshot(Request::builder().uri("/api/order/create").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
