use deadpool_postgres::Client;
use thiserror::Error;
use tokio_postgres::Row;

#[derive(Error, Debug)]
pub enum FindAllEmailTemplatesError {
    #[error("Failed to find all email templates")]
    FailedToFindAllEmailTemplates,
}

pub async fn find_all(db: &Client) -> Result<Vec<Row>, FindAllEmailTemplatesError> {
    let query = "
        SELECT et.content, eet.name 
        FROM email_templates et
        INNER JOIN email_events ee ON et.id = ee.template_id
        INNER JOIN email_events_types eet ON ee.event_type = eet.id
    ";

    let rows = db
        .query(query, &[])
        .await
        .map_err(|_| FindAllEmailTemplatesError::FailedToFindAllEmailTemplates)?;

    Ok(rows)
}
