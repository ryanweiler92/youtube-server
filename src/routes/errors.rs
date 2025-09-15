use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    FailedDBConnection(String),
    DatabaseError(String),
    AIServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::FailedDBConnection(msg) => write!(f, "Failed to connect to DB: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::AIServerError(msg) => write!(f, "AI Server  error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::FailedDBConnection(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::AIServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, error_message).into_response()
    }
}