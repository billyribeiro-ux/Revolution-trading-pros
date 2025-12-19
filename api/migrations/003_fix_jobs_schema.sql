-- Fix jobs table schema for Rust backend
-- The Laravel jobs table has a different schema than what the Rust backend expects
-- This migration renames the Laravel table and creates the correct one

-- Step 1: Rename existing Laravel jobs table if it exists and has the wrong schema
DO $$
BEGIN
    -- Check if jobs table exists but doesn't have the run_at column (Laravel schema)
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_name = 'jobs' AND table_schema = 'public'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'jobs' AND column_name = 'run_at' AND table_schema = 'public'
    ) THEN
        -- Rename the Laravel jobs table to preserve any existing jobs
        ALTER TABLE jobs RENAME TO laravel_jobs_backup;
        RAISE NOTICE 'Renamed existing Laravel jobs table to laravel_jobs_backup';
    END IF;
END $$;

-- Step 2: Drop old indexes if they exist on the renamed table
DROP INDEX IF EXISTS idx_jobs_status;
DROP INDEX IF EXISTS idx_jobs_queue;
DROP INDEX IF EXISTS idx_jobs_run_at;
DROP INDEX IF EXISTS idx_jobs_pending;

-- Step 3: Create the correct jobs table for Rust backend
CREATE TABLE IF NOT EXISTS jobs (
    id UUID PRIMARY KEY,
    queue VARCHAR(100) NOT NULL DEFAULT 'default',
    job_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}',
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    error TEXT,
    run_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Step 4: Create indexes for the jobs table
CREATE INDEX IF NOT EXISTS idx_jobs_status ON jobs(status);
CREATE INDEX IF NOT EXISTS idx_jobs_queue ON jobs(queue);
CREATE INDEX IF NOT EXISTS idx_jobs_run_at ON jobs(run_at);
CREATE INDEX IF NOT EXISTS idx_jobs_pending ON jobs(status, run_at) WHERE status = 'pending';

-- Step 5: Verify the run_at column exists (will fail if something went wrong)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'jobs' AND column_name = 'run_at' AND table_schema = 'public'
    ) THEN
        RAISE EXCEPTION 'Migration failed: run_at column does not exist in jobs table';
    END IF;
END $$;
