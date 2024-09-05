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

pub async fn get_task(db: web::Data<MongoDB>, id: web::Path<String>) -> Option<Task> {
    let object_id: ObjectId = ObjectId::parse_str(id.into_inner()).unwrap();
    let task: Option<Task> = repositories::task::get_task(db, object_id.into()).await;
    task
}
