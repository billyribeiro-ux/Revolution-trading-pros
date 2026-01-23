-- Migration: Add runner_stop column to room_trade_plans
-- Description: Adds the runner_stop field for tracking stop loss price on runner positions
-- Date: 2026-01-23

-- Add runner_stop column to room_trade_plans table
ALTER TABLE room_trade_plans ADD COLUMN IF NOT EXISTS runner_stop VARCHAR(20);

-- Add comment for documentation
COMMENT ON COLUMN room_trade_plans.runner_stop IS 'Stop loss price for runner position';
