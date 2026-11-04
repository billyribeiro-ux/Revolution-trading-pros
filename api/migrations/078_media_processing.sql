-- Originals remain immutable. Derived objects are published only after the
-- complete job succeeds; failed attempts retain their error and retry time.
CREATE TABLE media_optimization_presets (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT,
    quality JSONB NOT NULL,
    compression_mode TEXT NOT NULL CHECK (compression_mode IN ('lossless','lossy','auto')),
    convert_to_webp BOOLEAN NOT NULL DEFAULT TRUE,
    convert_to_avif BOOLEAN NOT NULL DEFAULT FALSE,
    generate_retina BOOLEAN NOT NULL DEFAULT FALSE,
    responsive_sizes JSONB NOT NULL DEFAULT '{}',
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE UNIQUE INDEX media_one_default_preset ON media_optimization_presets(is_default) WHERE is_default;
INSERT INTO media_optimization_presets(name,slug,description,quality,compression_mode,responsive_sizes,is_default)
VALUES ('Balanced','balanced','WebP with responsive sizes; original retained',
        '{"webp":80,"avif":60,"jpeg":80,"png":90}', 'lossy',
        '{"small":320,"medium":768,"large":1280}', TRUE);

CREATE TABLE media_optimization_jobs (
    id BIGSERIAL PRIMARY KEY,
    media_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    preset JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','processing','completed','failed','cancelled')),
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3 CHECK (max_attempts BETWEEN 1 AND 5),
    priority INTEGER NOT NULL DEFAULT 0 CHECK(priority BETWEEN 0 AND 10),
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    lease_until TIMESTAMPTZ,
    original_size BIGINT,
    optimized_size BIGINT,
    processing_time_ms BIGINT,
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);
CREATE UNIQUE INDEX media_one_active_job ON media_optimization_jobs(media_id) WHERE status IN ('pending','processing');
CREATE INDEX media_job_queue ON media_optimization_jobs(priority DESC, available_at, id) WHERE status IN ('pending','processing');
CREATE TABLE media_variants (
    id BIGSERIAL PRIMARY KEY,
    media_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    job_id BIGINT NOT NULL REFERENCES media_optimization_jobs(id),
    type TEXT NOT NULL,
    size TEXT,
    width INTEGER NOT NULL CHECK(width > 0),
    height INTEGER NOT NULL CHECK(height > 0),
    pages INTEGER NOT NULL CHECK(pages > 0),
    path TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    format TEXT NOT NULL,
    file_size BIGINT NOT NULL CHECK(file_size > 0),
    is_retina BOOLEAN NOT NULL DEFAULT FALSE,
    savings_percent DOUBLE PRECISION,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX media_variant_lookup ON media_variants(media_id, id);
