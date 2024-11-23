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

    pub async fn get_client(&self) -> Result<Client, PoolError> {
        self.pool.get().await.map_err(|_| PoolError::FailedToGetClient)
    }
}
