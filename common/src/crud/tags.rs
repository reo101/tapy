use crate::models::tag::{Tag, NewTag};
use diesel::{dsl::sql, prelude::*};

pub fn create_tag(conn: &SqliteConnection, name: &str) -> i32 {
    use crate::schema::tags;

    let new_tag = NewTag { name };

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

pub fn read_tag(conn: &SqliteConnection, id: i32) -> Option<Tag> {
    use crate::schema::tags;

    tags::table
        .find(id)
        .load::<Tag>(conn)
        .ok()?
        .first()
        .cloned()
}

pub fn update_tag(conn: &SqliteConnection, id: i32, path: &str) -> Option<usize> {
    use crate::schema::tags;

    diesel::update(tags::table.find(id))
        .set(tags::name.eq(path))
        .execute(conn)
        .ok()
}

pub fn delete_tag(conn: &SqliteConnection, id: i32) -> Option<usize> {
    use crate::schema::tags;

    diesel::delete(tags::table.filter(tags::id.eq(id)))
        .execute(conn)
        .ok()
}
