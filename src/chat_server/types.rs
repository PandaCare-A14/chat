use mongodb::bson::{DateTime, Uuid, oid::ObjectId};
use serde::{Deserialize, Serialize};

pub type RoomId = Uuid;
pub type ConnId = Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message {
    message_id: ObjectId,
    time_sent: DateTime,
    username: String,
    content: String,
}

pub enum Command {
    Join,
    Message { message: Message },
}
