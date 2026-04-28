-- Migration: 040_subscription_notifications_ict7.sql
-- ICT 7 Fix: Add notification tracking columns to user_memberships
-- Enables proper scheduling of renewal reminders and trial ending notifications

-- Add notification tracking columns to user_memberships
ALTER TABLE user_memberships
    ADD COLUMN IF NOT EXISTS renewal_reminder_sent_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS trial_ending_reminder_sent_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS payment_failure_count INTEGER DEFAULT 0,
    ADD COLUMN IF NOT EXISTS last_payment_failure TIMESTAMPTZ;

-- Add indexes for notification scheduling queries
CREATE INDEX IF NOT EXISTS idx_user_memberships_renewal_reminder
ON user_memberships (current_period_end, renewal_reminder_sent_at)
WHERE status = 'active' AND cancel_at_period_end = false;

CREATE INDEX IF NOT EXISTS idx_user_memberships_trial_ending
ON user_memberships (trial_ends_at, trial_ending_reminder_sent_at)
WHERE status = 'trial';

CREATE INDEX IF NOT EXISTS idx_user_memberships_grace_period
ON user_memberships (grace_period_end)
WHERE status = 'past_due';

-- Comment on columns for documentation
COMMENT ON COLUMN user_memberships.renewal_reminder_sent_at IS 'Timestamp when renewal reminder email was sent (prevents duplicate sends)';
COMMENT ON COLUMN user_memberships.trial_ending_reminder_sent_at IS 'Timestamp when trial ending notification was sent (prevents duplicate sends)';
COMMENT ON COLUMN user_memberships.payment_failure_count IS 'Number of consecutive payment failures for this subscription';
COMMENT ON COLUMN user_memberships.last_payment_failure IS 'Timestamp of the most recent payment failure';
