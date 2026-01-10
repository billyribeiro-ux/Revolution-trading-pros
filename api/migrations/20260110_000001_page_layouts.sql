-- Page Layouts for Course Page Builder
-- Apple Principal Engineer ICT 7 Grade - January 2026

-- ═══════════════════════════════════════════════════════════════════════════════════
-- PAGE LAYOUTS TABLE
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS page_layouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Course relationship (optional - layouts can be standalone)
    course_id UUID REFERENCES courses(id) ON DELETE SET NULL,
    
    -- Layout metadata
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE,
    description TEXT,
    
    -- Layout status
    status VARCHAR(50) DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    version INTEGER DEFAULT 1,
    
    -- The actual layout data (JSON)
    blocks JSONB NOT NULL DEFAULT '[]',
    
    -- Timestamps
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    
    -- Author tracking
    created_by UUID,
    updated_by UUID
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_page_layouts_course_id ON page_layouts(course_id);
CREATE INDEX IF NOT EXISTS idx_page_layouts_status ON page_layouts(status);
CREATE INDEX IF NOT EXISTS idx_page_layouts_slug ON page_layouts(slug);

-- Trigger to update updated_at
CREATE OR REPLACE FUNCTION update_page_layouts_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_page_layouts_updated_at ON page_layouts;
CREATE TRIGGER trigger_page_layouts_updated_at
    BEFORE UPDATE ON page_layouts
    FOR EACH ROW
    EXECUTE FUNCTION update_page_layouts_updated_at();

-- ═══════════════════════════════════════════════════════════════════════════════════
-- PAGE LAYOUT VERSIONS (for undo/redo and history)
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS page_layout_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    layout_id UUID NOT NULL REFERENCES page_layouts(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    blocks JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID,
    
    UNIQUE(layout_id, version)
);

CREATE INDEX IF NOT EXISTS idx_page_layout_versions_layout_id ON page_layout_versions(layout_id);

-- Comments
COMMENT ON TABLE page_layouts IS 'Stores page builder layouts with their block configurations';
COMMENT ON TABLE page_layout_versions IS 'Version history for page layouts';
COMMENT ON COLUMN page_layouts.blocks IS 'JSON array of block configurations with type, config, and order';
