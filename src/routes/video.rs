use axum::{Json, extract::{State, Path}};
use serde_json::{json, Value};
use serde::{Deserialize};
use yt_scraper::{YoutubeExtractor};
use crate::db::{
    connection::AppState,
    models::{CreateVideoInfoDto, CreateCommentDto},
    operations::{VideoInfoRepository, CommentRepository}
};
use crate::routes::errors::AppError;


#[derive(Deserialize)]
pub struct VideoRequest {
    video: String
}


pub async fn video_extraction(
    State(app_state): State<AppState>,
    Json(payload): Json<VideoRequest>
) -> Result<Json<Value>, AppError> {
    if payload.video.is_empty() {
        return Err(AppError::InvalidInput("Video ID cannot be empty".to_string()));
    }

    let video_id = payload.video;
    let extractor = YoutubeExtractor::new();

    let (video_info, comments) = extractor.extract(&video_id).await
        .map_err(|e| AppError::InvalidInput(format!("Failed to extract video: {}", e)))?;

    // Check if video already exists
    if let Some(_existing_video) = VideoInfoRepository::get_by_yt_id(&app_state.db_pool, &video_info.yt_id).await? {
        // Update stats if video exists
        let updated_video = VideoInfoRepository::update_stats(
            &app_state.db_pool,
            &video_info.yt_id,
            video_info.views,
            video_info.comment_count,
            video_info.like_count
        ).await?;
        
        // Delete existing comments for this video and add new ones
        CommentRepository::delete_by_video_id(&app_state.db_pool, &video_info.yt_id).await?;
        
        // Save new comments
        let comment_dtos: Vec<CreateCommentDto> = comments.into_iter().map(|comment| {
            CreateCommentDto {
                comment_id: comment.comment_id,
                channel_id: comment.channel_id,
                video_id: video_info.yt_id.clone(), // Use video's yt_id for foreign key
                display_name: comment.display_name,
                user_verified: comment.user_verified,
                thumbnail: comment.thumbnail,
                content: comment.content,
                published_time: comment.published_time,
                like_count: comment.like_count,
                reply_count: comment.reply_count,
                comment_level: comment.comment_level,
                reply_to: comment.reply_to,
                reply_order: comment.reply_order,
            }
        }).collect();

        let saved_comments = CommentRepository::create_batch(&app_state.db_pool, comment_dtos).await?;
        
        let response = json!({
            "status": "updated",
            "video_info": updated_video,
            "comments": saved_comments,
            "message": format!("Video stats updated and {} comments refreshed", saved_comments.len())
        });
        
        return Ok(Json(response));
    }

    // Create new video info and comments in a transaction
    let video_dto = CreateVideoInfoDto {
        title: video_info.title.clone(),
        channel: video_info.channel.clone(),
        channel_id: video_info.channel_id.clone(),
        description: video_info.description.clone(),
        yt_id: video_info.yt_id.clone(),
        views: video_info.views,
        comment_count: video_info.comment_count,
        like_count: video_info.like_count,
        video_thumbnail: video_info.video_thumbnail.clone(),
        upload_date: video_info.upload_date.clone(),
        channel_thumbnail: video_info.channel_thumbnail.clone(),
    };

    // Create comments DTOs
    let comment_dtos: Vec<CreateCommentDto> = comments.into_iter().map(|comment| {
        CreateCommentDto {
            comment_id: comment.comment_id,
            channel_id: comment.channel_id,
            video_id: video_info.yt_id.clone(), // Use video's yt_id for foreign key
            display_name: comment.display_name,
            user_verified: comment.user_verified,
            thumbnail: comment.thumbnail,
            content: comment.content,
            published_time: comment.published_time,
            like_count: comment.like_count,
            reply_count: comment.reply_count,
            comment_level: comment.comment_level,
            reply_to: comment.reply_to,
            reply_order: comment.reply_order,
        }
    }).collect();

    // Save video first
    let saved_video = VideoInfoRepository::create(&app_state.db_pool, video_dto).await?;
    
    // Then save comments
    let saved_comments = CommentRepository::create_batch(&app_state.db_pool, comment_dtos).await?;

    println!("Saved video: {}", saved_video.title);
    println!("Saved {} comments", saved_comments.len());

    let response = json!({
        "status": "created",
        "video_info": saved_video,
        "comments": saved_comments,
        "message": format!("Video and {} comments saved successfully", saved_comments.len())
    });

    Ok(Json(response))
}

// Get all saved videos
pub async fn get_videos(State(app_state): State<AppState>) -> Result<Json<Value>, AppError> {
    let videos = VideoInfoRepository::get_all(&app_state.db_pool).await?;
    
    let response = json!({
        "videos": videos,
        "count": videos.len()
    });
    
    Ok(Json(response))
}

// Get a specific video by yt_id
pub async fn get_video_by_id(
    State(app_state): State<AppState>,
    Path(yt_id): Path<String>
) -> Result<Json<Value>, AppError> {
    let video = VideoInfoRepository::get_by_yt_id(&app_state.db_pool, &yt_id).await?;
    
    match video {
        Some(v) => {
            let comments = CommentRepository::get_by_video_id(&app_state.db_pool, &yt_id).await?;
            
            let response = json!({
                "video": v,
                "comments": comments,
                "comment_count": comments.len()
            });
            
            Ok(Json(response))
        }
        None => Err(AppError::InvalidInput("Video not found".to_string()))
    }
}

// Get comments for a specific video
pub async fn get_comments_by_video_id(
    State(app_state): State<AppState>,
    Path(yt_id): Path<String>
) -> Result<Json<Value>, AppError> {
    let comments = CommentRepository::get_by_video_id(&app_state.db_pool, &yt_id).await?;
    
    let response = json!({
        "video_id": yt_id,
        "comments": comments,
        "count": comments.len()
    });
    
    Ok(Json(response))
}