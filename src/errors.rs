use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum AuthorizationError {
    TokenNotFound,
    TokenInvalid(String),
}

impl std::error::Error for AuthorizationError {}

impl Display for AuthorizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::TokenNotFound => write!(f, "Token not found in request header"),
            Self::TokenInvalid(s) => write!(f, "Token is invalid: {}", s),
        }
    }
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
            Self::TokenInvalid(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
