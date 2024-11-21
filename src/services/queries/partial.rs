use crate::error::types::QueryError;
use deadpool_postgres::Client;
use tokio_postgres::Row;

pub async fn get_all(client: &Client) -> Result<Vec<Row>, QueryError> {
    let query = "SELECT * FROM email_template_partials";
    let rows = client.query(query, &[]).await.map_err(|_| QueryError::Get("partials"))?;

    Ok(rows)
}
