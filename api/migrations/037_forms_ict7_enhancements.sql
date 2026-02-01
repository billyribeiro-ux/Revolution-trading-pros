-- ═══════════════════════════════════════════════════════════════════════════════════
-- ICT 7 Forms System Enhancements
-- Migration: 037_forms_ict7_enhancements.sql
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Add status column to form_submissions for complete submission management
ALTER TABLE form_submissions
ADD COLUMN IF NOT EXISTS status VARCHAR(20) DEFAULT 'unread';

-- Add is_published column to forms (separate from is_active)
ALTER TABLE forms
ADD COLUMN IF NOT EXISTS is_published BOOLEAN DEFAULT false;

-- Create index for efficient status filtering
CREATE INDEX IF NOT EXISTS idx_form_submissions_status ON form_submissions(status);

-- Create index for form analytics queries
CREATE INDEX IF NOT EXISTS idx_form_submissions_form_created ON form_submissions(form_id, created_at DESC);

-- Create index for published forms lookup
CREATE INDEX IF NOT EXISTS idx_forms_published ON forms(is_published) WHERE is_published = true;

-- Update existing submissions to have a default status
UPDATE form_submissions SET status = 'unread' WHERE status IS NULL;

-- For existing forms, set is_published based on is_active
UPDATE forms SET is_published = is_active WHERE is_published IS NULL OR is_published = false;

-- Add comment to explain status values
COMMENT ON COLUMN form_submissions.status IS 'Submission status: unread, read, starred, archived, spam, complete, partial';
