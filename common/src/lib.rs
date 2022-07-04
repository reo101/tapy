#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::item::{Item, NewItem};
use models::tag::{Tag, NewTag};
use models::item_tag::{ItemTag, NewItemTag};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_item(conn: &SqliteConnection, path: &str) -> usize {
    use schema::items;

    let new_item = NewItem { path };

    diesel::insert_into(items::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error saving new item")
}
