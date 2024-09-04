use actix_web::web;
use mongod::bson::oid::ObjectId;
use crate::components::{database::MongoDB, models::Task, repository};

pub async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> ObjectId {
    let created_task_id: ObjectId = repository::create_task(db, task).await;
    created_task_id
}