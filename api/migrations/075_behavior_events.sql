CREATE TABLE behavior_events (
    id BIGSERIAL PRIMARY KEY,
    event_hash TEXT NOT NULL UNIQUE,
    session_id VARCHAR(128) NOT NULL,
    visitor_id VARCHAR(128) NOT NULL,
    event_type VARCHAR(64) NOT NULL,
    page_url VARCHAR(500) NOT NULL,
    element_selector VARCHAR(500),
    event_value DOUBLE PRECISION,
    coordinates JSONB,
    happened_at TIMESTAMPTZ NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX behavior_events_time_idx ON behavior_events (happened_at DESC);
CREATE INDEX behavior_events_session_idx ON behavior_events (session_id, happened_at);
CREATE INDEX behavior_events_page_idx ON behavior_events (page_url, event_type, happened_at DESC);
