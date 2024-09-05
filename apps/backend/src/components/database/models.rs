use mongod::{bson::oid::ObjectId, Bson, Mongo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Bson, Mongo, Serialize, Deserialize)]
#[mongo(collection = "tasks", field, filter, update)]
pub struct Task {
    pub _id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: String,
    pub deadline: String,
}
