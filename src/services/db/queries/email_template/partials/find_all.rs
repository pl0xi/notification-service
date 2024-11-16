use deadpool_postgres::Client;
use thiserror::Error;
use tokio_postgres::Row;

#[derive(Error, Debug)]
pub enum FindAllPartialsError {
    #[error("Failed to find partials")]
    FailedToFindPartials,
}

pub async fn find_all_partials(client: &Client) -> Result<Vec<Row>, FindAllPartialsError> {
    let query = "SELECT * FROM email_template_partials";
    let rows = client.query(query, &[]).await.map_err(|_| FindAllPartialsError::FailedToFindPartials)?;

    Ok(rows)
}
