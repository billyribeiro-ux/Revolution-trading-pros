CREATE TABLE media_upload_tickets (
    file_key TEXT PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_type TEXT NOT NULL,
    expected_size BIGINT CHECK(expected_size BETWEEN 1 AND 52428800),
    expires_at TIMESTAMPTZ NOT NULL,
    confirmed_media_id BIGINT REFERENCES media(id) ON DELETE SET NULL,
    cleanup_pending BOOLEAN NOT NULL DEFAULT FALSE,
    cleanup_error TEXT,
    cleanup_after TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX media_upload_ticket_expiry ON media_upload_tickets(expires_at);
