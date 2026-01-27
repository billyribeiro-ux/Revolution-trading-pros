-- ═══════════════════════════════════════════════════════════════════════════════════════
-- ADVANCED CMS FEATURES - Apple ICT 7+ Principal Engineer Grade
-- Revolution Trading Pros - January 2026
--
-- This migration adds advanced features to the custom CMS:
-- - Additional enum types for trading-specific content
-- - Full-text search capabilities with ranking
-- - Optimized content retrieval functions
-- - Atomic publishing operations
-- - Structured data generation for SEO
-- - Row Level Security (RLS) policies
--
-- Design Principles:
-- 1. Performance-first with <10ms API response targets
-- 2. Comprehensive indexing for common query patterns
-- 3. Security through RLS at the database level
-- 4. Atomic operations for data integrity
-- ═══════════════════════════════════════════════════════════════════════════════════════

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 1. ADDITIONAL EXTENSIONS
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Trigram similarity for fuzzy search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- GIN indexes on scalar types
CREATE EXTENSION IF NOT EXISTS btree_gin;

-- Remove accents for search normalization
CREATE EXTENSION IF NOT EXISTS unaccent;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 2. ADDITIONAL ENUMS FOR TRADING-SPECIFIC CONTENT
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Alert service categories
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_alert_type') THEN
        CREATE TYPE cms_alert_type AS ENUM (
            'intraday',
            'swing',
            'options',
            'futures'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_alert_type IS 'Categories for alert service offerings';

-- Trading room categories
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_room_type') THEN
        CREATE TYPE cms_room_type AS ENUM (
            'day_trading',
            'swing_trading',
            'options',
            'futures',
            'small_accounts'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_room_type IS 'Categories for live trading room offerings';

-- Course difficulty levels
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_difficulty_level') THEN
        CREATE TYPE cms_difficulty_level AS ENUM (
            'beginner',
            'intermediate',
            'advanced',
            'all_levels'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_difficulty_level IS 'Difficulty levels for courses and content';

-- Content editing modes for blog posts
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_content_mode') THEN
        CREATE TYPE cms_content_mode AS ENUM (
            'richtext',
            'markdown',
            'html'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_content_mode IS 'Editing mode for content (rich text, markdown, or raw HTML)';

-- Asset type classification
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_asset_type') THEN
        CREATE TYPE cms_asset_type AS ENUM (
            'image',
            'video',
            'document',
            'audio'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_asset_type IS 'Classification of digital assets by media type';

-- Workflow action types for audit
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'cms_workflow_action') THEN
        CREATE TYPE cms_workflow_action AS ENUM (
            'submit_for_review',
            'approve',
            'request_changes',
            'reject',
            'schedule',
            'publish',
            'unpublish',
            'archive',
            'restore'
        );
    END IF;
END$$;

COMMENT ON TYPE cms_workflow_action IS 'Workflow transition actions for audit logging';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 3. ADD ASSET TYPE COLUMN TO CMS_ASSETS
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Add asset_type column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'cms_assets' AND column_name = 'asset_type'
    ) THEN
        ALTER TABLE cms_assets ADD COLUMN asset_type cms_asset_type;

        -- Populate based on mime_type
        UPDATE cms_assets SET asset_type =
            CASE
                WHEN mime_type LIKE 'image/%' THEN 'image'::cms_asset_type
                WHEN mime_type LIKE 'video/%' THEN 'video'::cms_asset_type
                WHEN mime_type LIKE 'audio/%' THEN 'audio'::cms_asset_type
                ELSE 'document'::cms_asset_type
            END;

        -- Make NOT NULL after population
        ALTER TABLE cms_assets ALTER COLUMN asset_type SET NOT NULL;

        -- Add index
        CREATE INDEX IF NOT EXISTS idx_cms_assets_type ON cms_assets(asset_type) WHERE deleted_at IS NULL;
    END IF;
END$$;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 4. ADD CONTENT MODE COLUMN TO CMS_CONTENT
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Add content_mode column if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'cms_content' AND column_name = 'content_mode'
    ) THEN
        ALTER TABLE cms_content ADD COLUMN content_mode cms_content_mode DEFAULT 'richtext';
    END IF;
END$$;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 5. FULL-TEXT SEARCH INDEXES
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Create text search configuration for English
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_ts_config WHERE cfgname = 'cms_english'
    ) THEN
        CREATE TEXT SEARCH CONFIGURATION cms_english (COPY = english);
        ALTER TEXT SEARCH CONFIGURATION cms_english
            ALTER MAPPING FOR hword, hword_part, word
            WITH unaccent, english_stem;
    END IF;
EXCEPTION WHEN duplicate_object THEN
    NULL; -- Configuration already exists
END$$;

-- Add tsvector column for efficient full-text search
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'cms_content' AND column_name = 'search_vector'
    ) THEN
        ALTER TABLE cms_content ADD COLUMN search_vector tsvector;

        -- Create GIN index for search
        CREATE INDEX IF NOT EXISTS idx_cms_content_search ON cms_content USING GIN(search_vector);
    END IF;
END$$;

-- Function to update search vector
CREATE OR REPLACE FUNCTION cms_update_search_vector()
RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector :=
        setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A') ||
        setweight(to_tsvector('english', COALESCE(NEW.excerpt, '')), 'B') ||
        setweight(to_tsvector('english', COALESCE(NEW.content, '')), 'C') ||
        setweight(to_tsvector('english', COALESCE(NEW.meta_description, '')), 'D');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-update search vector
DROP TRIGGER IF EXISTS cms_content_search_trigger ON cms_content;
CREATE TRIGGER cms_content_search_trigger
    BEFORE INSERT OR UPDATE OF title, excerpt, content, meta_description ON cms_content
    FOR EACH ROW EXECUTE FUNCTION cms_update_search_vector();

-- Update existing content
UPDATE cms_content SET search_vector =
    setweight(to_tsvector('english', COALESCE(title, '')), 'A') ||
    setweight(to_tsvector('english', COALESCE(excerpt, '')), 'B') ||
    setweight(to_tsvector('english', COALESCE(content, '')), 'C') ||
    setweight(to_tsvector('english', COALESCE(meta_description, '')), 'D')
WHERE search_vector IS NULL;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 6. TRIGRAM INDEXES FOR FUZZY SEARCH
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Content title fuzzy search
CREATE INDEX IF NOT EXISTS idx_cms_content_title_trgm ON cms_content
    USING GIN (title gin_trgm_ops) WHERE deleted_at IS NULL;

-- Asset filename fuzzy search
CREATE INDEX IF NOT EXISTS idx_cms_assets_filename_trgm ON cms_assets
    USING GIN (filename gin_trgm_ops) WHERE deleted_at IS NULL;

-- Tag name fuzzy search
CREATE INDEX IF NOT EXISTS idx_cms_tags_name_trgm ON cms_tags
    USING GIN (name gin_trgm_ops);

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 7. FULL-TEXT SEARCH FUNCTION
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_search_content(
    p_query TEXT,
    p_content_types cms_content_type[] DEFAULT NULL,
    p_tag_ids UUID[] DEFAULT NULL,
    p_locale TEXT DEFAULT 'en',
    p_limit INTEGER DEFAULT 20,
    p_offset INTEGER DEFAULT 0,
    p_published_only BOOLEAN DEFAULT true
)
RETURNS TABLE (
    id UUID,
    content_type cms_content_type,
    slug VARCHAR,
    title VARCHAR,
    excerpt TEXT,
    featured_image_id UUID,
    author_id UUID,
    published_at TIMESTAMPTZ,
    rank REAL,
    headline TEXT
)
LANGUAGE plpgsql
STABLE
SECURITY DEFINER
AS $$
DECLARE
    v_tsquery tsquery;
BEGIN
    -- Parse the search query
    v_tsquery := websearch_to_tsquery('english', p_query);

    RETURN QUERY
    WITH ranked_results AS (
        SELECT
            c.id,
            c.content_type,
            c.slug,
            c.title,
            c.excerpt,
            c.featured_image_id,
            c.author_id,
            c.published_at,
            -- Rank calculation with weighted fields
            ts_rank_cd(c.search_vector, v_tsquery, 32) +
            -- Boost for title similarity
            COALESCE(similarity(c.title, p_query) * 0.5, 0) +
            -- Boost for recent content (within 30 days)
            CASE WHEN c.published_at > NOW() - INTERVAL '30 days' THEN 0.1 ELSE 0 END
            AS rank,
            -- Generate headline with highlighted matches
            ts_headline('english',
                COALESCE(c.excerpt, LEFT(c.content, 300)),
                v_tsquery,
                'MaxFragments=2, MinWords=15, MaxWords=35, StartSel=<mark>, StopSel=</mark>'
            ) AS headline
        FROM cms_content c
        WHERE
            c.deleted_at IS NULL
            AND c.locale = p_locale
            AND (NOT p_published_only OR c.status = 'published')
            AND (p_content_types IS NULL OR c.content_type = ANY(p_content_types))
            AND (
                c.search_vector @@ v_tsquery
                OR c.title ILIKE '%' || p_query || '%'
                OR similarity(c.title, p_query) > 0.3
            )
    )
    SELECT r.*
    FROM ranked_results r
    WHERE
        p_tag_ids IS NULL
        OR EXISTS (
            SELECT 1 FROM cms_content_tags ct
            WHERE ct.content_id = r.id AND ct.tag_id = ANY(p_tag_ids)
        )
    ORDER BY r.rank DESC, r.published_at DESC NULLS LAST
    LIMIT p_limit
    OFFSET p_offset;
END;
$$;

COMMENT ON FUNCTION cms_search_content IS 'Full-text search with ranking, fuzzy matching, and tag filtering';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 8. OPTIMIZED CONTENT RETRIEVAL FUNCTION
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_get_content_full(
    p_content_type cms_content_type,
    p_slug VARCHAR,
    p_locale VARCHAR DEFAULT 'en',
    p_include_drafts BOOLEAN DEFAULT false
)
RETURNS JSONB
LANGUAGE plpgsql
STABLE
SECURITY DEFINER
AS $$
DECLARE
    v_result JSONB;
    v_content_id UUID;
BEGIN
    -- Get content ID first
    SELECT c.id INTO v_content_id
    FROM cms_content c
    WHERE c.content_type = p_content_type
    AND c.slug = p_slug
    AND c.locale = p_locale
    AND c.deleted_at IS NULL
    AND (p_include_drafts OR c.status = 'published');

    IF v_content_id IS NULL THEN
        RETURN NULL;
    END IF;

    -- Build full content response with relations
    SELECT jsonb_build_object(
        'id', c.id,
        'contentType', c.content_type,
        'slug', c.slug,
        'locale', c.locale,
        'title', c.title,
        'subtitle', c.subtitle,
        'excerpt', c.excerpt,
        'content', c.content,
        'contentMode', c.content_mode,
        'contentBlocks', c.content_blocks,
        'status', c.status,
        'version', c.version,
        'template', c.template,
        'customFields', c.custom_fields,

        -- SEO fields
        'seo', jsonb_build_object(
            'title', COALESCE(c.meta_title, c.title),
            'description', c.meta_description,
            'keywords', c.meta_keywords,
            'canonicalUrl', c.canonical_url,
            'robots', c.robots_directives,
            'structuredData', c.structured_data
        ),

        -- Featured image
        'featuredImage', CASE WHEN fi.id IS NOT NULL THEN jsonb_build_object(
            'id', fi.id,
            'url', fi.cdn_url,
            'width', fi.width,
            'height', fi.height,
            'alt', COALESCE(fi.alt_text, c.title),
            'blurhash', fi.blurhash,
            'variants', fi.variants
        ) END,

        -- OG image (fallback to featured)
        'ogImage', CASE WHEN oi.id IS NOT NULL THEN jsonb_build_object(
            'id', oi.id,
            'url', oi.cdn_url,
            'width', oi.width,
            'height', oi.height
        ) ELSE CASE WHEN fi.id IS NOT NULL THEN jsonb_build_object(
            'id', fi.id,
            'url', fi.cdn_url,
            'width', fi.width,
            'height', fi.height
        ) END END,

        -- Author
        'author', CASE WHEN u.id IS NOT NULL THEN jsonb_build_object(
            'id', u.id,
            'displayName', u.display_name,
            'avatarUrl', u.avatar_url,
            'bio', u.bio
        ) END,

        -- Tags
        'tags', COALESCE((
            SELECT jsonb_agg(jsonb_build_object(
                'id', t.id,
                'name', t.name,
                'slug', t.slug,
                'color', t.color
            ) ORDER BY ct.sort_order)
            FROM cms_content_tags ct
            JOIN cms_tags t ON ct.tag_id = t.id
            WHERE ct.content_id = c.id
        ), '[]'::jsonb),

        -- Related content
        'relatedContent', COALESCE((
            SELECT jsonb_agg(jsonb_build_object(
                'relationType', r.relation_type,
                'sortOrder', r.sort_order,
                'content', jsonb_build_object(
                    'id', rc.id,
                    'contentType', rc.content_type,
                    'slug', rc.slug,
                    'title', rc.title,
                    'excerpt', rc.excerpt,
                    'featuredImage', CASE WHEN rfi.id IS NOT NULL THEN jsonb_build_object(
                        'url', rfi.cdn_url,
                        'width', rfi.width,
                        'height', rfi.height,
                        'blurhash', rfi.blurhash
                    ) END
                )
            ) ORDER BY r.sort_order)
            FROM cms_content_relations r
            JOIN cms_content rc ON r.target_id = rc.id
            LEFT JOIN cms_assets rfi ON rc.featured_image_id = rfi.id
            WHERE r.source_id = c.id
            AND rc.deleted_at IS NULL
            AND (p_include_drafts OR rc.status = 'published')
        ), '[]'::jsonb),

        -- Timestamps
        'publishedAt', c.published_at,
        'createdAt', c.created_at,
        'updatedAt', c.updated_at
    ) INTO v_result
    FROM cms_content c
    LEFT JOIN cms_assets fi ON c.featured_image_id = fi.id
    LEFT JOIN cms_assets oi ON c.og_image_id = oi.id
    LEFT JOIN cms_users u ON c.author_id = u.id
    WHERE c.id = v_content_id;

    RETURN v_result;
END;
$$;

COMMENT ON FUNCTION cms_get_content_full IS 'Retrieve content with all relations in a single optimized query';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 9. ATOMIC PUBLISH FUNCTION
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_publish_content(
    p_content_id UUID,
    p_user_id UUID
)
RETURNS JSONB
LANGUAGE plpgsql
AS $$
DECLARE
    v_content RECORD;
    v_old_status cms_content_status;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    -- Lock the content row for update
    SELECT * INTO v_content
    FROM cms_content
    WHERE id = p_content_id
    AND deleted_at IS NULL
    FOR UPDATE;

    IF v_content IS NULL THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', 'Content not found or deleted'
        );
    END IF;

    v_old_status := v_content.status;

    -- Validate transition
    IF v_old_status NOT IN ('draft', 'in_review', 'approved', 'scheduled') THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', format('Cannot publish content with status: %s', v_old_status)
        );
    END IF;

    -- Update to published
    UPDATE cms_content
    SET
        status = 'published',
        published_at = COALESCE(published_at, v_now),
        updated_at = v_now,
        updated_by = p_user_id,
        version = version + 1
    WHERE id = p_content_id;

    -- Log workflow transition
    INSERT INTO cms_workflow_log (
        content_id, from_status, to_status, transitioned_by, created_at
    ) VALUES (
        p_content_id, v_old_status, 'published', p_user_id, v_now
    );

    -- Create revision
    PERFORM cms_create_revision(
        p_content_id,
        (SELECT row_to_json(c)::jsonb FROM cms_content c WHERE c.id = p_content_id),
        p_user_id,
        'Published',
        ARRAY['status']
    );

    -- Log to audit
    INSERT INTO cms_audit_log (
        action, entity_type, entity_id, user_id,
        old_values, new_values, created_at
    ) VALUES (
        'content.published', 'cms_content', p_content_id, p_user_id,
        jsonb_build_object('status', v_old_status),
        jsonb_build_object('status', 'published', 'publishedAt', v_now),
        v_now
    );

    RETURN jsonb_build_object(
        'success', true,
        'contentId', p_content_id,
        'publishedAt', v_now,
        'previousStatus', v_old_status
    );
END;
$$;

COMMENT ON FUNCTION cms_publish_content IS 'Atomically publish content with workflow logging and audit trail';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 10. UNPUBLISH FUNCTION
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_unpublish_content(
    p_content_id UUID,
    p_user_id UUID,
    p_target_status cms_content_status DEFAULT 'draft'
)
RETURNS JSONB
LANGUAGE plpgsql
AS $$
DECLARE
    v_content RECORD;
    v_old_status cms_content_status;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    -- Lock the content row
    SELECT * INTO v_content
    FROM cms_content
    WHERE id = p_content_id
    AND deleted_at IS NULL
    FOR UPDATE;

    IF v_content IS NULL THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', 'Content not found or deleted'
        );
    END IF;

    v_old_status := v_content.status;

    IF v_old_status != 'published' THEN
        RETURN jsonb_build_object(
            'success', false,
            'error', 'Content is not currently published'
        );
    END IF;

    -- Update status
    UPDATE cms_content
    SET
        status = p_target_status,
        updated_at = v_now,
        updated_by = p_user_id,
        version = version + 1
    WHERE id = p_content_id;

    -- Log workflow transition
    INSERT INTO cms_workflow_log (
        content_id, from_status, to_status, transitioned_by, created_at
    ) VALUES (
        p_content_id, v_old_status, p_target_status, p_user_id, v_now
    );

    -- Log to audit
    INSERT INTO cms_audit_log (
        action, entity_type, entity_id, user_id,
        old_values, new_values, created_at
    ) VALUES (
        'content.unpublished', 'cms_content', p_content_id, p_user_id,
        jsonb_build_object('status', v_old_status),
        jsonb_build_object('status', p_target_status),
        v_now
    );

    RETURN jsonb_build_object(
        'success', true,
        'contentId', p_content_id,
        'newStatus', p_target_status
    );
END;
$$;

COMMENT ON FUNCTION cms_unpublish_content IS 'Atomically unpublish content with workflow logging';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 11. GENERATE STRUCTURED DATA FOR SEO
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_generate_structured_data(
    p_content_id UUID
)
RETURNS JSONB
LANGUAGE plpgsql
STABLE
AS $$
DECLARE
    v_content RECORD;
    v_author RECORD;
    v_site RECORD;
    v_image RECORD;
    v_result JSONB;
    v_base_url TEXT := 'https://revolution-trading-pros.com';
BEGIN
    -- Get content
    SELECT * INTO v_content FROM cms_content WHERE id = p_content_id;
    IF v_content IS NULL THEN RETURN NULL; END IF;

    -- Get author
    SELECT * INTO v_author FROM cms_users WHERE id = v_content.author_id;

    -- Get site settings
    SELECT * INTO v_site FROM cms_site_settings LIMIT 1;

    -- Get featured image
    SELECT * INTO v_image FROM cms_assets WHERE id = v_content.featured_image_id;

    -- Build base structured data
    CASE v_content.content_type
        WHEN 'blog_post' THEN
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'BlogPosting',
                'headline', v_content.title,
                'description', COALESCE(v_content.meta_description, v_content.excerpt),
                'datePublished', v_content.published_at,
                'dateModified', v_content.updated_at,
                'mainEntityOfPage', jsonb_build_object(
                    '@type', 'WebPage',
                    '@id', v_base_url || '/blog/' || v_content.slug
                ),
                'publisher', jsonb_build_object(
                    '@type', 'Organization',
                    'name', COALESCE(v_site.site_name, 'Revolution Trading Pros'),
                    'url', v_base_url
                )
            );

        WHEN 'course' THEN
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'Course',
                'name', v_content.title,
                'description', COALESCE(v_content.meta_description, v_content.excerpt),
                'provider', jsonb_build_object(
                    '@type', 'Organization',
                    'name', COALESCE(v_site.site_name, 'Revolution Trading Pros'),
                    'url', v_base_url
                ),
                'dateCreated', v_content.created_at,
                'dateModified', v_content.updated_at
            );

        WHEN 'faq' THEN
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'FAQPage',
                'mainEntity', COALESCE(v_content.custom_fields->'faqs', '[]'::jsonb)
            );

        ELSE
            v_result := jsonb_build_object(
                '@context', 'https://schema.org',
                '@type', 'WebPage',
                'name', v_content.title,
                'description', COALESCE(v_content.meta_description, v_content.excerpt),
                'datePublished', v_content.published_at,
                'dateModified', v_content.updated_at
            );
    END CASE;

    -- Add author if exists
    IF v_author IS NOT NULL THEN
        v_result := v_result || jsonb_build_object(
            'author', jsonb_build_object(
                '@type', 'Person',
                'name', v_author.display_name,
                'url', v_base_url || '/authors/' || v_author.id
            )
        );
    END IF;

    -- Add image if exists
    IF v_image IS NOT NULL THEN
        v_result := v_result || jsonb_build_object(
            'image', jsonb_build_object(
                '@type', 'ImageObject',
                'url', v_image.cdn_url,
                'width', v_image.width,
                'height', v_image.height
            )
        );
    END IF;

    RETURN v_result;
END;
$$;

COMMENT ON FUNCTION cms_generate_structured_data IS 'Generate JSON-LD structured data for SEO based on content type';

-- Trigger to auto-generate structured data on publish
CREATE OR REPLACE FUNCTION cms_auto_generate_structured_data()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.status = 'published' AND NEW.structured_data IS NULL THEN
        NEW.structured_data := cms_generate_structured_data(NEW.id);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_content_structured_data_trigger ON cms_content;
CREATE TRIGGER cms_content_structured_data_trigger
    BEFORE UPDATE OF status ON cms_content
    FOR EACH ROW
    WHEN (NEW.status = 'published' AND OLD.status != 'published')
    EXECUTE FUNCTION cms_auto_generate_structured_data();

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 12. PROCESS SCHEDULED JOBS FUNCTION
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_process_scheduled_jobs(
    p_batch_size INTEGER DEFAULT 100
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_job RECORD;
    v_count INTEGER := 0;
    v_now TIMESTAMPTZ := NOW();
BEGIN
    FOR v_job IN
        SELECT * FROM cms_scheduled_jobs
        WHERE status = 'pending'
        AND scheduled_at <= v_now
        AND (locked_at IS NULL OR locked_at < v_now - INTERVAL '5 minutes')
        ORDER BY scheduled_at
        LIMIT p_batch_size
        FOR UPDATE SKIP LOCKED
    LOOP
        -- Lock the job
        UPDATE cms_scheduled_jobs
        SET
            status = 'running',
            locked_by = pg_backend_pid()::text,
            locked_at = v_now,
            started_at = v_now,
            attempts = attempts + 1
        WHERE id = v_job.id;

        BEGIN
            -- Process based on job type
            CASE v_job.job_type
                WHEN 'publish' THEN
                    PERFORM cms_publish_content(v_job.content_id, v_job.created_by);

                WHEN 'unpublish' THEN
                    PERFORM cms_unpublish_content(v_job.content_id, v_job.created_by);

                WHEN 'archive' THEN
                    UPDATE cms_content
                    SET status = 'archived', updated_by = v_job.created_by
                    WHERE id = v_job.content_id;

                ELSE
                    RAISE EXCEPTION 'Unknown job type: %', v_job.job_type;
            END CASE;

            -- Mark as completed
            UPDATE cms_scheduled_jobs
            SET
                status = 'completed',
                completed_at = NOW(),
                result = jsonb_build_object('success', true),
                locked_by = NULL,
                locked_at = NULL
            WHERE id = v_job.id;

            v_count := v_count + 1;

        EXCEPTION WHEN OTHERS THEN
            -- Mark as failed or retry
            UPDATE cms_scheduled_jobs
            SET
                status = CASE
                    WHEN attempts >= max_attempts THEN 'failed'
                    ELSE 'pending'
                END,
                error_message = SQLERRM,
                locked_by = NULL,
                locked_at = NULL
            WHERE id = v_job.id;
        END;
    END LOOP;

    RETURN v_count;
END;
$$;

COMMENT ON FUNCTION cms_process_scheduled_jobs IS 'Process pending scheduled jobs with locking and retry logic';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 13. CONTENT TYPE STATISTICS FUNCTION
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_get_content_stats()
RETURNS JSONB
LANGUAGE plpgsql
STABLE
AS $$
DECLARE
    v_result JSONB;
BEGIN
    SELECT jsonb_build_object(
        'content', jsonb_build_object(
            'total', COUNT(*) FILTER (WHERE deleted_at IS NULL),
            'published', COUNT(*) FILTER (WHERE status = 'published' AND deleted_at IS NULL),
            'draft', COUNT(*) FILTER (WHERE status = 'draft' AND deleted_at IS NULL),
            'inReview', COUNT(*) FILTER (WHERE status = 'in_review' AND deleted_at IS NULL),
            'scheduled', COUNT(*) FILTER (WHERE status = 'scheduled' AND deleted_at IS NULL),
            'archived', COUNT(*) FILTER (WHERE status = 'archived' AND deleted_at IS NULL),
            'byType', (
                SELECT jsonb_object_agg(content_type, cnt)
                FROM (
                    SELECT content_type, COUNT(*) as cnt
                    FROM cms_content
                    WHERE deleted_at IS NULL
                    GROUP BY content_type
                ) t
            )
        ),
        'assets', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_assets WHERE deleted_at IS NULL),
            'images', (SELECT COUNT(*) FROM cms_assets WHERE asset_type = 'image' AND deleted_at IS NULL),
            'videos', (SELECT COUNT(*) FROM cms_assets WHERE asset_type = 'video' AND deleted_at IS NULL),
            'documents', (SELECT COUNT(*) FROM cms_assets WHERE asset_type = 'document' AND deleted_at IS NULL),
            'totalSize', (SELECT COALESCE(SUM(file_size), 0) FROM cms_assets WHERE deleted_at IS NULL)
        ),
        'tags', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_tags)
        ),
        'menus', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_navigation_menus),
            'active', (SELECT COUNT(*) FROM cms_navigation_menus WHERE is_active = true)
        ),
        'redirects', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_redirects),
            'active', (SELECT COUNT(*) FROM cms_redirects WHERE is_active = true)
        ),
        'comments', jsonb_build_object(
            'total', (SELECT COUNT(*) FROM cms_comments WHERE deleted_at IS NULL),
            'unresolved', (SELECT COUNT(*) FROM cms_comments WHERE is_resolved = false AND deleted_at IS NULL)
        ),
        'scheduledJobs', jsonb_build_object(
            'pending', (SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'pending'),
            'running', (SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'running'),
            'failed', (SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'failed')
        )
    ) INTO v_result
    FROM cms_content;

    RETURN v_result;
END;
$$;

COMMENT ON FUNCTION cms_get_content_stats IS 'Get comprehensive CMS statistics for dashboard';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 14. ADDITIONAL INDEXES FOR PERFORMANCE
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Content by status and type (common dashboard query)
CREATE INDEX IF NOT EXISTS idx_cms_content_dashboard
    ON cms_content(content_type, status, updated_at DESC)
    WHERE deleted_at IS NULL;

-- Recent published content (common public API query)
CREATE INDEX IF NOT EXISTS idx_cms_content_recent_published
    ON cms_content(published_at DESC)
    WHERE status = 'published' AND deleted_at IS NULL;

-- Scheduled content for job processing
CREATE INDEX IF NOT EXISTS idx_cms_content_scheduled
    ON cms_content(scheduled_publish_at)
    WHERE scheduled_publish_at IS NOT NULL AND status = 'scheduled' AND deleted_at IS NULL;

-- Asset usage for cleanup jobs
CREATE INDEX IF NOT EXISTS idx_cms_assets_unused
    ON cms_assets(usage_count, created_at)
    WHERE usage_count = 0 AND deleted_at IS NULL;

-- Comments by content (common editor query)
CREATE INDEX IF NOT EXISTS idx_cms_comments_by_content
    ON cms_comments(content_id, created_at DESC)
    WHERE deleted_at IS NULL;

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 15. ROW LEVEL SECURITY (RLS) POLICIES
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Enable RLS on main tables
ALTER TABLE cms_content ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_assets ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_revisions ENABLE ROW LEVEL SECURITY;
ALTER TABLE cms_comments ENABLE ROW LEVEL SECURITY;

-- Helper function to get current CMS user role
CREATE OR REPLACE FUNCTION cms_current_user_role()
RETURNS cms_user_role AS $$
DECLARE
    v_role cms_user_role;
BEGIN
    -- This will be set by the application via SET LOCAL
    v_role := current_setting('cms.user_role', true)::cms_user_role;
    RETURN COALESCE(v_role, 'viewer'::cms_user_role);
EXCEPTION WHEN OTHERS THEN
    RETURN 'viewer'::cms_user_role;
END;
$$ LANGUAGE plpgsql STABLE;

-- Helper function to get current CMS user ID
CREATE OR REPLACE FUNCTION cms_current_user_id()
RETURNS UUID AS $$
BEGIN
    RETURN current_setting('cms.user_id', true)::uuid;
EXCEPTION WHEN OTHERS THEN
    RETURN NULL;
END;
$$ LANGUAGE plpgsql STABLE;

-- Policy: Super Admin and Marketing Manager have full access
CREATE POLICY cms_content_admin_access ON cms_content
    FOR ALL
    USING (cms_current_user_role() IN ('super_admin', 'marketing_manager'))
    WITH CHECK (cms_current_user_role() IN ('super_admin', 'marketing_manager'));

-- Policy: Content Editors can see all, edit own drafts
CREATE POLICY cms_content_editor_read ON cms_content
    FOR SELECT
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'weekly_editor', 'developer', 'viewer')
        OR (status = 'published' AND deleted_at IS NULL)
    );

CREATE POLICY cms_content_editor_insert ON cms_content
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'weekly_editor')
    );

CREATE POLICY cms_content_editor_update ON cms_content
    FOR UPDATE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager')
        OR (
            cms_current_user_role() IN ('content_editor', 'weekly_editor')
            AND (created_by = cms_current_user_id() OR author_id = cms_current_user_id())
            AND status IN ('draft', 'in_review')
        )
    );

-- Policy: Weekly editors can only edit weekly watchlist content
CREATE POLICY cms_content_weekly_editor ON cms_content
    FOR ALL
    USING (
        cms_current_user_role() = 'weekly_editor'
        AND content_type = 'weekly_watchlist'
    )
    WITH CHECK (
        cms_current_user_role() = 'weekly_editor'
        AND content_type = 'weekly_watchlist'
    );

-- Public read access for published content
CREATE POLICY cms_content_public_read ON cms_content
    FOR SELECT
    USING (
        status = 'published'
        AND deleted_at IS NULL
        AND (scheduled_publish_at IS NULL OR scheduled_publish_at <= NOW())
        AND (scheduled_unpublish_at IS NULL OR scheduled_unpublish_at > NOW())
    );

-- Asset policies
CREATE POLICY cms_assets_read ON cms_assets
    FOR SELECT
    USING (
        deleted_at IS NULL
        OR cms_current_user_role() IN ('super_admin', 'marketing_manager')
    );

CREATE POLICY cms_assets_write ON cms_assets
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'weekly_editor')
    );

CREATE POLICY cms_assets_update ON cms_assets
    FOR UPDATE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor')
    );

CREATE POLICY cms_assets_delete ON cms_assets
    FOR DELETE
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager')
    );

-- Revision policies (read-only for non-admins)
CREATE POLICY cms_revisions_read ON cms_revisions
    FOR SELECT
    USING (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'developer')
    );

CREATE POLICY cms_revisions_insert ON cms_revisions
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'weekly_editor')
    );

-- Comment policies
CREATE POLICY cms_comments_read ON cms_comments
    FOR SELECT
    USING (
        deleted_at IS NULL
        AND cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'weekly_editor', 'developer')
    );

CREATE POLICY cms_comments_write ON cms_comments
    FOR INSERT
    WITH CHECK (
        cms_current_user_role() IN ('super_admin', 'marketing_manager', 'content_editor', 'weekly_editor')
    );

CREATE POLICY cms_comments_update ON cms_comments
    FOR UPDATE
    USING (
        created_by = cms_current_user_id()
        OR cms_current_user_role() IN ('super_admin', 'marketing_manager')
    );

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 16. DELIVERY API HELPER FUNCTIONS
-- ─────────────────────────────────────────────────────────────────────────────────────────

-- Get recent published content by type
CREATE OR REPLACE FUNCTION cms_get_recent_content(
    p_content_type cms_content_type,
    p_limit INTEGER DEFAULT 10,
    p_offset INTEGER DEFAULT 0,
    p_locale VARCHAR DEFAULT 'en'
)
RETURNS JSONB
LANGUAGE plpgsql
STABLE
AS $$
BEGIN
    RETURN (
        SELECT jsonb_build_object(
            'data', COALESCE(jsonb_agg(item), '[]'::jsonb),
            'meta', jsonb_build_object(
                'total', (
                    SELECT COUNT(*) FROM cms_content
                    WHERE content_type = p_content_type
                    AND status = 'published'
                    AND deleted_at IS NULL
                    AND locale = p_locale
                ),
                'limit', p_limit,
                'offset', p_offset
            )
        )
        FROM (
            SELECT jsonb_build_object(
                'id', c.id,
                'slug', c.slug,
                'title', c.title,
                'excerpt', c.excerpt,
                'featuredImage', CASE WHEN a.id IS NOT NULL THEN jsonb_build_object(
                    'url', a.cdn_url,
                    'width', a.width,
                    'height', a.height,
                    'blurhash', a.blurhash
                ) END,
                'author', CASE WHEN u.id IS NOT NULL THEN jsonb_build_object(
                    'displayName', u.display_name,
                    'avatarUrl', u.avatar_url
                ) END,
                'publishedAt', c.published_at
            ) AS item
            FROM cms_content c
            LEFT JOIN cms_assets a ON c.featured_image_id = a.id
            LEFT JOIN cms_users u ON c.author_id = u.id
            WHERE c.content_type = p_content_type
            AND c.status = 'published'
            AND c.deleted_at IS NULL
            AND c.locale = p_locale
            ORDER BY c.published_at DESC
            LIMIT p_limit
            OFFSET p_offset
        ) subq
    );
END;
$$;

COMMENT ON FUNCTION cms_get_recent_content IS 'Get recent published content for delivery API';

-- Get all published content slugs for static generation
CREATE OR REPLACE FUNCTION cms_get_all_slugs(
    p_content_type cms_content_type DEFAULT NULL,
    p_locale VARCHAR DEFAULT NULL
)
RETURNS TABLE (
    content_type cms_content_type,
    slug VARCHAR,
    locale VARCHAR,
    updated_at TIMESTAMPTZ
)
LANGUAGE plpgsql
STABLE
AS $$
BEGIN
    RETURN QUERY
    SELECT c.content_type, c.slug, c.locale, c.updated_at
    FROM cms_content c
    WHERE c.status = 'published'
    AND c.deleted_at IS NULL
    AND (p_content_type IS NULL OR c.content_type = p_content_type)
    AND (p_locale IS NULL OR c.locale = p_locale)
    ORDER BY c.updated_at DESC;
END;
$$;

COMMENT ON FUNCTION cms_get_all_slugs IS 'Get all published content slugs for static site generation';

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- 17. WEBHOOK DELIVERY HELPER
-- ─────────────────────────────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION cms_queue_webhook_delivery(
    p_event_type TEXT,
    p_content_id UUID DEFAULT NULL,
    p_payload JSONB DEFAULT '{}'::jsonb
)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
    v_webhook RECORD;
    v_count INTEGER := 0;
BEGIN
    FOR v_webhook IN
        SELECT * FROM cms_webhooks
        WHERE is_active = true
        AND p_event_type = ANY(events)
        AND (content_types IS NULL OR (
            SELECT content_type FROM cms_content WHERE id = p_content_id
        ) = ANY(content_types))
    LOOP
        INSERT INTO cms_webhook_deliveries (
            webhook_id, event_type, content_id, payload, status
        ) VALUES (
            v_webhook.id, p_event_type, p_content_id, p_payload, 'pending'
        );
        v_count := v_count + 1;
    END LOOP;

    RETURN v_count;
END;
$$;

COMMENT ON FUNCTION cms_queue_webhook_delivery IS 'Queue webhook deliveries for matching webhooks';

-- Trigger to auto-queue webhooks on content events
CREATE OR REPLACE FUNCTION cms_content_webhook_trigger()
RETURNS TRIGGER AS $$
DECLARE
    v_event TEXT;
    v_payload JSONB;
BEGIN
    -- Determine event type
    IF TG_OP = 'INSERT' THEN
        v_event := 'content.created';
    ELSIF TG_OP = 'UPDATE' THEN
        IF NEW.status = 'published' AND OLD.status != 'published' THEN
            v_event := 'content.published';
        ELSIF NEW.status != 'published' AND OLD.status = 'published' THEN
            v_event := 'content.unpublished';
        ELSIF NEW.deleted_at IS NOT NULL AND OLD.deleted_at IS NULL THEN
            v_event := 'content.deleted';
        ELSE
            v_event := 'content.updated';
        END IF;
    END IF;

    -- Build payload
    v_payload := jsonb_build_object(
        'contentId', NEW.id,
        'contentType', NEW.content_type,
        'slug', NEW.slug,
        'title', NEW.title,
        'status', NEW.status,
        'timestamp', NOW()
    );

    -- Queue webhooks
    PERFORM cms_queue_webhook_delivery(v_event, NEW.id, v_payload);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS cms_content_webhook_trigger ON cms_content;
CREATE TRIGGER cms_content_webhook_trigger
    AFTER INSERT OR UPDATE ON cms_content
    FOR EACH ROW
    EXECUTE FUNCTION cms_content_webhook_trigger();

-- ─────────────────────────────────────────────────────────────────────────────────────────
-- MIGRATION COMPLETE
-- ─────────────────────────────────────────────────────────────────────────────────────────

COMMENT ON EXTENSION pg_trgm IS 'Trigram similarity for fuzzy text matching';
COMMENT ON EXTENSION btree_gin IS 'GIN support for scalar types';
COMMENT ON EXTENSION unaccent IS 'Remove accents for search normalization';
