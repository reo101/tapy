use crate::db::schema::tags;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub name: &'a str,
}
