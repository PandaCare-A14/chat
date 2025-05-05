use actix_web::{App, HttpServer};
use db::get_mongodb_client;

mod chat_history;
mod chat_room;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_client = match get_mongodb_client().await {
        Some(client) => client,
        None => panic!("An error occurred while trying to connect to the database"),
    };

    HttpServer::new(|| App::new().app_data(db_client.clone()).service())
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
