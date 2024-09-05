use crate::components::controllers;
use crate::components::database::{models::Task, mongodb::MongoDB};
use actix_web::{get, post, web, HttpResponse};

#[post("/tasks")]
async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> HttpResponse {
    println!("📝 Creating task");
    controllers::task::create_task(db, task).await
}

#[get("/tasks/{id}")]
async fn get_task(db: web::Data<MongoDB>, id: web::Path<String>) -> HttpResponse {
    println!("🔎 Getting task with id {}", id);
    controllers::task::get_task(db, id).await
}
