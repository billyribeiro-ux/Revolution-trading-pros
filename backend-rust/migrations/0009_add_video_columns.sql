-- Add missing columns to room_daily_videos for full video management
-- ICT 7 Principal Engineer Grade - Safe migration with table existence checks

-- Only run if room_daily_videos table exists (created in migration 8)
DO $$ 
BEGIN 
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'room_daily_videos') THEN
        -- Add slug column (required for URL routing)
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                       WHERE table_name = 'room_daily_videos' AND column_name = 'slug') THEN
            ALTER TABLE room_daily_videos ADD COLUMN slug VARCHAR(500);
        END IF;

        -- Add description column
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                       WHERE table_name = 'room_daily_videos' AND column_name = 'description') THEN
            ALTER TABLE room_daily_videos ADD COLUMN description TEXT;
        END IF;

        -- Add publishing flags
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                       WHERE table_name = 'room_daily_videos' AND column_name = 'is_published') THEN
            ALTER TABLE room_daily_videos ADD COLUMN is_published BOOLEAN NOT NULL DEFAULT true;
        END IF;

        IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                       WHERE table_name = 'room_daily_videos' AND column_name = 'is_featured') THEN
            ALTER TABLE room_daily_videos ADD COLUMN is_featured BOOLEAN NOT NULL DEFAULT false;
        END IF;

        -- Add view tracking
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                       WHERE table_name = 'room_daily_videos' AND column_name = 'views_count') THEN
            ALTER TABLE room_daily_videos ADD COLUMN views_count INTEGER NOT NULL DEFAULT 0;
        END IF;
    END IF;
END $$;

-- Only run if room_traders table exists
DO $$ 
BEGIN 
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'room_traders') THEN
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                       WHERE table_name = 'room_traders' AND column_name = 'slug') THEN
            ALTER TABLE room_traders ADD COLUMN slug VARCHAR(255);
        END IF;
    END IF;
END $$;

-- Create indexes only if tables exist
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'room_daily_videos') THEN
        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_room_videos_slug') THEN
            CREATE UNIQUE INDEX idx_room_videos_slug ON room_daily_videos(trading_room_id, slug);
        END IF;
    END IF;

    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'room_traders') THEN
        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = 'idx_room_traders_slug') THEN
            CREATE INDEX idx_room_traders_slug ON room_traders(slug);
        END IF;
    END IF;
END $$;

-- Update existing records only if tables exist
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'room_daily_videos') THEN
        UPDATE room_daily_videos 
        SET slug = LOWER(REGEXP_REPLACE(REGEXP_REPLACE(title, '[^a-zA-Z0-9\s]', '', 'g'), '\s+', '-', 'g'))
        WHERE slug IS NULL;
    END IF;

    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'room_traders') THEN
        UPDATE room_traders 
        SET slug = LOWER(REGEXP_REPLACE(REGEXP_REPLACE(name, '[^a-zA-Z0-9\s]', '', 'g'), '\s+', '-', 'g'))
        WHERE slug IS NULL;
    END IF;
END $$;
