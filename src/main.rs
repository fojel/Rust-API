use actix_web::{web, App, HttpServer, Responder};
use chrono::Utc;

async fn index() -> impl Responder {
    "API to get time"
}

async fn getTime() -> impl Responder {
    let time = Utc::now();
    time.to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/time", web::get().to(getTime()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}