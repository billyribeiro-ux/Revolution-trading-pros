-- ═══════════════════════════════════════════════════════════════════════════════════
-- REVOLUTION TRADING PROS - ADVANCED VIDEO FEATURES MIGRATION
-- Apple Principal Engineer ICT 7 Grade - January 2026
--
-- Features: Analytics, Series/Playlists, Chapters, Transcriptions, Scheduled Publishing
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. VIDEO ANALYTICS TRACKING
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Main analytics events table (append-only for performance)
CREATE TABLE IF NOT EXISTS video_analytics_events (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    session_id VARCHAR(100),

    -- Event details
    event_type VARCHAR(50) NOT NULL,  -- view, play, pause, complete, progress, seek, quality_change, speed_change, buffer
    event_data JSONB DEFAULT '{}',    -- Additional event-specific data

    -- Progress tracking
    watch_time_seconds INTEGER DEFAULT 0,
    progress_percent SMALLINT CHECK (progress_percent >= 0 AND progress_percent <= 100),

    -- Quality metrics
    buffer_count INTEGER DEFAULT 0,
    quality_level VARCHAR(20),        -- 240p, 360p, 480p, 720p, 1080p, 4k
    playback_speed DECIMAL(3, 2) DEFAULT 1.0,

    -- Device & environment
    device_type VARCHAR(20),          -- desktop, mobile, tablet, tv
    browser VARCHAR(50),
    os VARCHAR(50),
    screen_width INTEGER,
    screen_height INTEGER,
    connection_type VARCHAR(20),      -- 4g, wifi, ethernet, etc.

    -- Geolocation (privacy-safe)
    country_code CHAR(2),
    region_code VARCHAR(10),

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Aggregated daily stats for fast dashboard queries
CREATE TABLE IF NOT EXISTS video_analytics_daily (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    analytics_date DATE NOT NULL,

    -- View metrics
    unique_viewers INTEGER DEFAULT 0,
    total_views INTEGER DEFAULT 0,

    -- Watch time
    total_watch_time_seconds BIGINT DEFAULT 0,
    avg_watch_time_seconds INTEGER DEFAULT 0,

    -- Engagement
    avg_completion_percent SMALLINT DEFAULT 0,
    completed_count INTEGER DEFAULT 0,  -- Users who watched >= 90%

    -- Quality metrics
    avg_buffer_count DECIMAL(5, 2) DEFAULT 0,

    -- Device breakdown (JSONB for flexibility)
    device_breakdown JSONB DEFAULT '{}',  -- {"desktop": 45, "mobile": 55}
    quality_breakdown JSONB DEFAULT '{}', -- {"720p": 30, "1080p": 70}

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(video_id, analytics_date)
);

-- User watch history for personalization
CREATE TABLE IF NOT EXISTS video_watch_history (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,

    -- Progress
    last_position_seconds INTEGER DEFAULT 0,
    total_watch_time_seconds INTEGER DEFAULT 0,
    completion_percent SMALLINT DEFAULT 0,

    -- Status
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,

    -- Engagement
    rating SMALLINT CHECK (rating >= 1 AND rating <= 5),
    is_bookmarked BOOLEAN DEFAULT false,
    notes TEXT,

    -- Timestamps
    first_watched_at TIMESTAMPTZ DEFAULT NOW(),
    last_watched_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, video_id)
);

-- Indexes for analytics performance
CREATE INDEX IF NOT EXISTS idx_analytics_events_video_date ON video_analytics_events(video_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_analytics_events_user ON video_analytics_events(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_analytics_events_session ON video_analytics_events(session_id);
CREATE INDEX IF NOT EXISTS idx_analytics_events_type ON video_analytics_events(event_type);
CREATE INDEX IF NOT EXISTS idx_analytics_daily_video_date ON video_analytics_daily(video_id, analytics_date DESC);
CREATE INDEX IF NOT EXISTS idx_watch_history_user ON video_watch_history(user_id, last_watched_at DESC);
CREATE INDEX IF NOT EXISTS idx_watch_history_video ON video_watch_history(video_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. VIDEO SERIES & PLAYLISTS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_series (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    thumbnail_url VARCHAR(500),

    -- Classification
    content_type VARCHAR(50) NOT NULL,  -- learning_path, collection, course, playlist
    difficulty_level VARCHAR(50),       -- beginner, intermediate, advanced
    category VARCHAR(100),

    -- Access control
    is_published BOOLEAN DEFAULT false,
    is_premium BOOLEAN DEFAULT false,
    required_plan_id BIGINT,

    -- Ordering
    sort_order INTEGER DEFAULT 0,

    -- Metadata
    estimated_duration_minutes INTEGER,
    video_count INTEGER DEFAULT 0,
    tags JSONB DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}',

    -- Audit
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    updated_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Junction table for series videos with ordering
CREATE TABLE IF NOT EXISTS video_series_items (
    id BIGSERIAL PRIMARY KEY,
    series_id BIGINT NOT NULL REFERENCES video_series(id) ON DELETE CASCADE,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,

    -- Ordering within series
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Optional chapter/section grouping
    section_title VARCHAR(255),
    section_order INTEGER DEFAULT 0,

    -- Item-specific overrides
    custom_title VARCHAR(255),       -- Override video title for this series
    is_preview BOOLEAN DEFAULT false, -- Free preview even in premium series

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(series_id, video_id)
);

-- User progress through series
CREATE TABLE IF NOT EXISTS user_series_progress (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    series_id BIGINT NOT NULL REFERENCES video_series(id) ON DELETE CASCADE,

    -- Progress
    completed_videos INTEGER DEFAULT 0,
    total_videos INTEGER DEFAULT 0,
    completion_percent SMALLINT DEFAULT 0,

    -- Current position
    current_video_id BIGINT REFERENCES unified_videos(id) ON DELETE SET NULL,
    last_position_seconds INTEGER DEFAULT 0,

    -- Status
    is_started BOOLEAN DEFAULT false,
    is_completed BOOLEAN DEFAULT false,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, series_id)
);

-- Series indexes
CREATE INDEX IF NOT EXISTS idx_video_series_slug ON video_series(slug);
CREATE INDEX IF NOT EXISTS idx_video_series_content_type ON video_series(content_type);
CREATE INDEX IF NOT EXISTS idx_video_series_published ON video_series(is_published) WHERE is_published = true;
CREATE INDEX IF NOT EXISTS idx_series_items_series ON video_series_items(series_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_series_items_video ON video_series_items(video_id);
CREATE INDEX IF NOT EXISTS idx_user_series_progress ON user_series_progress(user_id, series_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. VIDEO CHAPTERS (Enhanced from JSONB)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_chapters (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,

    -- Chapter details
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_time_seconds INTEGER NOT NULL,
    end_time_seconds INTEGER,

    -- Visual
    thumbnail_url VARCHAR(500),
    thumbnail_time_seconds INTEGER,  -- Time to capture thumbnail from video

    -- Ordering
    chapter_number INTEGER NOT NULL,

    -- Metadata
    tags JSONB DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}',

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Chapter indexes
CREATE INDEX IF NOT EXISTS idx_video_chapters_video ON video_chapters(video_id, chapter_number);
CREATE INDEX IF NOT EXISTS idx_video_chapters_time ON video_chapters(video_id, start_time_seconds);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. VIDEO TRANSCRIPTIONS & CAPTIONS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_transcriptions (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,

    -- Language
    language_code VARCHAR(10) NOT NULL DEFAULT 'en',  -- ISO 639-1
    language_name VARCHAR(50) NOT NULL DEFAULT 'English',

    -- Transcription type
    transcription_type VARCHAR(50) NOT NULL,  -- auto, manual, professional

    -- Content
    full_text TEXT,                    -- Full plain text transcription
    segments JSONB,                     -- [{start: 0, end: 5, text: "..."}, ...]

    -- Caption formats (generated from segments)
    vtt_content TEXT,                   -- WebVTT format
    srt_content TEXT,                   -- SubRip format

    -- AI metadata
    summary TEXT,                       -- AI-generated summary
    key_topics JSONB DEFAULT '[]',      -- Extracted key topics
    sentiment_analysis JSONB,           -- Overall sentiment

    -- Status
    status VARCHAR(50) DEFAULT 'pending',  -- pending, processing, completed, failed
    error_message TEXT,

    -- Provider info
    provider VARCHAR(50),               -- openai, assemblyai, deepgram, manual
    provider_job_id VARCHAR(255),
    confidence_score DECIMAL(5, 4),     -- 0.0000 to 1.0000

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,

    UNIQUE(video_id, language_code)
);

-- Transcription indexes
CREATE INDEX IF NOT EXISTS idx_transcriptions_video ON video_transcriptions(video_id);
CREATE INDEX IF NOT EXISTS idx_transcriptions_status ON video_transcriptions(status);
CREATE INDEX IF NOT EXISTS idx_transcriptions_fulltext ON video_transcriptions USING GIN(to_tsvector('english', full_text));

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. SCHEDULED PUBLISHING QUEUE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS scheduled_publish_jobs (
    id BIGSERIAL PRIMARY KEY,

    -- Resource reference
    resource_type VARCHAR(50) NOT NULL,  -- video, series, blog_post
    resource_id BIGINT NOT NULL,

    -- Schedule
    scheduled_at TIMESTAMPTZ NOT NULL,
    timezone VARCHAR(50) DEFAULT 'America/New_York',

    -- Action
    action VARCHAR(50) NOT NULL DEFAULT 'publish',  -- publish, unpublish, feature, unfeature
    action_data JSONB DEFAULT '{}',

    -- Status
    status VARCHAR(50) DEFAULT 'pending',  -- pending, processing, completed, failed, cancelled
    error_message TEXT,
    processed_at TIMESTAMPTZ,

    -- Notification
    notify_on_publish BOOLEAN DEFAULT false,
    notification_recipients JSONB DEFAULT '[]',  -- Email addresses

    -- Audit
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    cancelled_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Scheduled jobs indexes
CREATE INDEX IF NOT EXISTS idx_scheduled_jobs_status_time ON scheduled_publish_jobs(status, scheduled_at)
    WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_scheduled_jobs_resource ON scheduled_publish_jobs(resource_type, resource_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. BUNNY.NET WEBHOOK EVENTS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS bunny_webhook_events (
    id BIGSERIAL PRIMARY KEY,

    -- Event identification
    event_id VARCHAR(100) UNIQUE,
    event_type VARCHAR(100) NOT NULL,  -- VideoCreated, VideoEncoded, VideoUploaded, VideoDeleted, etc.

    -- Video reference
    video_guid VARCHAR(100) NOT NULL,
    library_id BIGINT,

    -- Payload
    payload JSONB NOT NULL,

    -- Processing status
    status VARCHAR(50) DEFAULT 'received',  -- received, processing, completed, failed
    processed_at TIMESTAMPTZ,
    error_message TEXT,

    -- Timestamps
    received_at TIMESTAMPTZ DEFAULT NOW()
);

-- Webhook indexes
CREATE INDEX IF NOT EXISTS idx_bunny_webhook_guid ON bunny_webhook_events(video_guid);
CREATE INDEX IF NOT EXISTS idx_bunny_webhook_status ON bunny_webhook_events(status) WHERE status != 'completed';
CREATE INDEX IF NOT EXISTS idx_bunny_webhook_event_type ON bunny_webhook_events(event_type);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. ENHANCED UNIFIED_VIDEOS COLUMNS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add new columns to unified_videos if they don't exist
DO $$
BEGIN
    -- Series reference
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'series_id') THEN
        ALTER TABLE unified_videos ADD COLUMN series_id BIGINT REFERENCES video_series(id) ON DELETE SET NULL;
    END IF;

    -- Series position
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'series_order') THEN
        ALTER TABLE unified_videos ADD COLUMN series_order INTEGER DEFAULT 0;
    END IF;

    -- Transcription status
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'has_transcription') THEN
        ALTER TABLE unified_videos ADD COLUMN has_transcription BOOLEAN DEFAULT false;
    END IF;

    -- Captions status
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'has_captions') THEN
        ALTER TABLE unified_videos ADD COLUMN has_captions BOOLEAN DEFAULT false;
    END IF;

    -- Rich description (HTML/Markdown)
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'description_html') THEN
        ALTER TABLE unified_videos ADD COLUMN description_html TEXT;
    END IF;

    -- Description format
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'description_format') THEN
        ALTER TABLE unified_videos ADD COLUMN description_format VARCHAR(20) DEFAULT 'plain';  -- plain, markdown, html
    END IF;

    -- Encoding progress
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'encoding_progress') THEN
        ALTER TABLE unified_videos ADD COLUMN encoding_progress SMALLINT DEFAULT 0;
    END IF;

    -- Original filename
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'original_filename') THEN
        ALTER TABLE unified_videos ADD COLUMN original_filename VARCHAR(500);
    END IF;

    -- File size
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'file_size_bytes') THEN
        ALTER TABLE unified_videos ADD COLUMN file_size_bytes BIGINT;
    END IF;

    -- Resolution
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'width') THEN
        ALTER TABLE unified_videos ADD COLUMN width INTEGER;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'height') THEN
        ALTER TABLE unified_videos ADD COLUMN height INTEGER;
    END IF;

    -- Cloned from reference
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name = 'unified_videos' AND column_name = 'cloned_from_id') THEN
        ALTER TABLE unified_videos ADD COLUMN cloned_from_id BIGINT REFERENCES unified_videos(id) ON DELETE SET NULL;
    END IF;
END $$;

-- Additional indexes for new columns
CREATE INDEX IF NOT EXISTS idx_unified_videos_series ON unified_videos(series_id) WHERE series_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_unified_videos_transcription ON unified_videos(has_transcription) WHERE has_transcription = true;
CREATE INDEX IF NOT EXISTS idx_unified_videos_scheduled ON unified_videos(scheduled_at) WHERE scheduled_at IS NOT NULL;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 8. VIDEO UPLOAD QUEUE (for bulk uploads)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_upload_queue (
    id BIGSERIAL PRIMARY KEY,

    -- Batch grouping
    batch_id UUID NOT NULL,
    batch_order INTEGER NOT NULL DEFAULT 0,

    -- File info
    original_filename VARCHAR(500) NOT NULL,
    file_size_bytes BIGINT,
    content_type VARCHAR(100),

    -- Upload status
    status VARCHAR(50) DEFAULT 'pending',  -- pending, uploading, processing, completed, failed
    progress_percent SMALLINT DEFAULT 0,
    error_message TEXT,

    -- Bunny.net references
    bunny_video_guid VARCHAR(100),
    bunny_upload_url TEXT,

    -- Video metadata (to be used when creating video record)
    video_metadata JSONB NOT NULL,  -- title, description, content_type, tags, etc.

    -- Created video reference
    created_video_id BIGINT REFERENCES unified_videos(id) ON DELETE SET NULL,

    -- Audit
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Upload queue indexes
CREATE INDEX IF NOT EXISTS idx_upload_queue_batch ON video_upload_queue(batch_id, batch_order);
CREATE INDEX IF NOT EXISTS idx_upload_queue_status ON video_upload_queue(status) WHERE status NOT IN ('completed', 'failed');
CREATE INDEX IF NOT EXISTS idx_upload_queue_user ON video_upload_queue(created_by);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 9. FUNCTIONS & TRIGGERS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Function to update video_series video count
CREATE OR REPLACE FUNCTION update_series_video_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE video_series
        SET video_count = (SELECT COUNT(*) FROM video_series_items WHERE series_id = NEW.series_id),
            updated_at = NOW()
        WHERE id = NEW.series_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE video_series
        SET video_count = (SELECT COUNT(*) FROM video_series_items WHERE series_id = OLD.series_id),
            updated_at = NOW()
        WHERE id = OLD.series_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger for series video count
DROP TRIGGER IF EXISTS trigger_update_series_video_count ON video_series_items;
CREATE TRIGGER trigger_update_series_video_count
    AFTER INSERT OR DELETE ON video_series_items
    FOR EACH ROW
    EXECUTE FUNCTION update_series_video_count();

-- Function to aggregate daily analytics (called by cron/job)
CREATE OR REPLACE FUNCTION aggregate_video_analytics_daily(target_date DATE DEFAULT CURRENT_DATE - INTERVAL '1 day')
RETURNS INTEGER AS $$
DECLARE
    videos_processed INTEGER := 0;
BEGIN
    -- Insert or update daily aggregates
    INSERT INTO video_analytics_daily (
        video_id,
        analytics_date,
        unique_viewers,
        total_views,
        total_watch_time_seconds,
        avg_watch_time_seconds,
        avg_completion_percent,
        completed_count,
        avg_buffer_count,
        device_breakdown,
        quality_breakdown
    )
    SELECT
        video_id,
        target_date,
        COUNT(DISTINCT COALESCE(user_id::text, session_id)),
        COUNT(*) FILTER (WHERE event_type = 'view'),
        COALESCE(SUM(watch_time_seconds), 0),
        COALESCE(AVG(watch_time_seconds)::INTEGER, 0),
        COALESCE(AVG(progress_percent)::SMALLINT, 0),
        COUNT(*) FILTER (WHERE progress_percent >= 90),
        COALESCE(AVG(buffer_count), 0),
        jsonb_object_agg(
            COALESCE(device_type, 'unknown'),
            device_count
        ) FILTER (WHERE device_type IS NOT NULL),
        jsonb_object_agg(
            COALESCE(quality_level, 'auto'),
            quality_count
        ) FILTER (WHERE quality_level IS NOT NULL)
    FROM video_analytics_events
    LEFT JOIN LATERAL (
        SELECT device_type, COUNT(*) as device_count
        FROM video_analytics_events e2
        WHERE e2.video_id = video_analytics_events.video_id
          AND DATE(e2.created_at) = target_date
        GROUP BY device_type
    ) device_stats ON true
    LEFT JOIN LATERAL (
        SELECT quality_level, COUNT(*) as quality_count
        FROM video_analytics_events e3
        WHERE e3.video_id = video_analytics_events.video_id
          AND DATE(e3.created_at) = target_date
        GROUP BY quality_level
    ) quality_stats ON true
    WHERE DATE(created_at) = target_date
    GROUP BY video_id
    ON CONFLICT (video_id, analytics_date)
    DO UPDATE SET
        unique_viewers = EXCLUDED.unique_viewers,
        total_views = EXCLUDED.total_views,
        total_watch_time_seconds = EXCLUDED.total_watch_time_seconds,
        avg_watch_time_seconds = EXCLUDED.avg_watch_time_seconds,
        avg_completion_percent = EXCLUDED.avg_completion_percent,
        completed_count = EXCLUDED.completed_count,
        avg_buffer_count = EXCLUDED.avg_buffer_count,
        device_breakdown = EXCLUDED.device_breakdown,
        quality_breakdown = EXCLUDED.quality_breakdown,
        updated_at = NOW();

    GET DIAGNOSTICS videos_processed = ROW_COUNT;
    RETURN videos_processed;
END;
$$ LANGUAGE plpgsql;

-- Function to process scheduled publish jobs
CREATE OR REPLACE FUNCTION process_scheduled_jobs()
RETURNS INTEGER AS $$
DECLARE
    processed_count INTEGER := 0;
    job RECORD;
BEGIN
    FOR job IN
        SELECT * FROM scheduled_publish_jobs
        WHERE status = 'pending'
          AND scheduled_at <= NOW()
        ORDER BY scheduled_at
        FOR UPDATE SKIP LOCKED
        LIMIT 100
    LOOP
        -- Update job to processing
        UPDATE scheduled_publish_jobs SET status = 'processing', updated_at = NOW() WHERE id = job.id;

        BEGIN
            -- Execute the action based on resource type
            IF job.resource_type = 'video' THEN
                IF job.action = 'publish' THEN
                    UPDATE unified_videos SET is_published = true, published_at = NOW() WHERE id = job.resource_id;
                ELSIF job.action = 'unpublish' THEN
                    UPDATE unified_videos SET is_published = false WHERE id = job.resource_id;
                ELSIF job.action = 'feature' THEN
                    UPDATE unified_videos SET is_featured = true WHERE id = job.resource_id;
                ELSIF job.action = 'unfeature' THEN
                    UPDATE unified_videos SET is_featured = false WHERE id = job.resource_id;
                END IF;
            ELSIF job.resource_type = 'series' THEN
                IF job.action = 'publish' THEN
                    UPDATE video_series SET is_published = true, updated_at = NOW() WHERE id = job.resource_id;
                ELSIF job.action = 'unpublish' THEN
                    UPDATE video_series SET is_published = false, updated_at = NOW() WHERE id = job.resource_id;
                END IF;
            END IF;

            -- Mark as completed
            UPDATE scheduled_publish_jobs
            SET status = 'completed', processed_at = NOW(), updated_at = NOW()
            WHERE id = job.id;

            processed_count := processed_count + 1;
        EXCEPTION WHEN OTHERS THEN
            -- Mark as failed with error
            UPDATE scheduled_publish_jobs
            SET status = 'failed', error_message = SQLERRM, updated_at = NOW()
            WHERE id = job.id;
        END;
    END LOOP;

    RETURN processed_count;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 10. SAMPLE DATA (Optional - can be removed in production)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Insert sample series if table is empty
INSERT INTO video_series (title, slug, description, content_type, difficulty_level, is_published)
SELECT 'Options Trading Fundamentals', 'options-trading-fundamentals',
       'Complete beginner course covering options basics, strategies, and risk management.',
       'learning_path', 'beginner', true
WHERE NOT EXISTS (SELECT 1 FROM video_series LIMIT 1);

INSERT INTO video_series (title, slug, description, content_type, difficulty_level, is_published)
SELECT 'Technical Analysis Masterclass', 'technical-analysis-masterclass',
       'Advanced technical analysis patterns, indicators, and trading strategies.',
       'learning_path', 'advanced', true
WHERE NOT EXISTS (SELECT 1 FROM video_series WHERE slug = 'technical-analysis-masterclass');

-- Grant permissions (adjust as needed for your role setup)
-- GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO api_user;
-- GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO api_user;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- MIGRATION COMPLETE
-- ═══════════════════════════════════════════════════════════════════════════════════
