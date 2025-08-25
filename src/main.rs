use axum::{routing::{get, post}, Router};
use tower_http::cors::CorsLayer;

mod routes;
mod db;

use crate::db::connection::{get_connection, AppState};

async fn hello_world() -> &'static str {
    "Youtube Server"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_state = get_connection().await.unwrap();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(routes::health::health_check))
        .route("/video-extraction", post(routes::video::video_extraction))
        .route("/videos", get(routes::video::get_videos))
        .route("/videos/{yt_id}", get(routes::video::get_video_by_id))
        .route("/videos/{yt_id}/comments", get(routes::video::get_comments_by_video_id))
        .route("/reset-database", post(routes::database::reset_database))
        .layer(CorsLayer::permissive())
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}