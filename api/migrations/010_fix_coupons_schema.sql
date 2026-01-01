-- Fix coupons table schema - add missing description column
-- ICT 11+ Principal Engineer Grade - January 2026

DO $$ 
BEGIN
    -- Add description column if it doesn't exist
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='coupons' AND column_name='description') THEN
        ALTER TABLE coupons ADD COLUMN description TEXT;
    END IF;
END $$;
