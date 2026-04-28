-- Trial support for membership plans
-- trial_period_days: optional trial length; only 7, 14, or 30 days allowed
-- trial_requires_payment_method: if false, card is collected only if trial converts
ALTER TABLE membership_plans
    ADD COLUMN IF NOT EXISTS trial_period_days INTEGER
        CHECK (trial_period_days IS NULL OR trial_period_days IN (7, 14, 30)),
    ADD COLUMN IF NOT EXISTS trial_requires_payment_method BOOLEAN NOT NULL DEFAULT true;
