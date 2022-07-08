use crate::schema::items;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub path: String,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub path: &'a str,
}
