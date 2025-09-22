use axum::response::{IntoResponse, Response};
use http::StatusCode;
use tracing::error;

#[derive(Debug, Clone)]
pub struct AppError(pub String);

impl<E: std::fmt::Display> From<E> for AppError {
    fn from(e: E) -> Self { AppError(e.to_string()) }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!(error = %self.0, "request failed");

        let body = if cfg!(debug_assertions) {
            format!("Error: {}", self.0)
        } else {
            "Internal Server Error".to_string()
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
