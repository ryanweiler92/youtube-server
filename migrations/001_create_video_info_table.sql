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

CREATE INDEX idx_video_info_yt_id ON video_info(yt_id);
CREATE INDEX idx_video_info_channel_id ON video_info(channel_id);