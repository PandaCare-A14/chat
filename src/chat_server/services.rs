use actix_web::{self, HttpRequest, HttpResponse, Responder, web};
use tokio::sync::mpsc;

use crate::chat_server::types::{Command, RoomId};

const DB_NAME: &str = "public";
const COLL_NAME: &str = "chat_rooms";

#[actix_web::get("/room/{room_id}")]
pub async fn join_room(req: HttpRequest, path: web::Path<RoomId>) -> impl Responder {
    let room_id = path.into_inner();

    let (tx, rx) = mpsc::channel::<Command>(10);

    HttpResponse::Ok()
}
