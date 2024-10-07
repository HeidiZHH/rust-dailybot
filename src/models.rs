use diesel::prelude::*;
use crate::schema::items;

#[derive(Queryable, Selectable)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Item {
    pub id: i32,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub body: String,
}