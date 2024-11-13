use crate::email::client::EmailClient;
use axum::{extract::Extension, http::StatusCode};

#[allow(unused)]
pub async fn order_created(Extension(state): Extension<EmailClient>) -> StatusCode {
    StatusCode::OK
}
