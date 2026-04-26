-- Migration 044: Service Connections + Admin Audit Log
-- ═══════════════════════════════════════════════════════════════════════════
-- Backs the /admin/connections UI (api/src/routes/connections.rs).
-- Stores third-party service credentials (Stripe, Bunny.net, SendGrid, etc.)
-- so the runtime can resolve them at request time without a redeploy.
--
-- The CredentialResolver service (api/src/services/credential_resolver.rs)
-- reads from `service_connections` first, falling back to env vars. This is
-- the foundation of the PE7 "no Stripe-dashboard login required" contract.
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS service_connections (
    id BIGSERIAL PRIMARY KEY,
    service_key TEXT NOT NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    health_score INTEGER NOT NULL DEFAULT 0,
    health_status TEXT,
    -- "production" | "sandbox" — same row can hold both via uq below
    environment TEXT,
    -- Base64-wrapped JSON map of credential fields. Encryption is the
    -- responsibility of the connections route (today base64; AES-GCM later).
    credentials_encrypted TEXT,
    settings JSONB,
    webhook_url TEXT,
    -- Webhook signing secret (e.g. Stripe whsec_…). Encrypted at rest in
    -- the same way `credentials_encrypted` is.
    webhook_secret TEXT,
    api_calls_today INTEGER NOT NULL DEFAULT 0,
    api_calls_total BIGINT NOT NULL DEFAULT 0,
    last_error TEXT,
    last_verified_at TIMESTAMPTZ,
    connected_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT service_connections_status_check
        CHECK (status IN ('connected', 'pending', 'disconnected', 'error'))
);

-- A given (service, environment) pair has at most one connection row.
-- NULL environment is treated as a separate slot from named environments.
CREATE UNIQUE INDEX IF NOT EXISTS uq_service_connections_key_env
    ON service_connections (service_key, COALESCE(environment, ''));

CREATE INDEX IF NOT EXISTS idx_service_connections_category
    ON service_connections (category);

CREATE INDEX IF NOT EXISTS idx_service_connections_status
    ON service_connections (status);

-- Per-connection webhooks (outgoing). The connections route writes here when
-- an admin registers a webhook receiver against a connected service.
CREATE TABLE IF NOT EXISTS integration_webhooks (
    id BIGSERIAL PRIMARY KEY,
    connection_id BIGINT NOT NULL REFERENCES service_connections(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    secret TEXT,
    events JSONB,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_triggered_at TIMESTAMPTZ,
    last_status_code INTEGER,
    failure_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_integration_webhooks_connection
    ON integration_webhooks (connection_id);

-- Admin audit log used by connections.rs (and other admin mutation paths
-- that may want to share it). Append-only.
CREATE TABLE IF NOT EXISTS admin_audit_logs (
    id BIGSERIAL PRIMARY KEY,
    admin_id BIGINT NOT NULL,
    admin_email TEXT,
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id BIGINT,
    old_value TEXT,
    new_value TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_admin_audit_logs_entity
    ON admin_audit_logs (entity_type, entity_id);

CREATE INDEX IF NOT EXISTS idx_admin_audit_logs_admin
    ON admin_audit_logs (admin_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_admin_audit_logs_created
    ON admin_audit_logs (created_at DESC);

COMMENT ON TABLE service_connections IS
    'Third-party service credentials, env-aware, resolved at request-time by CredentialResolver.';
COMMENT ON COLUMN service_connections.environment IS
    'Optional environment scope (production|sandbox). One row per (service_key, environment).';
COMMENT ON COLUMN service_connections.webhook_secret IS
    'Webhook signing secret for inbound events from this provider (e.g. Stripe whsec_).';
COMMENT ON TABLE admin_audit_logs IS
    'Append-only audit trail for admin-initiated mutations (connections, plans, etc).';
