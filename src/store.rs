use tokio::io;
use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
pub enum Error {
    Database(mysql::Error),
    Url(mysql::UrlError),
}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Self {
        Self::Database(err)
    }
}

impl From<mysql::UrlError> for Error {
    fn from(err: mysql::UrlError) -> Self {
        Self::Url(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Database(ref err) => {
                write!(f, "Database error: {}", err)
            }
            Self::Url(ref err) => write!(f, "Databse URL error: {}", err),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Item {
    id: String,
    message: String,
}

impl Item {
    pub fn new(id: String, message: String) -> Item {
        Item {id: id, message: message}
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
    conn_pool: mysql::Pool,
}

impl Store {
    pub fn new(db: String) -> Result<Store, Error> {
        let opts = Opts::from_url(&db)?;
        let pool = Pool::new(opts)?;
        return Ok(Store {
            conn_pool: pool
        })
    }
    pub fn insert(&mut self, message: String) -> Result<Item, Error>{
        unimplemented!()
    }

    pub fn get(&self, id: String) -> Result<String, Error>{
        unimplemented!()
    }

    pub fn set_user_name(&mut self, username: String) -> Result<String, Error> {
        unimplemented!()
    }
}