use axum::http::StatusCode;

pub async fn order_created() -> StatusCode {
    StatusCode::OK
}
