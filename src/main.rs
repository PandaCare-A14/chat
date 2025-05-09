mod chat_history;
mod chat_room;
mod db;
mod errors;
mod utils;

use std::io::ErrorKind;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use mongodb::Client;

use chat_room::services::create_chat_room;
use db::get_mongodb_client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use env_logger;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

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
            .service(create_chat_room)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
