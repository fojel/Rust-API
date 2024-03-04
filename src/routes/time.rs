use actix_web::{web, HttpResponse};
use serde::Deserialize;
use reqwest::Client;
use chrono::{DateTime};

#[derive(Debug, Deserialize)]
pub struct TimeRequest {
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct ZoneResponse {
    pub timezone: String,
    pub datetime: String,
}

impl TimeRequest {
    pub fn is_valid(&self) -> bool {
        !self.country.is_empty()
    }
}

pub async fn get_time(info: web::Query<TimeRequest>) -> HttpResponse {
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
