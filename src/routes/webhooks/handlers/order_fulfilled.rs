use crate::services::{document::create_pdf, email::MailerTrait, template::Manager};
use crate::utils::{shopify::webhook_types::Customer, Email};
use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct FulfilledOrderWebhook {
    order_number: String,
    customer: Customer,
}

/// Handles the order fulfilled webhook
/// <https://shopify.dev/docs/api/webhooks?reference=toml#list-of-topics-orders/fulfilled>
/// # Returns
/// * `StatusCode` - The status code of the response
pub async fn order_fulfilled<T: MailerTrait>(
    Extension(mailer): Extension<T>,
    Extension(template_manager): Extension<Manager>,
    Json(payload): Json<FulfilledOrderWebhook>,
) -> StatusCode {
    let Ok(template_filled_invoice) = template_manager.get_template_filled("invoice", &payload) else {
        println!("Error getting template filled invoice for order {}", payload.order_number);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    let Ok(invoice) = create_pdf(&template_filled_invoice, "invoice") else {
        println!("Error creating PDF invoice for order {}", payload.order_number);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let Ok(template_filled_mail_content) = template_manager.get_template_filled("order_fulfilled", &payload) else {
        println!("Error getting template filled mail content for order {}", payload.order_number);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let email = Email {
        to: payload.customer.email,
        subject: "Order Fulfilled".to_string(),
        html_body: template_filled_mail_content,
        attachment: Some(invoice),
    };

    let mail = match mailer.create_mail(email) {
        Ok(mail) => mail,
        Err(e) => {
            println!("Error creating mail: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    match mailer.send_mail(mail).await {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::email::MailerError;
    use handlebars::Handlebars;
    use lettre::Message;

    #[derive(Clone)]
    struct MockMailer {
        should_fail_create: bool,
        should_fail_send: bool,
    }

    #[async_trait::async_trait]
    impl MailerTrait for MockMailer {
        fn new(_: String, _: String, _: &str, _: String, _: u16) -> Self {
            Self {
                should_fail_create: false,
                should_fail_send: false,
            }
        }

        fn create_mail(&self, email: Email) -> Result<Message, MailerError> {
            if self.should_fail_create {
                return Err(MailerError::BuildEmailError);
            }
            Ok(Message::builder()
                .from("test@test.com".parse().unwrap())
                .to(email.to.parse().unwrap())
                .subject(email.subject)
                .body("Test body".to_string())
                .unwrap())
        }

        async fn send_mail(&self, _: Message) -> Result<(), MailerError> {
            if self.should_fail_send {
                return Err(MailerError::SmtpSendError);
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_order_fulfilled_success() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: false,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("invoice", "Test invoice template").unwrap();
        handlebars.register_template_string("order_fulfilled", "Test email template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = FulfilledOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_fulfilled(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_order_fulfilled_invoice_template_error() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: false,
        };
        let handlebars = Handlebars::new(); // No template registered
        let template_manager = Manager::new(handlebars);

        let payload = FulfilledOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_fulfilled(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_order_fulfilled_email_template_error() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: false,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("invoice", "Test invoice template").unwrap();
        // Not registering order_fulfilled template
        let template_manager = Manager::new(handlebars);

        let payload = FulfilledOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_fulfilled(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_order_fulfilled_mail_creation_error() {
        let mailer = MockMailer {
            should_fail_create: true,
            should_fail_send: false,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("invoice", "Test invoice template").unwrap();
        handlebars.register_template_string("order_fulfilled", "Test email template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = FulfilledOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_fulfilled(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_order_fulfilled_mail_send_error() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: true,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("invoice", "Test invoice template").unwrap();
        handlebars.register_template_string("order_fulfilled", "Test email template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = FulfilledOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_fulfilled(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_order_fulfilled_pdf_creation_error() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: false,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("invoice", "<h1>Test invoice template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = FulfilledOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_fulfilled(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
