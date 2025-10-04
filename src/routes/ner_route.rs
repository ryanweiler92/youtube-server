use axum::{Json, extract::{State, Path}};

use serde::{Deserialize, Serialize};
use crate::db::{
    connection::AppState,
    models::{CreateVideoInfoDto, CreateCommentDto},
    operations::{VideoInfoRepository, CommentRepository}
};
use crate::ai::ner::{
    ner_request,
    build_ranked_annotations,
    RankedAnnotations,
    NERRequest,
    NERRequestResult,
    AnnotationObject
};
use axum::{response::IntoResponse};
use crate::error::AppError;


pub async fn ner_operation(
    State(app_state): State<AppState>,
    Json(payload): Json<NERRequest>
) -> Result<impl IntoResponse, AppError> {
    let result = ner_request(payload, State(app_state)).await?;
    Ok(Json(result))
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRankedAnnotationsRequest {
    video_id: String,
    #[serde(default)]
    threshold: Option<u32>
}
pub async fn get_ranked_annotations_route(
    State(app_state): State<AppState>,
    Json(payload): Json<GetRankedAnnotationsRequest>
) -> Result<impl IntoResponse, AppError> {
    let video_id = payload.video_id;
    let threshold = payload.threshold.unwrap_or(2);

    let ranked_annotations = build_ranked_annotations(&video_id, &threshold, State(app_state)).await?;

    Ok(Json(ranked_annotations))    
}


