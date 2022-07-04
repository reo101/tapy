// use actix_multipart::Multipart;
// use actix_web::{post, HttpResponse};
// // use form_data::{handle_multipart, Error, Field, FilenameGenerator, Form};
// use futures::Future;
// use std::path::PathBuf;
// use uuid::Uuid;
//
// #[post("/upload")]
// pub async fn upload(mut parts: awmp::Parts) -> Result<HttpResponse, actix_web::Error> {
//     let qs = parts.texts.to_query_string();
//     let place = "/data/data/com.termux/files/home/.local/share/tapy/new";
//
//     let file_parts = parts
//         .files
//         .take("file")
//         .pop()
//         .and_then(|f| f.persist_in(place).ok())
//         .map(|f| format!("File uploaded to: {}", f.display()))
//         .unwrap_or_default();
//
//     let body = [format!("Text parts: {}", &qs), file_parts].join(", ");
//
//     Ok(HttpResponse::Ok().body(body))
// }
//
// #[post("/upload_v2")]
// pub async fn upload_v2(
//     (mp, state): (Multipart, Data<Form>),
// ) -> Result<actix_web::HttpResponse, actix_web::Error> {
//     handle_multipart(mp, state.get_ref().clone())
//         .map(|uploaded_content| {
//             println!("Uploaded Content: {:?}", uploaded_content);
//             HttpResponse::Created().finish()
//         })
//         .await
// }
