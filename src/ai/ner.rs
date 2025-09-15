use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::db::{
    connection::AppState,
    models::{CreateVideoInfoDto, CreateCommentDto},
    operations::{VideoInfoRepository, CommentRepository}
};
use crate::routes::errors::AppError;

use reqwest;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]

pub struct NERRequest {
    video_id: String,
    labels: Vec<String>,
    threshold: f32
}

pub async fn ner_request(ner_request: NERRequest, State(app_state): State<AppState>) -> Result<Json<Value>, AppError> {
    let video_request_id = ner_request.video_id.as_str();
    let comments = CommentRepository::get_by_video_id(&app_state.db_pool, video_request_id).await?;

    let ten_comments = &comments[0..11];

    let comment_contents: Vec<String> = ten_comments.iter()
        .map(|comment| comment.content.clone())
        .collect();

    let payload = json!({
        "text": comment_contents,
        "labels": ner_request.labels,
        "threshold": ner_request.threshold
    });

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:8080/ner")
        .json(&payload)
        .send()
        .await.map_err(|e| AppError::AIServerError(e.to_string()))?;

    let result = response
        .json::<Value>()
        .await
        .map_err(|e| AppError::AIServerError(e.to_string()))?;

    Ok(Json(result))



}