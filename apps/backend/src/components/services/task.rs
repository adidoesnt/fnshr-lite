use crate::components::{
    database::{models::Task, mongodb::MongoDB},
    repositories,
};
use actix_web::web;
use mongod::bson::oid::ObjectId;

pub async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> ObjectId {
    let created_task_id: ObjectId = repositories::task::create_task(db, task).await;
    created_task_id
}
