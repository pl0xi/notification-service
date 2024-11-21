use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Failed to get {0}")]
    Get(&'static str),

    #[error("Failed to insert {0}")]
    Insert(&'static str),

    #[error("Failed to prepare statement")]
    PrepareStatement,
}
