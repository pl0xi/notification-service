use dotenv::dotenv;
use notification_service::app::app;

/// Main application entry point
/// # Panics
/// This function may panic if:
/// - Required environment variables are missing
/// - Database connection fails
/// - TCP listener fails to bind
#[tokio::main]
pub async fn main() {
    dotenv().ok();

    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
