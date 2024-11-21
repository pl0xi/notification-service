use crate::error::types::QueryError;
use deadpool_postgres::Client;
use tokio_postgres::Row;

pub async fn get_all(db: &Client) -> Result<Vec<Row>, QueryError> {
    let query = "
        SELECT et.content, eet.name 
        FROM email_templates et
        INNER JOIN email_events ee ON et.id = ee.template_id
        INNER JOIN email_events_types eet ON ee.event_type = eet.id
    ";

    let rows = db.query(query, &[]).await.map_err(|_| QueryError::Get("templates"))?;

    Ok(rows)
}

#[allow(unused)]
pub async fn get(client: &Client, name: &str) -> Result<Row, QueryError> {
    let query = "SELECT * FROM email_templates WHERE name = $1";
    let row = client.query_one(query, &[&name]).await.map_err(|_| QueryError::Get("template"))?;

    Ok(row)
}
