use deadpool_postgres::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateEventError {
    #[error("Failed to create event")]
    FailedToCreateEvent,

    #[error("Failed to prepare statement")]
    FailedToPrepareStatement,
}

pub async fn create_event(client: &Client, event_id: &str) -> Result<(), CreateEventError> {
    let query = client
        .prepare_cached("INSERT INTO events (event_id) VALUES ($1)")
        .await
        .map_err(|_| CreateEventError::FailedToPrepareStatement)?;

    client
        .execute(&query, &[&event_id])
        .await
        .map_err(|_| CreateEventError::FailedToCreateEvent)?;

    Ok(())
}
