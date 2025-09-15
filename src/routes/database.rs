use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use crate::db::connection::AppState;

pub async fn reset_database(State(app_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    tracing::info!("Starting database reset operation");

    let drop_queries = vec![
        "DROP TABLE IF EXISTS comments CASCADE;",
        "DROP TABLE IF EXISTS video_info CASCADE;",
    ];

    let create_queries = vec![
        r#"
    CREATE TABLE video_info (
        id SERIAL PRIMARY KEY,
        title VARCHAR NOT NULL,
        channel VARCHAR NOT NULL,
        channel_id VARCHAR NOT NULL,
        description TEXT,
        yt_id VARCHAR UNIQUE NOT NULL,
        views BIGINT NOT NULL DEFAULT 0,
        comment_count BIGINT NOT NULL DEFAULT 0,
        like_count BIGINT NOT NULL DEFAULT 0,
        video_thumbnail VARCHAR,
        upload_date VARCHAR,
        channel_thumbnail VARCHAR,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
    );
    "#,
        r#"
    CREATE INDEX idx_video_info_yt_id ON video_info(yt_id);
    "#,
        r#"
    CREATE INDEX idx_video_info_channel_id ON video_info(channel_id);
    "#,
        r#"
    CREATE TABLE comments (
        id SERIAL PRIMARY KEY,
        comment_id VARCHAR UNIQUE NOT NULL,
        channel_id VARCHAR NOT NULL,
        video_id VARCHAR NOT NULL,
        display_name VARCHAR NOT NULL,
        user_verified BOOLEAN DEFAULT FALSE,
        thumbnail VARCHAR,
        content TEXT NOT NULL,
        published_time VARCHAR,
        like_count INTEGER DEFAULT 0,
        reply_count INTEGER DEFAULT 0,
        comment_level INTEGER DEFAULT 0,
        reply_to VARCHAR DEFAULT '',
        reply_order INTEGER DEFAULT 0,
        annotations JSONB NOT NULL DEFAULT '{}'::jsonb, -- <-- new column here
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (video_id) REFERENCES video_info(yt_id) ON DELETE CASCADE
    );
    "#,
        r#"
    CREATE INDEX idx_comments_comment_id ON comments(comment_id);
    "#,
        r#"
    CREATE INDEX idx_comments_video_id ON comments(video_id);
    "#,
        r#"
    CREATE INDEX idx_comments_channel_id ON comments(channel_id);
    "#,
        r#"
    CREATE INDEX idx_comments_reply_to ON comments(reply_to);
    "#,
        r#"
    CREATE INDEX idx_comments_annotations_gin ON comments USING GIN (annotations jsonb_path_ops);
    "#
    ];

    for query in drop_queries {
        if let Err(e) = sqlx::query(query).execute(&*app_state.db_pool).await {
            tracing::error!("Failed to execute drop query {}: {}", query, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    for query in create_queries {
        if let Err(e) = sqlx::query(query).execute(&*app_state.db_pool).await {
            tracing::error!("Failed to execute create query: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    tracing::info!("Database reset completed successfully");
    Ok(Json(json!({
        "message": "Database tables dropped and recreated successfully",
        "status": "success"
    })))
}