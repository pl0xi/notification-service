use crate::error::types::QueryError;
use deadpool_postgres::Client;
use tokio_postgres::Row;

/// Gets all email templates.
///
/// # Errors
///
/// Returns `QueryError::Get("templates")` if the templates cannot be retrieved.
pub async fn get_all(db: &Client) -> Result<Vec<Row>, QueryError> {
    let query = "
        SELECT et.content, tt.name 
        FROM templates et
        INNER JOIN active_templates at ON et.id = at.template_id
        INNER JOIN template_types tt ON at.template_type_id = tt.id
    ";

    let rows = db.query(query, &[]).await.map_err(|_| QueryError::Get("templates"))?;

    Ok(rows)
}

/// Gets an email template by name.
///
/// # Errors
///
/// Returns `QueryError::Get("template")` if the template cannot be retrieved.
#[allow(unused)]
pub async fn get(client: &Client, name: &str) -> Result<Row, QueryError> {
    let query = "SELECT * FROM templates WHERE name = $1";
    let row = client.query_one(query, &[&name]).await.map_err(|_| QueryError::Get("template"))?;

    Ok(row)
}
