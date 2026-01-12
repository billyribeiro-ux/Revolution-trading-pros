-- ═══════════════════════════════════════════════════════════════════════════════
-- Migration 021: Bootstrap Developer Account
-- ═══════════════════════════════════════════════════════════════════════════════
-- ICT 7 Principal Engineer Grade - Secure, Idempotent, Auditable
-- ═══════════════════════════════════════════════════════════════════════════════

INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at) 
VALUES (
  'welberribeirodrums@gmail.com', 
  '$argon2id$v=19$m=65536,t=3,p=4$WtDOAgd0bfMBJCTrA4cnPg$aaC1+jrXQHqoqzksY6xrpyg0stuGSvlBvoACR4uYOU4', 
  'Billy Ribeiro', 
  'developer', 
  NOW(), 
  NOW(), 
  NOW()
) 
ON CONFLICT (email) DO UPDATE SET 
  password_hash = EXCLUDED.password_hash, 
  role = 'developer', 
  email_verified_at = COALESCE(users.email_verified_at, NOW()), 
  updated_at = NOW();
