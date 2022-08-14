use common::util::fs::*;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/add")]
async fn add(mut parts: awmp::Parts) -> impl Responder {
    let file = parts
        .files
        .take("file")
        .pop()
        .and_then(|f| {
            let sanitized_file_name = f.sanitized_file_name().to_string();

            let old_file_path = f.persist_in("/tmp").ok()?;
            let old_file = File::open(old_file_path).ok()?;

            let new_file_path = media_path()?.join(sanitized_file_name);
            let mut new_file = File::create(&new_file_path).ok()?;

            let mut reader = BufReader::new(old_file);
            let mut buffer = Vec::<u8>::new();

            reader.read_to_end(&mut buffer).ok()?;
            new_file.write_all(buffer.as_slice()).ok()?;

            Some(new_file_path)
        });

    match file {
        Some(file) => {
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
        },
        None => HttpResponse::InternalServerError().body("Error adding new item"),
    }
}

#[get("/get/{id}")]
async fn get_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::items::read_item(&conn, *id);

    match res {
        Option::None => HttpResponse::NotFound().body(format!("Item not found with ID {}", *id)),
        Option::Some(item) => {
            let path = item.path;
            let path_buf = PathBuf::from(&path);

            let mut f = File::open(&path).unwrap();

            let mut buf = Vec::<u8>::new();

            std::io::Read::read_to_end(&mut f, &mut buf).unwrap();

            HttpResponse::Ok()
                .insert_header((
                    "Content-Type",
                    mime_guess::from_ext(
                        path_buf
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or(""),
                    )
                    .first_or_octet_stream(),
                ))
                .insert_header((
                    "Content-Disposition",
                    format!(
                        "{}; filename=\"{}\"",
                        if true { "inline" } else { "attachment" },
                        PathBuf::from(&path).file_name().unwrap().to_str().unwrap()
                    ),
                ))
                .body(buf)
        }
    }
}

// #[get("/get")]
// async fn get_by_tags(parts: awmp::Parts) -> impl Responder {
//     let texts = parts.texts.as_hash_map();
//
//     // TODO: don't unwrap
//     let tags = texts.get("tags").unwrap().to_string();
//
//     let tags_vec = tags.split(',').collect::<Vec<_>>();
//
//     // NOTE: `.split` on an empty string doesn't return an empty iterator
//     let tags_vec = match &tags_vec[..] {
//         [""] => vec![],
//         _ => tags_vec,
//     };
//
//     let conn = common::db::establish_connection();
//     let res = common::crud::items::read_items_by_tags(&conn, tags_vec);
//
//     HttpResponse::Ok().json(res)
// }

#[get("/get")]
async fn get_all() -> impl Responder {
    let tags_vec = vec![];

    let conn = common::db::establish_connection();
    let res = common::crud::items::read_items_by_tags(&conn, tags_vec);

    HttpResponse::Ok().json(res)
}

#[delete("/delete/{id}")]
async fn delete_by_id(id: web::Path<i32>) -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::items::delete_item(&conn, *id);

    match res {
        Option::Some(count) => HttpResponse::Ok().body(format!("Deleted {} item(s)", count)),
        Option::None => HttpResponse::NotFound().body(format!("Item not found with ID {}", *id)),
    }
}
