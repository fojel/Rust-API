use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use validator::{Validate, ValidationErrors};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TimeRequest {
    #[serde(rename = "timezone")]
    timezone: String,
}

impl Validate for TimeRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.timezone.len() < 3 {
            return Err(ValidationErrors::new());
        }
        Ok(())
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("GET /time")
}

async fn get_time(info: web::Query<TimeRequest>) -> impl Responder {
    match info.validate() {
        Ok(_) => {
            let current_time = Utc::now();
            HttpResponse::Ok().body(format!("Hora actual en {}: {}", info.timezone, current_time))
        }
        Err(e) => {
            let error_message = "Error de validacion: Los datos proporcionados no son validos.";
            HttpResponse::BadRequest().body(error_message)
        }
    }
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