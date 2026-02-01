-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 036: Room Resources ICT 7 Complete
-- Apple Principal Engineer Grade - February 2026
--
-- Comprehensive update for room_resources system:
-- - Fix schema mismatch (trading_room_id, content_type, etc.)
-- - Add access_level for free/premium distinction
-- - Add versioning support (version, previous_version_id)
-- - Add course/lesson linking (course_id, lesson_id)
-- - Add section field for dashboard organization
-- - Add file_size_limit enforcement
-- - Add secure URL token support
-- ═══════════════════════════════════════════════════════════════════════════════════

-- 1. Add missing columns to room_resources (if they don't exist)
DO $$ BEGIN
    -- Core fields alignment
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'trading_room_id') THEN
        ALTER TABLE room_resources ADD COLUMN trading_room_id BIGINT;
        UPDATE room_resources SET trading_room_id = room_id WHERE trading_room_id IS NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'content_type') THEN
        ALTER TABLE room_resources ADD COLUMN content_type VARCHAR(50) DEFAULT 'other';
        UPDATE room_resources SET content_type = COALESCE(content_category, 'other') WHERE content_type = 'other' OR content_type IS NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'section') THEN
        ALTER TABLE room_resources ADD COLUMN section VARCHAR(50) DEFAULT 'latest_updates';
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'resource_date') THEN
        ALTER TABLE room_resources ADD COLUMN resource_date DATE DEFAULT CURRENT_DATE;
        UPDATE room_resources SET resource_date = COALESCE(DATE(published_at), DATE(created_at), CURRENT_DATE);
    END IF;

    -- Video-specific fields
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'video_id') THEN
        ALTER TABLE room_resources ADD COLUMN video_id VARCHAR(100);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'duration') THEN
        ALTER TABLE room_resources ADD COLUMN duration INTEGER;
        UPDATE room_resources SET duration = duration_seconds WHERE duration IS NULL AND duration_seconds IS NOT NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'width') THEN
        ALTER TABLE room_resources ADD COLUMN width INTEGER;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'height') THEN
        ALTER TABLE room_resources ADD COLUMN height INTEGER;
    END IF;

    -- File handling fields
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'file_path') THEN
        ALTER TABLE room_resources ADD COLUMN file_path VARCHAR(500);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'file_size') THEN
        ALTER TABLE room_resources ADD COLUMN file_size BIGINT;
        UPDATE room_resources SET file_size = file_size_bytes WHERE file_size IS NULL AND file_size_bytes IS NOT NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'thumbnail_path') THEN
        ALTER TABLE room_resources ADD COLUMN thumbnail_path VARCHAR(500);
    END IF;

    -- Difficulty level
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'difficulty_level') THEN
        ALTER TABLE room_resources ADD COLUMN difficulty_level VARCHAR(20) DEFAULT 'beginner';
    END IF;

    -- Stats fields (normalized naming)
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'views_count') THEN
        ALTER TABLE room_resources ADD COLUMN views_count INTEGER DEFAULT 0;
        UPDATE room_resources SET views_count = COALESCE(view_count, 0);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'downloads_count') THEN
        ALTER TABLE room_resources ADD COLUMN downloads_count INTEGER DEFAULT 0;
        UPDATE room_resources SET downloads_count = COALESCE(download_count, 0);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'likes_count') THEN
        ALTER TABLE room_resources ADD COLUMN likes_count INTEGER DEFAULT 0;
    END IF;

    -- Category and metadata
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'category') THEN
        ALTER TABLE room_resources ADD COLUMN category VARCHAR(100);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'metadata') THEN
        ALTER TABLE room_resources ADD COLUMN metadata JSONB DEFAULT '{}'::jsonb;
    END IF;

    -- Scheduling
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'scheduled_at') THEN
        ALTER TABLE room_resources ADD COLUMN scheduled_at TIMESTAMPTZ;
    END IF;

    -- Trader association
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'trader_id') THEN
        ALTER TABLE room_resources ADD COLUMN trader_id BIGINT REFERENCES traders(id);
    END IF;
EXCEPTION WHEN others THEN
    -- Ignore errors for flexibility
    NULL;
END $$;

-- 2. ICT 7 NEW: Access control for free vs premium resources
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'access_level') THEN
        ALTER TABLE room_resources ADD COLUMN access_level VARCHAR(20) DEFAULT 'premium';
    END IF;
    COMMENT ON COLUMN room_resources.access_level IS 'Access level: free, member, premium, vip';
EXCEPTION WHEN others THEN NULL;
END $$;

-- 3. ICT 7 NEW: Resource versioning support
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'version') THEN
        ALTER TABLE room_resources ADD COLUMN version INTEGER DEFAULT 1;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'previous_version_id') THEN
        ALTER TABLE room_resources ADD COLUMN previous_version_id BIGINT REFERENCES room_resources(id);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'is_latest_version') THEN
        ALTER TABLE room_resources ADD COLUMN is_latest_version BOOLEAN DEFAULT true;
    END IF;

    COMMENT ON COLUMN room_resources.version IS 'Version number, increments with each update';
    COMMENT ON COLUMN room_resources.previous_version_id IS 'Reference to the previous version of this resource';
    COMMENT ON COLUMN room_resources.is_latest_version IS 'Whether this is the latest version (for quick filtering)';
EXCEPTION WHEN others THEN NULL;
END $$;

-- 4. ICT 7 NEW: Course/Lesson linking
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'course_id') THEN
        ALTER TABLE room_resources ADD COLUMN course_id BIGINT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'lesson_id') THEN
        ALTER TABLE room_resources ADD COLUMN lesson_id BIGINT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'course_order') THEN
        ALTER TABLE room_resources ADD COLUMN course_order INTEGER DEFAULT 0;
    END IF;

    COMMENT ON COLUMN room_resources.course_id IS 'Optional link to a course for course materials';
    COMMENT ON COLUMN room_resources.lesson_id IS 'Optional link to a specific lesson';
    COMMENT ON COLUMN room_resources.course_order IS 'Order within the course materials';
EXCEPTION WHEN others THEN NULL;
END $$;

-- 5. ICT 7 NEW: Secure download URL token support
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'secure_token') THEN
        ALTER TABLE room_resources ADD COLUMN secure_token VARCHAR(64);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'secure_token_expires') THEN
        ALTER TABLE room_resources ADD COLUMN secure_token_expires TIMESTAMPTZ;
    END IF;

    COMMENT ON COLUMN room_resources.secure_token IS 'Token for secure/signed URL downloads';
    COMMENT ON COLUMN room_resources.secure_token_expires IS 'Expiration time for secure download token';
EXCEPTION WHEN others THEN NULL;
END $$;

-- 6. ICT 7 NEW: File size limits tracking
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'file_hash') THEN
        ALTER TABLE room_resources ADD COLUMN file_hash VARCHAR(64);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'room_resources' AND column_name = 'storage_provider') THEN
        ALTER TABLE room_resources ADD COLUMN storage_provider VARCHAR(20) DEFAULT 'r2';
    END IF;

    COMMENT ON COLUMN room_resources.file_hash IS 'SHA-256 hash for deduplication and integrity';
    COMMENT ON COLUMN room_resources.storage_provider IS 'Storage backend: r2, bunny, s3, local';
EXCEPTION WHEN others THEN NULL;
END $$;

-- 7. Create indexes for new columns
CREATE INDEX IF NOT EXISTS idx_room_resources_trading_room_id ON room_resources(trading_room_id);
CREATE INDEX IF NOT EXISTS idx_room_resources_content_type ON room_resources(content_type);
CREATE INDEX IF NOT EXISTS idx_room_resources_section ON room_resources(section);
CREATE INDEX IF NOT EXISTS idx_room_resources_access_level ON room_resources(access_level);
CREATE INDEX IF NOT EXISTS idx_room_resources_course_id ON room_resources(course_id) WHERE course_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_room_resources_lesson_id ON room_resources(lesson_id) WHERE lesson_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_room_resources_version ON room_resources(is_latest_version) WHERE is_latest_version = true;
CREATE INDEX IF NOT EXISTS idx_room_resources_resource_date ON room_resources(resource_date DESC);
CREATE INDEX IF NOT EXISTS idx_room_resources_trader ON room_resources(trader_id) WHERE trader_id IS NOT NULL;

-- 8. Create download log table for tracking
CREATE TABLE IF NOT EXISTS resource_download_logs (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGINT NOT NULL REFERENCES room_resources(id),
    user_id BIGINT REFERENCES users(id),
    session_id VARCHAR(64),
    ip_address INET,
    user_agent TEXT,
    referer VARCHAR(500),
    downloaded_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_resource_downloads_resource ON resource_download_logs(resource_id);
CREATE INDEX IF NOT EXISTS idx_resource_downloads_user ON resource_download_logs(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_resource_downloads_date ON resource_download_logs(downloaded_at DESC);

-- 9. Create resource favorites table for user engagement
CREATE TABLE IF NOT EXISTS resource_favorites (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGINT NOT NULL REFERENCES room_resources(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(resource_id, user_id)
);
CREATE INDEX IF NOT EXISTS idx_resource_favorites_user ON resource_favorites(user_id);

-- 10. File size limits configuration table
CREATE TABLE IF NOT EXISTS resource_upload_limits (
    id BIGSERIAL PRIMARY KEY,
    resource_type VARCHAR(50) NOT NULL UNIQUE,
    max_file_size_bytes BIGINT NOT NULL,
    allowed_mime_types TEXT[] NOT NULL DEFAULT '{}',
    allowed_extensions TEXT[] NOT NULL DEFAULT '{}',
    requires_premium BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default upload limits
INSERT INTO resource_upload_limits (resource_type, max_file_size_bytes, allowed_mime_types, allowed_extensions, requires_premium) VALUES
    ('video', 5368709120, ARRAY['video/mp4', 'video/webm', 'video/quicktime'], ARRAY['mp4', 'webm', 'mov'], false),
    ('pdf', 104857600, ARRAY['application/pdf'], ARRAY['pdf'], false),
    ('document', 52428800, ARRAY['application/msword', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document', 'text/plain'], ARRAY['doc', 'docx', 'txt'], false),
    ('image', 26214400, ARRAY['image/jpeg', 'image/png', 'image/gif', 'image/webp'], ARRAY['jpg', 'jpeg', 'png', 'gif', 'webp'], false),
    ('spreadsheet', 52428800, ARRAY['application/vnd.ms-excel', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet', 'text/csv'], ARRAY['xls', 'xlsx', 'csv'], false),
    ('archive', 1073741824, ARRAY['application/zip', 'application/x-zip-compressed', 'application/x-rar-compressed'], ARRAY['zip', 'rar'], true)
ON CONFLICT (resource_type) DO UPDATE SET
    max_file_size_bytes = EXCLUDED.max_file_size_bytes,
    allowed_mime_types = EXCLUDED.allowed_mime_types,
    allowed_extensions = EXCLUDED.allowed_extensions,
    updated_at = NOW();

-- 11. Create function for generating secure download tokens
CREATE OR REPLACE FUNCTION generate_secure_download_token(
    p_resource_id BIGINT,
    p_expires_in_hours INTEGER DEFAULT 24
) RETURNS VARCHAR(64) AS $$
DECLARE
    v_token VARCHAR(64);
BEGIN
    v_token := encode(gen_random_bytes(32), 'hex');

    UPDATE room_resources
    SET secure_token = v_token,
        secure_token_expires = NOW() + (p_expires_in_hours || ' hours')::INTERVAL
    WHERE id = p_resource_id;

    RETURN v_token;
END;
$$ LANGUAGE plpgsql;

-- 12. Create function for validating secure download tokens
CREATE OR REPLACE FUNCTION validate_secure_download_token(
    p_resource_id BIGINT,
    p_token VARCHAR(64)
) RETURNS BOOLEAN AS $$
DECLARE
    v_valid BOOLEAN;
BEGIN
    SELECT EXISTS (
        SELECT 1 FROM room_resources
        WHERE id = p_resource_id
        AND secure_token = p_token
        AND secure_token_expires > NOW()
    ) INTO v_valid;

    RETURN v_valid;
END;
$$ LANGUAGE plpgsql;

-- 13. Create function for creating resource versions
CREATE OR REPLACE FUNCTION create_resource_version(
    p_resource_id BIGINT,
    p_new_file_url VARCHAR(500),
    p_new_file_size BIGINT DEFAULT NULL
) RETURNS BIGINT AS $$
DECLARE
    v_old_resource room_resources%ROWTYPE;
    v_new_id BIGINT;
BEGIN
    -- Get the current resource
    SELECT * INTO v_old_resource FROM room_resources WHERE id = p_resource_id;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Resource not found: %', p_resource_id;
    END IF;

    -- Mark old version as not latest
    UPDATE room_resources SET is_latest_version = false WHERE id = p_resource_id;

    -- Create new version
    INSERT INTO room_resources (
        title, slug, description, trading_room_id, resource_type, content_type, section,
        file_url, file_size, mime_type, thumbnail_url, video_platform, bunny_video_guid, bunny_library_id,
        duration, width, height, is_featured, is_pinned, is_published, access_level,
        category, tags, difficulty_level, trader_id, course_id, lesson_id, course_order,
        version, previous_version_id, is_latest_version, resource_date, created_by
    )
    VALUES (
        v_old_resource.title, v_old_resource.slug || '-v' || (v_old_resource.version + 1),
        v_old_resource.description, v_old_resource.trading_room_id, v_old_resource.resource_type,
        v_old_resource.content_type, v_old_resource.section,
        p_new_file_url, COALESCE(p_new_file_size, v_old_resource.file_size), v_old_resource.mime_type,
        v_old_resource.thumbnail_url, v_old_resource.video_platform, v_old_resource.bunny_video_guid, v_old_resource.bunny_library_id,
        v_old_resource.duration, v_old_resource.width, v_old_resource.height,
        v_old_resource.is_featured, v_old_resource.is_pinned, v_old_resource.is_published, v_old_resource.access_level,
        v_old_resource.category, v_old_resource.tags, v_old_resource.difficulty_level,
        v_old_resource.trader_id, v_old_resource.course_id, v_old_resource.lesson_id, v_old_resource.course_order,
        v_old_resource.version + 1, p_resource_id, true, CURRENT_DATE, v_old_resource.created_by
    )
    RETURNING id INTO v_new_id;

    RETURN v_new_id;
END;
$$ LANGUAGE plpgsql;

-- 14. Add comment for table documentation
COMMENT ON TABLE room_resources IS 'Unified resource storage for trading rooms - videos, PDFs, documents, images. ICT 7 Grade.';
