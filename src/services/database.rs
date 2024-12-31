use deadpool_postgres::{Client, Config, Runtime};
use thiserror::Error;
use tokio_postgres::NoTls;

#[derive(Error, Debug, PartialEq)]
pub enum PoolError {
    #[error("Failed to get client from pool")]
    FailedToGetClient,
}

#[derive(Clone)]
pub struct Pool {
    pool: deadpool_postgres::Pool,
}

impl Pool {
    /// Creates a new database connection pool.
    ///
    /// # Panics
    ///
    /// Panics if the pool creation fails.
    #[must_use]
    pub fn new(db_name: String, db_url: String, db_user: String, db_password: String) -> Self {
        let mut setup_config = Config::new();
        setup_config.dbname = Some(db_name);
        setup_config.url = Some(db_url);
        setup_config.user = Some(db_user);
        setup_config.password = Some(db_password);

        Self {
            pool: setup_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap(),
        }
    }

    /// Gets a client from the pool.
    ///
    /// # Errors
    ///
    /// Returns `PoolError::FailedToGetClient` if the client cannot be retrieved from the pool.
    pub async fn get_client(&self) -> Result<Client, PoolError> {
        self.pool.get().await.map_err(|_| PoolError::FailedToGetClient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_client_error() {
        let pool = Pool::new(
            "invalid_db".to_string(),
            "postgres://invalid".to_string(),
            "invalid_user".to_string(),
            "invalid_pass".to_string(),
        );

        let result = pool.get_client().await;
        assert!(matches!(result, Err(PoolError::FailedToGetClient)));
    }
}
