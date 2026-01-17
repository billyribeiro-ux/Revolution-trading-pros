-- ═══════════════════════════════════════════════════════════════════════════════
-- Organization Tables Migration - Teams & Departments
-- Apple ICT 7 Principal Engineer Grade - January 2026
-- ═══════════════════════════════════════════════════════════════════════════════

-- Teams table
CREATE TABLE IF NOT EXISTS teams (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    color VARCHAR(50),
    icon VARCHAR(100),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_teams_slug ON teams(slug);
CREATE INDEX IF NOT EXISTS idx_teams_is_active ON teams(is_active);

-- Departments table
CREATE TABLE IF NOT EXISTS departments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    color VARCHAR(50),
    icon VARCHAR(100),
    parent_id BIGINT REFERENCES departments(id) ON DELETE SET NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_departments_slug ON departments(slug);
CREATE INDEX IF NOT EXISTS idx_departments_parent_id ON departments(parent_id);
CREATE INDEX IF NOT EXISTS idx_departments_is_active ON departments(is_active);

-- User-Team association (many-to-many)
CREATE TABLE IF NOT EXISTS user_teams (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    team_id BIGINT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member', -- member, lead, manager
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, team_id)
);

CREATE INDEX IF NOT EXISTS idx_user_teams_user_id ON user_teams(user_id);
CREATE INDEX IF NOT EXISTS idx_user_teams_team_id ON user_teams(team_id);

-- User-Department association (many-to-many)
CREATE TABLE IF NOT EXISTS user_departments (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    department_id BIGINT NOT NULL REFERENCES departments(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member', -- member, head, director
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, department_id)
);

CREATE INDEX IF NOT EXISTS idx_user_departments_user_id ON user_departments(user_id);
CREATE INDEX IF NOT EXISTS idx_user_departments_department_id ON user_departments(department_id);

-- Seed default teams
INSERT INTO teams (name, slug, description, color, is_active) VALUES
    ('Trading', 'trading', 'Trading team members', '#22c55e', true),
    ('Education', 'education', 'Education and content team', '#3b82f6', true),
    ('Support', 'support', 'Customer support team', '#f59e0b', true),
    ('Development', 'development', 'Technical development team', '#8b5cf6', true)
ON CONFLICT (slug) DO NOTHING;

-- Seed default departments
INSERT INTO departments (name, slug, description, color, is_active) VALUES
    ('Trading', 'trading', 'Trading operations', '#22c55e', true),
    ('Education', 'education', 'Educational content', '#3b82f6', true),
    ('Marketing', 'marketing', 'Marketing and growth', '#ec4899', true),
    ('Operations', 'operations', 'Business operations', '#f59e0b', true),
    ('Technology', 'technology', 'Technical infrastructure', '#8b5cf6', true)
ON CONFLICT (slug) DO NOTHING;
