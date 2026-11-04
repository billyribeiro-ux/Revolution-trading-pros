-- The legacy newsletter bulk endpoint shares the durable campaign outbox.
-- A campaign row lock reserves every provider attempt's pacing slot, including
-- retries, so concurrent workers cannot exceed the requested per-minute rate.
ALTER TABLE crm_campaigns
    ADD COLUMN delivery_rate_limit INTEGER CHECK (delivery_rate_limit BETWEEN 1 AND 500),
    ADD COLUMN next_delivery_at TIMESTAMPTZ;
