-- 072_audit_followup_schema_alignment.sql
-- Forward-only, idempotent schema alignment for admin backend follow-up audit fixes.

CREATE TABLE IF NOT EXISTS public.user_course_enrollments_enhanced (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    course_id BIGINT NOT NULL,
    enrolled_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    completed_at TIMESTAMP WITHOUT TIME ZONE,
    progress_percent INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    last_accessed_at TIMESTAMP WITHOUT TIME ZONE,
    certificate_issued BOOLEAN NOT NULL DEFAULT false,
    certificate_url TEXT,
    UNIQUE (user_id, course_id)
);

CREATE TABLE IF NOT EXISTS public.organization_profile (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL DEFAULT 'Revolution Trading Pros',
    slug VARCHAR(255) NOT NULL DEFAULT 'revolution-trading-pros',
    description TEXT,
    logo_url TEXT,
    favicon_url TEXT,
    primary_color VARCHAR(20),
    secondary_color VARCHAR(20),
    contact_email VARCHAR(255),
    contact_phone VARCHAR(50),
    address TEXT,
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100),
    postal_code VARCHAR(50),
    website_url TEXT,
    social_links JSONB,
    business_hours JSONB,
    timezone VARCHAR(100) DEFAULT 'America/New_York',
    currency VARCHAR(10) DEFAULT 'USD',
    tax_id VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE IF NOT EXISTS public.department_members (
    id BIGSERIAL PRIMARY KEY,
    department_id BIGINT NOT NULL REFERENCES public.departments(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    created_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE (department_id, user_id)
);

CREATE TABLE IF NOT EXISTS public.team_members (
    id BIGSERIAL PRIMARY KEY,
    team_id BIGINT NOT NULL REFERENCES public.teams(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES public.users(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member',
    created_at TIMESTAMPTZ DEFAULT now(),
    UNIQUE (team_id, user_id)
);

CREATE TABLE IF NOT EXISTS public.cms_asset_usage (
    id UUID PRIMARY KEY,
    asset_id UUID NOT NULL REFERENCES public.cms_assets(id) ON DELETE CASCADE,
    content_type VARCHAR(100) NOT NULL DEFAULT '',
    content_id UUID NOT NULL,
    field_name VARCHAR(255) NOT NULL DEFAULT 'unknown',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (asset_id, content_id, field_name)
);

ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS level VARCHAR(50);
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS duration_minutes INTEGER;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS price_cents BIGINT;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS is_free BOOLEAN DEFAULT false;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS instructor_name VARCHAR(255);
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS instructor_bio TEXT;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS instructor_avatar_url TEXT;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS what_you_learn JSONB;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS requirements JSONB;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS target_audience JSONB;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS enrollment_count INTEGER DEFAULT 0;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS avg_rating DOUBLE PRECISION;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS review_count INTEGER DEFAULT 0;
ALTER TABLE public.courses ADD COLUMN IF NOT EXISTS meta_keywords TEXT[];

ALTER TABLE public.course_modules_v2 ADD COLUMN IF NOT EXISTS duration_minutes INTEGER DEFAULT 0;
ALTER TABLE public.course_modules_v2 ADD COLUMN IF NOT EXISTS lesson_count INTEGER DEFAULT 0;
ALTER TABLE public.course_modules_v2 ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT false;
ALTER TABLE public.course_modules_v2 ADD COLUMN IF NOT EXISTS is_preview BOOLEAN DEFAULT false;
ALTER TABLE public.course_modules_v2 ADD COLUMN IF NOT EXISTS drip_days INTEGER;

ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS description_html TEXT;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS bunny_video_guid VARCHAR(255);
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS bunny_library_id BIGINT;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS thumbnail_url TEXT;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS duration_minutes INTEGER;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS position INTEGER DEFAULT 0;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS sort_order INTEGER DEFAULT 0;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS is_free BOOLEAN DEFAULT false;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT false;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS drip_days INTEGER;
ALTER TABLE public.lessons ADD COLUMN IF NOT EXISTS attachments JSONB;

ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS status VARCHAR(50) DEFAULT 'active';
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS progress_percent INTEGER DEFAULT 0;
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS completed_lesson_ids JSONB;
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS current_lesson_id UUID;
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS last_accessed_at TIMESTAMPTZ;
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS certificate_issued BOOLEAN DEFAULT false;
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS certificate_url TEXT;
ALTER TABLE public.user_course_enrollments ADD COLUMN IF NOT EXISTS certificate_issued_at TIMESTAMPTZ;

ALTER TABLE public.unified_videos ADD COLUMN IF NOT EXISTS content_type VARCHAR(50) DEFAULT 'other';
ALTER TABLE public.unified_videos ADD COLUMN IF NOT EXISTS video_date DATE DEFAULT CURRENT_DATE;
ALTER TABLE public.unified_videos ADD COLUMN IF NOT EXISTS is_featured BOOLEAN DEFAULT false;

ALTER TABLE public.member_filters ADD COLUMN IF NOT EXISTS is_default BOOLEAN NOT NULL DEFAULT false;

ALTER TABLE public.newsletter_subscribers ADD COLUMN IF NOT EXISTS gdpr_consent BOOLEAN DEFAULT false;
ALTER TABLE public.newsletter_subscribers ADD COLUMN IF NOT EXISTS consent_ip VARCHAR(100);
ALTER TABLE public.newsletter_subscribers ADD COLUMN IF NOT EXISTS consent_source VARCHAR(100);

ALTER TABLE public.categories ADD COLUMN IF NOT EXISTS is_visible BOOLEAN DEFAULT true;
ALTER TABLE public.categories ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT true;

ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS original_name VARCHAR(255);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS file_path TEXT;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS file_type VARCHAR(50);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS mime_type VARCHAR(100);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS checksum_sha256 VARCHAR(128);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS platform_version VARCHAR(100);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS storage_provider VARCHAR(50);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS storage_bucket VARCHAR(255);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS storage_key TEXT;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS cdn_url TEXT;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS is_current_version BOOLEAN DEFAULT true;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS changelog TEXT;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS display_name VARCHAR(255);
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS display_order INTEGER DEFAULT 0;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT true;
ALTER TABLE public.indicator_files ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT now();

ALTER TABLE public.indicator_videos ADD COLUMN IF NOT EXISTS embed_url TEXT;
ALTER TABLE public.indicator_videos ADD COLUMN IF NOT EXISTS play_url TEXT;
ALTER TABLE public.indicator_videos ADD COLUMN IF NOT EXISTS display_order INTEGER DEFAULT 0;
ALTER TABLE public.indicator_videos ADD COLUMN IF NOT EXISTS is_featured BOOLEAN DEFAULT false;
