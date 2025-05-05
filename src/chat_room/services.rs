use actix_web::{HttpRequest, HttpResponse, Responder, web};
use mongodb::{
    Client,
    bson::{Uuid, datetime},
};

use crate::{
    chat_history::{self, models::Message},
    chat_room::models::ChatRoom,
};

const DB_NAME: &str = "public";
const COLL_NAME: &str = "chat_history";

#[actix_web::post("/")]
pub async fn create_chat_room(_req: HttpRequest, db_client: web::Data<Client>) -> impl Responder {
    let room_id = Uuid::new();
    let created_time = datetime::DateTime::now();
    let chat_history = Vec::<Message>::new();

    let chat_room = ChatRoom {
        room_id,
        created_time,
        chat_history,
    };

    let collection = db_client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(chat_room).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("chat room successfully created"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
