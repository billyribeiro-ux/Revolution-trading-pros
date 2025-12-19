-- Add missing run_at column to jobs table
ALTER TABLE jobs ADD COLUMN IF NOT EXISTS run_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Create index for the run_at column
CREATE INDEX IF NOT EXISTS idx_jobs_run_at ON jobs(run_at);
CREATE INDEX IF NOT EXISTS idx_jobs_pending ON jobs(status, run_at) WHERE status = 'pending';
