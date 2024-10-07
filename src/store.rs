use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;
use thiserror::Error;
use tokio::io;
use crate::schema::items;
use crate::models::Item;
use crate::models::NewItem;

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
    #[error("Failed to read item: {0}")]
    ReadError(String),
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
    pub fn insert(&mut self, body: String) -> Result<Item, Error> {
        let conn = &mut self.pool.get().unwrap();
        let new_item = NewItem{
            body: body,
        };
        
        Ok(conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(items::table)
                .values(&new_item)
                .execute(conn).expect("Error while saving item");
    
            let res = items::table
                .order(items::id.desc())
                .select(Item::as_select())
                .first(conn).expect("Error while loading item");
            Ok(res)
        }).expect("Error while inserting item"))
    }

    pub fn get(&self, id: String) -> Result<String, Error> {
        let conn = &mut self.pool.get().unwrap();
        let parsed_id = id.parse::<i32>().expect("Error parsing id");
        let results = items::table
            .filter(items::id.eq(parsed_id))
            .select(items::body)
            .load::<String>(conn)
            .expect("Error loading item");
        if results.len() < 1 {
            return Err(Error::ReadError("No item found".into()));
        }
        return Ok(results[0].clone())
    }
}
