use crate::components::database::{models::Task, mongodb::MongoDB};
use actix_web::web;
use futures::StreamExt;
use mongod::{bson::oid::ObjectId, query::Find, AsFilter, Comparator};
use std::sync::Arc;

pub async fn create_task(db: web::Data<MongoDB>, task: web::Json<Task>) -> ObjectId {
    let db: Arc<MongoDB> = db.into_inner();
    let task: Task = task.into_inner();
    let created_task_id: ObjectId = db.client.insert_one(task).await.unwrap();
    println!("📝 Created task with id {}", created_task_id);
    created_task_id
}

pub async fn get_task(db: web::Data<MongoDB>, id: web::Path<ObjectId>) -> Option<Task> {
    let db: Arc<MongoDB> = db.into_inner();
    let mut filter = Task::filter();
    let id: ObjectId = id.into_inner();
    let id_clone: ObjectId = id.clone();
    filter._id = Some(Comparator::Eq(id));
    let mut cursor = match Find::<Task>::new()
        .filter(filter)
        .unwrap()
        .query(&(db.client))
        .await {
            Ok(cursor) => cursor,
            Err(err) => {
                println!("📝 Failed to get task with id {}: {}", id_clone, err);
                return None;
            }
        };
    match cursor.next().await {
        Some(Ok((doc_id, task))) => {
            println!("📝 Got task with id {}", doc_id);
            Some(task)
        },
        Some(Err(err)) => {
            println!("📝 Failed to get task with id {}: {}", id_clone, err);
            None
        },
        None => None,
    }
}
