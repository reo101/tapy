use crate::db::schema::items_tags;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize, Debug)]
pub struct ItemTag {
    pub item_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name = "items_tags"]
pub struct NewItemTag {
    pub item_id: i32,
    pub tag_id: i32,
}
