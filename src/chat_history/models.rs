use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    message_id: ObjectId,
    time_sent: DateTime,
    username: String,
    content: String,
}
