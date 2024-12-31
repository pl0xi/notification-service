use crate::services::{email::MailerTrait, template::Manager};
use crate::utils::{shopify::webhook_types::Customer, Email};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatedOrderWebhook {
    customer: Customer,
    order_number: String,
}

/// Handles the order created webhook
/// <https://shopify.dev/docs/api/webhooks?reference=toml#list-of-topics-orders/create>
/// # Arguments
/// * `mailer` - The mailer service
/// * `template_manager` - The template manager service
/// * `payload` - The created order webhook payload
/// # Returns
/// * `StatusCode` - The status code of the response
pub async fn order_created<T: MailerTrait>(
    Extension(mailer): Extension<T>,
    Extension(template_manager): Extension<Manager>,
    Json(payload): Json<CreatedOrderWebhook>,
) -> StatusCode {
    let template_filled = match template_manager.get_template_filled("order_created", &payload) {
        Ok(template_filled) => template_filled,
        Err(e) => {
            println!("Error getting template: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let email = Email {
        to: format!(
            "{} <{}>",
            payload.customer.first_name + " " + &payload.customer.last_name,
            payload.customer.email
        ),
        subject: format!("#{}: We have received your order", payload.order_number),
        html_body: template_filled,
        attachment: None,
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
        Err(e) => {
            println!("Error sending email: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
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
    async fn test_order_created_success() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: false,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("order_created", "Test template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = CreatedOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_created(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_order_created_template_error() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: false,
        };
        let handlebars = Handlebars::new(); // No template registered
        let template_manager = Manager::new(handlebars);

        let payload = CreatedOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_created(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_order_created_mail_creation_error() {
        let mailer = MockMailer {
            should_fail_create: true,
            should_fail_send: false,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("order_created", "Test template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = CreatedOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_created(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_order_created_mail_send_error() {
        let mailer = MockMailer {
            should_fail_create: false,
            should_fail_send: true,
        };
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("order_created", "Test template").unwrap();
        let template_manager = Manager::new(handlebars);

        let payload = CreatedOrderWebhook {
            customer: Customer {
                email: "test@test.com".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
            },
            order_number: "1234".to_string(),
        };

        let result = order_created(Extension(mailer), Extension(template_manager), Json(payload)).await;

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
