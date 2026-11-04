-- Keep an explicit revocation after hard deletion. Otherwise a configured
-- environment key could silently reactivate a connection the admin removed.
CREATE TABLE service_connection_revocations (
    service_key TEXT NOT NULL CHECK (length(service_key) BETWEEN 1 AND 100),
    environment TEXT NOT NULL CHECK (environment IN ('production', 'sandbox')),
    revoked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    reason TEXT NOT NULL CHECK (reason IN ('disconnected', 'deleted')),
    PRIMARY KEY (service_key, environment)
);

COMMENT ON TABLE service_connection_revocations IS
    'Authoritative credential revocation; only an explicit verified reconnect clears the selected environment.';
