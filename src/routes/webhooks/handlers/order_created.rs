use crate::services::{email::Mailer, template::Manager};
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
pub async fn order_created(
    Extension(mailer): Extension<Mailer>,
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
