use mongod::{Bson, Mongo};
use serde::Deserialize;

#[derive(Debug, Bson, Mongo, Deserialize)]
#[mongo(collection = "tasks", field, filter, update)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: String,
    pub deadline: String,
}
