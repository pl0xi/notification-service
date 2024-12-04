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
