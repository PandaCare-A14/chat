use mongodb::Client;
use std::{env, time::Duration};

const MAX_RETRIES: i32 = 10;

pub async fn get_mongodb_client() -> Option<Client> {
    dotenvy::dotenv().ok()?;

    let database_url = env::var("DATABASE_URL").ok()?;

    for i in 1..=MAX_RETRIES {
        match Client::with_uri_str(&database_url).await.ok() {
            Some(db_client) => return Some(db_client),
            None => {
                eprintln!("Attempt {} to connect to database failed. Retrying...", i);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    // Catch when all attemps have been exhausted
    None
}
