use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("encoding error: {0}")]
    Encode(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("invalid date format: {0}")]
    ParseError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Encode(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, message).into_response()
    }
}
