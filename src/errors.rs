use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use derive_more::derive::{Display, Error};

#[derive(Error, Display, Debug)]
pub enum AuthorizationError {
    #[display("no access token found")]
    TokenNotFound,

    #[display("access token is invalid")]
    TokenInvalid,
}

impl ResponseError for AuthorizationError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::TokenNotFound => StatusCode::UNAUTHORIZED,
            Self::TokenInvalid => StatusCode::UNAUTHORIZED,
        }
    }
}
