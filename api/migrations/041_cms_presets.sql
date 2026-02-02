-- ═══════════════════════════════════════════════════════════════════════════════════════
-- CMS PRESETS/TEMPLATES SYSTEM - Storyblok-Style Component Presets
-- Apple ICT 7+ Principal Engineer Grade
-- Revolution Trading Pros - February 2026
--
-- This migration adds a comprehensive preset/template system for block components:
-- - Pre-designed block configurations
-- - Category organization (default, brand, seasonal)
-- - Thumbnail previews
-- - Usage tracking
-- - Version control
--
-- Design Principles:
-- 1. Performance-first with efficient indexing
-- 2. Flexible preset data storage using JSONB
-- 3. Category-based organization for easy discovery
-- 4. Soft delete support for data retention
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 1. CREATE PRESET CATEGORY ENUM
-- ─────────────────────────────────────────────────────────────────────────────────────────

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_preset_category') THEN
        CREATE TYPE cms_preset_category AS ENUM (
            'default',      -- System defaults, always available
            'brand',        -- Brand-specific presets (colors, styles)
            'seasonal',     -- Holiday/event-specific presets
            'marketing',    -- Marketing campaign presets
            'trading',      -- Trading-specific presets
            'custom'        -- User-created custom presets
        );
    END IF;
END$$;

COMMENT ON TYPE cms_preset_category IS 'Categories for organizing block presets';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 2. CREATE CMS_PRESETS TABLE
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS cms_presets (
    -- Primary key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Block type reference (e.g., 'button', 'heading', 'callout')
    block_type VARCHAR(50) NOT NULL,

    -- Preset identification
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    description TEXT,

    -- Preset data (content + settings combined)
    preset_data JSONB NOT NULL DEFAULT '{}',

    -- Preview thumbnail
    thumbnail_url TEXT,
    thumbnail_blurhash VARCHAR(100),

    -- Organization
    category cms_preset_category NOT NULL DEFAULT 'custom',
    tags TEXT[] DEFAULT '{}',

    -- Flags
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    is_global BOOLEAN NOT NULL DEFAULT true,

    -- Usage tracking
    usage_count INTEGER NOT NULL DEFAULT 0,

    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    updated_by UUID REFERENCES cms_users(id) ON DELETE SET NULL,
    deleted_at TIMESTAMPTZ,

    -- Constraints
    CONSTRAINT cms_presets_slug_unique UNIQUE (block_type, slug) WHERE deleted_at IS NULL,
    CONSTRAINT cms_presets_name_check CHECK (LENGTH(TRIM(name)) > 0),
    CONSTRAINT cms_presets_slug_check CHECK (slug ~ '^[a-z0-9-]+$')
);

COMMENT ON TABLE cms_presets IS 'Storyblok-style component presets/templates for quick block insertion';
COMMENT ON COLUMN cms_presets.block_type IS 'The block type this preset applies to (e.g., button, heading)';
COMMENT ON COLUMN cms_presets.preset_data IS 'JSON containing content and settings for the preset';
COMMENT ON COLUMN cms_presets.is_default IS 'Whether this preset appears as the default for its block type';
COMMENT ON COLUMN cms_presets.is_locked IS 'Prevents editing by non-admin users';
COMMENT ON COLUMN cms_presets.is_global IS 'Available to all users vs. creator only';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 3. CREATE INDEXES FOR PERFORMANCE
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Primary lookup by block type
CREATE INDEX IF NOT EXISTS idx_cms_presets_block_type
ON cms_presets(block_type)
WHERE deleted_at IS NULL;

-- Category filtering
CREATE INDEX IF NOT EXISTS idx_cms_presets_category
ON cms_presets(category)
WHERE deleted_at IS NULL;

-- Default presets lookup
CREATE INDEX IF NOT EXISTS idx_cms_presets_defaults
ON cms_presets(block_type, is_default)
WHERE is_default = true AND deleted_at IS NULL;

-- Global presets listing
CREATE INDEX IF NOT EXISTS idx_cms_presets_global
ON cms_presets(is_global, block_type, category)
WHERE deleted_at IS NULL;

-- Usage count for popular presets
CREATE INDEX IF NOT EXISTS idx_cms_presets_popular
ON cms_presets(usage_count DESC)
WHERE deleted_at IS NULL;

-- Creator lookup
CREATE INDEX IF NOT EXISTS idx_cms_presets_created_by
ON cms_presets(created_by)
WHERE deleted_at IS NULL;

-- Full-text search on name and description
CREATE INDEX IF NOT EXISTS idx_cms_presets_search
ON cms_presets USING GIN (to_tsvector('english', COALESCE(name, '') || ' ' || COALESCE(description, '')))
WHERE deleted_at IS NULL;

-- Tags array search
CREATE INDEX IF NOT EXISTS idx_cms_presets_tags
ON cms_presets USING GIN (tags)
WHERE deleted_at IS NULL;

-- Soft delete index
CREATE INDEX IF NOT EXISTS idx_cms_presets_deleted_at
ON cms_presets(deleted_at)
WHERE deleted_at IS NULL;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 4. CREATE TRIGGER FOR UPDATED_AT
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_presets_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_presets_updated_at_trigger ON cms_presets;
CREATE TRIGGER cms_presets_updated_at_trigger
    BEFORE UPDATE ON cms_presets
    FOR EACH ROW
    EXECUTE FUNCTION cms_presets_updated_at();

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 5. HELPER FUNCTIONS
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Function to increment preset usage count
CREATE OR REPLACE FUNCTION cms_increment_preset_usage(p_preset_id UUID)
RETURNS VOID AS $$
BEGIN
    UPDATE cms_presets
    SET usage_count = usage_count + 1
    WHERE id = p_preset_id AND deleted_at IS NULL;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION cms_increment_preset_usage IS 'Increment usage count when a preset is applied to a block';

-- Function to get presets by block type with categories
CREATE OR REPLACE FUNCTION cms_get_presets_by_block_type(
    p_block_type VARCHAR(50),
    p_category cms_preset_category DEFAULT NULL,
    p_include_global BOOLEAN DEFAULT true,
    p_user_id UUID DEFAULT NULL
)
RETURNS TABLE (
    id UUID,
    block_type VARCHAR,
    name VARCHAR,
    slug VARCHAR,
    description TEXT,
    preset_data JSONB,
    thumbnail_url TEXT,
    category cms_preset_category,
    tags TEXT[],
    is_default BOOLEAN,
    usage_count INTEGER
)
LANGUAGE plpgsql
STABLE
AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.id,
        p.block_type,
        p.name,
        p.slug,
        p.description,
        p.preset_data,
        p.thumbnail_url,
        p.category,
        p.tags,
        p.is_default,
        p.usage_count
    FROM cms_presets p
    WHERE p.block_type = p_block_type
    AND p.deleted_at IS NULL
    AND (p_category IS NULL OR p.category = p_category)
    AND (p_include_global AND p.is_global = true OR p.created_by = p_user_id)
    ORDER BY
        p.is_default DESC,
        p.category,
        p.usage_count DESC,
        p.name;
END;
$$;

COMMENT ON FUNCTION cms_get_presets_by_block_type IS 'Get all presets for a specific block type with optional category filtering';

-- Function to get all block types that have presets
CREATE OR REPLACE FUNCTION cms_get_block_types_with_presets()
RETURNS TABLE (
    block_type VARCHAR,
    preset_count BIGINT,
    has_default BOOLEAN
)
LANGUAGE plpgsql
STABLE
AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.block_type,
        COUNT(*)::BIGINT AS preset_count,
        BOOL_OR(p.is_default) AS has_default
    FROM cms_presets p
    WHERE p.deleted_at IS NULL
    AND p.is_global = true
    GROUP BY p.block_type
    ORDER BY p.block_type;
END;
$$;

COMMENT ON FUNCTION cms_get_block_types_with_presets IS 'Get list of block types that have available presets';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 6. SEED DEFAULT PRESETS
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Insert default button presets
INSERT INTO cms_presets (block_type, name, slug, description, preset_data, category, is_default, is_global, tags) VALUES
-- Button presets
('button', 'Primary CTA', 'primary-cta', 'Bold primary call-to-action button',
 '{"content": {"text": "Get Started"}, "settings": {"buttonStyle": "primary", "buttonSize": "large", "backgroundColor": "#3b82f6", "textColor": "#ffffff", "borderRadius": "8px", "fontWeight": "600"}}',
 'default', true, true, ARRAY['cta', 'primary', 'action']),

('button', 'Secondary', 'secondary', 'Subtle secondary action button',
 '{"content": {"text": "Learn More"}, "settings": {"buttonStyle": "secondary", "buttonSize": "medium", "backgroundColor": "#f3f4f6", "textColor": "#374151", "borderRadius": "6px"}}',
 'default', false, true, ARRAY['secondary', 'subtle']),

('button', 'Ghost', 'ghost', 'Transparent ghost button with border',
 '{"content": {"text": "View Details"}, "settings": {"buttonStyle": "ghost", "buttonSize": "medium", "backgroundColor": "transparent", "textColor": "#3b82f6", "borderWidth": "1px", "borderColor": "#3b82f6", "borderRadius": "6px"}}',
 'default', false, true, ARRAY['ghost', 'transparent', 'outline']),

('button', 'Outline', 'outline', 'Outlined button style',
 '{"content": {"text": "Contact Us"}, "settings": {"buttonStyle": "outline", "buttonSize": "medium", "backgroundColor": "transparent", "textColor": "#1f2937", "borderWidth": "2px", "borderColor": "#1f2937", "borderRadius": "6px"}}',
 'default', false, true, ARRAY['outline', 'bordered']),

('button', 'Link Style', 'link-style', 'Text link styled as button',
 '{"content": {"text": "Read more"}, "settings": {"buttonStyle": "link", "buttonSize": "small", "textColor": "#3b82f6", "textTransform": "none"}}',
 'default', false, true, ARRAY['link', 'text', 'minimal']),

-- Heading presets
('heading', 'Hero Title', 'hero-title', 'Large hero section title',
 '{"content": {"text": "Welcome to Our Platform"}, "settings": {"level": 1, "textAlign": "center", "fontSize": "48px", "fontWeight": "700", "letterSpacing": "-0.02em", "lineHeight": "1.2"}}',
 'default', true, true, ARRAY['hero', 'title', 'large']),

('heading', 'Section Header', 'section-header', 'Standard section heading',
 '{"content": {"text": "Our Features"}, "settings": {"level": 2, "textAlign": "left", "fontSize": "32px", "fontWeight": "600", "marginBottom": "1.5rem"}}',
 'default', false, true, ARRAY['section', 'header']),

('heading', 'Subsection', 'subsection', 'Smaller subsection heading',
 '{"content": {"text": "Key Benefits"}, "settings": {"level": 3, "textAlign": "left", "fontSize": "24px", "fontWeight": "600", "marginBottom": "1rem"}}',
 'default', false, true, ARRAY['subsection', 'small']),

('heading', 'Trading Alert Title', 'trading-alert-title', 'Bold title for trading alerts',
 '{"content": {"text": "Market Alert"}, "settings": {"level": 2, "textAlign": "left", "fontSize": "28px", "fontWeight": "700", "textColor": "#ef4444", "textTransform": "uppercase", "letterSpacing": "0.05em"}}',
 'trading', false, true, ARRAY['trading', 'alert', 'urgent']),

-- Callout presets
('callout', 'Info Box', 'info-box', 'Blue informational callout',
 '{"content": {"text": "This is important information you should know."}, "settings": {"backgroundColor": "#eff6ff", "borderColor": "#3b82f6", "borderWidth": "4px", "borderStyle": "solid", "borderRadius": "8px", "padding": "1.5rem", "textColor": "#1e40af"}}',
 'default', true, true, ARRAY['info', 'blue', 'notice']),

('callout', 'Warning', 'warning', 'Yellow warning callout',
 '{"content": {"text": "Please pay attention to this warning."}, "settings": {"backgroundColor": "#fffbeb", "borderColor": "#f59e0b", "borderWidth": "4px", "borderStyle": "solid", "borderRadius": "8px", "padding": "1.5rem", "textColor": "#92400e"}}',
 'default', false, true, ARRAY['warning', 'yellow', 'caution']),

('callout', 'Success', 'success', 'Green success callout',
 '{"content": {"text": "Great job! Everything completed successfully."}, "settings": {"backgroundColor": "#ecfdf5", "borderColor": "#10b981", "borderWidth": "4px", "borderStyle": "solid", "borderRadius": "8px", "padding": "1.5rem", "textColor": "#065f46"}}',
 'default', false, true, ARRAY['success', 'green', 'positive']),

('callout', 'Error', 'error', 'Red error callout',
 '{"content": {"text": "Something went wrong. Please try again."}, "settings": {"backgroundColor": "#fef2f2", "borderColor": "#ef4444", "borderWidth": "4px", "borderStyle": "solid", "borderRadius": "8px", "padding": "1.5rem", "textColor": "#991b1b"}}',
 'default', false, true, ARRAY['error', 'red', 'danger']),

('callout', 'Pro Tip', 'pro-tip', 'Purple pro tip callout',
 '{"content": {"text": "Pro Tip: Here is an expert insight to help you succeed."}, "settings": {"backgroundColor": "#f5f3ff", "borderColor": "#8b5cf6", "borderWidth": "4px", "borderStyle": "solid", "borderRadius": "8px", "padding": "1.5rem", "textColor": "#5b21b6"}}',
 'default', false, true, ARRAY['tip', 'purple', 'expert']),

-- Quote presets
('quote', 'Simple Quote', 'simple-quote', 'Clean minimal quote style',
 '{"content": {"text": "The only way to do great work is to love what you do."}, "settings": {"borderColor": "#e5e7eb", "borderWidth": "4px", "borderStyle": "solid", "padding": "1.5rem", "fontStyle": "italic", "fontSize": "1.125rem"}}',
 'default', true, true, ARRAY['simple', 'minimal', 'clean']),

('quote', 'Highlight Quote', 'highlight-quote', 'Bold highlighted quote',
 '{"content": {"text": "Success is not final, failure is not fatal: it is the courage to continue that counts."}, "settings": {"backgroundColor": "#f8fafc", "borderColor": "#3b82f6", "borderWidth": "4px", "borderStyle": "solid", "borderRadius": "12px", "padding": "2rem", "fontSize": "1.25rem", "fontWeight": "500", "textAlign": "center"}}',
 'default', false, true, ARRAY['highlight', 'featured', 'bold']),

-- Card presets
('card', 'Feature Card', 'feature-card', 'Card for featuring a product or service',
 '{"content": {}, "settings": {"backgroundColor": "#ffffff", "borderRadius": "12px", "boxShadow": "0 4px 6px -1px rgba(0, 0, 0, 0.1)", "padding": "2rem"}}',
 'default', true, true, ARRAY['feature', 'product', 'service']),

('card', 'Pricing Card', 'pricing-card', 'Card designed for pricing tables',
 '{"content": {}, "settings": {"backgroundColor": "#ffffff", "borderRadius": "16px", "borderWidth": "1px", "borderColor": "#e5e7eb", "boxShadow": "0 10px 15px -3px rgba(0, 0, 0, 0.1)", "padding": "2.5rem", "textAlign": "center"}}',
 'default', false, true, ARRAY['pricing', 'plan', 'subscription']),

-- Separator presets
('separator', 'Simple Line', 'simple-line', 'Basic horizontal line',
 '{"content": {}, "settings": {"borderColor": "#e5e7eb", "borderWidth": "1px", "margin": "2rem 0"}}',
 'default', true, true, ARRAY['simple', 'basic']),

('separator', 'Gradient Line', 'gradient-line', 'Colorful gradient separator',
 '{"content": {}, "settings": {"backgroundGradient": "linear-gradient(90deg, #3b82f6, #8b5cf6, #ec4899)", "height": "3px", "borderRadius": "2px", "margin": "2.5rem 0"}}',
 'default', false, true, ARRAY['gradient', 'colorful', 'decorative']),

-- Trading-specific presets
('riskDisclaimer', 'Standard Disclaimer', 'standard-disclaimer', 'Standard trading risk disclaimer',
 '{"content": {"text": "Trading involves substantial risk of loss and is not suitable for all investors. Past performance is not indicative of future results. Only risk capital should be used for trading."}, "settings": {"backgroundColor": "#fef3c7", "borderColor": "#f59e0b", "borderWidth": "2px", "borderStyle": "solid", "borderRadius": "8px", "padding": "1.5rem", "fontSize": "0.875rem", "textColor": "#92400e"}}',
 'trading', true, true, ARRAY['risk', 'disclaimer', 'legal']),

('riskDisclaimer', 'Compact Disclaimer', 'compact-disclaimer', 'Shorter risk warning',
 '{"content": {"text": "Trading involves risk. Only trade with funds you can afford to lose."}, "settings": {"backgroundColor": "#fff7ed", "borderColor": "#ea580c", "borderWidth": "1px", "borderStyle": "solid", "borderRadius": "4px", "padding": "0.75rem", "fontSize": "0.75rem", "textColor": "#9a3412"}}',
 'trading', false, true, ARRAY['risk', 'compact', 'short']),

-- Brand-specific presets
('button', 'Brand Primary', 'brand-primary', 'Primary brand color button',
 '{"content": {"text": "Join Revolution"}, "settings": {"buttonStyle": "primary", "buttonSize": "large", "backgroundColor": "#1e3a5f", "textColor": "#ffffff", "borderRadius": "8px", "fontWeight": "700", "boxShadow": "0 4px 14px rgba(30, 58, 95, 0.4)"}}',
 'brand', false, true, ARRAY['brand', 'revolution', 'primary']),

('callout', 'Brand Announcement', 'brand-announcement', 'Brand-colored announcement box',
 '{"content": {"text": "Join our exclusive trading community today!"}, "settings": {"backgroundColor": "#1e3a5f", "borderRadius": "12px", "padding": "2rem", "textColor": "#ffffff", "textAlign": "center", "fontWeight": "600", "fontSize": "1.125rem"}}',
 'brand', false, true, ARRAY['brand', 'announcement', 'promo']),

-- Seasonal presets
('callout', 'Holiday Sale', 'holiday-sale', 'Holiday promotion callout',
 '{"content": {"text": "Limited Time Holiday Offer - Save 25%!"}, "settings": {"backgroundColor": "#dc2626", "borderRadius": "12px", "padding": "1.5rem", "textColor": "#ffffff", "textAlign": "center", "fontWeight": "700", "fontSize": "1.25rem"}}',
 'seasonal', false, true, ARRAY['holiday', 'sale', 'promo', 'christmas']),

('button', 'New Year CTA', 'new-year-cta', 'New Year promotion button',
 '{"content": {"text": "Start Your 2026 Journey"}, "settings": {"buttonStyle": "primary", "buttonSize": "large", "backgroundGradient": "linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%)", "textColor": "#1f2937", "borderRadius": "9999px", "fontWeight": "700", "boxShadow": "0 4px 14px rgba(245, 158, 11, 0.4)"}}',
 'seasonal', false, true, ARRAY['newyear', '2026', 'promotion'])

ON CONFLICT (block_type, slug) WHERE deleted_at IS NULL DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    preset_data = EXCLUDED.preset_data,
    category = EXCLUDED.category,
    is_default = EXCLUDED.is_default,
    tags = EXCLUDED.tags,
    updated_at = NOW();

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 7. ROW LEVEL SECURITY POLICIES
-- ─────────────────────────────────────────────────────────────────────────────────────────

ALTER TABLE cms_presets ENABLE ROW LEVEL SECURITY;

-- Read access: Everyone can read global presets, creators can read their own
CREATE POLICY cms_presets_read ON cms_presets
    FOR SELECT
    USING (
        deleted_at IS NULL
        AND (is_global = true OR created_by = cms_current_user_id())
    );

-- Insert: Editors and above can create presets
CREATE POLICY cms_presets_insert ON cms_presets
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

-- Update: Admins can update any, editors can update unlocked own presets
CREATE POLICY cms_presets_update ON cms_presets
    FOR UPDATE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager')
        OR (
            cms_current_user_role() = 'content_editor'
            AND created_by = cms_current_user_id()
            AND is_locked = false
        )
    );

-- Delete: Only admins can delete
CREATE POLICY cms_presets_delete ON cms_presets
    FOR DELETE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager')
    );

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- MIGRATION COMPLETE
-- ─────────────────────────────────────────────────────────────────────────────────────────

COMMENT ON TABLE cms_presets IS 'CMS Presets/Templates System - Storyblok-style component presets for the block editor';
