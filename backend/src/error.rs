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
    #[error("external API request error: {0}")]
    ExternalApiRequest(#[from] reqwest::Error),
    #[error("external API returned error: {0}")]
    ExternalApi(String),
    #[error("item with this external id and media type already exists")]
    Conflict,
    #[error("json parsing error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Encode(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ExternalApiRequest(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::ExternalApi(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::Conflict => (StatusCode::CONFLICT, self.to_string()),
            AppError::JsonParseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}
