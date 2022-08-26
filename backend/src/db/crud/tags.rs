use crate::db::models::tag::{Tag, NewTag};
use diesel::{dsl::sql, prelude::*};

pub fn create_tag(conn: &SqliteConnection, name: &str) -> i32 {
    use crate::db::schema::tags;

    let new_tag = NewTag { name };

    // NOTE: https://github.com/diesel-rs/diesel/discussions/2684

    diesel::insert_into(tags::table)
        .values(&new_tag)
        // .returning(tags::id)
        .execute(conn)
        .expect("Error saving new tag");

    tags::table
        .find(sql("last_insert_rowid()"))
        .get_result::<Tag>(conn)
        .expect("What?")
        .id
}

pub fn read_tags(conn: &SqliteConnection) -> Option<Vec<Tag>> {
    use crate::db::schema::tags;

    tags::table
        .get_results::<Tag>(conn)
        .ok()
}

pub fn read_tag_by_id(conn: &SqliteConnection, id: i32) -> Option<Tag> {
    use crate::db::schema::tags;

    tags::table
        .find(id)
        .get_result::<Tag>(conn)
        .ok()
}

pub fn read_tag_by_name(conn: &SqliteConnection, name: &str) -> Option<Tag> {
    use crate::db::schema::tags;

    tags::table
        .filter(tags::name.eq(name))
        .get_result::<Tag>(conn)
        .ok()
}

pub fn read_tags_by_item_id(conn: &SqliteConnection, id: i32) -> Option<Vec<Tag>> {
    use crate::db::schema::items;
    use crate::db::schema::items_tags;
    use crate::db::schema::tags;

    let res_tags = items_tags::table
        .inner_join(items::table)
        .inner_join(tags::table)
        .filter(items::id.eq(id))
        .select((tags::id, tags::name))
        .get_results::<Tag>(conn);

    res_tags.ok()
}

pub fn update_tag(conn: &SqliteConnection, id: i32, path: &str) -> Option<usize> {
    use crate::db::schema::tags;

    diesel::update(tags::table.find(id))
        .set(tags::name.eq(path))
        .execute(conn)
        .ok()
}

pub fn delete_tag(conn: &SqliteConnection, id: i32) -> Option<usize> {
    use crate::db::schema::tags;

    diesel::delete(tags::table.filter(tags::id.eq(id)))
        .execute(conn)
        .ok()
}
