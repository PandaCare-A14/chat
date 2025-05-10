use actix_web::{
    HttpRequest, HttpResponse,
    web::{self},
};
use jsonwebtoken::DecodingKey;
use mongodb::{
    Client,
    bson::{Uuid, datetime},
};

use crate::{
    chat_history::models::Message,
    chat_room::models::ChatRoom,
    errors::AuthorizationError,
    utils::{User, get_access_token_from_auth_header, get_user_details},
};

use super::models::RoomRequest;

const DB_NAME: &str = "public";
const COLL_NAME: &str = "chat_history";

#[actix_web::post("/")]
pub async fn create_chat_room(
    req: HttpRequest,
    room_request: web::Json<RoomRequest>,
    verifying_key: web::Data<DecodingKey>,
    db_client: web::Data<Client>,
) -> Result<HttpResponse, impl actix_web::error::ResponseError> {
    let room_id = Uuid::new();
    let created_time = datetime::DateTime::now();
    let chat_history = Vec::<Message>::new();

    let token = match get_access_token_from_auth_header(req) {
        Some(token) => token,
        None => return Err(AuthorizationError::TokenNotFound),
    };

    let user: User = get_user_details(&token, &verifying_key.into_inner())
        .map_err(|_err| AuthorizationError::TokenInvalid)?;

    let whitelist = vec![room_request.into_inner().target_id, user.user_id()];

    let chat_room = ChatRoom {
        whitelist,
        room_id,
        created_time,
        chat_history,
    };

    let collection = db_client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(chat_room).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("chat room successfully created")),
        Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string())),
    }
}
