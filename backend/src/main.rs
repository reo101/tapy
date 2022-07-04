use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web_lab::web::spa;

#[get("/add")]
async fn add(req_body: String) -> impl Responder {
    HttpResponse::NoContent().body("Shte iz4aka6 malko")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/test")]
async fn test(req_body: String) -> impl Responder {
    let conn = common::establish_connection();
    let kek = common::create_item(&conn, &req_body);

    HttpResponse::Ok().body(format!("Bytes: {}", kek))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            // API
            .service(test)
            // SPA
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
