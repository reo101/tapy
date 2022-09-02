use crate::db::schema::items_tags;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::db::models::{item::Item, tag::Tag};

#[derive(Associations, Identifiable, Queryable, Clone, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Item))]
#[diesel(belongs_to(Tag))]
#[diesel(primary_key(item_id, tag_id))]
#[diesel(table_name = items_tags)]
pub struct ItemTag {
    pub item_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = items_tags)]
pub struct NewItemTag {
    pub item_id: i32,
    pub tag_id: i32,
}
