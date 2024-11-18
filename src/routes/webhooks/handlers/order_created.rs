use std::fmt::Debug;

use crate::services::{email::client::Email, email::client::EmailClient, template::client::TemplateClient};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Customer {
    email: String,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderCreatedWebhook {
    customer: Customer,
}

pub async fn order_created(
    Extension(email_client): Extension<EmailClient>,
    Extension(template_client): Extension<TemplateClient>,
    Json(payload): Json<OrderCreatedWebhook>,
) -> StatusCode {
    let template_filled = template_client.get_template_filled("order_created", &payload).unwrap();

    let email = Email {
        to: format!(
            "{} <{}>",
            payload.customer.first_name + " " + &payload.customer.last_name,
            payload.customer.email
        ),
        subject: format!("#{}: We have received your order", "TODO"),
        html_body: template_filled,
    };

    match email_client.send_email(email).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Error sending email: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
