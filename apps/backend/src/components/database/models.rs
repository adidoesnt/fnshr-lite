use mongod::{Bson, Mongo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Bson, Mongo, Serialize, Deserialize)]
#[mongo(collection = "tasks", field, filter, update)]
#[mongo(oid)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: String,
    pub deadline: String,
}
