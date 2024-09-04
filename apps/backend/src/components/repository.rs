use actix_web::web;
use mongod::bson::oid::ObjectId;
use crate::components::{database::MongoDB, models::Task};
use std::sync::Arc;

pub async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> ObjectId {
    let db: Arc<MongoDB> = db.into_inner();
    let task: Task = task.into_inner();
    let created_task_id: ObjectId = db.client.insert_one(task).await.unwrap();
    created_task_id
}