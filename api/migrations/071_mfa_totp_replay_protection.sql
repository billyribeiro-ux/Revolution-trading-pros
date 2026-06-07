-- 071_mfa_totp_replay_protection.sql
--
-- FIX (RUST_DEEP_AUDIT_2026-06-07, P2-2): TOTP replay protection.
--
-- BACKGROUND
--   services/mfa.rs::verify_mfa accepted a valid TOTP code (with ±1-period
--   drift) but never recorded the consumed time-step, so the same 6-digit
--   code could be replayed for its full ~90s validity window. Backup codes
--   are already single-use; TOTP codes were not. RFC 6238 §5.2 requires that
--   a one-time password be accepted only once.
--
-- FIX
--   Track the most recently accepted TOTP time-step (Unix time / 30s) per
--   user. verify_mfa advances it with a conditional UPDATE
--   (WHERE last_totp_timestep < $new), so a replay of the same (or an older)
--   step updates zero rows and is rejected — atomic even under concurrent
--   submissions.
--
--   NULL = no TOTP accepted yet (first use passes). Nullable, no default, so
--   existing rows and INSERTs are unaffected.

ALTER TABLE user_mfa_secrets
    ADD COLUMN IF NOT EXISTS last_totp_timestep BIGINT;
