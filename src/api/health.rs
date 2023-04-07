use actix_web::{get, HttpResponse, Responder};
use crate::models::health::HealthResponse;

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse::new())
}
