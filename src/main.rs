use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime};
use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct TimeRequest {
    #[serde(rename = "country")]
    country: String,
}

#[derive(Debug, Deserialize)]
struct ZoneResponse {
    timezone: String,
    datetime: String,
}

impl TimeRequest {
    fn is_valid(&self) -> bool {
        !self.country.is_empty()
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("GET /time")
}

async fn get_time(info: web::Query<TimeRequest>) -> impl Responder {
    if !info.is_valid() {
        return HttpResponse::BadRequest().body("Error: Datos proporcionados no válidos.");
    }

    let country = &info.country;
    let client = Client::new();
    let url = format!("https://worldtimeapi.org/api/timezone/{}", country);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<ZoneResponse>().await {
                    Ok(timezone_response) => {
                        if let Ok(formatted_datetime) = format_datetime(&timezone_response.datetime) {
                            return HttpResponse::Ok().body(format!("Hora actual en {}: {}", country, formatted_datetime));
                        } else {
                            return HttpResponse::InternalServerError().body("Error al obtener la información de la zona horaria.");
                        }
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().body("Error al obtener la información de la zona horaria.");
                    }
                }
            } else {
                return HttpResponse::BadRequest().body(format!("Error: País {} no encontrado.", country));
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error al enviar la solicitud al servicio de hora.");
        }
    }
}

fn format_datetime(datetime: &str) -> Result<String, chrono::ParseError> {
    let datetime = DateTime::parse_from_rfc3339(datetime)?;
    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
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
