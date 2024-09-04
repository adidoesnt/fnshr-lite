use crate::components::controllers;
use actix_web::{get, HttpResponse};

#[get("/health")]
async fn get_health_status() -> HttpResponse {
    controllers::health::get_health_status()
}
