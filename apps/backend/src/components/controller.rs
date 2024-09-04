use crate::components::{database::MongoDB, models::Task, service};
use actix_web::{web, HttpResponse};
use mongod::bson::oid::ObjectId;

pub fn health_controller() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

pub async fn create_task_controller(db: web::Data<MongoDB>, task: web::Json<Task>) -> HttpResponse {
    let created_task_id: ObjectId = service::create_task(db, task).await;
    HttpResponse::Created().json(created_task_id)
}
