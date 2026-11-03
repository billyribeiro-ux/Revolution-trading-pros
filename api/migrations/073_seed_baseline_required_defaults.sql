-- 073_seed_baseline_required_defaults.sql
-- Forward-only and idempotent.
--
-- Databases bootstrapped from migrations/schema.sql (a --schema-only dump; see
-- migrations/README.md) never ran the INSERTs inside the historical migrations, so rows
-- the application cannot function without were missing:
--
--   * cms_site_settings singleton (seeded by 023). GET /api/admin/cms-v2/settings runs
--     `fetch_one` on this row; with the table empty it returned HTTP 500 and the admin
--     Settings page failed to load.
--   * the default CRM pipeline and its stages (seeded by 052), which the deals board needs.
--
-- Only structural defaults are restored here. Content-like seed rows (rooms, popups,
-- categories, presets, ...) are deliberately NOT re-created: re-inserting them could
-- resurrect rows an operator deleted on a long-lived database. Every statement is a
-- no-op when the data already exists.

INSERT INTO cms_site_settings (id)
VALUES ('00000000-0000-0000-0000-000000000001'::uuid)
ON CONFLICT (id) DO NOTHING;

-- Default pipeline: only when the table is completely empty, so a pipeline an operator
-- renamed or deleted is never recreated.
INSERT INTO crm_pipelines (name, description, is_default, is_active, color, position)
SELECT 'Sales Pipeline', 'Default sales pipeline for tracking deals', true, true, '#6366f1', 0
WHERE NOT EXISTS (SELECT 1 FROM crm_pipelines);

-- Default stages: only for a default pipeline that has none.
INSERT INTO crm_pipeline_stages (pipeline_id, name, position, probability, color, is_closed_won, is_closed_lost)
SELECT p.id, s.name, s.position, s.probability, s.color, s.is_closed_won, s.is_closed_lost
FROM (SELECT id FROM crm_pipelines WHERE is_default = true ORDER BY id LIMIT 1) AS p
CROSS JOIN (VALUES
    ('Lead', 0, 10, '#94a3b8', false, false),
    ('Qualified', 1, 25, '#22d3ee', false, false),
    ('Proposal', 2, 50, '#a78bfa', false, false),
    ('Negotiation', 3, 75, '#fbbf24', false, false),
    ('Closed Won', 4, 100, '#22c55e', true, false),
    ('Closed Lost', 5, 0, '#ef4444', false, true)
) AS s(name, position, probability, color, is_closed_won, is_closed_lost)
WHERE NOT EXISTS (SELECT 1 FROM crm_pipeline_stages WHERE pipeline_id = p.id);
