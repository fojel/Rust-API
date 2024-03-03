use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use validator::{Validate, ValidationErrors};
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct TimeRequest {
    #[serde(rename = "country")]
    country: String,
}

impl Validate for TimeRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        if self.country.is_empty() {
            return Err(ValidationErrors::new());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct ZoneResponse {
    timezone: String,
    datetime: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("GET /time")
}

async fn get_time(info: web::Query<TimeRequest>) -> impl Responder {
    match info.validate() {
        Ok(_) => {
            let country = &info.country;
            let client = Client::new();
            match client
                .get(&format!("https://worldtimeapi.org/api/timezone/{}", country))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<ZoneResponse>().await {
                            Ok(timezone_response) => {
                                println!("Response JSON: {:?}", timezone_response);
                                HttpResponse::Ok().body(format!("Hora actual en {}: {}", country, timezone_response.datetime))
                            }
                            Err(_) => {
                                let error_message = "Error al obtener la información de la zona horaria.";
                                HttpResponse::InternalServerError().body(error_message)
                            }
                        }
                    } else {
                        let error_message = format!("Error: País {} no encontrado.", country);
                        HttpResponse::BadRequest().body(error_message)
                    }
                }
                Err(_) => {
                    let error_message = "Error al enviar la solicitud al servicio de hora.";
                    HttpResponse::InternalServerError().body(error_message)
                }
            }
        }
        Err(_) => {
            let error_message = "Error: Datos proporcionados no válidos.";
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