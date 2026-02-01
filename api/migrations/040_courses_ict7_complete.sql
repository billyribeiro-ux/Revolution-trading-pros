-- ═══════════════════════════════════════════════════════════════════════════════════
-- Courses System ICT 7 Complete - February 2026
-- Quiz/Assessment, Categories/Tags, Reviews
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- COURSE CATEGORIES
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    icon VARCHAR(100),
    color VARCHAR(20),
    parent_id BIGINT REFERENCES course_categories(id) ON DELETE SET NULL,
    sort_order INT DEFAULT 0,
    is_featured BOOLEAN DEFAULT false,
    course_count INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_categories_slug ON course_categories(slug);
CREATE INDEX IF NOT EXISTS idx_course_categories_parent ON course_categories(parent_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- COURSE TAGS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    slug VARCHAR(50) NOT NULL UNIQUE,
    color VARCHAR(20),
    course_count INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_tags_slug ON course_tags(slug);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- COURSE CATEGORY/TAG MAPPINGS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_category_mappings (
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    category_id BIGINT NOT NULL REFERENCES course_categories(id) ON DELETE CASCADE,
    PRIMARY KEY (course_id, category_id)
);

CREATE TABLE IF NOT EXISTS course_tag_mappings (
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES course_tags(id) ON DELETE CASCADE,
    PRIMARY KEY (course_id, tag_id)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- COURSE QUIZZES
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_quizzes (
    id BIGSERIAL PRIMARY KEY,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    lesson_id UUID REFERENCES lessons(id) ON DELETE SET NULL,
    module_id BIGINT REFERENCES course_modules_v2(id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    quiz_type VARCHAR(20) DEFAULT 'graded', -- graded, practice, survey
    passing_score INT DEFAULT 70,
    time_limit_minutes INT,
    max_attempts INT,
    shuffle_questions BOOLEAN DEFAULT false,
    shuffle_answers BOOLEAN DEFAULT false,
    show_correct_answers BOOLEAN DEFAULT true,
    is_required BOOLEAN DEFAULT false,
    is_published BOOLEAN DEFAULT false,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_course_quizzes_course ON course_quizzes(course_id);
CREATE INDEX IF NOT EXISTS idx_course_quizzes_lesson ON course_quizzes(lesson_id);
CREATE INDEX IF NOT EXISTS idx_course_quizzes_module ON course_quizzes(module_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- QUIZ QUESTIONS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS quiz_questions (
    id BIGSERIAL PRIMARY KEY,
    quiz_id BIGINT NOT NULL REFERENCES course_quizzes(id) ON DELETE CASCADE,
    question_type VARCHAR(30) DEFAULT 'multiple_choice', -- multiple_choice, true_false, short_answer, essay
    question_text TEXT NOT NULL,
    question_html TEXT,
    explanation TEXT,
    points INT DEFAULT 1,
    sort_order INT DEFAULT 0,
    is_required BOOLEAN DEFAULT true,
    media_url TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_quiz_questions_quiz ON quiz_questions(quiz_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- QUIZ ANSWERS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS quiz_answers (
    id BIGSERIAL PRIMARY KEY,
    question_id BIGINT NOT NULL REFERENCES quiz_questions(id) ON DELETE CASCADE,
    answer_text TEXT NOT NULL,
    is_correct BOOLEAN DEFAULT false,
    sort_order INT DEFAULT 0,
    feedback TEXT
);

CREATE INDEX IF NOT EXISTS idx_quiz_answers_question ON quiz_answers(question_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- USER QUIZ ATTEMPTS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS user_quiz_attempts (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    quiz_id BIGINT NOT NULL REFERENCES course_quizzes(id) ON DELETE CASCADE,
    enrollment_id BIGINT NOT NULL REFERENCES user_course_enrollments(id) ON DELETE CASCADE,
    score INT,
    max_score INT NOT NULL,
    score_percent DOUBLE PRECISION,
    passed BOOLEAN,
    started_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP,
    time_spent_seconds INT,
    attempt_number INT DEFAULT 1,
    answers JSONB
);

CREATE INDEX IF NOT EXISTS idx_user_quiz_attempts_user ON user_quiz_attempts(user_id);
CREATE INDEX IF NOT EXISTS idx_user_quiz_attempts_quiz ON user_quiz_attempts(quiz_id);
CREATE INDEX IF NOT EXISTS idx_user_quiz_attempts_enrollment ON user_quiz_attempts(enrollment_id);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- COURSE REVIEWS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS course_reviews (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    rating SMALLINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(255),
    content TEXT,
    is_verified_purchase BOOLEAN DEFAULT false,
    is_approved BOOLEAN DEFAULT true,
    helpful_count INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(user_id, course_id)
);

CREATE INDEX IF NOT EXISTS idx_course_reviews_course ON course_reviews(course_id);
CREATE INDEX IF NOT EXISTS idx_course_reviews_user ON course_reviews(user_id);
CREATE INDEX IF NOT EXISTS idx_course_reviews_rating ON course_reviews(course_id, rating);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- REVIEW HELPFUL VOTES
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS review_helpful_votes (
    id BIGSERIAL PRIMARY KEY,
    review_id BIGINT NOT NULL REFERENCES course_reviews(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    helpful BOOLEAN NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(review_id, user_id)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- ADD PREREQUISITE_LESSON_IDS TO LESSONS IF NOT EXISTS
-- ═══════════════════════════════════════════════════════════════════════════════════

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'prerequisite_lesson_ids') THEN
        ALTER TABLE lessons ADD COLUMN prerequisite_lesson_ids JSONB;
    END IF;
END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- SEED DEFAULT CATEGORIES
-- ═══════════════════════════════════════════════════════════════════════════════════

INSERT INTO course_categories (name, slug, description, color, sort_order, is_featured) VALUES
    ('Day Trading', 'day-trading', 'Learn intraday trading strategies', '#3b82f6', 1, true),
    ('Swing Trading', 'swing-trading', 'Multi-day position trading strategies', '#10b981', 2, true),
    ('Technical Analysis', 'technical-analysis', 'Chart patterns and indicators', '#8b5cf6', 3, true),
    ('Options Trading', 'options-trading', 'Options strategies and Greeks', '#f59e0b', 4, false),
    ('Forex Trading', 'forex-trading', 'Currency trading fundamentals', '#ef4444', 5, false),
    ('Risk Management', 'risk-management', 'Position sizing and portfolio management', '#6366f1', 6, false),
    ('Psychology', 'psychology', 'Trading mindset and discipline', '#ec4899', 7, false)
ON CONFLICT (slug) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- SEED DEFAULT TAGS
-- ═══════════════════════════════════════════════════════════════════════════════════

INSERT INTO course_tags (name, slug, color) VALUES
    ('Beginner', 'beginner', '#10b981'),
    ('Intermediate', 'intermediate', '#f59e0b'),
    ('Advanced', 'advanced', '#ef4444'),
    ('Free', 'free', '#6366f1'),
    ('Popular', 'popular', '#ec4899'),
    ('New', 'new', '#8b5cf6'),
    ('Best Seller', 'best-seller', '#f97316'),
    ('Live Trading', 'live-trading', '#3b82f6'),
    ('Self-Paced', 'self-paced', '#84cc16')
ON CONFLICT (slug) DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- UPDATE FUNCTIONS FOR CATEGORY/TAG COUNTS
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION update_category_course_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE course_categories SET course_count = course_count + 1 WHERE id = NEW.category_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE course_categories SET course_count = GREATEST(course_count - 1, 0) WHERE id = OLD.category_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_tag_course_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE course_tags SET course_count = course_count + 1 WHERE id = NEW.tag_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE course_tags SET course_count = GREATEST(course_count - 1, 0) WHERE id = OLD.tag_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_category_count ON course_category_mappings;
CREATE TRIGGER trigger_update_category_count
    AFTER INSERT OR DELETE ON course_category_mappings
    FOR EACH ROW EXECUTE FUNCTION update_category_course_count();

DROP TRIGGER IF EXISTS trigger_update_tag_count ON course_tag_mappings;
CREATE TRIGGER trigger_update_tag_count
    AFTER INSERT OR DELETE ON course_tag_mappings
    FOR EACH ROW EXECUTE FUNCTION update_tag_course_count();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- ADD DRIP CONTENT FIELDS IF NOT EXISTS
-- ═══════════════════════════════════════════════════════════════════════════════════

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'lessons' AND column_name = 'drip_days') THEN
        ALTER TABLE lessons ADD COLUMN drip_days INT DEFAULT 0;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'course_modules_v2' AND column_name = 'drip_enabled') THEN
        ALTER TABLE course_modules_v2 ADD COLUMN drip_enabled BOOLEAN DEFAULT false;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'course_modules_v2' AND column_name = 'drip_days') THEN
        ALTER TABLE course_modules_v2 ADD COLUMN drip_days INT DEFAULT 0;
    END IF;
END $$;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- ADD CERTIFICATE FIELDS TO ENROLLMENTS IF NOT EXISTS
-- ═══════════════════════════════════════════════════════════════════════════════════

DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'user_course_enrollments' AND column_name = 'certificate_issued') THEN
        ALTER TABLE user_course_enrollments ADD COLUMN certificate_issued BOOLEAN DEFAULT false;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'user_course_enrollments' AND column_name = 'certificate_url') THEN
        ALTER TABLE user_course_enrollments ADD COLUMN certificate_url TEXT;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'user_course_enrollments' AND column_name = 'certificate_issued_at') THEN
        ALTER TABLE user_course_enrollments ADD COLUMN certificate_issued_at TIMESTAMP;
    END IF;
END $$;

-- ICT Level 7 Course System Complete
