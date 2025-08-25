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
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (video_id) REFERENCES video_info(yt_id) ON DELETE CASCADE
);

CREATE INDEX idx_comments_comment_id ON comments(comment_id);
CREATE INDEX idx_comments_video_id ON comments(video_id);
CREATE INDEX idx_comments_channel_id ON comments(channel_id);
CREATE INDEX idx_comments_reply_to ON comments(reply_to);