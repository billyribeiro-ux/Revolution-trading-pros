-- Add room_slug to bunny_uploads table for room identification
-- Migration: 021_add_room_slug_to_bunny_uploads.sql
-- Apple Principal Engineer ICT 7 Grade - January 2026

-- Add room_slug column to track which trading room each video belongs to
ALTER TABLE bunny_uploads 
ADD COLUMN IF NOT EXISTS room_slug VARCHAR(100);

-- Add index for faster room-based queries
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_room_slug 
ON bunny_uploads(room_slug);

-- Add composite index for room + status queries
CREATE INDEX IF NOT EXISTS idx_bunny_uploads_room_status 
ON bunny_uploads(room_slug, status);

-- Add comment for documentation
COMMENT ON COLUMN bunny_uploads.room_slug IS 'Trading room slug (explosive-swings, spx-profit-pulse, etc.) for video organization';
