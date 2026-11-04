-- Persist the design and ownership fields exposed by the template library.
ALTER TABLE crm_templates
    ADD COLUMN design_template TEXT NOT NULL DEFAULT 'raw'
        CHECK (design_template IN ('raw', 'visual')),
    ADD COLUMN template_config JSONB,
    ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN created_by BIGINT REFERENCES users(id) ON DELETE SET NULL;
