use crate::services::db::{client::DbClient, queries::email_template::get_email_template_by_name};
use crate::services::email::client::EmailClient;
use axum::{extract::Extension, http::StatusCode};

pub async fn order_created(Extension(email_client): Extension<EmailClient>, Extension(db_client): Extension<DbClient>) -> StatusCode {
    match db_client.get_client().await {
        Ok(client) => {
            let template = get_email_template_by_name(&client, "order_created").await;
        }
        Err(e) => return StatusCode::INTERNAL_SERVER_ERROR,
    }

    StatusCode::OK
}
