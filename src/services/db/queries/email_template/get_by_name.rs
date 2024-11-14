use deadpool_postgres::Client;
use thiserror::Error;
use tokio_postgres::Row;

#[derive(Debug, Error)]
pub enum GetEmailTemplateError {
    #[error("Failed to get template")]
    FailedToGetTemplate,
}

pub async fn get_email_template_by_name(client: &Client, name: &str) -> Result<Row, GetEmailTemplateError> {
    let query = "SELECT * FROM email_templates WHERE name = $1";
    let row = client
        .query_one(query, &[&name])
        .await
        .map_err(|_| GetEmailTemplateError::FailedToGetTemplate)?;

    Ok(row)
}
