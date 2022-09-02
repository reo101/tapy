#[macro_use]
extern crate diesel;
extern crate dotenvy;

pub mod db;
pub mod api;
pub mod services;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive().allow_any_origin();
        let pool = db::db::establish_connection();

        App::new()
            // Logging
            .wrap(logger)
            // CORS
            .wrap(cors)
            // DB
            .app_data(pool)
            // API
            .configure(services::api::init_routes)
            // SPA
            .configure(services::spa::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
