-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 016: Fix Course Schema for Admin System
-- Apple ICT 7 Principal Engineer - January 2026
-- 
-- FIXES:
-- 1. Add missing columns to courses table for admin_courses.rs compatibility
-- 2. Create proper course_downloads table with all required fields
-- 3. Add course_modules table linked to courses (UUID)
-- 4. Add enhanced lessons columns
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ENHANCE COURSES TABLE
-- Add missing columns required by admin_courses.rs
-- ═══════════════════════════════════════════════════════════════════════════════════

ALTER TABLE courses ADD COLUMN IF NOT EXISTS card_description TEXT;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS card_image_url VARCHAR(500);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS card_badge VARCHAR(100);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS card_badge_color VARCHAR(20);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS is_free BOOLEAN DEFAULT false;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS status VARCHAR(50) DEFAULT 'draft';
ALTER TABLE courses ADD COLUMN IF NOT EXISTS instructor_name VARCHAR(255);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS instructor_title VARCHAR(255);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS instructor_avatar_url VARCHAR(500);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS instructor_bio TEXT;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS what_you_learn JSONB DEFAULT '[]'::jsonb;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS requirements JSONB DEFAULT '[]'::jsonb;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS tags JSONB DEFAULT '[]'::jsonb;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS module_count INTEGER DEFAULT 0;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS lesson_count INTEGER DEFAULT 0;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS total_duration_minutes INTEGER DEFAULT 0;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS enrollment_count INTEGER DEFAULT 0;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS avg_rating NUMERIC(3,2) DEFAULT 0;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS review_count INTEGER DEFAULT 0;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS bunny_library_id VARCHAR(100);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS seo_title VARCHAR(255);
ALTER TABLE courses ADD COLUMN IF NOT EXISTS seo_description TEXT;
ALTER TABLE courses ADD COLUMN IF NOT EXISTS seo_keywords TEXT;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. CREATE COURSE_MODULES TABLE (linked to courses UUID)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_modules_v2 (
    id BIGSERIAL PRIMARY KEY,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_modules_v2_course ON course_modules_v2(course_id, sort_order);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ENHANCE LESSONS TABLE
-- Add missing columns required by admin_courses.rs
-- ═══════════════════════════════════════════════════════════════════════════════════

ALTER TABLE lessons ADD COLUMN IF NOT EXISTS module_id BIGINT REFERENCES course_modules_v2(id) ON DELETE SET NULL;
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS bunny_video_guid VARCHAR(100);
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS thumbnail_url VARCHAR(500);
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS content_html TEXT;
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS is_preview BOOLEAN DEFAULT false;
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT true;
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS sort_order INTEGER DEFAULT 0;
ALTER TABLE lessons ADD COLUMN IF NOT EXISTS drip_days INTEGER DEFAULT 0;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. CREATE PROPER COURSE_DOWNLOADS TABLE
-- Full schema required by admin_courses.rs for file management
-- ═══════════════════════════════════════════════════════════════════════════════════

DROP TABLE IF EXISTS course_downloads CASCADE;

CREATE TABLE course_downloads (
    id BIGSERIAL PRIMARY KEY,
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    module_id BIGINT REFERENCES course_modules_v2(id) ON DELETE SET NULL,
    lesson_id UUID REFERENCES lessons(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size_bytes BIGINT,
    file_type VARCHAR(50),
    mime_type VARCHAR(100),
    bunny_file_id VARCHAR(100),
    storage_zone VARCHAR(100),
    download_url VARCHAR(500),
    preview_url VARCHAR(500),
    category VARCHAR(50) DEFAULT 'resource',
    sort_order INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT false,
    require_enrollment BOOLEAN DEFAULT true,
    require_lesson_complete BOOLEAN DEFAULT false,
    download_count INTEGER DEFAULT 0,
    uploaded_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_downloads_course ON course_downloads(course_id);
CREATE INDEX IF NOT EXISTS idx_course_downloads_lesson ON course_downloads(lesson_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. CREATE USER_COURSE_ENROLLMENTS TABLE (linked to courses UUID)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_course_enrollments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    current_module_id BIGINT REFERENCES course_modules_v2(id) ON DELETE SET NULL,
    current_lesson_id UUID REFERENCES lessons(id) ON DELETE SET NULL,
    completed_lesson_ids JSONB DEFAULT '[]'::jsonb,
    progress_percent SMALLINT DEFAULT 0,
    status VARCHAR(50) DEFAULT 'active',
    enrolled_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    last_accessed_at TIMESTAMPTZ,
    access_expires_at TIMESTAMPTZ,
    is_lifetime_access BOOLEAN DEFAULT false,
    UNIQUE(user_id, course_id)
);

CREATE INDEX IF NOT EXISTS idx_user_enrollments_user ON user_course_enrollments(user_id);
CREATE INDEX IF NOT EXISTS idx_user_enrollments_course ON user_course_enrollments(course_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. CREATE USER_LESSON_PROGRESS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_lesson_progress (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    watch_position_seconds INTEGER DEFAULT 0,
    watch_time_total_seconds INTEGER DEFAULT 0,
    watch_percent SMALLINT DEFAULT 0,
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, lesson_id)
);

CREATE INDEX IF NOT EXISTS idx_lesson_progress_user_course ON user_lesson_progress(user_id, course_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. UPDATE TRIGGERS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Auto-update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_courses_updated_at ON courses;
CREATE TRIGGER update_courses_updated_at
    BEFORE UPDATE ON courses
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_lessons_updated_at ON lessons;
CREATE TRIGGER update_lessons_updated_at
    BEFORE UPDATE ON lessons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_course_downloads_updated_at ON course_downloads;
CREATE TRIGGER update_course_downloads_updated_at
    BEFORE UPDATE ON course_downloads
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_course_modules_v2_updated_at ON course_modules_v2;
CREATE TRIGGER update_course_modules_v2_updated_at
    BEFORE UPDATE ON course_modules_v2
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
