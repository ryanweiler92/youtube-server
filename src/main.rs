use axum::{Router, routing::{get, post}};
use error::AppError;
use tower_http::{
    cors::CorsLayer,
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse},
};
use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};

mod routes;
mod db;
mod ai;
mod error;

use crate::db::connection::{get_connection, AppState};

async fn hello_world() -> &'static str {
    "Youtube Server"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let app_state = get_connection().await.unwrap();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(routes::health::health_check))
        .route("/video-extraction", post(routes::video::video_extraction))
        .route("/videos", get(routes::video::get_videos))
        .route("/videos/{yt_id}", get(routes::video::get_video_by_id))
        .route("/videos/{yt_id}/comments", get(routes::video::get_comments_by_video_id))
        .route("/reset-database", post(routes::database::reset_database))
        .route("/ner", post(routes::ner_route::ner_operation))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{addr}");

    axum::serve(listener, app).await.unwrap();
}