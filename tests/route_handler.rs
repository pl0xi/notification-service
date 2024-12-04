use axum::{body::Body, extract::Request, http::StatusCode, middleware, routing::post, Extension, Router};
use handlebars::Handlebars;
use lettre::Message;
use notification_service::middlewares::verify_shopify_origin;
use notification_service::routes::webhooks::handlers::{order_cancelled, order_created, order_fulfilled};
use notification_service::services::database::Pool;
use notification_service::services::email::{MailerError, MailerTrait};
use notification_service::services::queries::{partial, template};
use notification_service::services::template::Manager;
use notification_service::utils::Email;
use tower::{ServiceBuilder, ServiceExt};

#[derive(Clone)]
pub struct MockMailer {}

#[async_trait::async_trait]
impl MailerTrait for MockMailer {
    #[allow(unused_variables)]
    fn new(smtp_username: String, smtp_password: String, smtp_host: &str, origin_email: String) -> Self {
        Self {}
    }

    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    fn create_mail(&self, email: Email) -> Result<Message, MailerError> {
        Ok(Message::builder()
            .from("mock@test.com".parse().unwrap())
            .to(email.to.parse().unwrap())
            .subject(email.subject)
            .body("Mock email body".to_string())
            .unwrap())
    }

    #[allow(clippy::unused_async, clippy::missing_errors_doc)]
    async fn send_mail(&self, _email: Message) -> Result<(), MailerError> {
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
    #[allow(unused_variables)]
    let mailer = MockMailer::new(String::new(), String::new(), "", String::new());

    let db_client = Pool::new(
        std::env::var("postgres_db").unwrap(),
        std::env::var("postgres_url").unwrap(),
        std::env::var("postgres_user").unwrap(),
        std::env::var("postgres_password").unwrap(),
    );

    let mut templates = Handlebars::new();
    let templates_from_db = template::get_all(&db_client.get_client().await.unwrap()).await.unwrap();
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
        .route("/api/order/create", post(order_created::<MockMailer>))
        .route("/api/order/cancel", post(order_cancelled::<MockMailer>))
        .route("/api/order/fulfilled", post(order_fulfilled::<MockMailer>))
        .layer(ServiceBuilder::new().layer(Extension(mailer)).layer(Extension(template_manager)))
        .route_layer(middleware::from_fn_with_state(db_client, verify_shopify_origin)))
}

mod tests {
    use super::*;
    use axum::http::request::Builder;
    static mut SHOPIFY_EVENT_ID: u32 = 0;

    lazy_static::lazy_static! {
        static ref SHOPIFY_SHOP_URL: String = std::env::var("shopify_shop_url").unwrap();
        static ref SHOPIFY_WEBHOOK_SECRET: String = std::env::var("shopify_webhook_secret").unwrap();
        static ref SHOPIFY_API_VERSION: String = std::env::var("shopify_api_version").unwrap();
    }

    fn create_request_builder() -> Builder {
        // Unsafe because possible overflow (Will prob never happen, in this case)
        unsafe {
            SHOPIFY_EVENT_ID += 1;
        }

        Request::builder()
            .method("POST")
            .header("X-Shopify-Topic", "orders/create")
            .header("X-Shopify-Webhook-Id", "1234567890")
            .header("X-Shopify-Event-Id", unsafe { SHOPIFY_EVENT_ID })
            .header("X-Shopify-Shop-Domain", SHOPIFY_SHOP_URL.to_string())
            .header("X-Shopify-Hmac-Sha256", SHOPIFY_WEBHOOK_SECRET.to_string())
            .header("X-Shopify-Api-Version", SHOPIFY_API_VERSION.to_string())
            .header("Content-Type", "application/json")
    }

    #[tokio::test]
    async fn test_routes_no_headers() {
        let app = setup_app().await.unwrap();

        let create_order_response = app
            .clone()
            .oneshot(Request::builder().uri("/api/order/create").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let cancelled_order_response = app
            .oneshot(Request::builder().uri("/api/order/cancel").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(create_order_response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(cancelled_order_response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_routes_invalid_hmac_sha256() {
        // TODO: Add order cancelled test
        let app = setup_app().await.unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("X-Shopify-Topic", "orders/create")
                    .header("X-Shopify-Webhook-Id", "1234567890")
                    .header("X-Shopify-Event-Id", unsafe { SHOPIFY_EVENT_ID })
                    .header("X-Shopify-Shop-Domain", SHOPIFY_SHOP_URL.to_string())
                    .header("X-Shopify-Hmac-Sha256", "invalid")
                    .header("X-Shopify-Api-Version", SHOPIFY_API_VERSION.to_string())
                    .header("Content-Type", "application/json")
                    .uri("/api/order/create")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[ignore]
    // TODO: Implement null body check
    async fn test_routes_no_body() {
        let app = setup_app().await.unwrap();

        let create_order_response = app
            .clone()
            .oneshot(create_request_builder().uri("/api/order/create").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let cancelled_order_response = app
            .oneshot(create_request_builder().uri("/api/order/cancel").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(create_order_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(cancelled_order_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_order_route() {
        let app = setup_app().await.unwrap();
        let json_body = serde_json::json!({
            "order_number": "1234567890",
            "customer": {
                "email": "test@test.com",
                "first_name": "John",
                "last_name": "Doe"
            }
        });

        let request = create_request_builder()
            .uri("/api/order/create")
            .body(Body::from(json_body.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        println!("{response:?}");
    }
}
