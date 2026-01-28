-- ═══════════════════════════════════════════════════════════════════════════════════
-- CONSOLIDATED SCHEMA - Revolution Trading Pros
-- Apple ICT 7 Principal Engineer Grade - January 17, 2026
--
-- This migration adds ALL tables required by backend routes that don't exist in 001-014.
-- Forensically verified against api/src/routes/*.rs
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. TRADING ROOMS (required by trading_rooms.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS trading_rooms (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    short_description VARCHAR(500),
    thumbnail_url VARCHAR(500),
    banner_url VARCHAR(500),
    room_type VARCHAR(50) DEFAULT 'trading',
    access_level VARCHAR(50) DEFAULT 'member',
    is_active BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    is_public BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0,
    available_sections JSONB DEFAULT '[]'::jsonb,
    features JSONB DEFAULT '{}'::jsonb,
    icon VARCHAR(50),
    color VARCHAR(20),
    settings JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_slug ON trading_rooms(slug);
CREATE INDEX IF NOT EXISTS idx_trading_rooms_active ON trading_rooms(is_active);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. ROOM TRADERS (required by trading_rooms.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS room_traders (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE,
    title VARCHAR(255),
    bio TEXT,
    avatar_url VARCHAR(500),
    cover_image_url VARCHAR(500),
    twitter_url VARCHAR(500),
    youtube_url VARCHAR(500),
    instagram_url VARCHAR(500),
    trading_style VARCHAR(100),
    specialties JSONB DEFAULT '[]'::jsonb,
    years_experience INTEGER,
    is_active BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_room_traders_slug ON room_traders(slug);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. MEDIA LIBRARY (required by media.rs)
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

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 4. COURSES ENHANCED (required by admin_courses.rs, member_courses.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS courses_enhanced (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    subtitle VARCHAR(500),
    description TEXT,
    description_html TEXT,
    thumbnail_url VARCHAR(500),
    trailer_video_url VARCHAR(500),
    trailer_bunny_guid VARCHAR(100),
    difficulty_level VARCHAR(50) DEFAULT 'beginner',
    category VARCHAR(100),
    tags JSONB DEFAULT '[]'::jsonb,
    instructor_id BIGINT REFERENCES room_traders(id) ON DELETE SET NULL,
    estimated_duration_minutes INTEGER DEFAULT 0,
    total_lessons INTEGER DEFAULT 0,
    total_sections INTEGER DEFAULT 0,
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_free BOOLEAN DEFAULT false,
    required_plan_id BIGINT,
    price_cents INTEGER,
    prerequisite_course_ids JSONB DEFAULT '[]'::jsonb,
    certificate_enabled BOOLEAN DEFAULT true,
    certificate_template VARCHAR(100) DEFAULT 'default',
    completion_threshold_percent INTEGER DEFAULT 90,
    meta_title VARCHAR(255),
    meta_description TEXT,
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_slug ON courses_enhanced(slug);
CREATE INDEX IF NOT EXISTS idx_courses_enhanced_published ON courses_enhanced(is_published);

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
CREATE INDEX IF NOT EXISTS idx_course_sections_course ON course_sections(course_id, sort_order);

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
CREATE INDEX IF NOT EXISTS idx_course_lessons_section ON course_lessons(section_id, sort_order);

CREATE TABLE IF NOT EXISTS course_modules (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS course_resources (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    section_id BIGINT REFERENCES course_sections(id) ON DELETE CASCADE,
    lesson_id BIGINT REFERENCES course_lessons(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_url VARCHAR(500) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(50),
    file_size_bytes BIGINT,
    sort_order INTEGER DEFAULT 0,
    version VARCHAR(50) DEFAULT '1.0',
    download_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS course_live_sessions (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    section_id BIGINT REFERENCES course_sections(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    session_date DATE NOT NULL,
    session_time TIME,
    video_url VARCHAR(500),
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    duration_seconds INTEGER,
    thumbnail_url VARCHAR(500),
    replay_available_until TIMESTAMPTZ,
    is_published BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS course_downloads (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    resource_id BIGINT,
    downloaded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_course_enrollments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    enrolled_at TIMESTAMPTZ DEFAULT NOW(),
    enrollment_source VARCHAR(50),
    access_expires_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    completed_lessons INTEGER DEFAULT 0,
    total_lessons INTEGER DEFAULT 0,
    progress_percent INTEGER DEFAULT 0,
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,
    certificate_issued BOOLEAN DEFAULT false,
    certificate_url VARCHAR(500),
    last_lesson_id BIGINT REFERENCES course_lessons(id) ON DELETE SET NULL,
    last_position_seconds INTEGER DEFAULT 0,
    last_activity_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, course_id)
);
CREATE INDEX IF NOT EXISTS idx_enrollments_user ON user_course_enrollments(user_id);
CREATE INDEX IF NOT EXISTS idx_enrollments_course ON user_course_enrollments(course_id);

CREATE TABLE IF NOT EXISTS user_lesson_progress (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    course_id BIGINT NOT NULL REFERENCES courses_enhanced(id) ON DELETE CASCADE,
    lesson_id BIGINT NOT NULL REFERENCES course_lessons(id) ON DELETE CASCADE,
    watch_position_seconds INTEGER DEFAULT 0,
    watch_time_total_seconds INTEGER DEFAULT 0,
    watch_percent INTEGER DEFAULT 0,
    is_completed BOOLEAN DEFAULT false,
    completed_at TIMESTAMPTZ,
    notes TEXT,
    bookmarked BOOLEAN DEFAULT false,
    first_watched_at TIMESTAMPTZ DEFAULT NOW(),
    last_watched_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, lesson_id)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 5. INDICATORS ENHANCED (required by admin_indicators.rs, member_indicators.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS indicator_platforms (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE,
    icon_url VARCHAR(500),
    file_extensions JSONB DEFAULT '[]'::jsonb,
    installation_guide_url VARCHAR(500),
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true
);
INSERT INTO indicator_platforms (name, slug, file_extensions, sort_order) VALUES
    ('ThinkorSwim', 'thinkorswim', '[".tos", ".ts"]', 1),
    ('TradingView', 'tradingview', '[".pine", ".txt"]', 2),
    ('NinjaTrader', 'ninjatrader', '[".cs", ".zip"]', 3)
ON CONFLICT (slug) DO NOTHING;

CREATE TABLE IF NOT EXISTS indicators_enhanced (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    subtitle VARCHAR(500),
    description TEXT,
    description_html TEXT,
    thumbnail_url VARCHAR(500),
    banner_url VARCHAR(500),
    category VARCHAR(100),
    tags JSONB DEFAULT '[]'::jsonb,
    creator_id BIGINT REFERENCES room_traders(id) ON DELETE SET NULL,
    current_version VARCHAR(50) DEFAULT '1.0.0',
    changelog TEXT,
    supported_platforms JSONB DEFAULT '[]'::jsonb,
    is_published BOOLEAN DEFAULT false,
    is_featured BOOLEAN DEFAULT false,
    is_free BOOLEAN DEFAULT false,
    required_plan_id BIGINT,
    price_cents INTEGER,
    download_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    meta_title VARCHAR(255),
    meta_description TEXT,
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_indicators_enhanced_slug ON indicators_enhanced(slug);

CREATE TABLE IF NOT EXISTS indicator_videos (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    video_url VARCHAR(500),
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    duration_seconds INTEGER,
    thumbnail_url VARCHAR(500),
    video_type VARCHAR(50) DEFAULT 'tutorial',
    sort_order INTEGER DEFAULT 0,
    is_published BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indicator_platform_files (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    platform_slug VARCHAR(100) NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_display_name VARCHAR(255),
    file_url VARCHAR(500) NOT NULL,
    original_filename VARCHAR(255),
    file_size_bytes BIGINT,
    file_type VARCHAR(50),
    version VARCHAR(50) DEFAULT '1.0',
    installation_notes TEXT,
    sort_order INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indicator_documentation (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    content_type VARCHAR(50) DEFAULT 'file',
    content_html TEXT,
    file_url VARCHAR(500),
    external_url VARCHAR(500),
    doc_type VARCHAR(50) DEFAULT 'guide',
    sort_order INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    is_published BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indicator_tradingview_access (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,
    tradingview_username VARCHAR(255) NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    granted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revoke_reason TEXT,
    notification_sent BOOLEAN DEFAULT false,
    notification_sent_at TIMESTAMPTZ,
    requested_at TIMESTAMPTZ DEFAULT NOW(),
    requested_by BIGINT,
    admin_notes TEXT,
    UNIQUE(indicator_id, tradingview_username)
);

CREATE TABLE IF NOT EXISTS user_indicator_access (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    access_expires_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    access_source VARCHAR(50),
    last_download_at TIMESTAMPTZ,
    total_downloads INTEGER DEFAULT 0,
    UNIQUE(user_id, indicator_id)
);

CREATE TABLE IF NOT EXISTS indicator_download_log (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT NOT NULL REFERENCES indicators_enhanced(id) ON DELETE CASCADE,
    file_id BIGINT REFERENCES indicator_platform_files(id) ON DELETE SET NULL,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    platform_slug VARCHAR(100),
    file_name VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    downloaded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indicator_files (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT REFERENCES indicators(id) ON DELETE CASCADE,
    platform VARCHAR(50),
    file_url VARCHAR(500),
    file_name VARCHAR(255),
    version VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indicator_downloads (
    id BIGSERIAL PRIMARY KEY,
    indicator_id BIGINT REFERENCES indicators(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    file_id BIGINT,
    downloaded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_indicator_ownership (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    indicator_id BIGINT NOT NULL REFERENCES indicators(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    source VARCHAR(50),
    UNIQUE(user_id, indicator_id)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 6. UNIFIED VIDEOS (required by admin_videos.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS unified_videos (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) NOT NULL,
    description TEXT,
    video_url VARCHAR(500),
    video_platform VARCHAR(50) DEFAULT 'bunny',
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    thumbnail_url VARCHAR(500),
    duration INTEGER,
    views_count INTEGER DEFAULT 0,
    content_type VARCHAR(50) NOT NULL DEFAULT 'daily_video',
    room_id BIGINT,
    room_slug VARCHAR(100),
    video_date DATE,
    week_of DATE,
    is_featured BOOLEAN DEFAULT false,
    is_published BOOLEAN DEFAULT true,
    published_at TIMESTAMPTZ,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_unified_videos_slug ON unified_videos(slug);
CREATE INDEX IF NOT EXISTS idx_unified_videos_room ON unified_videos(room_id, content_type);
CREATE INDEX IF NOT EXISTS idx_unified_videos_date ON unified_videos(video_date DESC);

CREATE TABLE IF NOT EXISTS video_room_assignments (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100),
    is_primary BOOLEAN DEFAULT false,
    sort_order INTEGER DEFAULT 0,
    assigned_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(video_id, room_id)
);

CREATE TABLE IF NOT EXISTS video_series (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS video_chapters (
    id BIGSERIAL PRIMARY KEY,
    video_id BIGINT NOT NULL REFERENCES unified_videos(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    start_time_seconds INTEGER NOT NULL DEFAULT 0,
    end_time_seconds INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 7. ROOM CONTENT (required by room_content.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS room_sections (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS room_trade_plans (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    week_of DATE NOT NULL DEFAULT CURRENT_DATE,
    ticker VARCHAR(10) NOT NULL,
    bias VARCHAR(20) NOT NULL,
    entry VARCHAR(20),
    target1 VARCHAR(20),
    target2 VARCHAR(20),
    target3 VARCHAR(20),
    runner VARCHAR(20),
    stop VARCHAR(20),
    options_strike VARCHAR(50),
    options_exp DATE,
    notes TEXT,
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS room_alerts (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    alert_type VARCHAR(20) NOT NULL DEFAULT 'ENTRY',
    ticker VARCHAR(10) NOT NULL,
    message TEXT NOT NULL,
    price VARCHAR(20),
    is_active BOOLEAN DEFAULT true,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS room_weekly_videos (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    week_of DATE NOT NULL,
    week_title VARCHAR(255),
    video_title VARCHAR(500),
    video_id BIGINT,
    title VARCHAR(500),
    video_url TEXT,
    video_platform VARCHAR(50) DEFAULT 'bunny',
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    thumbnail_url TEXT,
    duration VARCHAR(20),
    description TEXT,
    is_current BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT true,
    is_published BOOLEAN DEFAULT true,
    published_at TIMESTAMPTZ DEFAULT NOW(),
    archived_at TIMESTAMPTZ,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(room_slug, week_of)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 8. ROOM RESOURCES (required by room_resources.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS room_resources (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    slug VARCHAR(500) NOT NULL,
    description TEXT,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL DEFAULT 'video',
    content_category VARCHAR(100),
    file_url VARCHAR(500),
    video_url VARCHAR(500),
    video_platform VARCHAR(50),
    bunny_video_guid VARCHAR(100),
    bunny_library_id BIGINT,
    thumbnail_url VARCHAR(500),
    duration_seconds INTEGER,
    file_size_bytes BIGINT,
    mime_type VARCHAR(100),
    is_featured BOOLEAN DEFAULT false,
    is_pinned BOOLEAN DEFAULT false,
    is_published BOOLEAN DEFAULT true,
    published_at TIMESTAMPTZ,
    sort_order INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    tags JSONB DEFAULT '[]'::jsonb,
    meta_title VARCHAR(255),
    meta_description TEXT,
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    UNIQUE(room_slug, slug)
);
CREATE INDEX IF NOT EXISTS idx_room_resources_room ON room_resources(room_id, resource_type);

CREATE TABLE IF NOT EXISTS room_stats_cache (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT UNIQUE,
    room_slug VARCHAR(100) NOT NULL UNIQUE,
    -- Resource stats
    total_videos INTEGER DEFAULT 0,
    total_pdfs INTEGER DEFAULT 0,
    total_documents INTEGER DEFAULT 0,
    total_images INTEGER DEFAULT 0,
    total_resources INTEGER DEFAULT 0,
    last_video_at TIMESTAMPTZ,
    last_resource_at TIMESTAMPTZ,
    -- Trading stats (used by migration 028)
    win_rate DECIMAL(5,2),
    weekly_profit VARCHAR(50),
    monthly_profit VARCHAR(50),
    active_trades INTEGER DEFAULT 0,
    closed_this_week INTEGER DEFAULT 0,
    total_trades INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    avg_win DECIMAL(12,2),
    avg_loss DECIMAL(12,2),
    profit_factor DECIMAL(6,2),
    avg_holding_days DECIMAL(6,2),
    largest_win DECIMAL(12,2),
    largest_loss DECIMAL(12,2),
    current_streak INTEGER DEFAULT 0,
    daily_pnl_30d JSONB DEFAULT '[]',
    calculated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 9. WATCHLIST (required by watchlist.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS watchlist_entries (
    id BIGSERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    room_slug VARCHAR(100) NOT NULL,
    week_of DATE NOT NULL DEFAULT CURRENT_DATE,
    ticker VARCHAR(10) NOT NULL,
    company_name VARCHAR(255),
    bias VARCHAR(20) DEFAULT 'NEUTRAL',
    entry_price VARCHAR(20),
    target_price VARCHAR(20),
    stop_price VARCHAR(20),
    options_play TEXT,
    notes TEXT,
    chart_url VARCHAR(500),
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_watchlist_room_week ON watchlist_entries(room_slug, week_of);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 10. MEMBER SEGMENTS & TAGS (required by admin_members.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS member_segments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    color VARCHAR(7) DEFAULT '#6366f1',
    icon VARCHAR(50),
    filter_criteria JSONB DEFAULT '{}'::jsonb,
    is_dynamic BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    member_count INTEGER DEFAULT 0,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS member_tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7) DEFAULT '#10b981',
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    usage_count INTEGER DEFAULT 0,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS member_filters (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    filter_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_public BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    usage_count INTEGER DEFAULT 0,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_member_segments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    segment_id BIGINT NOT NULL REFERENCES member_segments(id) ON DELETE CASCADE,
    added_by BIGINT,
    added_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, segment_id)
);

CREATE TABLE IF NOT EXISTS user_member_tags (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tag_id BIGINT NOT NULL REFERENCES member_tags(id) ON DELETE CASCADE,
    added_by BIGINT,
    added_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, tag_id)
);

CREATE TABLE IF NOT EXISTS member_notes (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    note TEXT NOT NULL,
    note_type VARCHAR(50) DEFAULT 'general',
    is_pinned BOOLEAN DEFAULT false,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS member_emails (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    subject VARCHAR(500) NOT NULL,
    body TEXT NOT NULL,
    template_id BIGINT,
    status VARCHAR(50) DEFAULT 'sent',
    sent_by BIGINT,
    sent_at TIMESTAMPTZ DEFAULT NOW(),
    opened_at TIMESTAMPTZ,
    clicked_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS user_status (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE UNIQUE,
    status VARCHAR(50) DEFAULT 'active',
    status_reason TEXT,
    banned_at TIMESTAMPTZ,
    banned_by BIGINT,
    ban_reason TEXT,
    ban_expires_at TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ,
    login_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_activity_log (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50),
    entity_id BIGINT,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_activity_user ON user_activity_log(user_id, created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 11. ORGANIZATION (required by organization.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS teams (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    color VARCHAR(7) DEFAULT '#3b82f6',
    icon VARCHAR(50),
    is_active BOOLEAN DEFAULT true,
    member_count INTEGER DEFAULT 0,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS departments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    parent_id BIGINT REFERENCES departments(id) ON DELETE SET NULL,
    color VARCHAR(7) DEFAULT '#8b5cf6',
    icon VARCHAR(50),
    is_active BOOLEAN DEFAULT true,
    member_count INTEGER DEFAULT 0,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_teams (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id BIGINT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, team_id)
);

CREATE TABLE IF NOT EXISTS user_departments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    department_id BIGINT NOT NULL REFERENCES departments(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, department_id)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 12. PAGE LAYOUTS (required by admin_page_layouts.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS page_layouts (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE,
    description TEXT,
    status VARCHAR(50) DEFAULT 'draft',
    version INTEGER DEFAULT 1,
    blocks JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    created_by BIGINT,
    updated_by BIGINT
);

CREATE TABLE IF NOT EXISTS page_layout_versions (
    id BIGSERIAL PRIMARY KEY,
    layout_id BIGINT NOT NULL REFERENCES page_layouts(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    blocks JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by BIGINT,
    UNIQUE(layout_id, version)
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 13. FORMS (required by forms.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS forms (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    fields JSONB NOT NULL DEFAULT '[]',
    settings JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    submission_count INTEGER DEFAULT 0,
    created_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS form_submissions (
    id BIGSERIAL PRIMARY KEY,
    form_id BIGINT NOT NULL REFERENCES forms(id) ON DELETE CASCADE,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    data JSONB NOT NULL,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 14. CRM (required by crm.rs)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS crm_tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    color VARCHAR(7) DEFAULT '#6366f1',
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_lists (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    subscriber_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_segments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    conditions JSONB DEFAULT '[]'::jsonb,
    is_active BOOLEAN DEFAULT true,
    member_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_campaigns (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    subject VARCHAR(500),
    content TEXT,
    status VARCHAR(50) DEFAULT 'draft',
    scheduled_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    stats JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_recurring_campaigns (
    id BIGSERIAL PRIMARY KEY,
    campaign_id BIGINT REFERENCES crm_campaigns(id) ON DELETE CASCADE,
    schedule JSONB NOT NULL,
    next_run_at TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_templates (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    subject VARCHAR(500),
    content TEXT NOT NULL,
    category VARCHAR(100),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_automations (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    trigger_type VARCHAR(100) NOT NULL,
    trigger_config JSONB DEFAULT '{}'::jsonb,
    actions JSONB DEFAULT '[]'::jsonb,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_sequences (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    steps JSONB DEFAULT '[]'::jsonb,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_webhooks (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(500) NOT NULL,
    events JSONB DEFAULT '[]'::jsonb,
    secret VARCHAR(255),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_smart_links (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    url VARCHAR(500) NOT NULL,
    short_code VARCHAR(50) UNIQUE,
    click_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS crm_companies (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    domain VARCHAR(255),
    industry VARCHAR(100),
    size VARCHAR(50),
    website VARCHAR(500),
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 15. MISC TABLES (required by various routes)
-- ═══════════════════════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS security_events (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    event_type VARCHAR(100) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    details JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_security_events_user ON security_events(user_id, created_at DESC);

CREATE TABLE IF NOT EXISTS bunny_uploads (
    id BIGSERIAL PRIMARY KEY,
    video_guid VARCHAR(100) NOT NULL UNIQUE,
    library_id BIGINT NOT NULL,
    title VARCHAR(500),
    status VARCHAR(50) DEFAULT 'uploading',
    file_size_bytes BIGINT,
    duration_seconds INTEGER,
    thumbnail_url VARCHAR(500),
    video_url VARCHAR(500),
    uploaded_by BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_coupons (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    coupon_id BIGINT NOT NULL REFERENCES coupons(id) ON DELETE CASCADE,
    used_at TIMESTAMPTZ DEFAULT NOW(),
    order_id BIGINT,
    UNIQUE(user_id, coupon_id)
);

CREATE TABLE IF NOT EXISTS settings (
    id BIGSERIAL PRIMARY KEY,
    key VARCHAR(255) NOT NULL UNIQUE,
    value JSONB,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 16. ADD MISSING COLUMNS TO EXISTING TABLES
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add columns to indicators table
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS tagline VARCHAR(500);
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS price_cents INTEGER DEFAULT 0;
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS is_free BOOLEAN DEFAULT false;
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS sale_price_cents INTEGER;
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS logo_url VARCHAR(500);
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS card_image_url VARCHAR(500);
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS status VARCHAR(50) DEFAULT 'draft';
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT false;
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS view_count INTEGER DEFAULT 0;
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS download_count INTEGER DEFAULT 0;
ALTER TABLE indicators ADD COLUMN IF NOT EXISTS supported_platforms JSONB DEFAULT '[]'::jsonb;

-- Add columns to locales table
ALTER TABLE locales ADD COLUMN IF NOT EXISTS flag_emoji VARCHAR(10);
ALTER TABLE locales ADD COLUMN IF NOT EXISTS direction VARCHAR(3) DEFAULT 'ltr';
ALTER TABLE locales ADD COLUMN IF NOT EXISTS date_format VARCHAR(50) DEFAULT 'YYYY-MM-DD';
ALTER TABLE locales ADD COLUMN IF NOT EXISTS time_format VARCHAR(50) DEFAULT 'HH:mm:ss';
ALTER TABLE locales ADD COLUMN IF NOT EXISTS currency_code VARCHAR(3) DEFAULT 'USD';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END OF CONSOLIDATED SCHEMA
-- All tables verified against api/src/routes/*.rs
-- Apple ICT 7 Principal Engineer Grade
-- ═══════════════════════════════════════════════════════════════════════════════════
