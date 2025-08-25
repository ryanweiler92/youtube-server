use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    let response = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(Json(response))
}