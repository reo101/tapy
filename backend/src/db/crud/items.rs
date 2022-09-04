use std::collections::HashSet;

use crate::db::{
    db::{DbPool, DbPoolConn},
    models::item::{Item, NewItem},
};
use diesel::{dsl::sql, prelude::*};

pub fn create_item(mut conn: DbPoolConn, path: &str) -> i32 {
    use crate::db::schema::items;

    let new_item = NewItem { path };

    // NOTE: https://github.com/diesel-rs/diesel/discussions/2684

    diesel::insert_into(items::table)
        .values(&new_item)
        // .returning(items::id)
        .execute(&mut conn)
        .expect("Error saving new item"); // as i32

    items::table
        .find(sql("last_insert_rowid()"))
        .get_result::<Item>(&mut conn)
        .expect("What?")
        .id
}

pub fn create_item_with_tags(conns: &DbPool, path: &str, tags_vec: Vec<&str>) -> i32 {
    let item_id = create_item(conns.get().unwrap(), path);

    tags_vec
        .into_iter()
        .map(|name| {
            if let Some(tag) =
                crate::db::crud::tags::read_tag_by_name(&mut conns.get().unwrap(), name)
            {
                tag.id
            } else {
                crate::db::crud::tags::create_tag(&mut conns.get().unwrap(), name)
            }
        })
        .for_each(|tag_id| {
            crate::db::crud::items_tags::create_item_tag(
                &mut conns.get().unwrap(),
                item_id,
                tag_id,
            );
        });

    item_id
}

pub fn read_item(mut conn: DbPoolConn, id: i32) -> Option<Item> {
    use crate::db::schema::items;

    items::table.find(id).get_result::<Item>(&mut conn).ok()
}

pub fn read_items_by_tags(conns: &DbPool, tags_vec: Vec<&str>) -> Option<HashSet<i32>> {
    use crate::db::models::item_tag::ItemTag;

    Some(match tags_vec.len() {
        0 => todo!(),
        _ => {
            let tags_vec = tags_vec
                .into_iter()
                .filter_map(|tag| {
                    use crate::db::crud::tags::read_tag_by_name;

                    read_tag_by_name(&mut conns.get().unwrap(), tag)
                })
                .collect::<Vec<_>>();

            ItemTag::belonging_to(&tags_vec)
                .load::<ItemTag>(&mut conns.get().unwrap())
                .unwrap()
                .grouped_by(&tags_vec)
                .into_iter()
                .map(|vec| {
                    vec.into_iter()
                        .map(|item_tag| item_tag.item_id)
                        .collect::<HashSet<_>>()
                })
                .reduce(|acc, set| &acc & &set)
                .unwrap_or_default()
        }
    })
}

pub fn update_item(mut conn: DbPoolConn, id: i32, path: &str) -> Option<usize> {
    use crate::db::schema::items;

    diesel::update(items::table.find(id))
        .set(items::path.eq(path))
        .execute(&mut conn)
        .ok()
}

pub fn delete_item(mut conn: DbPoolConn, id: i32) -> Option<usize> {
    use crate::db::schema::items;

    diesel::delete(items::table.filter(items::id.eq(id)))
        .execute(&mut conn)
        .ok()
}
