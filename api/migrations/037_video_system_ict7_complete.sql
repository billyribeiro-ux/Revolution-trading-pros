-- ICT Level 7 Video System Completion
-- ═══════════════════════════════════════════════════════════════════════════════
-- Apple Principal Engineer Grade - February 2026
--
-- This migration adds:
-- 1. video_watch_progress table for resume functionality
-- 2. Video transcript table for captions/subtitles
-- 3. Performance indexes for video queries
-- ═══════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════
-- VIDEO WATCH PROGRESS TABLE
-- Tracks user's watch position for resume functionality
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_watch_progress (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    current_time_seconds INTEGER NOT NULL DEFAULT 0,
    completion_percent INTEGER NOT NULL DEFAULT 0 CHECK (completion_percent >= 0 AND completion_percent <= 100),
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    last_watched_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- Unique constraint: one progress record per user per video
    CONSTRAINT uq_user_video_progress UNIQUE (user_id, video_id)
);

-- Index for fast user history queries
CREATE INDEX IF NOT EXISTS idx_watch_progress_user_date ON video_watch_progress(user_id, last_watched_at DESC);

-- Index for video analytics
CREATE INDEX IF NOT EXISTS idx_watch_progress_video ON video_watch_progress(video_id);

-- Index for finding incomplete videos (for "continue watching")
CREATE INDEX IF NOT EXISTS idx_watch_progress_incomplete ON video_watch_progress(user_id, completed, last_watched_at DESC)
WHERE completed = FALSE;

COMMENT ON TABLE video_watch_progress IS 'Tracks user video watch progress for resume and analytics';
COMMENT ON COLUMN video_watch_progress.current_time_seconds IS 'Current playback position in seconds';
COMMENT ON COLUMN video_watch_progress.completion_percent IS 'Percentage of video watched (0-100)';
COMMENT ON COLUMN video_watch_progress.completed IS 'Whether user watched at least 90% of video';

-- ═══════════════════════════════════════════════════════════════════════════════
-- VIDEO TRANSCRIPTS TABLE
-- Stores VTT/SRT transcript data for captions
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS video_transcripts (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    language_code VARCHAR(10) NOT NULL DEFAULT 'en',
    language_label VARCHAR(100) NOT NULL DEFAULT 'English',
    transcript_type VARCHAR(50) NOT NULL DEFAULT 'subtitles' CHECK (transcript_type IN ('subtitles', 'captions', 'descriptions')),
    vtt_url TEXT,
    srt_url TEXT,
    raw_text TEXT,
    is_auto_generated BOOLEAN NOT NULL DEFAULT FALSE,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- One transcript per language per video
    CONSTRAINT uq_video_transcript_lang UNIQUE (video_id, language_code)
);

-- Index for finding transcripts by video
CREATE INDEX IF NOT EXISTS idx_transcript_video ON video_transcripts(video_id);

-- Index for default transcripts
CREATE INDEX IF NOT EXISTS idx_transcript_default ON video_transcripts(video_id, is_default)
WHERE is_default = TRUE;

COMMENT ON TABLE video_transcripts IS 'Video transcripts and captions for accessibility';
COMMENT ON COLUMN video_transcripts.transcript_type IS 'Type: subtitles, captions (for deaf), or descriptions (for blind)';

-- ═══════════════════════════════════════════════════════════════════════════════
-- PERFORMANCE INDEXES FOR VIDEO QUERIES
-- Optimizes common video listing and filtering operations
-- ═══════════════════════════════════════════════════════════════════════════════

-- Weekly watchlist videos index
CREATE INDEX IF NOT EXISTS idx_videos_weekly_watchlist ON unified_videos(video_date DESC, created_at DESC)
WHERE content_type = 'weekly_watchlist' AND is_published = TRUE AND deleted_at IS NULL;

-- Tag filtering index (GIN for JSONB array containment)
CREATE INDEX IF NOT EXISTS idx_videos_tags_gin ON unified_videos USING GIN (tags jsonb_path_ops)
WHERE deleted_at IS NULL;

-- Related videos index (content_type + recent)
CREATE INDEX IF NOT EXISTS idx_videos_related ON unified_videos(content_type, video_date DESC)
WHERE is_published = TRUE AND deleted_at IS NULL;

-- Views count index for popular videos
CREATE INDEX IF NOT EXISTS idx_videos_popular ON unified_videos(views_count DESC)
WHERE is_published = TRUE AND deleted_at IS NULL;

-- ═══════════════════════════════════════════════════════════════════════════════
-- FUNCTION: Update timestamps automatically
-- ═══════════════════════════════════════════════════════════════════════════════

-- Ensure trigger function exists
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add triggers for automatic updated_at
DROP TRIGGER IF EXISTS trigger_watch_progress_updated ON video_watch_progress;
CREATE TRIGGER trigger_watch_progress_updated
    BEFORE UPDATE ON video_watch_progress
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_transcript_updated ON video_transcripts;
CREATE TRIGGER trigger_transcript_updated
    BEFORE UPDATE ON video_transcripts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ═══════════════════════════════════════════════════════════════════════════════
-- ANALYTICS HELPER: Get video completion stats
-- ═══════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION get_video_completion_stats(p_video_id BIGINT)
RETURNS TABLE (
    total_views BIGINT,
    unique_viewers BIGINT,
    avg_completion_percent NUMERIC,
    completion_rate NUMERIC,
    total_watch_time_seconds BIGINT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        COUNT(*)::BIGINT as total_views,
        COUNT(DISTINCT user_id)::BIGINT as unique_viewers,
        ROUND(AVG(completion_percent)::NUMERIC, 2) as avg_completion_percent,
        ROUND((COUNT(*) FILTER (WHERE completed = TRUE)::NUMERIC / NULLIF(COUNT(*)::NUMERIC, 0)) * 100, 2) as completion_rate,
        SUM(current_time_seconds)::BIGINT as total_watch_time_seconds
    FROM video_watch_progress
    WHERE video_id = p_video_id;
END;
$$ LANGUAGE plpgsql STABLE;

COMMENT ON FUNCTION get_video_completion_stats IS 'Returns completion statistics for a video';

-- ═══════════════════════════════════════════════════════════════════════════════
-- GRANT PERMISSIONS (if roles exist)
-- ═══════════════════════════════════════════════════════════════════════════════

-- These will silently fail if roles don't exist
DO $$
BEGIN
    EXECUTE 'GRANT SELECT, INSERT, UPDATE ON video_watch_progress TO app_user';
    EXECUTE 'GRANT SELECT, INSERT, UPDATE, DELETE ON video_transcripts TO app_admin';
EXCEPTION WHEN undefined_object THEN
    -- Role doesn't exist, skip
    NULL;
END $$;
