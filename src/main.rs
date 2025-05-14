mod chat_room;
mod chat_server;
mod db;
mod errors;
mod utils;

use std::{fs, io::ErrorKind};

use actix_web::{
    App, HttpServer,
    middleware::Logger,
    web::{self, Data},
};
use jsonwebtoken::DecodingKey;
use mongodb::Client;

use chat_room::services::create_chat_room;
use db::get_mongodb_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use env_logger;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let key_file = fs::read_to_string("private.key")?;

    let verifying_key = DecodingKey::from_base64_secret(&key_file)
        .map_err(|err| std::io::Error::new(ErrorKind::Other, err))?;

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
            .wrap(Logger::default())
            .app_data(Data::new(db_client.clone()))
            .app_data(Data::new(verifying_key.clone()))
            .service(
                web::scope("api").service(
                    web::scope("chat")
                        .service(create_chat_room)
                        .service(web::scope("history")),
                ),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
