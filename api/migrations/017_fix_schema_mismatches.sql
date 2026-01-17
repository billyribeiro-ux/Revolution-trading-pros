-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 017: Fix Schema Mismatches
-- Apple ICT 7 Principal Engineer - January 2026
-- 
-- FIXES:
-- 1. Add 'attempts' column to webhook_deliveries (code expects 'attempts', has 'attempt_count')
-- 2. Ensure jobs table has all required columns
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. FIX WEBHOOK_DELIVERIES TABLE
-- Add 'attempts' column that queue worker expects
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add attempts column if it doesn't exist (code expects this)
ALTER TABLE webhook_deliveries ADD COLUMN IF NOT EXISTS attempts INTEGER DEFAULT 0;

-- Copy data from attempt_count to attempts if both exist
UPDATE webhook_deliveries SET attempts = attempt_count WHERE attempts = 0 AND attempt_count > 0;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. ENSURE JOBS TABLE HAS ALL REQUIRED COLUMNS
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Create jobs table if it doesn't exist (should exist from 001, but ensure it does)
CREATE TABLE IF NOT EXISTS jobs (
    id BIGSERIAL PRIMARY KEY,
    queue VARCHAR(255) NOT NULL DEFAULT 'default',
    job_type VARCHAR(255) NOT NULL,
    payload JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    attempts INTEGER DEFAULT 0,
    max_attempts INTEGER DEFAULT 3,
    error TEXT,
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reserved_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add any missing columns to jobs table
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS available_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS queue VARCHAR(255) NOT NULL DEFAULT 'default';
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS job_type VARCHAR(255);
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS payload JSONB;
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS status VARCHAR(50) NOT NULL DEFAULT 'pending';
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS attempts INTEGER DEFAULT 0;
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS max_attempts INTEGER DEFAULT 3;
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS reserved_at TIMESTAMPTZ;
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS started_at TIMESTAMPTZ;
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS completed_at TIMESTAMPTZ;

-- Create indexes if they don't exist
CREATE INDEX IF NOT EXISTS idx_jobs_queue ON jobs(queue, available_at);
CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
CREATE INDEX IF NOT EXISTS idx_jobs_pending ON jobs(status, available_at) WHERE status = 'pending';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. ENSURE ANALYTICS_EVENTS TABLE EXISTS
-- Required for performance tracking endpoint
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS analytics_events (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    session_id VARCHAR(255),
    event_type VARCHAR(100) NOT NULL,
    event_name VARCHAR(255),
    page_url TEXT,
    referrer TEXT,
    user_agent TEXT,
    ip_address VARCHAR(45),
    properties JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_analytics_user ON analytics_events(user_id);
CREATE INDEX IF NOT EXISTS idx_analytics_type ON analytics_events(event_type);
CREATE INDEX IF NOT EXISTS idx_analytics_created ON analytics_events(created_at);

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
