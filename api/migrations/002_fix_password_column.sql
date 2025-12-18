-- Migration to fix password column name for Rust API
-- This handles transition from Laravel schema (password) to Rust schema (password_hash)

-- Add password_hash column if it doesn't exist
DO $$ 
BEGIN
    -- Check if password column exists and password_hash doesn't
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'password')
       AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'password_hash')
    THEN
        -- Rename password to password_hash
        ALTER TABLE users RENAME COLUMN password TO password_hash;
    END IF;
    
    -- Ensure id column is UUID type (Laravel uses bigint, Rust uses UUID)
    -- If id is bigint, we need to recreate the table or add a uuid column
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'id' AND data_type = 'bigint')
    THEN
        -- Add uuid_id column if it doesn't exist
        IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'uuid_id')
        THEN
            ALTER TABLE users ADD COLUMN uuid_id UUID DEFAULT gen_random_uuid();
        END IF;
    END IF;
    
    -- Add missing columns for Rust schema
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'role')
    THEN
        ALTER TABLE users ADD COLUMN role VARCHAR(50) NOT NULL DEFAULT 'user';
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'avatar_url')
    THEN
        ALTER TABLE users ADD COLUMN avatar_url TEXT;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'users' AND column_name = 'stripe_customer_id')
    THEN
        ALTER TABLE users ADD COLUMN stripe_customer_id VARCHAR(255);
    END IF;
END $$;
