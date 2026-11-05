-- Operational convention for legacy TIMESTAMP WITHOUT TIME ZONE membership fields: UTC.
-- This removes dependency on the PostgreSQL session timezone for new writes and comparisons.
-- Historical timezone provenance is unverified: do not rewrite or backfill existing timestamps.
-- True TIMESTAMPTZ observation columns continue using real NOW().
ALTER TABLE user_memberships
 ALTER COLUMN starts_at SET DEFAULT (NOW() AT TIME ZONE 'UTC'),
 ALTER COLUMN created_at SET DEFAULT (NOW() AT TIME ZONE 'UTC'),
 ALTER COLUMN updated_at SET DEFAULT (NOW() AT TIME ZONE 'UTC');
COMMENT ON COLUMN user_memberships.starts_at IS 'Operational UTC wall-clock timestamp; historical timezone provenance is unverified and values were not backfilled.';
COMMENT ON COLUMN user_memberships.expires_at IS 'Operational UTC wall-clock timestamp; historical timezone provenance is unverified and values were not backfilled.';
COMMENT ON COLUMN user_memberships.trial_ends_at IS 'Operational UTC wall-clock timestamp; historical timezone provenance is unverified and values were not backfilled.';

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
 FLOOR(EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE 'UTC')-u.created_at))/86400)::bigint AS days_since_signup,
 (SELECT FLOOR(EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE 'UTC')-MAX(COALESCE(o.completed_at,o.created_at))))/86400)::bigint FROM orders o WHERE o.user_id=u.id AND o.status IN ('completed','paid','refunded','partial_refund','partially_refunded')) AS days_since_last_order,
 CASE WHEN EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE LOWER(d.email)=LOWER(u.email) AND d.opened_at>=NOW()-INTERVAL '7 days') OR EXISTS(SELECT 1 FROM email_logs e WHERE LOWER(e.to_email)=LOWER(u.email) AND e.opened_at>=(NOW() AT TIME ZONE 'UTC')-INTERVAL '7 days') THEN 'engaged' ELSE 'not_engaged' END AS email_engagement
FROM users u WHERE u.deleted_at IS NULL;

-- Keep prior observations tied to their original session-dependent criteria semantics.
UPDATE member_segments SET authoring_version=authoring_version+1,updated_at=NOW()
WHERE EXISTS(
 SELECT 1 FROM jsonb_array_elements(CASE
  WHEN jsonb_typeof(filter_criteria)='array' THEN filter_criteria
  WHEN jsonb_typeof(filter_criteria->'conditions')='array' THEN filter_criteria->'conditions'
  ELSE '[]'::jsonb END) rule
 WHERE rule->>'field' IN('status','subscription_status','days_since_signup','days_since_last_order','email_engagement')
);
