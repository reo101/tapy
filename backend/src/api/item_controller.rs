use crate::db::{db::DbPool, util::fs::*};
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};

#[post("/add")]
async fn add(req: HttpRequest, mut parts: awmp::Parts) -> impl Responder {
    let file = parts.files.take("file").pop().and_then(|f| {
        let sanitized_file_name = f.sanitized_file_name().to_string();

        // FIXME: revamp whole file saving (hardcoded `/tmp` is terrible)
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

            let conns = req.app_data::<DbPool>().unwrap();

            let res = crate::db::crud::items::create_item_with_tags(
                conns,
                &file.into_os_string().into_string().unwrap(),
                tags_vec,
            );

            HttpResponse::Ok().json(res)
        }
        None => HttpResponse::InternalServerError().body("Error adding new item"),
    }
}

#[get("/get/{id}")]
async fn get_by_id(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let conn = req.app_data::<DbPool>().unwrap().get().unwrap();
    let res = crate::db::crud::items::read_item(conn, *id);

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
                        // TODO: make this configurable
                        if true { "inline" } else { "attachment" },
                        PathBuf::from(&path).file_name().unwrap().to_str().unwrap()
                    ),
                ))
                .body(buf)
        }
    }
}

#[get("/get")]
async fn get_by_tags(req: HttpRequest, parts: awmp::Parts) -> impl Responder {
    let texts = parts.texts.as_hash_map();

    let tags = texts.get("tags");

    match tags {
        Option::None => HttpResponse::BadRequest().body("Bad request: missing tags field"),
        Option::Some(tags) => {
            let tags_vec = tags.split(',').collect::<Vec<_>>();

            // NOTE: `.split` on an empty string doesn't return an empty iterator
            let tags_vec = match &tags_vec[..] {
                [""] => vec![],
                _ => tags_vec,
            };

            let conns = req.app_data::<DbPool>().unwrap();
            let res = crate::db::crud::items::read_items_by_tags(conns, tags_vec);

            HttpResponse::Ok().json(res)
        }
    }
}

#[get("/get/all")]
async fn get_all(req: HttpRequest) -> impl Responder {
    let tags_vec = vec![];

    // let conn = req.app_data::<DbPool>().unwrap().get().unwrap();
    let conns = req.app_data::<DbPool>().unwrap();
    let res = crate::db::crud::items::read_items_by_tags(conns, tags_vec);

    HttpResponse::Ok().json(res)
}

#[delete("/delete/{id}")]
async fn delete_by_id(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let conn = req.app_data::<DbPool>().unwrap().get().unwrap();
    let res = crate::db::crud::items::delete_item(conn, *id);

    match res {
        Option::Some(count) => HttpResponse::Ok().body(format!("Deleted {} item(s)", count)),
        Option::None => HttpResponse::NotFound().body(format!("Item not found with ID {}", *id)),
    }
}

#[allow(unused_imports, unused_variables)]
#[cfg(test)]
mod tests {
    use crate::db::{db::establish_connection, models::item};
    use actix_web::{
        http::header::ContentType,
        test::{call_service, init_service, read_body, read_body_json, TestRequest},
        App,
    };

    use super::*;

    #[allow(clippy::assertions_on_constants)]
    #[actix_rt::test]
    async fn succeed_create_item_from_api() {
        let pool = establish_connection();
        let app = init_service(
            App::new()
                .app_data(pool.clone())
                .configure(crate::services::api::init_routes),
        )
        .await;

        // TODO: find a way to make actix_web and awmp play nicely in tests
        //
        // NOTE: example request (generated with curl and netcat)
        // POST / HTTP/1.1
        // Host: localhost:4444
        // User-Agent: curl/7.64.1
        // Accept: */*
        // Content-Length: 300
        // Content-Type: multipart/form-data; boundary=------------------------1e0c2bc1ff7efc93
        //
        // --------------------------1e0c2bc1ff7efc93
        // Content-Disposition: form-data; name="file"; filename="kek"
        // Content-Type: application/octet-stream
        //
        // aloda
        //
        // --------------------------1e0c2bc1ff7efc93
        // Content-Disposition: form-data; name="tags"
        //
        // alo,da
        // --------------------------1e0c2bc1ff7efc93--

        assert!(true);
    }

    #[actix_rt::test]
    async fn fail_create_item_from_api() {
        let pool = establish_connection();
        let app = init_service(
            App::new()
                .app_data(pool.clone())
                .configure(crate::services::api::init_routes),
        )
        .await;

        let request = TestRequest::post()
            .uri("/api/add")
            .insert_header(ContentType::octet_stream())
            .to_request();

        let response = call_service(&app, request).await;
        assert!(!response.status().is_success());
    }

    // FIXME: blocked from writing any other tests until I figure out how to send requests with multipart/form-data
}
