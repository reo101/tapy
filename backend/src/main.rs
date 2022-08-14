use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web_lab::web::spa;

mod api;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            // Logging
            .wrap(logger)
            // API
            .service(
                web::scope("/api")
                    .service(api::item_controller::add)
                    // .service(api::item_controller::get_by_tags)
                    .service(api::item_controller::get_all)
                    .service(api::item_controller::get_by_id)
                    .service(api::item_controller::delete_by_id)
                    .service(api::tag_controller::get_tags)
            )
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
