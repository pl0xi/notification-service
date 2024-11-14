use deadpool_postgres::{Client, Config, Pool, Runtime};
use std::env;
use thiserror::Error;
use tokio_postgres::NoTls;

#[derive(Error, Debug, PartialEq)]
pub enum DbClientError {
    #[error("Failed to get client from pool")]
    FailedToGetClient,
}

#[derive(Clone)]
pub struct DbClient {
    pool: Pool,
}

impl DbClient {
    pub fn new() -> Self {
        let mut setup_config = Config::new();
        setup_config.dbname = Some(env::var("postgres_db").unwrap());
        setup_config.url = Some(env::var("postgres_url").unwrap());
        setup_config.user = Some(env::var("postgres_user").unwrap());
        setup_config.password = Some(env::var("postgres_password").unwrap());
        Self {
            pool: setup_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap(),
        }
    }

    pub async fn get_client(&self) -> Result<Client, DbClientError> {
        self.pool.get().await.map_err(|_| DbClientError::FailedToGetClient)
    }
}
