-- Forward repair for the established unified-video handlers. The consolidated
-- installation created a smaller table than the actual CRUD/provider queries.
-- Unknown author, provider metadata and publication dates remain unknown: this
-- migration neither copies author names nor assigns/backfills publication dates.
ALTER TABLE unified_videos
    ADD COLUMN IF NOT EXISTS trader_id BIGINT,
    ADD COLUMN IF NOT EXISTS difficulty_level VARCHAR(50),
    ADD COLUMN IF NOT EXISTS category VARCHAR(100),
    ADD COLUMN IF NOT EXISTS session_type VARCHAR(50),
    ADD COLUMN IF NOT EXISTS video_id VARCHAR(100),
    ADD COLUMN IF NOT EXISTS bunny_encoding_status VARCHAR(50),
    ADD COLUMN IF NOT EXISTS bunny_thumbnail_url TEXT,
    ADD COLUMN IF NOT EXISTS metadata JSONB;

-- Existing foreign identifiers, if present in an older installation, must be
-- genuine trader rows. Do not repair an invalid author by choosing another one.
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
        WHERE conrelid = 'unified_videos'::regclass
          AND conname = 'unified_videos_trader_id_fkey'
    ) THEN
        ALTER TABLE unified_videos
            ADD CONSTRAINT unified_videos_trader_id_fkey
            FOREIGN KEY (trader_id) REFERENCES traders(id) ON DELETE SET NULL;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_unified_videos_trader_current
    ON unified_videos (trader_id, video_date DESC)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_unified_videos_bunny_guid
    ON unified_videos (bunny_video_guid)
    WHERE bunny_video_guid IS NOT NULL;
