-- Migration: 029_cms_reusable_blocks_schema_fix
-- Purpose: Ensure cms_reusable_blocks table has all required columns for soft delete support
-- Author: System
-- Date: 2026-01-28

-- Add deleted_at column for soft delete functionality
ALTER TABLE cms_reusable_blocks 
ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

-- Add cms_user_id column for tracking which CMS user created/owns the block
ALTER TABLE cms_reusable_blocks 
ADD COLUMN IF NOT EXISTS cms_user_id UUID REFERENCES cms_users(id) ON DELETE SET NULL;

-- Create index for efficient soft delete queries
CREATE INDEX IF NOT EXISTS idx_cms_reusable_blocks_deleted_at 
ON cms_reusable_blocks(deleted_at) 
WHERE deleted_at IS NULL;

-- Create index for cms_user_id lookups
CREATE INDEX IF NOT EXISTS idx_cms_reusable_blocks_cms_user_id 
ON cms_reusable_blocks(cms_user_id) 
WHERE cms_user_id IS NOT NULL;
