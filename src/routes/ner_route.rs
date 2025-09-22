use axum::{Json, extract::{State, Path}};

use serde::{Deserialize};
use crate::db::{
    connection::AppState,
    models::{CreateVideoInfoDto, CreateCommentDto},
    operations::{VideoInfoRepository, CommentRepository}
};
use crate::ai::ner::{
    ner_request,
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



