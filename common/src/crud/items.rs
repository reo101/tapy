use crate::models::{
    item::{Item, NewItem},
    item_tag::ItemTag,
    tag::Tag,
};
use diesel::{dsl::sql, prelude::*};

pub fn create_item(conn: &SqliteConnection, path: &str) -> i32 {
    use crate::schema::items;

    let new_item = NewItem { path };

    // NOTE: https://github.com/diesel-rs/diesel/discussions/2684

    diesel::insert_into(items::table)
        .values(&new_item)
        // .returning(items::id)
        .execute(conn)
        .expect("Error saving new item");

    items::table
        .find(sql("last_insert_rowid()"))
        .get_result::<Item>(conn)
        .expect("What?")
        .id
}

pub fn read_item(conn: &SqliteConnection, id: i32) -> Option<Item> {
    use crate::schema::items;

    items::table.find(id).get_result::<Item>(conn).ok()
}

pub fn read_items_by_tags(conn: &SqliteConnection, tags_vec: Vec<&str>) -> Option<Vec<Item>> {
    use crate::schema::items;
    use crate::schema::items_tags;
    use crate::schema::tags;

    let res_items = items_tags::table
        .inner_join(items::table)
        .inner_join(tags::table)
        .select((items::id, items::path, tags::name))
        .get_results::<(i32, String, String)>(conn)
        // .default_selection()
        // .get_results::<ItemTag>(conn)
        .map(|res_items: Vec<(i32, String, String)>| {
            res_items
                .into_iter()
                // NOTE:                                          &&tag[..] == &tag.as_str()
                .filter(|(_, _, tag)| tags_vec.is_empty() || tags_vec.contains(&tag.as_str()))
                .map(|(id, path, _)| Item { id, path })
                .collect::<Vec<Item>>()
        })
        .unwrap_or_default();

    Some(res_items)
}

pub fn update_item(conn: &SqliteConnection, id: i32, path: &str) -> Option<usize> {
    use crate::schema::items;

    diesel::update(items::table.find(id))
        .set(items::path.eq(path))
        .execute(conn)
        .ok()
}

pub fn delete_item(conn: &SqliteConnection, id: i32) -> Option<usize> {
    use crate::schema::items;

    diesel::delete(items::table.filter(items::id.eq(id)))
        .execute(conn)
        .ok()
}
