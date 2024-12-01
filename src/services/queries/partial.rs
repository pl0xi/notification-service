use crate::error::types::QueryError;
use deadpool_postgres::Client;
use tokio_postgres::Row;

/// Gets all email template partials.
///
/// # Errors
///
/// Returns `QueryError::Get("partials")` if the partials cannot be retrieved.
pub async fn get_all(client: &Client) -> Result<Vec<Row>, QueryError> {
    let query = "SELECT * FROM template_partials";
    let rows = client.query(query, &[]).await.map_err(|_| QueryError::Get("partials"))?;

    Ok(rows)
}
