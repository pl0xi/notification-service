use crate::error::types::QueryError;
use deadpool_postgres::Client;
use tokio_postgres::Row;

pub async fn create(client: &Client, event_id: &str) -> Result<(), QueryError> {
    let query = client
        .prepare_cached("INSERT INTO events (event_id) VALUES ($1)")
        .await
        .map_err(|_| QueryError::PrepareStatement)?;

    client.execute(&query, &[&event_id]).await.map_err(|_| QueryError::Insert("event"))?;

    Ok(())
}

pub async fn get(client: &Client, event_id: &str) -> Result<Row, QueryError> {
    let query = client
        .prepare_cached("SELECT * FROM events WHERE event_id = $1")
        .await
        .map_err(|_| QueryError::PrepareStatement)?;

    let row = client.query_one(&query, &[&event_id]).await.map_err(|_| QueryError::Get("event"))?;

    Ok(row)
}
