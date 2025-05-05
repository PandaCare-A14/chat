#[cfg(test)]
mod tests {
    use actix_web::{
        App,
        test::{self, TestRequest},
    };
    use mongodb::bson::Uuid;

    use crate::chat_room::{self, models::RoomRequest};

    #[actix_web::test]
    async fn test_create_room() {
        let mock_data = RoomRequest {
            target_id: Uuid::new(),
            persistent: true,
        };

        let app =
            test::init_service(App::new().service(chat_room::services::create_chat_room)).await;
        let test_request = TestRequest::post().set_json(mock_data).to_request();
        let response = test::call_service(&app, test_request).await;
        assert!(response.status().is_success());
    }
}
