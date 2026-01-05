-- Add missing columns to room_daily_videos for full video management
-- ICT 11+ Principal Engineer Grade

-- Add slug column (required for URL routing)
ALTER TABLE room_daily_videos 
ADD COLUMN IF NOT EXISTS slug VARCHAR(500);

-- Add description column
ALTER TABLE room_daily_videos 
ADD COLUMN IF NOT EXISTS description TEXT;

-- Add publishing flags
ALTER TABLE room_daily_videos 
ADD COLUMN IF NOT EXISTS is_published BOOLEAN NOT NULL DEFAULT true;

ALTER TABLE room_daily_videos 
ADD COLUMN IF NOT EXISTS is_featured BOOLEAN NOT NULL DEFAULT false;

-- Add view tracking
ALTER TABLE room_daily_videos 
ADD COLUMN IF NOT EXISTS views_count INTEGER NOT NULL DEFAULT 0;

-- Add slug column to room_traders if missing
ALTER TABLE room_traders 
ADD COLUMN IF NOT EXISTS slug VARCHAR(255);

-- Create unique index on video slug within a trading room
CREATE UNIQUE INDEX IF NOT EXISTS idx_room_videos_slug 
ON room_daily_videos(trading_room_id, slug);

-- Create index on room_traders slug
CREATE INDEX IF NOT EXISTS idx_room_traders_slug 
ON room_traders(slug);

-- Update existing videos to have a slug based on title
UPDATE room_daily_videos 
SET slug = LOWER(REGEXP_REPLACE(REGEXP_REPLACE(title, '[^a-zA-Z0-9\s]', '', 'g'), '\s+', '-', 'g'))
WHERE slug IS NULL;

-- Update existing traders to have a slug based on name
UPDATE room_traders 
SET slug = LOWER(REGEXP_REPLACE(REGEXP_REPLACE(name, '[^a-zA-Z0-9\s]', '', 'g'), '\s+', '-', 'g'))
WHERE slug IS NULL;
