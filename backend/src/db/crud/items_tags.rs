use crate::db::models::item_tag::{ItemTag, NewItemTag};
use diesel::prelude::*;

pub fn create_item_tag(conn: &SqliteConnection, item_id: i32, tag_id: i32) {
    use crate::db::schema::items_tags;

    let new_item_tag = NewItemTag { item_id, tag_id };

    // NOTE: https://github.com/diesel-rs/diesel/discussions/2684

    diesel::insert_into(items_tags::table)
        .values(&new_item_tag)
        // .returning(items_tags::id)
        .execute(conn)
        .expect("Error saving new item_tag");
}

pub fn read_item_tag(conn: &SqliteConnection, item_id: i32, tag_id: i32) -> Option<ItemTag> {
    use crate::db::schema::items_tags;

    items_tags::table
        .find((item_id, tag_id))
        .get_result::<ItemTag>(conn)
        .ok()
}

pub fn delete_item_tag(conn: &SqliteConnection, item_id: i32, tag_id: i32) -> Option<usize> {
    use crate::db::schema::items_tags;

    diesel::delete(items_tags::table.filter(items_tags::item_id.eq(item_id).and(items_tags::tag_id.eq(tag_id))))
        .execute(conn)
        .ok()
}
