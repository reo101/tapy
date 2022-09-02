use crate::db::schema::tags;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Clone, Serialize, Deserialize, Debug)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag<'a> {
    pub name: &'a str,
}
