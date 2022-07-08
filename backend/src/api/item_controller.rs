use common::util::fs::*;
use std::path::Path;

use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/add")]
async fn add(mut parts: awmp::Parts) -> impl Responder {
    let file = parts
        .files
        .take("file")
        .pop()
        .and_then(|f| f.persist_in(media_path().unwrap()).ok())
        .unwrap();

    let texts = parts.texts.as_hash_map();

    let tags = texts.get("tags").unwrap().to_string();

    let tags_vec = tags.split(',').collect::<Vec<_>>();

    let tags_vec = match &tags_vec[..] {
        [""] => vec![],
        _ => tags_vec,
    };

    let conn = common::db::establish_connection();
    let res = common::crud::items::create_item_with_tags(
        &conn,
        &file.into_os_string().into_string().unwrap(),
        tags_vec,
    );

    HttpResponse::Ok().json(res)
}

#[get("/get/{id}")]
async fn get_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::items::read_item(&conn, *id);

    match res {
        Option::None => HttpResponse::NotFound().body(format!("Item not found with ID {}", *id)),
        Option::Some(item) => HttpResponse::Ok().json(item),
    }
}

#[get("/get")]
async fn get_by_tags(parts: awmp::Parts) -> impl Responder {
    let texts = parts.texts.as_hash_map();

    let tags = texts.get("tags").unwrap().to_string();

    let tags_vec = tags.split(',').collect::<Vec<_>>();

    let tags_vec = match &tags_vec[..] {
        [""] => vec![],
        _ => tags_vec,
    };

    let conn = common::db::establish_connection();
    let res = common::crud::items::read_items_by_tags(&conn, tags_vec);

    HttpResponse::Ok().json(res)
}

#[delete("/delete/{id}")]
async fn delete_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::items::delete_item(&conn, *id);

    match res {
        Option::None => HttpResponse::NotFound().body(format!("Item not found with ID {}", *id)),
        Option::Some(count) => HttpResponse::Ok().body(format!("Deleted {} item(s)", count)),
    }
}
