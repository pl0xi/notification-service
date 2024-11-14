use deadpool_postgres::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateEventError {
    #[error("Failed to create event")]
    FailedToCreateEvent,
}

pub async fn create_event(client: &Client, event_id: &str) -> Result<(), CreateEventError> {
    let sql = "INSERT INTO events (event_id) VALUES ($1)";
    client
        .execute(sql, &[&event_id])
        .await
        .map_err(|_| CreateEventError::FailedToCreateEvent)?;

    Ok(())
}
