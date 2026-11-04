ALTER TABLE media ADD COLUMN content_version BIGINT NOT NULL DEFAULT 1 CHECK(content_version>0);
ALTER TABLE media_optimization_jobs ADD COLUMN source_version BIGINT NOT NULL DEFAULT 1;
ALTER TABLE media_variants ADD COLUMN source_version BIGINT NOT NULL DEFAULT 1;
CREATE TABLE media_versions (
    id BIGSERIAL PRIMARY KEY,
    media_id BIGINT NOT NULL REFERENCES media(id) ON DELETE CASCADE,
    version_number BIGINT NOT NULL CHECK(version_number>0),
    snapshot JSONB NOT NULL CHECK(jsonb_typeof(snapshot)='object'),
    changes TEXT NOT NULL,
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(media_id,version_number)
);
-- These are first observed versions, not fabricated historical revisions.
INSERT INTO media_versions(media_id,version_number,snapshot,changes,created_by)
SELECT id,content_version,to_jsonb(media),'Version tracking initialized',NULL FROM media;
CREATE INDEX media_versions_lookup ON media_versions(media_id,id DESC);
