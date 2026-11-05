-- Durable authority for account state. Redis is never the role or token-epoch authority.
-- JWTs minted before authority_version=1 must reauthenticate; no Redis epoch is guessed.
ALTER TABLE users
 ADD COLUMN auth_version BIGINT NOT NULL DEFAULT 0 CHECK(auth_version>=0),
 ADD COLUMN account_version BIGINT NOT NULL DEFAULT 1 CHECK(account_version>0),
 ALTER COLUMN created_at SET DEFAULT (NOW() AT TIME ZONE 'UTC'),
 ALTER COLUMN updated_at SET DEFAULT (NOW() AT TIME ZONE 'UTC');
COMMENT ON COLUMN users.created_at IS 'New operational writes use UTC wall time. Historical timezone provenance is unknown; no values were backfilled.';
COMMENT ON COLUMN users.updated_at IS 'New operational writes use UTC wall time. Historical timezone provenance is unknown; no values were backfilled.';

CREATE FUNCTION member_account_versions() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF (NEW.name,NEW.email,NEW.role,NEW.avatar_url,NEW.password_hash,NEW.is_active,NEW.deleted_at,NEW.email_verified_at,NEW.mfa_enabled,(NEW.metadata->'first_name'),(NEW.metadata->'last_name'))
    IS DISTINCT FROM
    (OLD.name,OLD.email,OLD.role,OLD.avatar_url,OLD.password_hash,OLD.is_active,OLD.deleted_at,OLD.email_verified_at,OLD.mfa_enabled,(OLD.metadata->'first_name'),(OLD.metadata->'last_name')) THEN
  NEW.account_version := OLD.account_version+1;
 ELSE
  IF NEW.account_version<OLD.account_version THEN RAISE EXCEPTION 'Account version cannot move backwards' USING ERRCODE='23514'; END IF;
 END IF;
 IF (NEW.email,NEW.role,NEW.password_hash,NEW.is_active,NEW.deleted_at,NEW.email_verified_at,NEW.mfa_enabled)
    IS DISTINCT FROM
    (OLD.email,OLD.role,OLD.password_hash,OLD.is_active,OLD.deleted_at,OLD.email_verified_at,OLD.mfa_enabled) THEN
  NEW.auth_version := OLD.auth_version+1;
 ELSE
  IF NEW.auth_version<OLD.auth_version THEN RAISE EXCEPTION 'Account authority cannot move backwards' USING ERRCODE='23514'; END IF;
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER member_account_versions BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION member_account_versions();

CREATE TABLE auth_revoked_tokens(
 jti UUID PRIMARY KEY,
 user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
 token_type TEXT NOT NULL CHECK(token_type IN('access','refresh')),
 reason TEXT NOT NULL CHECK(reason IN('logout','rotated')),
 expires_at TIMESTAMPTZ NOT NULL,
 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX auth_revoked_tokens_expiry ON auth_revoked_tokens(expires_at);

ALTER TABLE user_status
 ADD COLUMN restriction_original_is_active BOOLEAN,
 ADD COLUMN restriction_prior_recorded BOOLEAN NOT NULL DEFAULT false,
 ADD COLUMN restriction_auth_version BIGINT;
COMMENT ON COLUMN user_status.restriction_prior_recorded IS 'True only when this workflow captured the actual pre-restriction nullable users.is_active. Legacy values are not inferred.';
CREATE INDEX member_restriction_expiry ON user_status(ban_expires_at,user_id) WHERE status IN('banned','suspended') AND ban_expires_at IS NOT NULL;

CREATE TABLE member_admin_creations(
 request_id UUID PRIMARY KEY CHECK(request_id<>'00000000-0000-0000-0000-000000000000'),
 created_by BIGINT REFERENCES users(id) ON DELETE SET NULL,
 original_actor_id BIGINT NOT NULL,
 user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
 original_member_id BIGINT NOT NULL,
 source_hash TEXT NOT NULL CHECK(length(source_hash)=64),
 response_snapshot JSONB NOT NULL CHECK(jsonb_typeof(response_snapshot)='object'),
 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE member_welcome_requests(
 user_id BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
 request_id UUID NOT NULL UNIQUE REFERENCES member_admin_creations(request_id),
 email_log_id BIGINT REFERENCES email_logs(id) ON DELETE SET NULL,
 job_id UUID REFERENCES jobs(id) ON DELETE SET NULL,
 recipient TEXT NOT NULL,
 content_snapshot JSONB NOT NULL CHECK(jsonb_typeof(content_snapshot)='object'),
 claimed_at TIMESTAMPTZ,
 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
COMMENT ON TABLE member_welcome_requests IS 'One frozen welcome intent per admin-created account. Claimed work is never automatically resubmitted after a crash or an unknown provider outcome.';

-- Preserve historical case collisions, but never admit a new ambiguous case-insensitive email.
CREATE FUNCTION member_email_identity() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP='UPDATE' AND NEW.email IS NOT DISTINCT FROM OLD.email THEN RETURN NEW; END IF;
 PERFORM pg_advisory_xact_lock(hashtextextended(LOWER(NEW.email),117));
 IF EXISTS(SELECT 1 FROM users WHERE LOWER(email)=LOWER(NEW.email) AND id<>NEW.id) THEN
  RAISE EXCEPTION 'Account email is already recorded' USING ERRCODE='23505';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER member_email_identity BEFORE INSERT OR UPDATE OF email ON users FOR EACH ROW EXECUTE FUNCTION member_email_identity();

-- Explicit historical deletion marker excludes legacy deleted accounts without inventing a date.
CREATE OR REPLACE VIEW admin_member_facts AS
SELECT u.id,u.name,u.email,u.avatar_url AS avatar,u.created_at AS joined_at,u.created_at,u.updated_at,
 u.metadata->>'first_name' AS first_name,u.metadata->>'last_name' AS last_name,
 u.email_verified_at IS NOT NULL AS email_verified,u.last_login_at AS last_login,
 (SELECT COUNT(*) FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status='active' AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND (s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC'))) AS active_subscriptions_count,
 (SELECT p.name FROM user_memberships s JOIN membership_plans p ON p.id=s.plan_id WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status IN ('active','trial','trialing') AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND (s.status='active' OR s.trial_ends_at IS NULL OR s.trial_ends_at>(NOW() AT TIME ZONE 'UTC')) AND (s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC')) ORDER BY CASE WHEN s.status='active' THEN 0 ELSE 1 END,s.created_at DESC,s.id DESC LIMIT 1) AS current_plan,
 NULL::text AS current_product,
 COALESCE((SELECT jsonb_agg(t.name ORDER BY t.name) FROM user_member_tags assignment JOIN member_tags t ON t.id=assignment.tag_id WHERE assignment.user_id=u.id),'[]'::jsonb) AS tags,
 CASE WHEN EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status='active' AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND (s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC'))) THEN 'active'
      WHEN EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status IN ('trial','trialing') AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND (s.trial_ends_at IS NULL OR s.trial_ends_at>(NOW() AT TIME ZONE 'UTC')) AND (s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC'))) THEN 'trial'
      WHEN EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND (s.status IN ('cancelled','canceled','expired') OR s.expires_at<=(NOW() AT TIME ZONE 'UTC') OR (s.status IN ('trial','trialing') AND s.trial_ends_at<=(NOW() AT TIME ZONE 'UTC')))) THEN 'churned'
      ELSE 'never_subscribed' END AS status,
 COALESCE((SELECT s.status FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL ORDER BY s.created_at DESC,s.id DESC LIMIT 1),'none') AS subscription_status,
 COALESCE((SELECT SUM(GREATEST(o.total-COALESCE(o.refund_amount,0),0)) FROM orders o WHERE o.user_id=u.id AND o.currency='USD' AND o.status IN ('completed','paid','refunded','partial_refund','partially_refunded')),0) AS total_spent,
 CASE WHEN isfinite(u.created_at) THEN FLOOR(EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE 'UTC')-u.created_at))/86400)::bigint ELSE NULL::bigint END AS days_since_signup,
 (SELECT CASE WHEN isfinite(MAX(COALESCE(o.completed_at,o.created_at))) THEN FLOOR(EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE 'UTC')-MAX(COALESCE(o.completed_at,o.created_at))))/86400)::bigint ELSE NULL::bigint END FROM orders o WHERE o.user_id=u.id AND o.status IN ('completed','paid','refunded','partial_refund','partially_refunded')) AS days_since_last_order,
 CASE WHEN EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE LOWER(d.email)=LOWER(u.email) AND d.opened_at>=NOW()-INTERVAL '7 days') OR EXISTS(SELECT 1 FROM email_logs e WHERE LOWER(e.to_email)=LOWER(u.email) AND e.opened_at>=(NOW() AT TIME ZONE 'UTC')-INTERVAL '7 days') THEN 'engaged' ELSE 'not_engaged' END AS email_engagement
FROM users u WHERE u.deleted_at IS NULL AND COALESCE(LOWER(u.role),'')<>'deleted';

-- The member universe changes for every criteria definition, including empty/all-member rules.
UPDATE member_segments SET authoring_version=authoring_version+1,updated_at=NOW();

-- Legacy recovery material is hashed before the newly mounted workflows accept requests.
-- pgcrypto is trusted standard PostgreSQL contrib; no application secret is embedded in SQL.
CREATE EXTENSION IF NOT EXISTS pgcrypto;
ALTER TABLE user_mfa_secrets ALTER COLUMN totp_secret TYPE TEXT;
ALTER TABLE mfa_attempts ALTER COLUMN ip_address DROP NOT NULL;
COMMENT ON COLUMN mfa_attempts.ip_address IS 'Actual trusted client address when observed; unknown is NULL, never an invented address.';
-- Only complete known hash encodings or the actual old XXXX-XXXX code format survive.
-- A malformed element invalidates the whole recovery array; no prefix is guessed to be a hash.
CREATE FUNCTION member_backup_hash(value TEXT) RETURNS BOOLEAN LANGUAGE SQL IMMUTABLE AS $$
 SELECT value ~ '^\$argon2id\$v=19\$m=[1-9][0-9]{0,8},t=[1-9][0-9]{0,2},p=[1-9][0-9]{0,2}\$[A-Za-z0-9+/]{16,128}\$[A-Za-z0-9+/]{32,128}$'
     OR value ~ '^\$2[aby]\$(0[4-9]|[12][0-9]|3[01])\$[./A-Za-z0-9]{53}$'
$$;
UPDATE user_mfa_secrets m SET backup_codes=CASE
 WHEN jsonb_typeof(m.backup_codes) IS DISTINCT FROM 'array' THEN NULL
 WHEN EXISTS(SELECT 1 FROM jsonb_array_elements(m.backup_codes) x WHERE jsonb_typeof(x)<>'string' OR NOT (member_backup_hash(x#>>'{}') OR (x#>>'{}')~'^[A-Z0-9]{4}-[A-Z0-9]{4}$')) THEN NULL
 ELSE (SELECT COALESCE(jsonb_agg(to_jsonb(CASE WHEN member_backup_hash(code) THEN code ELSE crypt(REPLACE(code,'-',''),gen_salt('bf',10)) END)),'[]'::jsonb) FROM jsonb_array_elements_text(m.backup_codes) code)
 END;
CREATE FUNCTION member_backup_hashes(value JSONB) RETURNS BOOLEAN LANGUAGE SQL IMMUTABLE AS $$
 SELECT CASE WHEN value IS NULL THEN true WHEN jsonb_typeof(value)='array' THEN NOT EXISTS(SELECT 1 FROM jsonb_array_elements(value) code WHERE jsonb_typeof(code)<>'string' OR NOT member_backup_hash(code#>>'{}')) ELSE false END
$$;
ALTER TABLE user_mfa_secrets ADD CONSTRAINT member_backup_hash_storage CHECK(member_backup_hashes(backup_codes));
COMMENT ON COLUMN user_mfa_secrets.backup_codes IS 'One-way recovery hashes only. Valid legacy plaintext arrays were bcrypt-hashed, malformed legacy recovery data is NULL/unavailable; new values use Argon2id.';

CREATE FUNCTION member_mfa_credential_version() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE member BIGINT; changed BOOLEAN; consumed BOOLEAN;
BEGIN
 member:=CASE WHEN TG_OP='DELETE' THEN OLD.user_id ELSE NEW.user_id END;
 IF TG_OP='UPDATE' THEN
  -- The one exception is an exact one-code removal with a matching consumption counter.
  -- Any replacement/rotation revokes prior tokens even if a writer forgot generated_at.
  consumed:=false;
  IF jsonb_typeof(OLD.backup_codes)='array' AND jsonb_typeof(NEW.backup_codes)='array'
     AND NEW.backup_codes_used=OLD.backup_codes_used+1 THEN
   consumed:=EXISTS(SELECT 1 FROM jsonb_array_elements(OLD.backup_codes) WITH ORDINALITY x(code,ord) WHERE OLD.backup_codes-(ord::int-1)=NEW.backup_codes);
  END IF;
  changed:=(NEW.totp_secret,NEW.totp_verified_at,NEW.backup_codes_generated_at) IS DISTINCT FROM (OLD.totp_secret,OLD.totp_verified_at,OLD.backup_codes_generated_at)
     OR (NEW.backup_codes IS DISTINCT FROM OLD.backup_codes AND NOT consumed);
 ELSE changed:=true; END IF;
 IF changed THEN
  UPDATE users SET auth_version=auth_version+1,account_version=account_version+1 WHERE id=member;
 END IF;
 RETURN NULL;
END $$;
CREATE TRIGGER member_mfa_credential_version AFTER INSERT OR UPDATE OR DELETE ON user_mfa_secrets FOR EACH ROW EXECUTE FUNCTION member_mfa_credential_version();

-- Capabilities are bound to their actual recipient and DB authority. Legacy NULL links must
-- be requested again; no historical email identity, epoch or wall-clock provenance is guessed.
ALTER TABLE email_verification_tokens ADD COLUMN recipient TEXT,ADD COLUMN issued_auth_version BIGINT;
ALTER TABLE password_resets ADD COLUMN user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,ADD COLUMN issued_auth_version BIGINT;
COMMENT ON COLUMN email_verification_tokens.issued_auth_version IS 'Only newly issued recipient-bound links are accepted; legacy NULL links require a resend.';
COMMENT ON COLUMN password_resets.issued_auth_version IS 'Only newly issued user-id/recipient-bound links are accepted; legacy NULL links require a new request. New naive expiry writes use UTC.';
