-- Fix-up migration: add columns that earlier migrations referenced in indexes
-- or that handler code depends on but were never actually added.
-- Discovered during Task 7 E2E verification — code in subscriptions.rs and
-- payments.rs queries these columns; without them, /my/subscriptions GET,
-- invoice.payment_failed handler, and charge.refunded handler all fail
-- with "column does not exist".
ALTER TABLE user_memberships
    ADD COLUMN IF NOT EXISTS trial_ends_at         TIMESTAMP,
    ADD COLUMN IF NOT EXISTS grace_period_end      TIMESTAMP,
    ADD COLUMN IF NOT EXISTS payment_failure_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS last_payment_failure  TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS renewal_reminder_sent_at      TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS trial_ending_reminder_sent_at TIMESTAMPTZ;

ALTER TABLE orders
    ADD COLUMN IF NOT EXISTS refund_status TEXT,
    ADD COLUMN IF NOT EXISTS refund_amount NUMERIC(10,2) NOT NULL DEFAULT 0;

ALTER TABLE products
    ADD COLUMN IF NOT EXISTS course_id    UUID,
    ADD COLUMN IF NOT EXISTS indicator_id BIGINT;

-- user_course_enrollments.course_id was BIGINT but courses.id is UUID
-- (the FK pointed to a stale courses_enhanced table that no longer matches).
-- Re-point FK to current courses table.
ALTER TABLE user_course_enrollments DROP CONSTRAINT IF EXISTS user_course_enrollments_course_id_fkey;
DO $$ BEGIN
  IF (SELECT data_type FROM information_schema.columns WHERE table_name='user_course_enrollments' AND column_name='course_id') = 'bigint' THEN
    ALTER TABLE user_course_enrollments ALTER COLUMN course_id TYPE UUID USING NULL;
  END IF;
END $$;
ALTER TABLE user_course_enrollments
    ADD CONSTRAINT user_course_enrollments_course_id_fkey
        FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE;
