-- ===============================================================================================
-- CMS DATASOURCES SYSTEM - Reusable Option Lists
-- Apple ICT 7+ Principal Engineer Grade
-- Revolution Trading Pros - February 2026
--
-- This migration adds a comprehensive datasource system for reusable option lists:
-- - Datasources (containers for key-value entries)
-- - Datasource entries (key-value pairs with dimensions for translations)
-- - CSV import/export support
-- - Dimension support for localization
-- - Sort order for drag-to-reorder
--
-- Example datasources: Countries, US States, Trading Symbols, Color Palettes, Grid Options
-- ===============================================================================================

-- -----------------------------------------------------------------------------------------------
-- 1. CREATE CMS_DATASOURCES TABLE
-- -----------------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS cms_datasources (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Identification
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,

    -- Organization
    icon VARCHAR(50),
    color VARCHAR(20),

    -- Entry count (denormalized for performance)
    entry_count INTEGER NOT NULL DEFAULT 0,

    -- Flags
    is_system BOOLEAN NOT NULL DEFAULT false,
    is_locked BOOLEAN NOT NULL DEFAULT false,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT cms_datasources_name_check CHECK (LENGTH(TRIM(name)) > 0),
    CONSTRAINT cms_datasources_slug_check CHECK (slug ~ '^[a-z0-9-]+$')
);

COMMENT ON TABLE cms_datasources IS 'Datasources for reusable option lists';
COMMENT ON COLUMN cms_datasources.slug IS 'URL-friendly unique identifier for API access';
COMMENT ON COLUMN cms_datasources.is_system IS 'System datasources cannot be deleted';
COMMENT ON COLUMN cms_datasources.is_locked IS 'Locked datasources cannot be edited by non-admins';

-- -----------------------------------------------------------------------------------------------
-- 2. CREATE CMS_DATASOURCE_ENTRIES TABLE
-- -----------------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS cms_datasource_entries (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Parent datasource
    datasource_id UUID NOT NULL REFERENCES cms_datasources(id) ON DELETE CASCADE,

    -- Entry data
    name VARCHAR(255) NOT NULL,
    value VARCHAR(500) NOT NULL,

    -- Dimension support (for localization)
    dimension VARCHAR(50) DEFAULT 'default',

    -- Sort order for drag-to-reorder
    sort_order INTEGER NOT NULL DEFAULT 0,

    -- Extra metadata (JSON for flexibility)
    metadata JSONB DEFAULT '{}',

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT cms_datasource_entries_name_check CHECK (LENGTH(TRIM(name)) > 0),
    CONSTRAINT cms_datasource_entries_value_check CHECK (LENGTH(TRIM(value)) > 0),
    CONSTRAINT cms_datasource_entries_unique UNIQUE (datasource_id, value, dimension)
);

COMMENT ON TABLE cms_datasource_entries IS 'Key-value entries within a datasource';
COMMENT ON COLUMN cms_datasource_entries.name IS 'Display name (label) for the entry';
COMMENT ON COLUMN cms_datasource_entries.value IS 'Actual value used in forms/selects';
COMMENT ON COLUMN cms_datasource_entries.dimension IS 'Dimension for translations (e.g., en, de, fr, default)';
COMMENT ON COLUMN cms_datasource_entries.sort_order IS 'Manual sort order for drag-to-reorder';
COMMENT ON COLUMN cms_datasource_entries.metadata IS 'Optional extra data (icon, color, disabled state, etc.)';

-- -----------------------------------------------------------------------------------------------
-- 3. CREATE INDEXES FOR PERFORMANCE
-- -----------------------------------------------------------------------------------------------

-- Datasources indexes
CREATE INDEX IF NOT EXISTS idx_cms_datasources_slug
ON cms_datasources(slug)
WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_cms_datasources_name
ON cms_datasources(name)
WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_cms_datasources_created_by
ON cms_datasources(created_by)
WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_cms_datasources_deleted_at
ON cms_datasources(deleted_at)
WHERE deleted_at IS NULL;

-- Full-text search on datasources
CREATE INDEX IF NOT EXISTS idx_cms_datasources_search
ON cms_datasources USING GIN (to_tsvector('english', COALESCE(name, '') || ' ' || COALESCE(description, '')))
WHERE deleted_at IS NULL;

-- Entries indexes
CREATE INDEX IF NOT EXISTS idx_cms_datasource_entries_datasource
ON cms_datasource_entries(datasource_id);

CREATE INDEX IF NOT EXISTS idx_cms_datasource_entries_dimension
ON cms_datasource_entries(datasource_id, dimension);

CREATE INDEX IF NOT EXISTS idx_cms_datasource_entries_sort
ON cms_datasource_entries(datasource_id, dimension, sort_order);

CREATE INDEX IF NOT EXISTS idx_cms_datasource_entries_value
ON cms_datasource_entries(datasource_id, value);

-- Full-text search on entries
CREATE INDEX IF NOT EXISTS idx_cms_datasource_entries_search
ON cms_datasource_entries USING GIN (to_tsvector('english', COALESCE(name, '') || ' ' || COALESCE(value, '')));

-- -----------------------------------------------------------------------------------------------
-- 4. CREATE TRIGGERS FOR UPDATED_AT AND ENTRY_COUNT
-- -----------------------------------------------------------------------------------------------

-- Auto-update updated_at for datasources
CREATE OR REPLACE FUNCTION cms_datasources_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_datasources_updated_at_trigger ON cms_datasources;
CREATE TRIGGER cms_datasources_updated_at_trigger
    BEFORE UPDATE ON cms_datasources
    FOR EACH ROW
    EXECUTE FUNCTION cms_datasources_updated_at();

-- Auto-update updated_at for entries
CREATE OR REPLACE FUNCTION cms_datasource_entries_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_datasource_entries_updated_at_trigger ON cms_datasource_entries;
CREATE TRIGGER cms_datasource_entries_updated_at_trigger
    BEFORE UPDATE ON cms_datasource_entries
    FOR EACH ROW
    EXECUTE FUNCTION cms_datasource_entries_updated_at();

-- Auto-update entry_count when entries change
CREATE OR REPLACE FUNCTION cms_datasource_entry_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE cms_datasources SET entry_count = entry_count + 1 WHERE id = NEW.datasource_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE cms_datasources SET entry_count = entry_count - 1 WHERE id = OLD.datasource_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_datasource_entry_count_trigger ON cms_datasource_entries;
CREATE TRIGGER cms_datasource_entry_count_trigger
    AFTER INSERT OR DELETE ON cms_datasource_entries
    FOR EACH ROW
    EXECUTE FUNCTION cms_datasource_entry_count();

-- -----------------------------------------------------------------------------------------------
-- 5. HELPER FUNCTIONS
-- -----------------------------------------------------------------------------------------------

-- Function to get entries for a datasource
CREATE OR REPLACE FUNCTION cms_get_datasource_entries(
    p_datasource_slug VARCHAR(100),
    p_dimension VARCHAR(50) DEFAULT 'default'
)
RETURNS TABLE (
    id UUID,
    name VARCHAR,
    value VARCHAR,
    dimension VARCHAR,
    sort_order INTEGER,
    metadata JSONB
)
LANGUAGE plpgsql
STABLE
AS $$
BEGIN
    RETURN QUERY
    SELECT
        e.id,
        e.name,
        e.value,
        e.dimension,
        e.sort_order,
        e.metadata
    FROM cms_datasource_entries e
    JOIN cms_datasources d ON e.datasource_id = d.id
    WHERE d.slug = p_datasource_slug
    AND d.deleted_at IS NULL
    AND e.dimension = p_dimension
    ORDER BY e.sort_order, e.name;
END;
$$;

COMMENT ON FUNCTION cms_get_datasource_entries IS 'Get all entries for a datasource by slug and dimension';

-- Function to get next sort order
CREATE OR REPLACE FUNCTION cms_get_next_entry_sort_order(p_datasource_id UUID, p_dimension VARCHAR(50) DEFAULT 'default')
RETURNS INTEGER
LANGUAGE plpgsql
STABLE
AS $$
DECLARE
    max_sort INTEGER;
BEGIN
    SELECT COALESCE(MAX(sort_order), -1) + 1 INTO max_sort
    FROM cms_datasource_entries
    WHERE datasource_id = p_datasource_id AND dimension = p_dimension;
    RETURN max_sort;
END;
$$;

COMMENT ON FUNCTION cms_get_next_entry_sort_order IS 'Get next available sort order for new entry';

-- Function to bulk reorder entries
CREATE OR REPLACE FUNCTION cms_reorder_datasource_entries(
    p_entry_ids UUID[],
    p_sort_orders INTEGER[]
)
RETURNS VOID
LANGUAGE plpgsql
AS $$
DECLARE
    i INTEGER;
BEGIN
    FOR i IN 1..array_length(p_entry_ids, 1) LOOP
        UPDATE cms_datasource_entries
        SET sort_order = p_sort_orders[i]
        WHERE id = p_entry_ids[i];
    END LOOP;
END;
$$;

COMMENT ON FUNCTION cms_reorder_datasource_entries IS 'Bulk update sort orders for drag-to-reorder';

-- -----------------------------------------------------------------------------------------------
-- 6. SEED DEFAULT DATASOURCES
-- -----------------------------------------------------------------------------------------------

-- Countries datasource
INSERT INTO cms_datasources (id, name, slug, description, icon, is_system)
VALUES (
    'a0000000-0000-0000-0000-000000000001',
    'Countries',
    'countries',
    'List of world countries for dropdown selections',
    'globe',
    true
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    updated_at = NOW();

-- US States datasource
INSERT INTO cms_datasources (id, name, slug, description, icon, is_system)
VALUES (
    'a0000000-0000-0000-0000-000000000002',
    'US States',
    'us-states',
    'List of US states and territories',
    'map-pin',
    true
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    updated_at = NOW();

-- Trading Symbols datasource
INSERT INTO cms_datasources (id, name, slug, description, icon, color, is_system)
VALUES (
    'a0000000-0000-0000-0000-000000000003',
    'Trading Symbols',
    'trading-symbols',
    'Common trading symbols and tickers',
    'chart-candlestick',
    '#10b981',
    true
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    color = EXCLUDED.color,
    updated_at = NOW();

-- Color Palettes datasource
INSERT INTO cms_datasources (id, name, slug, description, icon, is_system)
VALUES (
    'a0000000-0000-0000-0000-000000000004',
    'Color Palettes',
    'color-palettes',
    'Brand color options for styling',
    'palette',
    true
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    updated_at = NOW();

-- Grid Columns datasource
INSERT INTO cms_datasources (id, name, slug, description, icon, is_system)
VALUES (
    'a0000000-0000-0000-0000-000000000005',
    'Grid Columns',
    'grid-columns',
    'Grid column count options (1-12)',
    'layout-columns',
    true
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    updated_at = NOW();

-- Animation Presets datasource
INSERT INTO cms_datasources (id, name, slug, description, icon, color, is_system)
VALUES (
    'a0000000-0000-0000-0000-000000000006',
    'Animation Presets',
    'animation-presets',
    'Pre-defined animation options',
    'sparkles',
    '#8b5cf6',
    true
)
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    icon = EXCLUDED.icon,
    color = EXCLUDED.color,
    updated_at = NOW();

-- -----------------------------------------------------------------------------------------------
-- 7. SEED DEFAULT ENTRIES
-- -----------------------------------------------------------------------------------------------

-- Sample US States
INSERT INTO cms_datasource_entries (datasource_id, name, value, sort_order) VALUES
('a0000000-0000-0000-0000-000000000002', 'Alabama', 'AL', 0),
('a0000000-0000-0000-0000-000000000002', 'Alaska', 'AK', 1),
('a0000000-0000-0000-0000-000000000002', 'Arizona', 'AZ', 2),
('a0000000-0000-0000-0000-000000000002', 'Arkansas', 'AR', 3),
('a0000000-0000-0000-0000-000000000002', 'California', 'CA', 4),
('a0000000-0000-0000-0000-000000000002', 'Colorado', 'CO', 5),
('a0000000-0000-0000-0000-000000000002', 'Connecticut', 'CT', 6),
('a0000000-0000-0000-0000-000000000002', 'Delaware', 'DE', 7),
('a0000000-0000-0000-0000-000000000002', 'Florida', 'FL', 8),
('a0000000-0000-0000-0000-000000000002', 'Georgia', 'GA', 9),
('a0000000-0000-0000-0000-000000000002', 'Hawaii', 'HI', 10),
('a0000000-0000-0000-0000-000000000002', 'Idaho', 'ID', 11),
('a0000000-0000-0000-0000-000000000002', 'Illinois', 'IL', 12),
('a0000000-0000-0000-0000-000000000002', 'Indiana', 'IN', 13),
('a0000000-0000-0000-0000-000000000002', 'Iowa', 'IA', 14),
('a0000000-0000-0000-0000-000000000002', 'Kansas', 'KS', 15),
('a0000000-0000-0000-0000-000000000002', 'Kentucky', 'KY', 16),
('a0000000-0000-0000-0000-000000000002', 'Louisiana', 'LA', 17),
('a0000000-0000-0000-0000-000000000002', 'Maine', 'ME', 18),
('a0000000-0000-0000-0000-000000000002', 'Maryland', 'MD', 19),
('a0000000-0000-0000-0000-000000000002', 'Massachusetts', 'MA', 20),
('a0000000-0000-0000-0000-000000000002', 'Michigan', 'MI', 21),
('a0000000-0000-0000-0000-000000000002', 'Minnesota', 'MN', 22),
('a0000000-0000-0000-0000-000000000002', 'Mississippi', 'MS', 23),
('a0000000-0000-0000-0000-000000000002', 'Missouri', 'MO', 24),
('a0000000-0000-0000-0000-000000000002', 'Montana', 'MT', 25),
('a0000000-0000-0000-0000-000000000002', 'Nebraska', 'NE', 26),
('a0000000-0000-0000-0000-000000000002', 'Nevada', 'NV', 27),
('a0000000-0000-0000-0000-000000000002', 'New Hampshire', 'NH', 28),
('a0000000-0000-0000-0000-000000000002', 'New Jersey', 'NJ', 29),
('a0000000-0000-0000-0000-000000000002', 'New Mexico', 'NM', 30),
('a0000000-0000-0000-0000-000000000002', 'New York', 'NY', 31),
('a0000000-0000-0000-0000-000000000002', 'North Carolina', 'NC', 32),
('a0000000-0000-0000-0000-000000000002', 'North Dakota', 'ND', 33),
('a0000000-0000-0000-0000-000000000002', 'Ohio', 'OH', 34),
('a0000000-0000-0000-0000-000000000002', 'Oklahoma', 'OK', 35),
('a0000000-0000-0000-0000-000000000002', 'Oregon', 'OR', 36),
('a0000000-0000-0000-0000-000000000002', 'Pennsylvania', 'PA', 37),
('a0000000-0000-0000-0000-000000000002', 'Rhode Island', 'RI', 38),
('a0000000-0000-0000-0000-000000000002', 'South Carolina', 'SC', 39),
('a0000000-0000-0000-0000-000000000002', 'South Dakota', 'SD', 40),
('a0000000-0000-0000-0000-000000000002', 'Tennessee', 'TN', 41),
('a0000000-0000-0000-0000-000000000002', 'Texas', 'TX', 42),
('a0000000-0000-0000-0000-000000000002', 'Utah', 'UT', 43),
('a0000000-0000-0000-0000-000000000002', 'Vermont', 'VT', 44),
('a0000000-0000-0000-0000-000000000002', 'Virginia', 'VA', 45),
('a0000000-0000-0000-0000-000000000002', 'Washington', 'WA', 46),
('a0000000-0000-0000-0000-000000000002', 'West Virginia', 'WV', 47),
('a0000000-0000-0000-0000-000000000002', 'Wisconsin', 'WI', 48),
('a0000000-0000-0000-0000-000000000002', 'Wyoming', 'WY', 49)
ON CONFLICT (datasource_id, value, dimension) DO NOTHING;

-- Trading Symbols
INSERT INTO cms_datasource_entries (datasource_id, name, value, sort_order, metadata) VALUES
('a0000000-0000-0000-0000-000000000003', 'S&P 500', 'SPY', 0, '{"type": "index", "market": "US"}'),
('a0000000-0000-0000-0000-000000000003', 'Nasdaq 100', 'QQQ', 1, '{"type": "index", "market": "US"}'),
('a0000000-0000-0000-0000-000000000003', 'Dow Jones', 'DIA', 2, '{"type": "index", "market": "US"}'),
('a0000000-0000-0000-0000-000000000003', 'Russell 2000', 'IWM', 3, '{"type": "index", "market": "US"}'),
('a0000000-0000-0000-0000-000000000003', 'Apple', 'AAPL', 4, '{"type": "stock", "sector": "Technology"}'),
('a0000000-0000-0000-0000-000000000003', 'Microsoft', 'MSFT', 5, '{"type": "stock", "sector": "Technology"}'),
('a0000000-0000-0000-0000-000000000003', 'Amazon', 'AMZN', 6, '{"type": "stock", "sector": "Consumer"}'),
('a0000000-0000-0000-0000-000000000003', 'Google', 'GOOGL', 7, '{"type": "stock", "sector": "Technology"}'),
('a0000000-0000-0000-0000-000000000003', 'Tesla', 'TSLA', 8, '{"type": "stock", "sector": "Automotive"}'),
('a0000000-0000-0000-0000-000000000003', 'NVIDIA', 'NVDA', 9, '{"type": "stock", "sector": "Technology"}'),
('a0000000-0000-0000-0000-000000000003', 'Bitcoin', 'BTC', 10, '{"type": "crypto", "market": "Crypto"}'),
('a0000000-0000-0000-0000-000000000003', 'Ethereum', 'ETH', 11, '{"type": "crypto", "market": "Crypto"}'),
('a0000000-0000-0000-0000-000000000003', 'Gold', 'GLD', 12, '{"type": "commodity", "market": "Commodities"}'),
('a0000000-0000-0000-0000-000000000003', 'Silver', 'SLV', 13, '{"type": "commodity", "market": "Commodities"}'),
('a0000000-0000-0000-0000-000000000003', 'Crude Oil', 'USO', 14, '{"type": "commodity", "market": "Commodities"}')
ON CONFLICT (datasource_id, value, dimension) DO NOTHING;

-- Color Palettes
INSERT INTO cms_datasource_entries (datasource_id, name, value, sort_order, metadata) VALUES
('a0000000-0000-0000-0000-000000000004', 'Primary Blue', '#3b82f6', 0, '{"category": "primary"}'),
('a0000000-0000-0000-0000-000000000004', 'Primary Indigo', '#6366f1', 1, '{"category": "primary"}'),
('a0000000-0000-0000-0000-000000000004', 'Primary Purple', '#8b5cf6', 2, '{"category": "primary"}'),
('a0000000-0000-0000-0000-000000000004', 'Success Green', '#10b981', 3, '{"category": "status"}'),
('a0000000-0000-0000-0000-000000000004', 'Warning Amber', '#f59e0b', 4, '{"category": "status"}'),
('a0000000-0000-0000-0000-000000000004', 'Error Red', '#ef4444', 5, '{"category": "status"}'),
('a0000000-0000-0000-0000-000000000004', 'Info Cyan', '#06b6d4', 6, '{"category": "status"}'),
('a0000000-0000-0000-0000-000000000004', 'Neutral Gray', '#6b7280', 7, '{"category": "neutral"}'),
('a0000000-0000-0000-0000-000000000004', 'Dark Navy', '#1e3a5f', 8, '{"category": "brand"}'),
('a0000000-0000-0000-0000-000000000004', 'Revolution Gold', '#d4af37', 9, '{"category": "brand"}')
ON CONFLICT (datasource_id, value, dimension) DO NOTHING;

-- Grid Columns
INSERT INTO cms_datasource_entries (datasource_id, name, value, sort_order) VALUES
('a0000000-0000-0000-0000-000000000005', '1 Column', '1', 0),
('a0000000-0000-0000-0000-000000000005', '2 Columns', '2', 1),
('a0000000-0000-0000-0000-000000000005', '3 Columns', '3', 2),
('a0000000-0000-0000-0000-000000000005', '4 Columns', '4', 3),
('a0000000-0000-0000-0000-000000000005', '5 Columns', '5', 4),
('a0000000-0000-0000-0000-000000000005', '6 Columns', '6', 5),
('a0000000-0000-0000-0000-000000000005', '8 Columns', '8', 6),
('a0000000-0000-0000-0000-000000000005', '10 Columns', '10', 7),
('a0000000-0000-0000-0000-000000000005', '12 Columns', '12', 8)
ON CONFLICT (datasource_id, value, dimension) DO NOTHING;

-- Animation Presets
INSERT INTO cms_datasource_entries (datasource_id, name, value, sort_order, metadata) VALUES
('a0000000-0000-0000-0000-000000000006', 'None', 'none', 0, '{"duration": 0}'),
('a0000000-0000-0000-0000-000000000006', 'Fade In', 'fade-in', 1, '{"duration": 0.5}'),
('a0000000-0000-0000-0000-000000000006', 'Fade Up', 'fade-up', 2, '{"duration": 0.5}'),
('a0000000-0000-0000-0000-000000000006', 'Fade Down', 'fade-down', 3, '{"duration": 0.5}'),
('a0000000-0000-0000-0000-000000000006', 'Fade Left', 'fade-left', 4, '{"duration": 0.5}'),
('a0000000-0000-0000-0000-000000000006', 'Fade Right', 'fade-right', 5, '{"duration": 0.5}'),
('a0000000-0000-0000-0000-000000000006', 'Zoom In', 'zoom-in', 6, '{"duration": 0.4}'),
('a0000000-0000-0000-0000-000000000006', 'Zoom Out', 'zoom-out', 7, '{"duration": 0.4}'),
('a0000000-0000-0000-0000-000000000006', 'Bounce', 'bounce', 8, '{"duration": 0.6}'),
('a0000000-0000-0000-0000-000000000006', 'Slide Up', 'slide-up', 9, '{"duration": 0.4}'),
('a0000000-0000-0000-0000-000000000006', 'Slide Down', 'slide-down', 10, '{"duration": 0.4}'),
('a0000000-0000-0000-0000-000000000006', 'Flip', 'flip', 11, '{"duration": 0.6}')
ON CONFLICT (datasource_id, value, dimension) DO NOTHING;

-- Update entry counts
UPDATE cms_datasources SET entry_count = (SELECT COUNT(*) FROM cms_datasource_entries WHERE datasource_id = cms_datasources.id);

-- -----------------------------------------------------------------------------------------------
-- 8. ROW LEVEL SECURITY POLICIES
-- -----------------------------------------------------------------------------------------------

ALTER TABLE cms_datasources ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_datasource_entries ENABLE ROW LEVEL SECURITY;

-- Datasources: Everyone can read, editors can write
CREATE POLICY cms_datasources_read ON cms_datasources
    FOR SELECT
    USING (deleted_at IS NULL);

CREATE POLICY cms_datasources_insert ON cms_datasources
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

CREATE POLICY cms_datasources_update ON cms_datasources
    FOR UPDATE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager')
        OR (
            cms_current_user_role() = 'content_editor'
            AND is_locked = false
            AND is_system = false
        )
    );

CREATE POLICY cms_datasources_delete ON cms_datasources
    FOR DELETE
    USING (
        cms_current_user_role() IN ('super_admin')
        AND is_system = false
    );

-- Entries: Everyone can read, editors can write
CREATE POLICY cms_datasource_entries_read ON cms_datasource_entries
    FOR SELECT
    USING (TRUE);

CREATE POLICY cms_datasource_entries_insert ON cms_datasource_entries
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

CREATE POLICY cms_datasource_entries_update ON cms_datasource_entries
    FOR UPDATE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

CREATE POLICY cms_datasource_entries_delete ON cms_datasource_entries
    FOR DELETE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

-- -----------------------------------------------------------------------------------------------
-- MIGRATION COMPLETE
-- -----------------------------------------------------------------------------------------------

COMMENT ON TABLE cms_datasources IS 'CMS Datasources System - Reusable option lists for dropdowns and selections';
