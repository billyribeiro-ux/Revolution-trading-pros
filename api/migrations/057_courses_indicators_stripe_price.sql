-- Migration 057: Add Stripe Price / Product columns to courses and indicators.
-- Enables the admin price-change feature (Task 3.3) for one-time products.
-- For one-time purchases only "new_only" applies — these are not subscriptions.

ALTER TABLE courses
    ADD COLUMN IF NOT EXISTS stripe_price_id   TEXT,
    ADD COLUMN IF NOT EXISTS stripe_product_id TEXT;

ALTER TABLE indicators
    ADD COLUMN IF NOT EXISTS stripe_price_id   TEXT,
    ADD COLUMN IF NOT EXISTS stripe_product_id TEXT;

COMMENT ON COLUMN courses.stripe_price_id   IS 'Current active Stripe Price ID for this course (one-time purchase)';
COMMENT ON COLUMN courses.stripe_product_id IS 'Stripe Product ID for this course';
COMMENT ON COLUMN indicators.stripe_price_id   IS 'Current active Stripe Price ID for this indicator (one-time purchase)';
COMMENT ON COLUMN indicators.stripe_product_id IS 'Stripe Product ID for this indicator';
