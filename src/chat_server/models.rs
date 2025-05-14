use std::collections::{HashMap, HashSet};

use tokio::sync::mpsc;

use super::types::{ConnId, Message, RoomId};

pub struct ChatServer {
    connections: HashMap<ConnId, mpsc::Sender<Message>>,
    rooms: HashMap<RoomId, HashSet<ConnId>>,
}
