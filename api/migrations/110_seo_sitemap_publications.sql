-- Explicit public content eligibility; no publication data is synthesized.
CREATE TABLE seo_sitemap_settings (
    kind TEXT PRIMARY KEY CHECK (kind IN ('news','video')),
    config JSONB NOT NULL CHECK (jsonb_typeof(config)='object'),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version>0),
    updated_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
INSERT INTO seo_sitemap_settings(kind,config) VALUES
('news','{"enabled":false,"publication_name":"Revolution Trading Pros","language":"en","max_age_hours":48,"exclude_categories":[]}'::jsonb),
('video','{"enabled":false,"include_thumbnails":true,"include_uploader":true,"detect_from_content":true,"platforms":["self-hosted","bunny","youtube","vimeo","dailymotion","ted","wistia"]}'::jsonb);
CREATE TABLE seo_video_publications (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL UNIQUE REFERENCES unified_videos(id) ON DELETE CASCADE,
    author_slug TEXT NOT NULL,
    source_snapshot JSONB NOT NULL CHECK (jsonb_typeof(source_snapshot)='object'),
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE seo_sitemap_generations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    kind TEXT NOT NULL REFERENCES seo_sitemap_settings(kind),
    settings_version BIGINT NOT NULL,
    entry_count BIGINT NOT NULL CHECK (entry_count>=0),
    source_hash TEXT NOT NULL,
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE seo_sitemap_audit (
    id BIGSERIAL PRIMARY KEY,
    kind TEXT NOT NULL REFERENCES seo_sitemap_settings(kind),
    action TEXT NOT NULL,
    details JSONB NOT NULL,
    actor_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
