-- ═══════════════════════════════════════════════════════════════════════════════════
-- REVOLUTION TRADING PROS - COMPLETE COURSE MANAGEMENT SYSTEM
-- Apple Principal Engineer ICT 7 Grade - January 2026
--
-- Features: Courses, Modules, Lessons, Downloads, Progress Tracking, Reviews
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ENHANCED COURSES TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add new columns to existing courses table
DO $$
BEGIN
    -- Card Display
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'card_image_url') THEN
        ALTER TABLE courses ADD COLUMN card_image_url VARCHAR(500);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'card_description') THEN
        ALTER TABLE courses ADD COLUMN card_description TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'card_badge') THEN
        ALTER TABLE courses ADD COLUMN card_badge VARCHAR(50);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'card_badge_color') THEN
        ALTER TABLE courses ADD COLUMN card_badge_color VARCHAR(20) DEFAULT '#10b981';
    END IF;
    
    -- Course Details
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'what_you_learn') THEN
        ALTER TABLE courses ADD COLUMN what_you_learn JSONB DEFAULT '[]'::jsonb;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'requirements') THEN
        ALTER TABLE courses ADD COLUMN requirements JSONB DEFAULT '[]'::jsonb;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'target_audience') THEN
        ALTER TABLE courses ADD COLUMN target_audience JSONB DEFAULT '[]'::jsonb;
    END IF;
    
    -- Instructor
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'instructor_name') THEN
        ALTER TABLE courses ADD COLUMN instructor_name VARCHAR(255);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'instructor_title') THEN
        ALTER TABLE courses ADD COLUMN instructor_title VARCHAR(255);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'instructor_avatar_url') THEN
        ALTER TABLE courses ADD COLUMN instructor_avatar_url VARCHAR(500);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'instructor_bio') THEN
        ALTER TABLE courses ADD COLUMN instructor_bio TEXT;
    END IF;
    
    -- Pricing & Access
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'is_free') THEN
        ALTER TABLE courses ADD COLUMN is_free BOOLEAN DEFAULT false;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'is_premium') THEN
        ALTER TABLE courses ADD COLUMN is_premium BOOLEAN DEFAULT false;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'required_membership_ids') THEN
        ALTER TABLE courses ADD COLUMN required_membership_ids JSONB DEFAULT '[]'::jsonb;
    END IF;
    
    -- Video Integration
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'bunny_library_id') THEN
        ALTER TABLE courses ADD COLUMN bunny_library_id BIGINT;
    END IF;
    
    -- SEO
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'meta_title') THEN
        ALTER TABLE courses ADD COLUMN meta_title VARCHAR(255);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'meta_description') THEN
        ALTER TABLE courses ADD COLUMN meta_description TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'og_image_url') THEN
        ALTER TABLE courses ADD COLUMN og_image_url VARCHAR(500);
    END IF;
    
    -- Status
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'status') THEN
        ALTER TABLE courses ADD COLUMN status VARCHAR(20) DEFAULT 'draft';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'published_at') THEN
        ALTER TABLE courses ADD COLUMN published_at TIMESTAMPTZ;
    END IF;
    
    -- Analytics
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'enrollment_count') THEN
        ALTER TABLE courses ADD COLUMN enrollment_count INTEGER DEFAULT 0;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'completion_rate') THEN
        ALTER TABLE courses ADD COLUMN completion_rate DECIMAL(5,2) DEFAULT 0;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'avg_rating') THEN
        ALTER TABLE courses ADD COLUMN avg_rating DECIMAL(3,2);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'review_count') THEN
        ALTER TABLE courses ADD COLUMN review_count INTEGER DEFAULT 0;
    END IF;
    
    -- Lesson/Module counts
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'module_count') THEN
        ALTER TABLE courses ADD COLUMN module_count INTEGER DEFAULT 0;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'lesson_count') THEN
        ALTER TABLE courses ADD COLUMN lesson_count INTEGER DEFAULT 0;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'courses' AND column_name = 'total_duration_minutes') THEN
        ALTER TABLE courses ADD COLUMN total_duration_minutes INTEGER DEFAULT 0;
    END IF;
END $$;

-- Course indexes
CREATE INDEX IF NOT EXISTS idx_courses_status ON courses(status);
CREATE INDEX IF NOT EXISTS idx_courses_published ON courses(is_published, published_at DESC);
CREATE INDEX IF NOT EXISTS idx_courses_slug ON courses(slug);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. COURSE MODULES
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_modules (
    id BIGSERIAL PRIMARY KEY,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    
    -- Content
    title VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Organization
    sort_order INTEGER NOT NULL DEFAULT 0,
    
    -- Status
    is_published BOOLEAN DEFAULT true,
    
    -- Drip Content
    drip_enabled BOOLEAN DEFAULT false,
    drip_days INTEGER DEFAULT 0,
    drip_date TIMESTAMPTZ,
    
    -- Metadata
    lesson_count INTEGER DEFAULT 0,
    total_duration_minutes INTEGER DEFAULT 0,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_modules_course ON course_modules(course_id, sort_order);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ENHANCED LESSONS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add new columns to existing lessons table
DO $$
BEGIN
    -- Module association
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'module_id') THEN
        ALTER TABLE lessons ADD COLUMN module_id BIGINT REFERENCES course_modules(id) ON DELETE SET NULL;
    END IF;
    
    -- Video Integration
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'video_id') THEN
        ALTER TABLE lessons ADD COLUMN video_id BIGINT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'bunny_video_guid') THEN
        ALTER TABLE lessons ADD COLUMN bunny_video_guid VARCHAR(100);
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'thumbnail_url') THEN
        ALTER TABLE lessons ADD COLUMN thumbnail_url VARCHAR(500);
    END IF;
    
    -- Lesson Content
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'content_html') THEN
        ALTER TABLE lessons ADD COLUMN content_html TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'resources') THEN
        ALTER TABLE lessons ADD COLUMN resources JSONB DEFAULT '[]'::jsonb;
    END IF;
    
    -- Access Control
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'is_preview') THEN
        ALTER TABLE lessons ADD COLUMN is_preview BOOLEAN DEFAULT false;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'is_published') THEN
        ALTER TABLE lessons ADD COLUMN is_published BOOLEAN DEFAULT true;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'drip_days') THEN
        ALTER TABLE lessons ADD COLUMN drip_days INTEGER DEFAULT 0;
    END IF;
    
    -- Prerequisites
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'prerequisite_lesson_ids') THEN
        ALTER TABLE lessons ADD COLUMN prerequisite_lesson_ids JSONB DEFAULT '[]'::jsonb;
    END IF;
    
    -- Sort within module
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'sort_order') THEN
        ALTER TABLE lessons ADD COLUMN sort_order INTEGER DEFAULT 0;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_lessons_module ON lessons(module_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_lessons_video ON lessons(bunny_video_guid) WHERE bunny_video_guid IS NOT NULL;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. COURSE DOWNLOADS (Self-hosted file system - REPLACES BOX.COM)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_downloads (
    id BIGSERIAL PRIMARY KEY,
    
    -- Parent Reference (polymorphic - can belong to course, module, or lesson)
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    module_id BIGINT REFERENCES course_modules(id) ON DELETE CASCADE,
    lesson_id UUID REFERENCES lessons(id) ON DELETE CASCADE,
    
    -- File Info
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size_bytes BIGINT,
    file_type VARCHAR(50),
    mime_type VARCHAR(100),
    
    -- Bunny Storage
    bunny_file_id VARCHAR(100),
    storage_zone VARCHAR(100),
    
    -- CDN URLs
    download_url VARCHAR(500),
    preview_url VARCHAR(500),
    
    -- Organization
    category VARCHAR(100) DEFAULT 'resource',
    sort_order INTEGER DEFAULT 0,
    
    -- Access Control
    is_public BOOLEAN DEFAULT false,
    require_enrollment BOOLEAN DEFAULT true,
    require_lesson_complete BOOLEAN DEFAULT false,
    
    -- Analytics
    download_count INTEGER DEFAULT 0,
    
    -- Audit
    uploaded_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Ensure at least one parent reference
    CONSTRAINT course_downloads_parent_check CHECK (
        course_id IS NOT NULL OR module_id IS NOT NULL OR lesson_id IS NOT NULL
    )
);

CREATE INDEX IF NOT EXISTS idx_course_downloads_course ON course_downloads(course_id) WHERE course_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_course_downloads_module ON course_downloads(module_id) WHERE module_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_course_downloads_lesson ON course_downloads(lesson_id) WHERE lesson_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_course_downloads_category ON course_downloads(category);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. USER COURSE ENROLLMENTS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_course_enrollments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    
    -- Progress
    current_module_id BIGINT REFERENCES course_modules(id) ON DELETE SET NULL,
    current_lesson_id UUID REFERENCES lessons(id) ON DELETE SET NULL,
    completed_lesson_ids JSONB DEFAULT '[]'::jsonb,
    progress_percent SMALLINT DEFAULT 0 CHECK (progress_percent >= 0 AND progress_percent <= 100),
    
    -- Status
    status VARCHAR(20) DEFAULT 'enrolled',
    enrolled_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    last_accessed_at TIMESTAMPTZ,
    
    -- Access
    access_expires_at TIMESTAMPTZ,
    is_lifetime_access BOOLEAN DEFAULT false,
    
    -- Purchase info
    order_id BIGINT,
    price_paid_cents INTEGER DEFAULT 0,
    
    -- Certificate
    certificate_issued BOOLEAN DEFAULT false,
    certificate_url VARCHAR(500),
    certificate_issued_at TIMESTAMPTZ,
    
    -- Notes & Bookmarks
    notes JSONB DEFAULT '{}'::jsonb,
    bookmarks JSONB DEFAULT '[]'::jsonb,
    
    UNIQUE(user_id, course_id)
);

CREATE INDEX IF NOT EXISTS idx_enrollments_user ON user_course_enrollments(user_id, last_accessed_at DESC);
CREATE INDEX IF NOT EXISTS idx_enrollments_course ON user_course_enrollments(course_id);
CREATE INDEX IF NOT EXISTS idx_enrollments_status ON user_course_enrollments(status);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. LESSON PROGRESS (Granular tracking)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_lesson_progress (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    enrollment_id BIGINT NOT NULL REFERENCES user_course_enrollments(id) ON DELETE CASCADE,
    
    -- Video Progress
    video_position_seconds INTEGER DEFAULT 0,
    video_duration_seconds INTEGER DEFAULT 0,
    video_watch_percent SMALLINT DEFAULT 0,
    
    -- Completion
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,
    
    -- Engagement
    time_spent_seconds INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 1,
    
    -- Timestamps
    first_accessed_at TIMESTAMPTZ DEFAULT NOW(),
    last_accessed_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(user_id, lesson_id)
);

CREATE INDEX IF NOT EXISTS idx_lesson_progress_user ON user_lesson_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_lesson_progress_enrollment ON user_lesson_progress(enrollment_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. COURSE REVIEWS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_reviews (
    id BIGSERIAL PRIMARY KEY,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    enrollment_id BIGINT REFERENCES user_course_enrollments(id) ON DELETE SET NULL,
    
    -- Review Content
    rating SMALLINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    content TEXT,
    
    -- Status
    is_verified_purchase BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_approved BOOLEAN DEFAULT false,
    
    -- Helpfulness
    helpful_count INTEGER DEFAULT 0,
    not_helpful_count INTEGER DEFAULT 0,
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(course_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_reviews_course ON course_reviews(course_id, is_approved, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_reviews_user ON course_reviews(user_id);
CREATE INDEX IF NOT EXISTS idx_reviews_rating ON course_reviews(course_id, rating);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 8. DOWNLOAD TRACKING
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS download_events (
    id BIGSERIAL PRIMARY KEY,
    download_id BIGINT NOT NULL REFERENCES course_downloads(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    
    -- Event Info
    ip_address VARCHAR(45),
    user_agent TEXT,
    
    -- Timestamp
    downloaded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_download_events_download ON download_events(download_id);
CREATE INDEX IF NOT EXISTS idx_download_events_user ON download_events(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_download_events_date ON download_events(downloaded_at);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 9. FUNCTIONS & TRIGGERS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Function to update course module/lesson counts
CREATE OR REPLACE FUNCTION update_course_counts()
RETURNS TRIGGER AS $$
DECLARE
    v_course_id UUID;
BEGIN
    -- Determine course_id based on operation
    IF TG_TABLE_NAME = 'course_modules' THEN
        IF TG_OP = 'DELETE' THEN
            v_course_id := OLD.course_id;
        ELSE
            v_course_id := NEW.course_id;
        END IF;
        
        -- Update module count
        UPDATE courses SET
            module_count = (SELECT COUNT(*) FROM course_modules WHERE course_id = v_course_id AND is_published = true),
            updated_at = NOW()
        WHERE id = v_course_id;
        
    ELSIF TG_TABLE_NAME = 'lessons' THEN
        IF TG_OP = 'DELETE' THEN
            v_course_id := OLD.course_id;
        ELSE
            v_course_id := NEW.course_id;
        END IF;
        
        -- Update lesson count and duration
        UPDATE courses SET
            lesson_count = (SELECT COUNT(*) FROM lessons WHERE course_id = v_course_id AND is_published = true),
            total_duration_minutes = COALESCE((SELECT SUM(duration_minutes) FROM lessons WHERE course_id = v_course_id AND is_published = true), 0),
            updated_at = NOW()
        WHERE id = v_course_id;
        
        -- Update module lesson count if module_id exists
        IF NEW.module_id IS NOT NULL THEN
            UPDATE course_modules SET
                lesson_count = (SELECT COUNT(*) FROM lessons WHERE module_id = NEW.module_id AND is_published = true),
                total_duration_minutes = COALESCE((SELECT SUM(duration_minutes) FROM lessons WHERE module_id = NEW.module_id AND is_published = true), 0),
                updated_at = NOW()
            WHERE id = NEW.module_id;
        END IF;
    END IF;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Triggers for course counts
DROP TRIGGER IF EXISTS trigger_update_course_module_count ON course_modules;
CREATE TRIGGER trigger_update_course_module_count
    AFTER INSERT OR UPDATE OR DELETE ON course_modules
    FOR EACH ROW
    EXECUTE FUNCTION update_course_counts();

DROP TRIGGER IF EXISTS trigger_update_course_lesson_count ON lessons;
CREATE TRIGGER trigger_update_course_lesson_count
    AFTER INSERT OR UPDATE OR DELETE ON lessons
    FOR EACH ROW
    EXECUTE FUNCTION update_course_counts();

-- Function to update course rating stats
CREATE OR REPLACE FUNCTION update_course_rating_stats()
RETURNS TRIGGER AS $$
DECLARE
    v_course_id UUID;
BEGIN
    IF TG_OP = 'DELETE' THEN
        v_course_id := OLD.course_id;
    ELSE
        v_course_id := NEW.course_id;
    END IF;
    
    UPDATE courses SET
        avg_rating = (SELECT AVG(rating)::DECIMAL(3,2) FROM course_reviews WHERE course_id = v_course_id AND is_approved = true),
        review_count = (SELECT COUNT(*) FROM course_reviews WHERE course_id = v_course_id AND is_approved = true),
        updated_at = NOW()
    WHERE id = v_course_id;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_course_rating ON course_reviews;
CREATE TRIGGER trigger_update_course_rating
    AFTER INSERT OR UPDATE OR DELETE ON course_reviews
    FOR EACH ROW
    EXECUTE FUNCTION update_course_rating_stats();

-- Function to update enrollment count
CREATE OR REPLACE FUNCTION update_enrollment_count()
RETURNS TRIGGER AS $$
DECLARE
    v_course_id UUID;
BEGIN
    IF TG_OP = 'DELETE' THEN
        v_course_id := OLD.course_id;
    ELSE
        v_course_id := NEW.course_id;
    END IF;
    
    UPDATE courses SET
        enrollment_count = (SELECT COUNT(*) FROM user_course_enrollments WHERE course_id = v_course_id),
        updated_at = NOW()
    WHERE id = v_course_id;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_enrollment_count ON user_course_enrollments;
CREATE TRIGGER trigger_update_enrollment_count
    AFTER INSERT OR DELETE ON user_course_enrollments
    FOR EACH ROW
    EXECUTE FUNCTION update_enrollment_count();

-- Function to update download count
CREATE OR REPLACE FUNCTION update_download_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE course_downloads SET
        download_count = download_count + 1
    WHERE id = NEW.download_id;
    
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_download_count ON download_events;
CREATE TRIGGER trigger_update_download_count
    AFTER INSERT ON download_events
    FOR EACH ROW
    EXECUTE FUNCTION update_download_count();

-- Function to calculate enrollment progress
CREATE OR REPLACE FUNCTION calculate_enrollment_progress(p_enrollment_id BIGINT)
RETURNS SMALLINT AS $$
DECLARE
    v_total_lessons INTEGER;
    v_completed_lessons INTEGER;
    v_progress SMALLINT;
BEGIN
    SELECT COUNT(*) INTO v_total_lessons
    FROM lessons l
    JOIN user_course_enrollments e ON l.course_id = e.course_id
    WHERE e.id = p_enrollment_id AND l.is_published = true;
    
    SELECT COUNT(*) INTO v_completed_lessons
    FROM user_lesson_progress lp
    WHERE lp.enrollment_id = p_enrollment_id AND lp.is_completed = true;
    
    IF v_total_lessons = 0 THEN
        RETURN 0;
    END IF;
    
    v_progress := ((v_completed_lessons::DECIMAL / v_total_lessons::DECIMAL) * 100)::SMALLINT;
    
    -- Update enrollment progress
    UPDATE user_course_enrollments SET
        progress_percent = v_progress,
        completed_lesson_ids = (
            SELECT jsonb_agg(lesson_id)
            FROM user_lesson_progress
            WHERE enrollment_id = p_enrollment_id AND is_completed = true
        ),
        status = CASE WHEN v_progress >= 100 THEN 'completed' ELSE 'in_progress' END,
        completed_at = CASE WHEN v_progress >= 100 THEN NOW() ELSE NULL END,
        updated_at = NOW()
    WHERE id = p_enrollment_id;
    
    RETURN v_progress;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 10. SAMPLE DATA (for development)
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Insert sample course if none exist
INSERT INTO courses (
    id, title, slug, description, price_cents, level, is_published, status,
    card_description, instructor_name, instructor_title,
    what_you_learn, requirements, target_audience
)
SELECT 
    gen_random_uuid(),
    'Quickstart To Precision Trading',
    'quickstart-precision-trading-c',
    'Master the fundamentals of precision trading with this comprehensive course. Learn entry techniques, risk management, and proven strategies.',
    0,
    'beginner',
    true,
    'published',
    'Master the fundamentals of precision trading in just 2 hours',
    'Moxie',
    'Lead Trading Instructor',
    '["Understand market structure and price action", "Master entry and exit techniques", "Develop risk management skills", "Build a consistent trading routine"]'::jsonb,
    '["TradingView or ThinkorSwim account", "Basic understanding of markets"]'::jsonb,
    '["Beginner traders", "Those looking to improve consistency", "Traders wanting a systematic approach"]'::jsonb
WHERE NOT EXISTS (SELECT 1 FROM courses WHERE slug = 'quickstart-precision-trading-c');

-- ═══════════════════════════════════════════════════════════════════════════════════
-- MIGRATION COMPLETE
-- ═══════════════════════════════════════════════════════════════════════════════════
