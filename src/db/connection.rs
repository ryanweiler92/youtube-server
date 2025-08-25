use sqlx::{PgPool};
use std::{sync::Arc};

use crate::routes::errors::AppError;


#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

pub async fn get_connection() -> Result<AppState, AppError>  {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:admin@localhost/youtube_db".to_string());
    
    let db_pool = PgPool::connect(&database_url)
        .await
        .map_err(|e| AppError::FailedDBConnection(e.to_string()))?;

    let state = AppState {
        db_pool: Arc::new(db_pool)
    };

    Ok(state)
}