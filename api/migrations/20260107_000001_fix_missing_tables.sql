-- ═══════════════════════════════════════════════════════════════════════════════════
-- FIX MISSING TABLES - Revolution Trading Pros
-- Apple Principal Engineer ICT 7+ Grade - January 17, 2026
--
-- This migration adds all missing tables that are causing 500 errors:
-- 1. room_traders - Required by courses_enhanced
-- 2. courses_enhanced - Enhanced course management
-- 3. media - Media library
-- 4. page_layouts - Page builder layouts
-- 5. locales - i18n/Localization
-- 6. indicators.tagline - Missing column
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ROOM TRADERS TABLE (instructors/traders)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS room_traders (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,

    -- Basic Info
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE,
    title VARCHAR(255),
    bio TEXT,

    -- Media
    avatar_url VARCHAR(500),
    cover_image_url VARCHAR(500),

    -- Social Links
    twitter_url VARCHAR(500),
    youtube_url VARCHAR(500),
    instagram_url VARCHAR(500),

    -- Trading Style
    trading_style VARCHAR(100),
    specialties JSONB DEFAULT '[]'::jsonb,
    years_experience INTEGER,

    -- Status
    is_active BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,

    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_room_traders_slug ON room_traders(slug);
CREATE INDEX IF NOT EXISTS idx_room_traders_user_id ON room_traders(user_id);
CREATE INDEX IF NOT EXISTS idx_room_traders_is_active ON room_traders(is_active);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. COURSES ENHANCED TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════
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
    difficulty_level VARCHAR(50) DEFAULT 'beginner',
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

CREATE INDEX IF NOT EXISTS idx_courses_enhanced_slug ON courses_enhanced(slug);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_is_published ON courses_enhanced(is_published);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_category ON courses_enhanced(category);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_instructor ON courses_enhanced(instructor_id);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_deleted_at ON courses_enhanced(deleted_at);

-- Course sections (modules)
CREATE TABLE IF NOT EXISTS course_sections (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,

    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    section_type VARCHAR(50) DEFAULT 'module',
    unlock_type VARCHAR(50) DEFAULT 'sequential',
    unlock_after_section_id BIGINT REFERENCES course_sections(id) ON DELETE SET NULL,
    unlock_date TIMESTAMPTZ,
    is_published BOOLEAN DEFAULT true,
    estimated_duration_minutes INTEGER DEFAULT 0,
    lesson_count INTEGER DEFAULT 0,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_sections_course_id ON course_sections(course_id);
CREATE INDEX IF NOT EXISTS idx_course_sections_sort_order ON course_sections(course_id, sort_order);

-- Course lessons
CREATE TABLE IF NOT EXISTS course_lessons (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    section_id BIGINT NOT NULL REFERENCES course_sections(id) ON DELETE CASCADE,

    title VARCHAR(255) NOT NULL,
    description TEXT,
    content_html TEXT,

    video_url VARCHAR(500),
    video_platform VARCHAR(50) DEFAULT 'bunny',
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    duration_seconds INTEGER,

    thumbnail_url VARCHAR(500),
    sort_order INTEGER NOT NULL DEFAULT 0,
    lesson_type VARCHAR(50) DEFAULT 'video',
    is_preview BOOLEAN DEFAULT false,
    is_published BOOLEAN DEFAULT true,
    completion_type VARCHAR(50) DEFAULT 'video_complete',
    required_watch_percent INTEGER DEFAULT 90,

    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_lessons_course_id ON course_lessons(course_id);
CREATE INDEX IF NOT EXISTS idx_course_lessons_section_id ON course_lessons(section_id);
CREATE INDEX IF NOT EXISTS idx_course_lessons_sort_order ON course_lessons(section_id, sort_order);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. MEDIA LIBRARY TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════
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
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_media_filename ON media(filename);
CREATE INDEX IF NOT EXISTS idx_media_collection ON media(collection);
CREATE INDEX IF NOT EXISTS idx_media_mime_type ON media(mime_type);
CREATE INDEX IF NOT EXISTS idx_media_created_at ON media(created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. PAGE LAYOUTS TABLE (Page Builder)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS page_layouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID REFERENCES courses(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE,
    description TEXT,
    status VARCHAR(50) DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    version INTEGER DEFAULT 1,
    blocks JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    created_by UUID,
    updated_by UUID
);

CREATE INDEX IF NOT EXISTS idx_page_layouts_course_id ON page_layouts(course_id);
CREATE INDEX IF NOT EXISTS idx_page_layouts_status ON page_layouts(status);
CREATE INDEX IF NOT EXISTS idx_page_layouts_slug ON page_layouts(slug);

-- Page layout versions (history)
CREATE TABLE IF NOT EXISTS page_layout_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    layout_id UUID NOT NULL REFERENCES page_layouts(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    blocks JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    UNIQUE(layout_id, version)
);

CREATE INDEX IF NOT EXISTS idx_page_layout_versions_layout_id ON page_layout_versions(layout_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. LOCALES TABLE (i18n)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS locales (
    id BIGSERIAL PRIMARY KEY,
    code VARCHAR(10) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    native_name VARCHAR(100),
    direction VARCHAR(3) DEFAULT 'ltr' CHECK (direction IN ('ltr', 'rtl')),
    is_default BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    date_format VARCHAR(50) DEFAULT 'YYYY-MM-DD',
    time_format VARCHAR(50) DEFAULT 'HH:mm:ss',
    currency_code VARCHAR(3) DEFAULT 'USD',
    flag_emoji VARCHAR(10),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_locales_code ON locales(code);
CREATE INDEX IF NOT EXISTS idx_locales_is_active ON locales(is_active);

-- Add missing columns to locales if table already exists from earlier migration
ALTER TABLE locales ADD COLUMN IF NOT EXISTS flag_emoji VARCHAR(10);
ALTER TABLE locales ADD COLUMN IF NOT EXISTS direction VARCHAR(3) DEFAULT 'ltr';
ALTER TABLE locales ADD COLUMN IF NOT EXISTS date_format VARCHAR(50) DEFAULT 'YYYY-MM-DD';
ALTER TABLE locales ADD COLUMN IF NOT EXISTS time_format VARCHAR(50) DEFAULT 'HH:mm:ss';
ALTER TABLE locales ADD COLUMN IF NOT EXISTS currency_code VARCHAR(3) DEFAULT 'USD';

-- Seed default locale
INSERT INTO locales (code, name, native_name, is_default, is_active)
VALUES ('en', 'English', 'English', true, true)
ON CONFLICT (code) DO UPDATE SET flag_emoji = '🇺🇸';

-- Content translations table already created in migration 14
-- Just add any missing columns
ALTER TABLE content_translations ADD COLUMN IF NOT EXISTS field_name VARCHAR(100);
ALTER TABLE content_translations ADD COLUMN IF NOT EXISTS translated_value TEXT;
ALTER TABLE content_translations ADD COLUMN IF NOT EXISTS is_machine_translated BOOLEAN DEFAULT false;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. ADD MISSING COLUMNS TO INDICATORS TABLE
-- The member_indicators route expects these columns
-- ═══════════════════════════════════════════════════════════════════════════════════
DO $$
BEGIN
    -- Add tagline
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'tagline') THEN
        ALTER TABLE indicators ADD COLUMN tagline VARCHAR(500);
    END IF;

    -- Add price_cents (convert from price decimal to cents integer)
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'price_cents') THEN
        ALTER TABLE indicators ADD COLUMN price_cents INTEGER DEFAULT 0;
        -- Migrate existing price data to price_cents
        UPDATE indicators SET price_cents = COALESCE((price * 100)::INTEGER, 0) WHERE price IS NOT NULL;
    END IF;

    -- Add is_free
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'is_free') THEN
        ALTER TABLE indicators ADD COLUMN is_free BOOLEAN DEFAULT false;
        UPDATE indicators SET is_free = (price_cents = 0 OR price_cents IS NULL);
    END IF;

    -- Add sale_price_cents
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'sale_price_cents') THEN
        ALTER TABLE indicators ADD COLUMN sale_price_cents INTEGER;
    END IF;

    -- Add logo_url
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'logo_url') THEN
        ALTER TABLE indicators ADD COLUMN logo_url VARCHAR(500);
        -- Use thumbnail as logo if exists
        UPDATE indicators SET logo_url = thumbnail WHERE thumbnail IS NOT NULL AND logo_url IS NULL;
    END IF;

    -- Add card_image_url
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'card_image_url') THEN
        ALTER TABLE indicators ADD COLUMN card_image_url VARCHAR(500);
    END IF;

    -- Add status
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'status') THEN
        ALTER TABLE indicators ADD COLUMN status VARCHAR(50) DEFAULT 'draft';
        UPDATE indicators SET status = CASE WHEN is_active = true THEN 'published' ELSE 'draft' END;
    END IF;

    -- Add is_published
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'is_published') THEN
        ALTER TABLE indicators ADD COLUMN is_published BOOLEAN DEFAULT false;
        UPDATE indicators SET is_published = (is_active = true);
    END IF;

    -- Add view_count
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'view_count') THEN
        ALTER TABLE indicators ADD COLUMN view_count INTEGER DEFAULT 0;
    END IF;

    -- Add download_count
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'download_count') THEN
        ALTER TABLE indicators ADD COLUMN download_count INTEGER DEFAULT 0;
    END IF;

    -- Add supported_platforms (JSONB for multiple platforms)
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'indicators' AND column_name = 'supported_platforms') THEN
        ALTER TABLE indicators ADD COLUMN supported_platforms JSONB DEFAULT '[]'::jsonb;
        -- Migrate existing platform column to supported_platforms array
        UPDATE indicators SET supported_platforms = jsonb_build_array(platform) WHERE platform IS NOT NULL;
    END IF;
END $$;

-- Add indexes for indicator queries
CREATE INDEX IF NOT EXISTS idx_indicators_is_published ON indicators(is_published);
CREATE INDEX IF NOT EXISTS idx_indicators_status ON indicators(status);
CREATE INDEX IF NOT EXISTS idx_indicators_is_free ON indicators(is_free);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. TRIGGERS FOR UPDATED_AT
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Generic updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply to new tables
DROP TRIGGER IF EXISTS trigger_room_traders_updated_at ON room_traders;
CREATE TRIGGER trigger_room_traders_updated_at
    BEFORE UPDATE ON room_traders FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_courses_enhanced_updated_at ON courses_enhanced;
CREATE TRIGGER trigger_courses_enhanced_updated_at
    BEFORE UPDATE ON courses_enhanced FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_media_updated_at ON media;
CREATE TRIGGER trigger_media_updated_at
    BEFORE UPDATE ON media FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_page_layouts_updated_at ON page_layouts;
CREATE TRIGGER trigger_page_layouts_updated_at
    BEFORE UPDATE ON page_layouts FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS trigger_locales_updated_at ON locales;
CREATE TRIGGER trigger_locales_updated_at
    BEFORE UPDATE ON locales FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 8. COMMENTS
-- ═══════════════════════════════════════════════════════════════════════════════════
COMMENT ON TABLE room_traders IS 'Instructors/traders for courses and trading rooms';
COMMENT ON TABLE courses_enhanced IS 'Enhanced course management with sections and lessons';
COMMENT ON TABLE media IS 'Media library for images, videos, and documents';
COMMENT ON TABLE page_layouts IS 'Page builder layouts with block configurations';
COMMENT ON TABLE locales IS 'Supported locales for i18n/localization';
COMMENT ON TABLE content_translations IS 'Translated content for multi-language support';

-- Done!
