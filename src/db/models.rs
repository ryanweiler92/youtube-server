use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

use crate::ai::ner::Annotations;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoInfo {
    pub id: Option<i32>,
    pub title: String,
    pub channel: String,
    pub channel_id: String,
    pub description: Option<String>,
    pub yt_id: String,
    pub views: i64,
    pub comment_count: i64,
    pub like_count: i64,
    pub video_thumbnail: Option<String>,
    pub upload_date: Option<String>,
    pub channel_thumbnail: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Option<i32>,
    pub comment_id: String,
    pub channel_id: String,
    pub video_id: String,
    pub display_name: String,
    pub user_verified: Option<bool>,
    pub thumbnail: Option<String>,
    pub content: String,
    pub published_time: Option<String>,
    pub like_count: Option<i32>,
    pub reply_count: Option<i32>,
    pub comment_level: Option<i32>,
    pub reply_to: Option<String>,
    pub reply_order: Option<i32>,
    pub annotations: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Input DTOs for API endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVideoInfoDto {
    pub title: String,
    pub channel: String,
    pub channel_id: String,
    pub description: String,
    pub yt_id: String,
    pub views: u64,
    pub comment_count: u64,
    pub like_count: u64,
    pub video_thumbnail: String,
    pub upload_date: String,
    pub channel_thumbnail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentDto {
    pub comment_id: String,
    pub channel_id: String,
    pub video_id: String,
    pub display_name: String,
    pub user_verified: bool,
    pub thumbnail: String,
    pub content: String,
    pub published_time: String,
    pub like_count: i32,
    pub reply_count: i32,
    pub comment_level: i32,
    pub reply_to: String,
    pub reply_order: i32,
    pub annotations: serde_json::Value
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentContentAndId {
    pub id: String,
    pub comment: String
}