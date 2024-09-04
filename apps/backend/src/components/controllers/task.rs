use crate::components::{
    database::{models::Task, mongodb::MongoDB},
    services,
};
use actix_web::{web, HttpResponse};
use mongod::bson::oid::ObjectId;

pub async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> HttpResponse {
    let created_task_id: ObjectId = services::task::create_task(db, task).await;
    HttpResponse::Created().json(created_task_id)
}
