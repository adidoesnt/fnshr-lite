use crate::components::controller::{create_task_controller, health_controller};
use crate::components::{database::MongoDB, models::Task};
use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn index() -> HttpResponse {
    health_controller()
}

#[get("/health")]
async fn health() -> HttpResponse {
    health_controller()
}

#[post("/tasks")]
async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> HttpResponse {
    create_task_controller(db, task).await
}
