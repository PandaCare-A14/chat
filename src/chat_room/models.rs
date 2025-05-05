use crate::chat_history::models::Message;
use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RoomRequest {
    pub target_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRoom {
    pub room_id: Uuid,
    pub created_time: DateTime,
    pub chat_history: Vec<Message>,
}
