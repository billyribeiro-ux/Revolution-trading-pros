-- ═══════════════════════════════════════════════════════════════════════════════════
-- REVOLUTION TRADING PROS - ENHANCED COURSES & INDICATORS SYSTEM
-- Apple Principal Engineer ICT 7 Grade - January 2026
--
-- Complete course management with sections, lessons, resources, progress tracking
-- Full indicator system with platform files, documentation, TradingView access
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ENHANCED COURSES
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Main courses table (enhanced)
CREATE TABLE IF NOT EXISTS courses_enhanced (
    id BIGSERIAL PRIMARY KEY,

    -- Basic info
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    subtitle VARCHAR(500),
    description TEXT,
    description_html TEXT,

    -- Media
    thumbnail_url VARCHAR(500),
    trailer_video_url VARCHAR(500),
    trailer_bunny_guid VARCHAR(100),

    -- Classification
    difficulty_level VARCHAR(50) DEFAULT 'beginner',  -- beginner, intermediate, advanced
    category VARCHAR(100),
    tags JSONB DEFAULT '[]'::jsonb,

    -- Instructor
    instructor_id BIGINT REFERENCES room_traders(id) ON DELETE SET NULL,

    -- Duration & Content
    estimated_duration_minutes INTEGER DEFAULT 0,
    total_lessons INTEGER DEFAULT 0,
    total_sections INTEGER DEFAULT 0,

    -- Access Control
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_free BOOLEAN DEFAULT false,
    required_plan_id BIGINT,
    price_cents INTEGER,

    -- Prerequisites
    prerequisite_course_ids JSONB DEFAULT '[]'::jsonb,

    -- Completion
    certificate_enabled BOOLEAN DEFAULT true,
    certificate_template VARCHAR(100) DEFAULT 'default',
    completion_threshold_percent INTEGER DEFAULT 90,

    -- SEO
    meta_title VARCHAR(255),
    meta_description TEXT,

    -- Audit
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

-- Course sections (modules)
CREATE TABLE IF NOT EXISTS course_sections (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,

    -- Content
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Ordering
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Type
    section_type VARCHAR(50) DEFAULT 'module',  -- module, live_sessions, resources, bonus

    -- Access Rules
    unlock_type VARCHAR(50) DEFAULT 'sequential',  -- sequential, immediate, date, completion
    unlock_after_section_id BIGINT REFERENCES course_sections(id) ON DELETE SET NULL,
    unlock_date TIMESTAMPTZ,

    -- Status
    is_published BOOLEAN DEFAULT true,

    -- Metadata
    estimated_duration_minutes INTEGER DEFAULT 0,
    lesson_count INTEGER DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Course lessons
CREATE TABLE IF NOT EXISTS course_lessons (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    section_id BIGINT NOT NULL REFERENCES course_sections(id) ON DELETE CASCADE,

    -- Content
    title VARCHAR(255) NOT NULL,
    description TEXT,
    content_html TEXT,  -- Rich text content alongside video

    -- Video
    video_url VARCHAR(500),
    video_platform VARCHAR(50) DEFAULT 'bunny',
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    duration_seconds INTEGER,

    -- Thumbnail
    thumbnail_url VARCHAR(500),

    -- Ordering
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Type
    lesson_type VARCHAR(50) DEFAULT 'video',  -- video, text, quiz, assignment

    -- Access
    is_preview BOOLEAN DEFAULT false,  -- Free preview even for paid courses
    is_published BOOLEAN DEFAULT true,

    -- Completion
    completion_type VARCHAR(50) DEFAULT 'video_complete',  -- video_complete, manual, quiz_pass
    required_watch_percent INTEGER DEFAULT 90,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Course resources (downloadable files)
CREATE TABLE IF NOT EXISTS course_resources (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    section_id BIGINT REFERENCES course_sections(id) ON DELETE CASCADE,  -- NULL = course-level
    lesson_id BIGINT REFERENCES course_lessons(id) ON DELETE CASCADE,    -- NULL = section-level

    -- File info
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_url VARCHAR(500) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(50),  -- pdf, docx, xlsx, tos, zip, etc.
    file_size_bytes BIGINT,

    -- Ordering
    sort_order INTEGER DEFAULT 0,

    -- Version control
    version VARCHAR(50) DEFAULT '1.0',

    -- Access tracking
    download_count INTEGER DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Course live sessions (dated recordings)
CREATE TABLE IF NOT EXISTS course_live_sessions (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    section_id BIGINT REFERENCES course_sections(id) ON DELETE CASCADE,

    -- Session info
    title VARCHAR(255) NOT NULL,
    description TEXT,
    session_date DATE NOT NULL,
    session_time TIME,

    -- Video
    video_url VARCHAR(500),
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    duration_seconds INTEGER,
    thumbnail_url VARCHAR(500),

    -- Access
    replay_available_until TIMESTAMPTZ,  -- NULL = forever
    is_published BOOLEAN DEFAULT true,

    -- Ordering
    sort_order INTEGER DEFAULT 0,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- User course enrollment
CREATE TABLE IF NOT EXISTS user_course_enrollments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,

    -- Enrollment
    enrolled_at TIMESTAMPTZ DEFAULT NOW(),
    enrollment_source VARCHAR(50),  -- purchase, gift, admin, promotion

    -- Access
    access_expires_at TIMESTAMPTZ,  -- NULL = lifetime
    is_active BOOLEAN DEFAULT true,

    -- Progress summary
    completed_lessons INTEGER DEFAULT 0,
    total_lessons INTEGER DEFAULT 0,
    progress_percent INTEGER DEFAULT 0,

    -- Completion
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,
    certificate_issued BOOLEAN DEFAULT false,
    certificate_url VARCHAR(500),

    -- Last activity
    last_lesson_id BIGINT REFERENCES course_lessons(id) ON DELETE SET NULL,
    last_position_seconds INTEGER DEFAULT 0,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, course_id)
);

-- User lesson progress
CREATE TABLE IF NOT EXISTS user_lesson_progress (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    lesson_id BIGINT NOT NULL REFERENCES course_lessons(id) ON DELETE CASCADE,

    -- Watch progress
    watch_position_seconds INTEGER DEFAULT 0,
    watch_time_total_seconds INTEGER DEFAULT 0,
    watch_percent INTEGER DEFAULT 0,

    -- Completion
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,

    -- Engagement
    notes TEXT,
    bookmarked BOOLEAN DEFAULT false,

    -- Timestamps
    first_watched_at TIMESTAMPTZ DEFAULT NOW(),
    last_watched_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, lesson_id)
);

-- Course indexes
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_slug ON courses_enhanced(slug);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_published ON courses_enhanced(is_published) WHERE is_published = true;
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_instructor ON courses_enhanced(instructor_id);
CREATE INDEX IF NOT EXISTS idx_course_sections_course ON course_sections(course_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_course_lessons_section ON course_lessons(section_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_course_lessons_course ON course_lessons(course_id);
CREATE INDEX IF NOT EXISTS idx_course_resources_course ON course_resources(course_id);
CREATE INDEX IF NOT EXISTS idx_course_resources_lesson ON course_resources(lesson_id);
CREATE INDEX IF NOT EXISTS idx_course_live_sessions_course ON course_live_sessions(course_id, session_date DESC);
CREATE INDEX IF NOT EXISTS idx_user_enrollments_user ON user_course_enrollments(user_id);
CREATE INDEX IF NOT EXISTS idx_user_enrollments_course ON user_course_enrollments(course_id);
CREATE INDEX IF NOT EXISTS idx_user_lesson_progress_user ON user_lesson_progress(user_id, course_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. ENHANCED INDICATORS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Platforms enum-like table for flexibility
CREATE TABLE IF NOT EXISTS indicator_platforms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE,
    icon_url VARCHAR(500),
    file_extensions JSONB DEFAULT '[]'::jsonb,  -- [".tos", ".ts"]
    installation_guide_url VARCHAR(500),
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true
);

-- Seed default platforms
INSERT INTO indicator_platforms (name, slug, file_extensions, sort_order) VALUES
    ('ThinkorSwim', 'thinkorswim', '[".tos", ".ts"]', 1),
    ('TradingView', 'tradingview', '[".pine", ".txt"]', 2),
    ('NinjaTrader', 'ninjatrader', '[".cs", ".zip"]', 3),
    ('MetaTrader 4', 'metatrader4', '[".mq4", ".ex4"]', 4),
    ('MetaTrader 5', 'metatrader5', '[".mq5", ".ex5"]', 5),
    ('TradeStation', 'tradestation', '[".eld", ".els"]', 6)
ON CONFLICT (slug) DO NOTHING;

-- Main indicators table
CREATE TABLE IF NOT EXISTS indicators_enhanced (
    id BIGSERIAL PRIMARY KEY,

    -- Basic info
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    subtitle VARCHAR(500),
    description TEXT,
    description_html TEXT,

    -- Media
    thumbnail_url VARCHAR(500),
    banner_url VARCHAR(500),

    -- Classification
    category VARCHAR(100),
    tags JSONB DEFAULT '[]'::jsonb,

    -- Creator
    creator_id BIGINT REFERENCES room_traders(id) ON DELETE SET NULL,

    -- Versioning
    current_version VARCHAR(50) DEFAULT '1.0.0',
    changelog TEXT,

    -- Platforms (which platforms this indicator supports)
    supported_platforms JSONB DEFAULT '[]'::jsonb,  -- ["thinkorswim", "tradingview"]

    -- Access Control
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_free BOOLEAN DEFAULT false,
    required_plan_id BIGINT,
    price_cents INTEGER,

    -- Stats
    download_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,

    -- SEO
    meta_title VARCHAR(255),
    meta_description TEXT,

    -- Audit
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

-- Indicator tutorial videos
CREATE TABLE IF NOT EXISTS indicator_videos (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,

    -- Video info
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Video source
    video_url VARCHAR(500),
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    duration_seconds INTEGER,
    thumbnail_url VARCHAR(500),

    -- Type
    video_type VARCHAR(50) DEFAULT 'tutorial',  -- overview, installation, usage, advanced, update

    -- Ordering
    sort_order INTEGER DEFAULT 0,

    -- Status
    is_published BOOLEAN DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indicator platform files (downloadable indicator files per platform)
CREATE TABLE IF NOT EXISTS indicator_platform_files (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    platform_slug VARCHAR(100) NOT NULL,  -- thinkorswim, tradingview, etc.

    -- File info
    file_name VARCHAR(255) NOT NULL,  -- e.g., "Volume Max Indicator"
    file_display_name VARCHAR(255),   -- e.g., "Volume Max Indicator v2.0"
    file_url VARCHAR(500) NOT NULL,
    original_filename VARCHAR(255),
    file_size_bytes BIGINT,
    file_type VARCHAR(50),            -- .tos, .pine, etc.

    -- Version
    version VARCHAR(50) DEFAULT '1.0',

    -- Installation instructions (platform-specific)
    installation_notes TEXT,

    -- Ordering within platform
    sort_order INTEGER DEFAULT 0,

    -- Stats
    download_count INTEGER DEFAULT 0,

    -- Status
    is_active BOOLEAN DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indicator documentation
CREATE TABLE IF NOT EXISTS indicator_documentation (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,

    -- Doc info
    title VARCHAR(255) NOT NULL,
    description TEXT,

    -- Content (either rich text OR file)
    content_type VARCHAR(50) DEFAULT 'file',  -- file, richtext, link
    content_html TEXT,                        -- If richtext
    file_url VARCHAR(500),                    -- If file
    external_url VARCHAR(500),                -- If link

    -- Classification
    doc_type VARCHAR(50) DEFAULT 'guide',  -- guide, faq, troubleshooting, settings, video

    -- Ordering
    sort_order INTEGER DEFAULT 0,

    -- Stats
    view_count INTEGER DEFAULT 0,

    -- Status
    is_published BOOLEAN DEFAULT true,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- TradingView access management
CREATE TABLE IF NOT EXISTS indicator_tradingview_access (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,

    -- TradingView info
    tradingview_username VARCHAR(255) NOT NULL,

    -- Access status
    status VARCHAR(50) DEFAULT 'pending',  -- pending, granted, revoked
    granted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoke_reason TEXT,

    -- Notification
    notification_sent BOOLEAN DEFAULT false,
    notification_sent_at TIMESTAMPTZ,

    -- Request info
    requested_at TIMESTAMPTZ DEFAULT NOW(),
    requested_by BIGINT,  -- Admin who processed the request

    -- Notes
    admin_notes TEXT,

    UNIQUE(indicator_id, tradingview_username)
);

-- User indicator access (for non-TradingView platforms)
CREATE TABLE IF NOT EXISTS user_indicator_access (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,

    -- Access
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    access_expires_at TIMESTAMPTZ,  -- NULL = lifetime
    is_active BOOLEAN DEFAULT true,

    -- Source
    access_source VARCHAR(50),  -- purchase, gift, admin, promotion

    -- Download tracking
    last_download_at TIMESTAMPTZ,
    total_downloads INTEGER DEFAULT 0,

    UNIQUE(user_id, indicator_id)
);

-- Indicator download log
CREATE TABLE IF NOT EXISTS indicator_download_log (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    file_id BIGINT REFERENCES indicator_platform_files(id) ON DELETE SET NULL,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Download info
    platform_slug VARCHAR(100),
    file_name VARCHAR(255),

    -- Client info
    ip_address INET,
    user_agent TEXT,

    -- Timestamp
    downloaded_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indicator indexes
CREATE INDEX IF NOT EXISTS idx_indicators_enhanced_slug ON indicators_enhanced(slug);
CREATE INDEX IF NOT EXISTS idx_indicators_enhanced_published ON indicators_enhanced(is_published) WHERE is_published = true;
CREATE INDEX IF NOT EXISTS idx_indicators_enhanced_creator ON indicators_enhanced(creator_id);
CREATE INDEX IF NOT EXISTS idx_indicator_videos_indicator ON indicator_videos(indicator_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_indicator_files_indicator ON indicator_platform_files(indicator_id, platform_slug);
CREATE INDEX IF NOT EXISTS idx_indicator_docs_indicator ON indicator_documentation(indicator_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_tradingview_access_indicator ON indicator_tradingview_access(indicator_id, status);
CREATE INDEX IF NOT EXISTS idx_tradingview_access_username ON indicator_tradingview_access(tradingview_username);
CREATE INDEX IF NOT EXISTS idx_user_indicator_access_user ON user_indicator_access(user_id);
CREATE INDEX IF NOT EXISTS idx_indicator_downloads_indicator ON indicator_download_log(indicator_id, downloaded_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. TRIGGERS & FUNCTIONS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Function to update course stats
CREATE OR REPLACE FUNCTION update_course_stats()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_TABLE_NAME = 'course_lessons' THEN
        -- Update section lesson count
        UPDATE course_sections SET
            lesson_count = (SELECT COUNT(*) FROM course_lessons WHERE section_id = COALESCE(NEW.section_id, OLD.section_id)),
            updated_at = NOW()
        WHERE id = COALESCE(NEW.section_id, OLD.section_id);

        -- Update course total lessons
        UPDATE courses_enhanced SET
            total_lessons = (SELECT COUNT(*) FROM course_lessons WHERE course_id = COALESCE(NEW.course_id, OLD.course_id)),
            updated_at = NOW()
        WHERE id = COALESCE(NEW.course_id, OLD.course_id);
    END IF;

    IF TG_TABLE_NAME = 'course_sections' THEN
        -- Update course total sections
        UPDATE courses_enhanced SET
            total_sections = (SELECT COUNT(*) FROM course_sections WHERE course_id = COALESCE(NEW.course_id, OLD.course_id)),
            updated_at = NOW()
        WHERE id = COALESCE(NEW.course_id, OLD.course_id);
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Triggers for course stats
DROP TRIGGER IF EXISTS trigger_update_course_lesson_stats ON course_lessons;
CREATE TRIGGER trigger_update_course_lesson_stats
    AFTER INSERT OR DELETE ON course_lessons
    FOR EACH ROW
    EXECUTE FUNCTION update_course_stats();

DROP TRIGGER IF EXISTS trigger_update_course_section_stats ON course_sections;
CREATE TRIGGER trigger_update_course_section_stats
    AFTER INSERT OR DELETE ON course_sections
    FOR EACH ROW
    EXECUTE FUNCTION update_course_stats();

-- Function to update user enrollment progress
CREATE OR REPLACE FUNCTION update_enrollment_progress()
RETURNS TRIGGER AS $$
DECLARE
    v_total INTEGER;
    v_completed INTEGER;
    v_percent INTEGER;
BEGIN
    -- Get total and completed lessons
    SELECT COUNT(*) INTO v_total
    FROM course_lessons
    WHERE course_id = NEW.course_id AND is_published = true;

    SELECT COUNT(*) INTO v_completed
    FROM user_lesson_progress
    WHERE user_id = NEW.user_id AND course_id = NEW.course_id AND is_completed = true;

    -- Calculate percent
    v_percent := CASE WHEN v_total > 0 THEN (v_completed * 100 / v_total) ELSE 0 END;

    -- Update enrollment
    UPDATE user_course_enrollments SET
        completed_lessons = v_completed,
        total_lessons = v_total,
        progress_percent = v_percent,
        is_completed = (v_percent >= (SELECT completion_threshold_percent FROM courses_enhanced WHERE id = NEW.course_id)),
        completed_at = CASE WHEN v_percent >= (SELECT completion_threshold_percent FROM courses_enhanced WHERE id = NEW.course_id) AND completed_at IS NULL THEN NOW() ELSE completed_at END,
        last_activity_at = NOW()
    WHERE user_id = NEW.user_id AND course_id = NEW.course_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for enrollment progress
DROP TRIGGER IF EXISTS trigger_update_enrollment_progress ON user_lesson_progress;
CREATE TRIGGER trigger_update_enrollment_progress
    AFTER INSERT OR UPDATE OF is_completed ON user_lesson_progress
    FOR EACH ROW
    EXECUTE FUNCTION update_enrollment_progress();

-- Function to increment download counts
CREATE OR REPLACE FUNCTION increment_download_count()
RETURNS TRIGGER AS $$
BEGIN
    -- Increment file download count
    IF NEW.file_id IS NOT NULL THEN
        UPDATE indicator_platform_files SET download_count = download_count + 1 WHERE id = NEW.file_id;
    END IF;

    -- Increment indicator download count
    UPDATE indicators_enhanced SET download_count = download_count + 1 WHERE id = NEW.indicator_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for download count
DROP TRIGGER IF EXISTS trigger_increment_download_count ON indicator_download_log;
CREATE TRIGGER trigger_increment_download_count
    AFTER INSERT ON indicator_download_log
    FOR EACH ROW
    EXECUTE FUNCTION increment_download_count();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- MIGRATION COMPLETE
-- ═══════════════════════════════════════════════════════════════════════════════════
