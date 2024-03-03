use actix_web::{web, App, HttpServer, Responder};
use chrono::{DateTime, Utc};
use validator::Validate;
use serde::Deserialize;

#[derive(Debug, Validate, Deserialize)]
struct TimeRequest {
    #[serde(rename = "timezone")]
    #[validate(length(min = 3))]
    timezone: String,
}

async fn index() -> impl Responder {
    "¡Hola, mundo!"
}

async fn get_time(info: web::Query<TimeRequest>) -> impl Responder {
    if let Err(e) = info.validate() {
        return format!("Error de validación: {:?}", e).into();
    }

    let current_time = Utc::now();
    format!("Hora actual en {}: {}", info.timezone, current_time)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/time", web::get().to(get_time))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}