use actix_web::{web, App, HttpServer, Responder};
use chrono::{Utc};
use validator::{Validate, ValidationError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TimeRequest {
    #[serde(rename = "timezone")]
    timezone: String,
}

impl Validate for TimeRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.timezone.len() < 3 {
            return Err(ValidationError::new("timezone").with_reason("debe tener al menos 3 caracteres"));
        }
        Ok(())
    }
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