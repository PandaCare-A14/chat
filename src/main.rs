mod chat_history;
mod chat_room;
mod db;
mod errors;
mod utils;

use std::io::ErrorKind;

use actix_web::{App, HttpServer};
use mongodb::Client;

use chat_room::services::create_chat_room;
use db::get_mongodb_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_client: Client = match get_mongodb_client().await {
        Some(db_client) => db_client,
        None => {
            return Err(std::io::Error::new(
                ErrorKind::ConnectionRefused,
                "Failed to establish connection to DB",
            ));
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(db_client.clone())
            .service(create_chat_room)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
