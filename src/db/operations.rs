use sqlx::PgPool;
use crate::db::models::{VideoInfo, Comment, CreateVideoInfoDto, CreateCommentDto, CommentContentAndId};
use crate::routes::errors::AppError;

pub struct VideoInfoRepository;

impl VideoInfoRepository {
    pub async fn create(pool: &PgPool, video_dto: CreateVideoInfoDto) -> Result<VideoInfo, AppError> {
        let video = sqlx::query_as!(
            VideoInfo,
            r#"
            INSERT INTO video_info 
            (title, channel, channel_id, description, yt_id, views, comment_count, like_count, video_thumbnail, upload_date, channel_thumbnail)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
            video_dto.title,
            video_dto.channel,
            video_dto.channel_id,
            Some(video_dto.description),
            video_dto.yt_id,
            video_dto.views as i64,
            video_dto.comment_count as i64,
            video_dto.like_count as i64,
            Some(video_dto.video_thumbnail),
            Some(video_dto.upload_date),
            Some(video_dto.channel_thumbnail)
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(video)
    }

    pub async fn get_by_yt_id(pool: &PgPool, yt_id: &str) -> Result<Option<VideoInfo>, AppError> {
        let video = sqlx::query_as!(
            VideoInfo,
            "SELECT * FROM video_info WHERE yt_id = $1",
            yt_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(video)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<VideoInfo>, AppError> {
        let videos = sqlx::query_as!(
            VideoInfo,
            "SELECT * FROM video_info ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(videos)
    }

    pub async fn update_stats(
        pool: &PgPool, 
        yt_id: &str, 
        views: u64, 
        comment_count: u64, 
        like_count: u64
    ) -> Result<VideoInfo, AppError> {
        let video = sqlx::query_as!(
            VideoInfo,
            r#"
            UPDATE video_info 
            SET views = $2, comment_count = $3, like_count = $4, updated_at = CURRENT_TIMESTAMP
            WHERE yt_id = $1
            RETURNING *
            "#,
            yt_id,
            views as i64,
            comment_count as i64,
            like_count as i64
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(video)
    }

    pub async fn create_with_comments(
        pool: &PgPool,
        video_dto: CreateVideoInfoDto,
        comment_dtos: Vec<CreateCommentDto>,
    ) -> Result<(VideoInfo, Vec<Comment>), AppError> {
        let mut tx = pool.begin().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // First create the video
        let video = sqlx::query_as!(
            VideoInfo,
            r#"
            INSERT INTO video_info 
            (title, channel, channel_id, description, yt_id, views, comment_count, like_count, video_thumbnail, upload_date, channel_thumbnail)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
            video_dto.title,
            video_dto.channel,
            video_dto.channel_id,
            Some(video_dto.description),
            video_dto.yt_id,
            video_dto.views as i64,
            video_dto.comment_count as i64,
            video_dto.like_count as i64,
            Some(video_dto.video_thumbnail),
            Some(video_dto.upload_date),
            Some(video_dto.channel_thumbnail)
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Then create all comments
        let mut comments = Vec::new();
        for comment_dto in comment_dtos {
            let comment = sqlx::query_as!(
                Comment,
                r#"
                INSERT INTO comments 
                (comment_id, channel_id, video_id, display_name, user_verified, thumbnail, content, 
                 published_time, like_count, reply_count, comment_level, reply_to, reply_order, annotations)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                RETURNING *
                "#,
                comment_dto.comment_id,
                comment_dto.channel_id,
                video.yt_id,
                comment_dto.display_name,
                Some(comment_dto.user_verified),
                Some(comment_dto.thumbnail),
                comment_dto.content,
                Some(comment_dto.published_time),
                Some(comment_dto.like_count),
                Some(comment_dto.reply_count),
                Some(comment_dto.comment_level),
                Some(comment_dto.reply_to),
                Some(comment_dto.reply_order),
                Some(comment_dto.annotations),
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
            comments.push(comment);
        }

        tx.commit().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok((video, comments))
    }
}

pub struct CommentRepository;

impl CommentRepository {
    pub async fn create(pool: &PgPool, comment_dto: CreateCommentDto) -> Result<Comment, AppError> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments 
            (comment_id, channel_id, video_id, display_name, user_verified, thumbnail, content, 
             published_time, like_count, reply_count, comment_level, reply_to, reply_order, annotations)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
            comment_dto.comment_id,
            comment_dto.channel_id,
            comment_dto.video_id,
            comment_dto.display_name,
            Some(comment_dto.user_verified),
            Some(comment_dto.thumbnail),
            comment_dto.content,
            Some(comment_dto.published_time),
            Some(comment_dto.like_count),
            Some(comment_dto.reply_count),
            Some(comment_dto.comment_level),
            Some(comment_dto.reply_to),
            Some(comment_dto.reply_order),
            Some(comment_dto.annotations)
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(comment)
    }

    pub async fn create_batch(pool: &PgPool, comments: Vec<CreateCommentDto>) -> Result<Vec<Comment>, AppError> {
        let mut created_comments = Vec::new();
        
        for comment_dto in comments {
            let comment = Self::create(pool, comment_dto).await?;
            created_comments.push(comment);
        }
        
        Ok(created_comments)
    }

    pub async fn get_by_video_id(pool: &PgPool, video_id: &str) -> Result<Vec<Comment>, AppError> {
        let comments = sqlx::query_as!(
            Comment,
            r#"
            SELECT comment_id, channel_id, video_id, display_name, user_verified, thumbnail, content,
                   published_time, like_count, reply_count, comment_level, reply_to, reply_order, annotations, created_at, updated_at, id
            FROM comments
            WHERE video_id = $1
            ORDER BY comment_level ASC, reply_order ASC, published_time ASC
            "#,
            video_id
        )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(comments)
    }

    pub async fn get_by_comment_id(pool: &PgPool, comment_id: &str) -> Result<Option<Comment>, AppError> {
        let comment = sqlx::query_as!(
        Comment,
        r#"
        SELECT comment_id, channel_id, video_id, display_name, user_verified, thumbnail, content,
               published_time, like_count, reply_count, comment_level, reply_to, reply_order, annotations, created_at, updated_at, id
        FROM comments
        WHERE comment_id = $1
        "#,
        comment_id
    )
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(comment)
    }

    pub async fn delete_by_video_id(pool: &PgPool, video_id: &str) -> Result<u64, AppError> {
        let result = sqlx::query!(
            "DELETE FROM comments WHERE video_id = $1",
            video_id
        )
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }

    pub fn get_comment_content_and_ids(comments: Vec<Comment>) -> Vec<CommentContentAndId> {
        let mut comments_with_ids: Vec<CommentContentAndId> = Vec::new();

        for comment in comments {
            comments_with_ids.push(
                CommentContentAndId{
                        id: comment.comment_id,
                        comment: comment.content
                    })
            }
        comments_with_ids
    }
}