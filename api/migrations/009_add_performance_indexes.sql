-- ICT 11+ Performance Optimization: Database Indexes
-- These indexes dramatically improve query performance for common operations

-- ═══════════════════════════════════════════════════════════════════════════
-- Users Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- Email lookup (login, password reset, user search)
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Email verification status filtering
CREATE INDEX IF NOT EXISTS idx_users_email_verified ON users(email_verified_at) WHERE email_verified_at IS NOT NULL;

-- Role-based filtering (admin panels, role checks)
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role) WHERE role IS NOT NULL;

-- Created at for recent users, sorting
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════
-- Products Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- Slug lookup (product pages, SEO)
CREATE INDEX IF NOT EXISTS idx_products_slug ON products(slug);

-- Type filtering (course, indicator, bundle pages)
CREATE INDEX IF NOT EXISTS idx_products_type ON products(type);

-- Active products only (public listings)
CREATE INDEX IF NOT EXISTS idx_products_active ON products(is_active) WHERE is_active = true;

-- Combined index for common query pattern
CREATE INDEX IF NOT EXISTS idx_products_active_type ON products(is_active, type) WHERE is_active = true;

-- Created at for recent products, sorting
CREATE INDEX IF NOT EXISTS idx_products_created_at ON products(created_at DESC);

-- Search optimization (name contains)
CREATE INDEX IF NOT EXISTS idx_products_name_trgm ON products USING gin(name gin_trgm_ops);

-- ═══════════════════════════════════════════════════════════════════════════
-- User Memberships Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- User's memberships lookup
CREATE INDEX IF NOT EXISTS idx_user_memberships_user_id ON user_memberships(user_id);

-- Active memberships filtering
CREATE INDEX IF NOT EXISTS idx_user_memberships_status ON user_memberships(status);

-- Combined user + status for dashboard queries
CREATE INDEX IF NOT EXISTS idx_user_memberships_user_status ON user_memberships(user_id, status);

-- Expiration tracking for renewal notifications
CREATE INDEX IF NOT EXISTS idx_user_memberships_expires_at ON user_memberships(expires_at) WHERE expires_at IS NOT NULL;

-- ═══════════════════════════════════════════════════════════════════════════
-- Courses Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- Slug lookup (course pages)
CREATE INDEX IF NOT EXISTS idx_courses_slug ON courses(slug);

-- Published courses only
CREATE INDEX IF NOT EXISTS idx_courses_published ON courses(is_published) WHERE is_published = true;

-- Instructor's courses
CREATE INDEX IF NOT EXISTS idx_courses_instructor ON courses(instructor_id);

-- ═══════════════════════════════════════════════════════════════════════════
-- Posts Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- Slug lookup (blog posts)
CREATE INDEX IF NOT EXISTS idx_posts_slug ON posts(slug);

-- Published posts for public blog
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status) WHERE status = 'published';

-- Author's posts
CREATE INDEX IF NOT EXISTS idx_posts_author ON posts(author_id);

-- Published date for sorting
CREATE INDEX IF NOT EXISTS idx_posts_published_at ON posts(published_at DESC) WHERE published_at IS NOT NULL;

-- ═══════════════════════════════════════════════════════════════════════════
-- Orders Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- User's order history
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);

-- Order status filtering
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);

-- Stripe payment tracking
CREATE INDEX IF NOT EXISTS idx_orders_stripe_session ON orders(stripe_session_id) WHERE stripe_session_id IS NOT NULL;

-- Order date for reports
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at DESC);

-- ═══════════════════════════════════════════════════════════════════════════
-- Password Resets Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- Token lookup (password reset flow)
CREATE INDEX IF NOT EXISTS idx_password_resets_token ON password_resets(token);

-- Email lookup (finding user's reset requests)
CREATE INDEX IF NOT EXISTS idx_password_resets_email ON password_resets(email);

-- Expiration cleanup
CREATE INDEX IF NOT EXISTS idx_password_resets_expires ON password_resets(expires_at);

-- ═══════════════════════════════════════════════════════════════════════════
-- Email Verification Tokens Table Indexes
-- ═══════════════════════════════════════════════════════════════════════════

-- Token lookup (verification flow)
CREATE INDEX IF NOT EXISTS idx_email_verification_token ON email_verification_tokens(token);

-- User's tokens (cleanup)
CREATE INDEX IF NOT EXISTS idx_email_verification_user ON email_verification_tokens(user_id);

-- ═══════════════════════════════════════════════════════════════════════════
-- Jobs Table Indexes (Background Processing)
-- ═══════════════════════════════════════════════════════════════════════════

-- Pending jobs pickup
CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status) WHERE status = 'pending';

-- Job type filtering
CREATE INDEX IF NOT EXISTS idx_jobs_type ON jobs(job_type);

-- Scheduled job execution
CREATE INDEX IF NOT EXISTS idx_jobs_scheduled ON jobs(scheduled_for) WHERE scheduled_for IS NOT NULL;

-- ═══════════════════════════════════════════════════════════════════════════
-- Enable pg_trgm extension for text search (if not already enabled)
-- ═══════════════════════════════════════════════════════════════════════════

-- Note: This may require superuser privileges on some PostgreSQL installations
-- If it fails, the search will still work, just without trigram optimization
DO $$
BEGIN
    CREATE EXTENSION IF NOT EXISTS pg_trgm;
EXCEPTION
    WHEN insufficient_privilege THEN
        RAISE NOTICE 'pg_trgm extension requires superuser - skipping';
END
$$;
