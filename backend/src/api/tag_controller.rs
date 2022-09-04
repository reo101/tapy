use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::db::db::DbPool;

#[get("/tags/all")]
async fn get_all(req: HttpRequest) -> impl Responder {
    let mut conn = req
        .app_data::<DbPool>()
        .unwrap()
        .get()
        .unwrap();
    let res = crate::db::crud::tags::read_tags(&mut conn);

    HttpResponse::Ok().json(res)
}

#[get("/tags/{item_id}")]
async fn get_tags(req: HttpRequest, item_id: web::Path<i32>) -> impl Responder {
    let mut conn = req
        .app_data::<DbPool>()
        .unwrap()
        .get()
        .unwrap();
    let res = crate::db::crud::tags::read_tags_by_item_id(&mut conn, *item_id);

    HttpResponse::Ok().json(res)
}

#[allow(unused_imports, unused_variables)]
#[cfg(test)]
mod tests {
    use crate::db::{
        db::establish_connection,
        models::{item, tag::Tag},
    };
    use actix_web::{
        http::header::ContentType,
        test::{call_service, init_service, read_body, read_body_json, TestRequest},
        App,
    };
    use serde::{Deserialize, Serialize};

    use super::*;

    #[actix_rt::test]
    async fn succeed_get_all_zero_tags() {
        let pool = establish_connection();
        let app = init_service(
            App::new()
                .app_data(pool.clone())
                .configure(crate::services::api::init_routes),
        )
        .await;

        let request = TestRequest::get().uri("/api/tags/all").to_request();

        let response = call_service(&app, request).await;

        #[derive(Serialize, Deserialize)]
        struct Body(Option<Vec<Tag>>);

        assert!(response.status().is_success());

        let body: Body = read_body_json(response).await;

        assert_eq!(
            match body.0 {
                Some(tags) => tags,
                None => vec![],
            }
            .len(),
            0
        );
    }

    #[actix_rt::test]
    async fn succeed_get_all_one_tag() {
        let pool = establish_connection();
        let app = init_service(
            App::new()
                .app_data(pool.clone())
                .configure(crate::services::api::init_routes),
        )
        .await;

        crate::db::crud::tags::create_tag(&mut pool.get().unwrap(), "test");

        let request = TestRequest::get().uri("/api/tags/all").to_request();

        let response = call_service(&app, request).await;

        #[derive(Serialize, Deserialize)]
        struct Body(Option<Vec<Tag>>);

        assert!(response.status().is_success());

        let body: Body = read_body_json(response).await;

        assert_eq!(body.0.as_ref().unwrap().len(), 1);
        assert_eq!(body.0.unwrap().get(0).unwrap().name, "test");
    }

    #[actix_rt::test]
    async fn succeed_get_zero_tags_after_adding_and_removing_one() {
        let pool = establish_connection();
        let app = init_service(
            App::new()
                .app_data(pool.clone())
                .configure(crate::services::api::init_routes),
        )
        .await;

        crate::db::crud::tags::create_tag(&mut pool.get().unwrap(), "test");

        let request = TestRequest::get().uri("/api/tags/all").to_request();

        let response = call_service(&app, request).await;

        #[derive(Serialize, Deserialize)]
        struct Body(Option<Vec<Tag>>);

        assert!(response.status().is_success());

        let body: Body = read_body_json(response).await;

        assert_eq!(body.0.as_ref().unwrap().len(), 1);
        assert_eq!(body.0.unwrap().get(0).unwrap().name, "test");
    }
}
