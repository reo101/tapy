use actix_web::{get, web, HttpResponse, Responder};

#[get("/tags/all")]
async fn get_all() -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::tags::read_tags(&conn);

    HttpResponse::Ok().json(res)
}

#[get("/tags/{item_id}")]
async fn get_tags(item_id: web::Path<i32>) -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::tags::read_tags_by_item_id(&conn, *item_id);

    HttpResponse::Ok().json(res)
}
