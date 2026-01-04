-- Revolution Trading Pros - Trading Room Schedules
-- ICT 11+ Apple Principal Engineer Grade
-- January 2026
--
-- Per-room schedule management system with:
-- - Recurring weekly schedules
-- - Exception handling (holidays, cancellations)
-- - Audit trail
-- - Full CMS support

-- ═══════════════════════════════════════════════════════════════════════════
-- TRADING ROOM SCHEDULES
-- Each trading room/mentorship can have its own weekly schedule
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS trading_room_schedules (
    id BIGSERIAL PRIMARY KEY,
    
    -- Link to membership plan (trading room)
    plan_id BIGINT NOT NULL REFERENCES membership_plans(id) ON DELETE CASCADE,
    
    -- Event details
    title VARCHAR(255) NOT NULL,
    description TEXT,
    trader_name VARCHAR(255),
    trader_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    
    -- Weekly scheduling (recurring)
    day_of_week INTEGER NOT NULL, -- 0=Sunday, 1=Monday, 2=Tuesday, etc.
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    timezone VARCHAR(50) DEFAULT 'America/New_York',
    
    -- Recurrence settings
    is_recurring BOOLEAN DEFAULT true,
    effective_from DATE DEFAULT CURRENT_DATE,
    effective_until DATE, -- NULL = no end date
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    
    -- Room/meeting details
    room_url VARCHAR(500),
    room_type VARCHAR(50) DEFAULT 'live', -- live, recorded, hybrid
    
    -- Metadata for extensibility
    metadata JSONB DEFAULT '{}',
    
    -- Audit trail
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    updated_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT valid_day_of_week CHECK (day_of_week >= 0 AND day_of_week <= 6),
    CONSTRAINT valid_time_range CHECK (end_time > start_time)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_schedules_plan ON trading_room_schedules(plan_id);
CREATE INDEX IF NOT EXISTS idx_schedules_day ON trading_room_schedules(day_of_week);
CREATE INDEX IF NOT EXISTS idx_schedules_active ON trading_room_schedules(is_active);
CREATE INDEX IF NOT EXISTS idx_schedules_effective ON trading_room_schedules(effective_from, effective_until);

-- ═══════════════════════════════════════════════════════════════════════════
-- SCHEDULE EXCEPTIONS
-- Handle holidays, cancellations, rescheduled events
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS schedule_exceptions (
    id BIGSERIAL PRIMARY KEY,
    
    -- Link to schedule
    schedule_id BIGINT NOT NULL REFERENCES trading_room_schedules(id) ON DELETE CASCADE,
    
    -- Exception date
    exception_date DATE NOT NULL,
    
    -- Exception type
    exception_type VARCHAR(20) NOT NULL, -- cancelled, rescheduled, special
    
    -- For rescheduled events
    new_start_time TIME,
    new_end_time TIME,
    new_trader_name VARCHAR(255),
    
    -- Reason/notes
    reason TEXT,
    
    -- Audit
    created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    
    -- Unique constraint: one exception per schedule per date
    UNIQUE(schedule_id, exception_date)
);

CREATE INDEX IF NOT EXISTS idx_exceptions_schedule ON schedule_exceptions(schedule_id);
CREATE INDEX IF NOT EXISTS idx_exceptions_date ON schedule_exceptions(exception_date);
CREATE INDEX IF NOT EXISTS idx_exceptions_type ON schedule_exceptions(exception_type);

-- ═══════════════════════════════════════════════════════════════════════════
-- SEED SAMPLE SCHEDULES FOR TRADING ROOMS
-- Day Trading Room, Swing Trading Room, Small Account Mentorship
-- ═══════════════════════════════════════════════════════════════════════════

-- Day Trading Room Schedule (plan_id = 1)
INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Morning Market Analysis',
    'Taylor Horton',
    1, -- Monday
    '09:20:00',
    '10:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'day-trading-room'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Midday Trading Session',
    'Sam Shames',
    1, -- Monday
    '10:30:00',
    '11:30:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'day-trading-room'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Afternoon Power Hour',
    'Neil Yeager',
    1, -- Monday
    '14:00:00',
    '15:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'day-trading-room'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Market Close Analysis',
    'Bruce Marshall',
    1, -- Monday
    '15:30:00',
    '16:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'day-trading-room'
ON CONFLICT DO NOTHING;

-- Tuesday Schedule
INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Pre-Market Prep',
    'Henry Gambell',
    2, -- Tuesday
    '09:15:00',
    '09:45:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'day-trading-room'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Forex & Futures Focus',
    'Raghee Horner',
    2, -- Tuesday
    '10:30:00',
    '11:30:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'day-trading-room'
ON CONFLICT DO NOTHING;

-- Swing Trading Room Schedule (plan_id = 2)
INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Weekly Swing Setup Review',
    'David Starr',
    1, -- Monday
    '11:00:00',
    '12:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'swing-trading-room'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Swing Trade Watchlist',
    'Danielle Shay',
    3, -- Wednesday
    '10:00:00',
    '11:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'swing-trading-room'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'End of Week Review',
    'John Carter',
    5, -- Friday
    '14:00:00',
    '15:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'swing-trading-room'
ON CONFLICT DO NOTHING;

-- Small Account Mentorship Schedule (plan_id = 3)
INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Small Account Strategy Session',
    'Tim Justice',
    2, -- Tuesday
    '12:00:00',
    '13:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'small-account-mentorship'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Risk Management Workshop',
    'Tim Justice',
    4, -- Thursday
    '12:00:00',
    '13:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'small-account-mentorship'
ON CONFLICT DO NOTHING;

INSERT INTO trading_room_schedules (plan_id, title, trader_name, day_of_week, start_time, end_time, timezone, is_recurring, is_active, room_type)
SELECT 
    p.id,
    'Position Sizing Masterclass',
    'Tim Justice',
    5, -- Friday
    '11:00:00',
    '12:00:00',
    'America/New_York',
    true,
    true,
    'live'
FROM membership_plans p WHERE p.slug = 'small-account-mentorship'
ON CONFLICT DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════════════
-- HELPER FUNCTION: Get upcoming events for a trading room
-- ═══════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION get_upcoming_schedule_events(
    p_plan_slug VARCHAR(255),
    p_days_ahead INTEGER DEFAULT 7
)
RETURNS TABLE (
    id BIGINT,
    title VARCHAR(255),
    trader_name VARCHAR(255),
    event_date DATE,
    start_time TIME,
    end_time TIME,
    timezone VARCHAR(50),
    is_cancelled BOOLEAN,
    exception_reason TEXT
) AS $$
BEGIN
    RETURN QUERY
    WITH date_series AS (
        SELECT generate_series(
            CURRENT_DATE,
            CURRENT_DATE + p_days_ahead,
            '1 day'::interval
        )::date AS event_date
    ),
    schedule_events AS (
        SELECT 
            s.id,
            s.title,
            s.trader_name,
            ds.event_date,
            COALESCE(e.new_start_time, s.start_time) AS start_time,
            COALESCE(e.new_end_time, s.end_time) AS end_time,
            s.timezone,
            CASE WHEN e.exception_type = 'cancelled' THEN true ELSE false END AS is_cancelled,
            e.reason AS exception_reason
        FROM trading_room_schedules s
        JOIN membership_plans p ON s.plan_id = p.id
        CROSS JOIN date_series ds
        LEFT JOIN schedule_exceptions e ON e.schedule_id = s.id AND e.exception_date = ds.event_date
        WHERE p.slug = p_plan_slug
          AND s.is_active = true
          AND EXTRACT(DOW FROM ds.event_date) = s.day_of_week
          AND (s.effective_from IS NULL OR ds.event_date >= s.effective_from)
          AND (s.effective_until IS NULL OR ds.event_date <= s.effective_until)
    )
    SELECT * FROM schedule_events
    WHERE NOT is_cancelled
    ORDER BY event_date, start_time;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════════════
-- AUDIT TRIGGER: Auto-update updated_at timestamp
-- ═══════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION update_schedule_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_schedule_timestamp ON trading_room_schedules;
CREATE TRIGGER trigger_update_schedule_timestamp
    BEFORE UPDATE ON trading_room_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_schedule_timestamp();
