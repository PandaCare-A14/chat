use crate::chat_server::types::Message;
use mongodb::bson::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoomRequest {
    pub target_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRoom {
    pub whitelist: Vec<Uuid>,
    pub room_id: Uuid,
    pub created_at: DateTime,
    pub chat_history: Vec<Message>,
}
