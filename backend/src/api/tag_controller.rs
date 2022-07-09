use common::util::fs::*;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[get("/tags")]
async fn get_tags() -> impl Responder {
    let conn = common::db::establish_connection();
    let res = common::crud::tags::read_tags(&conn);

    HttpResponse::Ok().json(res)
}
