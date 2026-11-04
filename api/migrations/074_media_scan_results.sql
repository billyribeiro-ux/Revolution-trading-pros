-- Unscanned files are pending, never implicitly clean.
ALTER TABLE media ADD COLUMN IF NOT EXISTS scan_status TEXT NOT NULL DEFAULT 'pending';
ALTER TABLE media ADD COLUMN IF NOT EXISTS scan_result JSONB;
ALTER TABLE media ADD CONSTRAINT media_scan_status_valid
    CHECK (scan_status IN ('pending', 'clean', 'infected', 'failed', 'unavailable'));
