use crate::components::controller::health_controller;

use actix_web::{get, HttpResponse};

#[get("/")]
async fn index() -> HttpResponse {
    health_controller()
}

#[get("/health")]
async fn health() -> HttpResponse {
    health_controller()
}

