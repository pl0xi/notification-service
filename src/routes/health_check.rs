use axum::http::StatusCode;

// Used to check if axum has started
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
