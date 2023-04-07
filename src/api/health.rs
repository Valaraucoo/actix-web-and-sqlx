use actix_web::{get, HttpResponse, Responder};
use crate::models::health::HealthResponse;

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    let response = HealthResponse::new().await;
    HttpResponse::Ok().json(response)
}
