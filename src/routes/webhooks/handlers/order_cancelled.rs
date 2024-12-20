use crate::services::{email::MailerTrait, template::Manager};
use crate::utils::{shopify::webhook_types::Customer, Email};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct CancelledOrderWebhook {
    customer: Customer,
    order_number: String,
}

/// Handles the order cancelled webhook
/// <https://shopify.dev/docs/api/webhooks?reference=toml#list-of-topics-orders/cancelled>
/// # Arguments
/// * `mailer` - The mailer service
/// * `template_manager` - The template manager service
/// * `payload` - The cancelled order webhook payload
/// # Returns
/// * `StatusCode` - The status code of the response
pub async fn order_cancelled<T: MailerTrait>(
    Extension(mailer): Extension<T>,
    Extension(template_manager): Extension<Manager>,
    Json(payload): Json<CancelledOrderWebhook>,
) -> StatusCode {
    let template_filled = match template_manager.get_template_filled("order_cancelled", &payload) {
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
        subject: format!("#{}: Your order has been cancelled", payload.order_number),
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
