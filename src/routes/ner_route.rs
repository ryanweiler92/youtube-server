use axum::{Json, extract::{State, Path}};
use serde_json::{json, Value};
use serde::{Deserialize};
use crate::db::{
    connection::AppState,
    models::{CreateVideoInfoDto, CreateCommentDto},
    operations::{VideoInfoRepository, CommentRepository}
};
use crate::ai::ner::ner_request;
use crate::ai::ner::NERRequest;

use crate::routes::errors::AppError;
use crate::ai::ner_models::NERRequestResult;

pub async fn ner_operation(
    State(app_state): State<AppState>,
    Json(payload): Json<NERRequest>
) -> Result<Json<NERRequestResult>, AppError> {
    let result = ner_request(payload, State(app_state)).await?;
    Ok(Json(result))
}



