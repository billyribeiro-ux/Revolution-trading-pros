-- ============================================================================
-- POPUPS SYSTEM - Complete Schema
-- Revolution Trading Pros - Apple ICT 7+ Principal Engineer Grade
-- February 2026
-- ============================================================================
-- This migration creates the complete popup/modal management system with:
-- - Full CRUD support for popups
-- - Multiple trigger types (time, scroll, exit intent, click)
-- - Frequency controls and device targeting
-- - A/B testing support
-- - Analytics tracking (impressions, conversions)
-- - Scheduling (start/end dates)
-- - Form integration
-- ============================================================================

-- ============================================================================
-- POPUPS TABLE - Main popup entities
-- ============================================================================
CREATE TABLE IF NOT EXISTS popups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL DEFAULT 'timed',
    status VARCHAR(20) NOT NULL DEFAULT 'draft',
    priority INTEGER NOT NULL DEFAULT 10,

    -- Content
    title TEXT,
    content TEXT,
    cta_text VARCHAR(255),
    cta_url TEXT,
    cta_new_tab BOOLEAN NOT NULL DEFAULT FALSE,

    -- Display settings
    position VARCHAR(20) NOT NULL DEFAULT 'center',
    size VARCHAR(10) NOT NULL DEFAULT 'md',
    animation VARCHAR(20) NOT NULL DEFAULT 'zoom',
    show_close_button BOOLEAN NOT NULL DEFAULT TRUE,
    close_on_overlay_click BOOLEAN NOT NULL DEFAULT TRUE,
    close_on_escape BOOLEAN NOT NULL DEFAULT TRUE,
    auto_close_after INTEGER,

    -- Form integration
    has_form BOOLEAN NOT NULL DEFAULT FALSE,
    form_id INTEGER REFERENCES forms(id) ON DELETE SET NULL,

    -- Rules stored as JSONB
    trigger_rules JSONB NOT NULL DEFAULT '{}',
    frequency_rules JSONB NOT NULL DEFAULT '{"frequency": "once_per_session"}',
    display_rules JSONB NOT NULL DEFAULT '{"devices": ["desktop", "tablet", "mobile"]}',
    design JSONB NOT NULL DEFAULT '{
        "background_color": "#ffffff",
        "title_color": "#1f2937",
        "text_color": "#4b5563",
        "button_color": "#3b82f6",
        "button_text_color": "#ffffff",
        "button_border_radius": 8,
        "overlay_color": "#000000",
        "overlay_opacity": 50,
        "border_radius": 12
    }',

    -- A/B Testing
    ab_test_id INTEGER,
    variant_name VARCHAR(50),
    traffic_allocation INTEGER,

    -- Scheduling
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,

    -- Analytics
    total_views BIGINT NOT NULL DEFAULT 0,
    total_conversions BIGINT NOT NULL DEFAULT 0,
    conversion_rate DOUBLE PRECISION NOT NULL DEFAULT 0.0,

    -- Metadata
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- POPUP A/B TESTS TABLE - A/B test configurations
-- ============================================================================
CREATE TABLE IF NOT EXISTS popup_ab_tests (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'draft',
    base_popup_id INTEGER NOT NULL REFERENCES popups(id) ON DELETE CASCADE,
    winner_popup_id INTEGER REFERENCES popups(id) ON DELETE SET NULL,
    confidence_threshold DOUBLE PRECISION NOT NULL DEFAULT 0.95,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add foreign key for ab_test_id after popup_ab_tests exists
ALTER TABLE popups
ADD CONSTRAINT fk_popup_ab_test
FOREIGN KEY (ab_test_id) REFERENCES popup_ab_tests(id) ON DELETE SET NULL;

-- ============================================================================
-- POPUP EVENTS TABLE - Analytics tracking
-- ============================================================================
CREATE TABLE IF NOT EXISTS popup_events (
    id BIGSERIAL PRIMARY KEY,
    popup_id INTEGER NOT NULL REFERENCES popups(id) ON DELETE CASCADE,
    event_type VARCHAR(20) NOT NULL,
    session_id VARCHAR(100),
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    page_url TEXT,
    device_type VARCHAR(20),
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- POPUP FORM SUBMISSIONS TABLE - Form submission tracking
-- ============================================================================
CREATE TABLE IF NOT EXISTS popup_form_submissions (
    id BIGSERIAL PRIMARY KEY,
    popup_id INTEGER NOT NULL REFERENCES popups(id) ON DELETE CASCADE,
    form_id INTEGER REFERENCES forms(id) ON DELETE SET NULL,
    data JSONB NOT NULL,
    session_id VARCHAR(100),
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- INDEXES - Performance optimization
-- ============================================================================

-- Popup status and scheduling indexes
CREATE INDEX IF NOT EXISTS idx_popups_status ON popups(status);
CREATE INDEX IF NOT EXISTS idx_popups_type ON popups(type);
CREATE INDEX IF NOT EXISTS idx_popups_priority ON popups(priority DESC);
CREATE INDEX IF NOT EXISTS idx_popups_schedule ON popups(start_date, end_date) WHERE status = 'published';
CREATE INDEX IF NOT EXISTS idx_popups_ab_test ON popups(ab_test_id) WHERE ab_test_id IS NOT NULL;

-- Event analytics indexes
CREATE INDEX IF NOT EXISTS idx_popup_events_popup ON popup_events(popup_id);
CREATE INDEX IF NOT EXISTS idx_popup_events_type ON popup_events(event_type);
CREATE INDEX IF NOT EXISTS idx_popup_events_created ON popup_events(created_at);
CREATE INDEX IF NOT EXISTS idx_popup_events_session ON popup_events(session_id) WHERE session_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_popup_events_device ON popup_events(device_type) WHERE device_type IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_popup_events_analytics ON popup_events(popup_id, event_type, created_at);

-- Form submissions indexes
CREATE INDEX IF NOT EXISTS idx_popup_submissions_popup ON popup_form_submissions(popup_id);
CREATE INDEX IF NOT EXISTS idx_popup_submissions_form ON popup_form_submissions(form_id) WHERE form_id IS NOT NULL;

-- ============================================================================
-- FUNCTIONS - Analytics helpers
-- ============================================================================

-- Function to update popup conversion rate
CREATE OR REPLACE FUNCTION update_popup_conversion_rate()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE popups
    SET conversion_rate = CASE
        WHEN total_views > 0 THEN (total_conversions::float / total_views::float) * 100
        ELSE 0
    END
    WHERE id = NEW.popup_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to update conversion rate on event insert
DROP TRIGGER IF EXISTS trg_popup_conversion_rate ON popup_events;
CREATE TRIGGER trg_popup_conversion_rate
AFTER INSERT ON popup_events
FOR EACH ROW
EXECUTE FUNCTION update_popup_conversion_rate();

-- Function to get popup analytics summary
CREATE OR REPLACE FUNCTION get_popup_analytics(p_popup_id INTEGER, p_days INTEGER DEFAULT 30)
RETURNS TABLE (
    total_views BIGINT,
    total_conversions BIGINT,
    conversion_rate DOUBLE PRECISION,
    daily_avg_views DOUBLE PRECISION,
    daily_avg_conversions DOUBLE PRECISION,
    top_device VARCHAR(20),
    top_page TEXT
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        COUNT(*) FILTER (WHERE event_type = 'view') as total_views,
        COUNT(*) FILTER (WHERE event_type = 'conversion') as total_conversions,
        CASE
            WHEN COUNT(*) FILTER (WHERE event_type = 'view') > 0
            THEN (COUNT(*) FILTER (WHERE event_type = 'conversion')::float /
                  COUNT(*) FILTER (WHERE event_type = 'view')::float) * 100
            ELSE 0
        END as conversion_rate,
        COUNT(*) FILTER (WHERE event_type = 'view')::float / p_days as daily_avg_views,
        COUNT(*) FILTER (WHERE event_type = 'conversion')::float / p_days as daily_avg_conversions,
        (SELECT device_type FROM popup_events
         WHERE popup_id = p_popup_id AND device_type IS NOT NULL
         GROUP BY device_type ORDER BY COUNT(*) DESC LIMIT 1) as top_device,
        (SELECT page_url FROM popup_events
         WHERE popup_id = p_popup_id AND page_url IS NOT NULL
         GROUP BY page_url ORDER BY COUNT(*) DESC LIMIT 1) as top_page
    FROM popup_events
    WHERE popup_id = p_popup_id
      AND created_at >= NOW() - (p_days || ' days')::interval;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- DEFAULT POPUPS - Seed data
-- ============================================================================
INSERT INTO popups (name, type, status, title, content, cta_text, cta_url, position, size, animation, trigger_rules, frequency_rules)
VALUES
    (
        'Welcome Newsletter Signup',
        'timed',
        'draft',
        'Join Our Trading Community',
        '<p>Get exclusive trading insights, market analysis, and educational content delivered straight to your inbox.</p><p>Join over 10,000+ traders who trust Revolution Trading Pros.</p>',
        'Subscribe Now',
        '/subscribe',
        'center',
        'md',
        'zoom',
        '{"delay": 5000}',
        '{"frequency": "once"}'
    ),
    (
        'Exit Intent Offer',
        'exit_intent',
        'draft',
        'Wait! Before You Go...',
        '<p>Get 10% off your first month of Pro membership!</p><p>Use code: <strong>WELCOME10</strong></p>',
        'Claim Offer',
        '/pricing?coupon=WELCOME10',
        'center',
        'md',
        'bounce',
        '{"exit_intent": true}',
        '{"frequency": "once_per_session"}'
    )
ON CONFLICT DO NOTHING;

-- ============================================================================
-- DONE
-- ============================================================================
COMMENT ON TABLE popups IS 'Revolution Trading Pros - Complete popup/modal system with analytics and A/B testing';
COMMENT ON TABLE popup_ab_tests IS 'A/B test configurations for popup optimization';
COMMENT ON TABLE popup_events IS 'Analytics events tracking for popups (views, conversions, etc.)';
COMMENT ON TABLE popup_form_submissions IS 'Form submission data from popup forms';
