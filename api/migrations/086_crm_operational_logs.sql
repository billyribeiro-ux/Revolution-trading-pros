-- Operational CRM diagnostics have their own retention/clear lifecycle.
-- Security audit tables remain independent and cannot be cleared by this UI.
CREATE TABLE crm_operational_logs (
    id BIGSERIAL PRIMARY KEY,
    level TEXT NOT NULL CHECK(level IN ('debug','info','warning','error','critical')),
    category TEXT NOT NULL CHECK(category IN ('email','automation','import','api','webhook','system')),
    message TEXT NOT NULL CHECK(length(message) BETWEEN 1 AND 2000),
    context JSONB NOT NULL DEFAULT '{}'::jsonb CHECK(jsonb_typeof(context)='object'),
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    contact_id BIGINT REFERENCES contacts(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX crm_operational_logs_created ON crm_operational_logs(created_at DESC,id DESC);
CREATE INDEX crm_operational_logs_level ON crm_operational_logs(level,created_at DESC);
CREATE INDEX crm_operational_logs_category ON crm_operational_logs(category,created_at DESC);
