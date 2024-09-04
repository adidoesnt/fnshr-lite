use crate::components::controllers;
use crate::components::database::{models::Task, mongodb::MongoDB};
use actix_web::{post, web, HttpResponse};

#[post("/tasks")]
async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> HttpResponse {
    controllers::task::create_task(db, task).await
}
