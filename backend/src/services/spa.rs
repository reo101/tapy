use actix_web::web;
use actix_web_lab::web::spa;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        spa()
            .index_file("./dist/index.html")
            .static_resources_mount("/")
            .static_resources_location("./dist")
            .finish(),
    );
}
