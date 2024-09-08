use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;
use thiserror::Error;
use tokio::io;

pub fn establish_conn_pool() -> Result<Pool<ConnectionManager<MysqlConnection>>, Error> {
    dotenv().ok();

    // Check for DATABASE_URL and return a custom error if it's missing
    let database_url =
        env::var("DATABASE_URL").map_err(|_| Error::URLMissing("DATABASE_URL not found".into()))?;

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .map_err(|e| Error::ConnectionError(e.to_string()))
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("DATABASE_URL must be set")]
    URLMissing(String),
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Item {
    id: String,
    message: String,
}

impl Item {
    pub fn new(id: String, message: String) -> Item {
        Item {
            id: id,
            message: message,
        }
    }
    pub fn message(&self) -> String {
        self.message.clone()
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Clone)]
pub struct Store {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Store {
    pub fn new() -> Result<Store, Error> {
        let pool = establish_conn_pool()?;
        return Ok(Store { pool: pool });
    }
    pub fn insert(&mut self, message: String) -> Result<Item, Error> {
        unimplemented!()
    }

    pub fn get(&self, id: String) -> Result<String, Error> {
        unimplemented!()
    }

    pub fn set_user_name(&mut self, username: String) -> Result<String, Error> {
        unimplemented!()
    }
}
