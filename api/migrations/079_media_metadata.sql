ALTER TABLE media ADD COLUMN tags JSONB NOT NULL DEFAULT '[]' CHECK(jsonb_typeof(tags)='array');
ALTER TABLE media ADD COLUMN metadata JSONB NOT NULL DEFAULT '{}' CHECK(jsonb_typeof(metadata)='object');
ALTER TABLE media ADD COLUMN created_by BIGINT REFERENCES users(id) ON DELETE SET NULL;
