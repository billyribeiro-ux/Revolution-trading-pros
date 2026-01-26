-- ═══════════════════════════════════════════════════════════════════════════════════
-- Migration 022: Add Position Invalidation and Update Tracking Fields
-- Apple Principal Engineer ICT 7+ Grade - January 26, 2026
--
-- Adds fields to support:
-- - Trade/position invalidation (for setups that didn't trigger)
-- - Update tracking (show when positions were modified)
-- - Soft delete support
-- ═══════════════════════════════════════════════════════════════════════════════════

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 1. ADD INVALIDATION AND UPDATE TRACKING COLUMNS TO room_trades
-- ═══════════════════════════════════════════════════════════════════════════════════

-- Track if position was manually updated (shows UPDATED badge)
ALTER TABLE room_trades ADD COLUMN IF NOT EXISTS was_updated BOOLEAN DEFAULT false;

-- Store invalidation reason when trade is marked as invalidated
ALTER TABLE room_trades ADD COLUMN IF NOT EXISTS invalidation_reason TEXT;

-- Add 'invalidated' as a valid status
-- Current statuses: 'open', 'closed', 'partial'
-- New status: 'invalidated' (for trades that didn't trigger)
COMMENT ON COLUMN room_trades.status IS 'Trade status: open, closed, partial, invalidated';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 2. CREATE INDEXES FOR EFFICIENT QUERYING
-- ═══════════════════════════════════════════════════════════════════════════════════

CREATE INDEX IF NOT EXISTS idx_room_trades_was_updated ON room_trades(was_updated) WHERE was_updated = true;
CREATE INDEX IF NOT EXISTS idx_room_trades_invalidated ON room_trades(status) WHERE status = 'invalidated';

-- ═══════════════════════════════════════════════════════════════════════════════════
-- 3. UPDATE EXISTING TRADES TO HAVE CORRECT DEFAULTS
-- ═══════════════════════════════════════════════════════════════════════════════════

UPDATE room_trades SET was_updated = false WHERE was_updated IS NULL;

-- ═══════════════════════════════════════════════════════════════════════════════════
-- END MIGRATION
-- ═══════════════════════════════════════════════════════════════════════════════════
