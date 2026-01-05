-- Weekly Watchlist Entries Table Migration
-- ═══════════════════════════════════════════════════════════════════════════
-- Apple ICT 11+ Principal Engineer Grade - January 2026
--
-- Supports the automated Weekly Watchlist system with:
-- - Multiple date versions (date switcher)
-- - Room targeting
-- - Video and spreadsheet URLs
-- - Auto-generated slugs

-- Main watchlist_entries table
CREATE TABLE IF NOT EXISTS watchlist_entries (
    id BIGSERIAL PRIMARY KEY,
    
    -- Core Data
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    subtitle VARCHAR(255),
    trader VARCHAR(255) NOT NULL,
    trader_image VARCHAR(500),
    
    -- Dates
    date_posted VARCHAR(100) NOT NULL,
    week_of DATE NOT NULL,
    
    -- Video Data
    video_url VARCHAR(500) NOT NULL,
    video_poster VARCHAR(500),
    video_title VARCHAR(255) NOT NULL,
    
    -- Spreadsheet Data
    spreadsheet_url VARCHAR(500) NOT NULL,
    
    -- Date Switcher (Multiple spreadsheet versions)
    -- Format: [{"date": "1/3/2026", "spreadsheetUrl": "https://..."}, ...]
    watchlist_dates JSONB,
    
    -- Content
    description TEXT,
    
    -- Publishing
    status VARCHAR(50) DEFAULT 'draft' CHECK (status IN ('published', 'draft', 'archived')),
    
    -- Room Targeting (which rooms can see this watchlist)
    -- Format: ["day-trading-room", "swing-trading-room", ...]
    rooms JSONB DEFAULT '[]'::jsonb,
    
    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_watchlist_entries_slug ON watchlist_entries(slug);
CREATE INDEX IF NOT EXISTS idx_watchlist_entries_week_of ON watchlist_entries(week_of DESC);
CREATE INDEX IF NOT EXISTS idx_watchlist_entries_status ON watchlist_entries(status);
CREATE INDEX IF NOT EXISTS idx_watchlist_entries_trader ON watchlist_entries(trader);
CREATE INDEX IF NOT EXISTS idx_watchlist_entries_rooms ON watchlist_entries USING GIN(rooms);
CREATE INDEX IF NOT EXISTS idx_watchlist_entries_search ON watchlist_entries USING GIN(to_tsvector('english', title || ' ' || trader || ' ' || COALESCE(description, '')));

-- Trigger for updated_at
CREATE OR REPLACE FUNCTION update_watchlist_entries_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_watchlist_entries_updated_at ON watchlist_entries;
CREATE TRIGGER trigger_watchlist_entries_updated_at
    BEFORE UPDATE ON watchlist_entries
    FOR EACH ROW
    EXECUTE FUNCTION update_watchlist_entries_updated_at();

-- Seed with example data
INSERT INTO watchlist_entries (
    slug, title, subtitle, trader, trader_image,
    date_posted, week_of,
    video_url, video_poster, video_title,
    spreadsheet_url, watchlist_dates,
    description, status, rooms
) VALUES (
    '01032026-melissa-beegle',
    'Weekly Watchlist with Melissa Beegle',
    'Week of January 3, 2026',
    'Melissa Beegle',
    'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
    'January 3, 2026',
    '2026-01-03',
    'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-MB-01032026.mp4',
    'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
    'Weekly Watchlist with Melissa Beegle',
    'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml',
    '[
        {"date": "1/3/2026", "spreadsheetUrl": "https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml"},
        {"date": "5/28/2025", "spreadsheetUrl": "https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml"},
        {"date": "3/9/2025", "spreadsheetUrl": "https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0rA7tUzJl2-au8DRWf486KZyPbbjeaVNp4fJ1ZyO0qPWLUkHia-TWEdDRCnPFPMJjFc-1V/pubhtml"}
    ]'::jsonb,
    'Week starting on January 3, 2026.',
    'published',
    '["day-trading-room", "swing-trading-room", "small-account-mentorship", "explosive-swings", "spx-profit-pulse", "high-octane-scanner"]'::jsonb
), (
    '12292025-david-starr',
    'Weekly Watchlist with David Starr',
    'Week of December 29, 2025',
    'David Starr',
    'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
    'December 29, 2025',
    '2025-12-29',
    'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-DS-12292025.mp4',
    'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
    'Weekly Watchlist with David Starr',
    'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml',
    NULL,
    'Week of December 29, 2025.',
    'published',
    '["day-trading-room", "swing-trading-room", "small-account-mentorship", "explosive-swings", "spx-profit-pulse", "high-octane-scanner"]'::jsonb
), (
    '12222025-tg-watkins',
    'Weekly Watchlist with TG Watkins',
    'Week of December 22, 2025',
    'TG Watkins',
    'https://cdn.simplertrading.com/2025/05/07135326/SimplerCentral_TG.jpg',
    'December 22, 2025',
    '2025-12-22',
    'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TG-12222025.mp4',
    'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
    'Weekly Watchlist with TG Watkins',
    'https://docs.google.com/spreadsheets/d/e/2PACX-1vSsdQCEUZLymwpLK8j35e5B6qjdRPz1k2tX8U2yL0z30EsEv06i-74m7V-cPgCyxZe528DA_3gdMUKy/pubhtml',
    NULL,
    'Week of December 22, 2025.',
    'published',
    '["day-trading-room", "swing-trading-room", "small-account-mentorship", "explosive-swings", "spx-profit-pulse", "high-octane-scanner"]'::jsonb
) ON CONFLICT (slug) DO NOTHING;
