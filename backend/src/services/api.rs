use actix_web::web;

use crate::api;

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            // Item Controller
            .service(api::item_controller::add)
            .service(api::item_controller::get_all)
            .service(api::item_controller::get_by_id)
            .service(api::item_controller::get_by_tags)
            .service(api::item_controller::delete_by_id)
            // Tag Controller
            .service(api::tag_controller::get_all)
            .service(api::tag_controller::get_tags),
    );
}
