-- Migration: Add redirects table for URL redirect management
-- ICT 11+ Principal Engineer - SEO and URL management

CREATE TABLE IF NOT EXISTS redirects (
    id BIGSERIAL PRIMARY KEY,
    from_path VARCHAR(500) NOT NULL UNIQUE,
    to_path VARCHAR(500) NOT NULL,
    status_code INTEGER NOT NULL DEFAULT 301,
    redirect_type VARCHAR(50) NOT NULL DEFAULT 'manual',
    is_active BOOLEAN NOT NULL DEFAULT true,
    hit_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_redirects_from_path ON redirects(from_path);
CREATE INDEX IF NOT EXISTS idx_redirects_is_active ON redirects(is_active);
CREATE INDEX IF NOT EXISTS idx_redirects_created_at ON redirects(created_at DESC);

-- Comments
COMMENT ON TABLE redirects IS 'URL redirect rules for SEO and site maintenance';
COMMENT ON COLUMN redirects.from_path IS 'Source URL path to redirect from';
COMMENT ON COLUMN redirects.to_path IS 'Destination URL path to redirect to';
COMMENT ON COLUMN redirects.status_code IS 'HTTP status code (301, 302, 307, 308)';
COMMENT ON COLUMN redirects.redirect_type IS 'Type of redirect (manual, automatic, migration)';
COMMENT ON COLUMN redirects.hit_count IS 'Number of times this redirect was triggered';
