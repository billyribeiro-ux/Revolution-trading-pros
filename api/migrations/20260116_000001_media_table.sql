-- Media Library Table - Revolution Trading Pros
-- Apple ICT 7 Grade - January 2026
-- Cloudflare R2 Storage Integration

CREATE TABLE IF NOT EXISTS media (
    id BIGSERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255),
    mime_type VARCHAR(100),
    size BIGINT,
    path TEXT,
    url TEXT,
    title VARCHAR(500),
    alt_text VARCHAR(500),
    caption TEXT,
    description TEXT,
    collection VARCHAR(100),
    is_optimized BOOLEAN DEFAULT false,
    width INTEGER,
    height INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_media_filename ON media(filename);
CREATE INDEX IF NOT EXISTS idx_media_collection ON media(collection);
CREATE INDEX IF NOT EXISTS idx_media_mime_type ON media(mime_type);
CREATE INDEX IF NOT EXISTS idx_media_created_at ON media(created_at DESC);

-- Seed some example data (optional)
-- INSERT INTO media (filename, original_filename, mime_type, size, url, title, collection)
-- VALUES ('example.jpg', 'example.jpg', 'image/jpeg', 102400, 'https://cdn.example.com/example.jpg', 'Example Image', 'general')
-- ON CONFLICT DO NOTHING;
