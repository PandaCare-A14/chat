use mongodb::Client;
use std::env;

pub async fn get_mongodb_client() -> Option<Client> {
    dotenvy::dotenv().ok()?;
    let database_url = env::var("DATABASE_URL").ok()?;
    Client::with_uri_str(database_url).await.ok()
}

