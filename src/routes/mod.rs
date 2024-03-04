use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

mod time;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/time").route(web::get().to(time::get_time)));
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("GET /time")
}
