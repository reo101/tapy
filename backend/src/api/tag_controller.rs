use actix_web::{get, HttpResponse, Responder};

#[get("/tags")]
async fn get_tags() -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::tags::read_tags(&conn);

    HttpResponse::Ok().json(res)
}
