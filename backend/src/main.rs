use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web_lab::web::spa;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            // .service(hello)
            // .service(echo)
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish()
            )
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
    .await
}
