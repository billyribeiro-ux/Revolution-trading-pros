-- Preserve existing counts, including unknown legacy NULLs; widen without backfill.
ALTER TABLE member_filters
    ALTER COLUMN usage_count TYPE BIGINT,
    ALTER COLUMN usage_count SET DEFAULT 0,
    ADD CONSTRAINT member_filter_usage_nonnegative CHECK(usage_count>=0),
    ADD COLUMN last_applied_at TIMESTAMPTZ;
