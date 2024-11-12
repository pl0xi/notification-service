use deadpool_postgres::Client;
use thiserror::Error;
use tokio_postgres::Row;

#[derive(Debug, Error)]
pub enum GetEventError {
    #[error("Failed to get event")]
    FailedToGetEvent,
}

pub async fn get_event(client: &Client, event_id: &str) -> Result<Row, GetEventError> {
    let query = "SELECT * FROM events WHERE event_id = $1";
    let row = client.query_one(query, &[&event_id]).await.map_err(|_| GetEventError::FailedToGetEvent)?;

    Ok(row)
}
