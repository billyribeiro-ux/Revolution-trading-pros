-- Shared, derived member facts. No fabricated financial or engagement data.
-- Spending is completed USD order value less recorded refunds, in dollars.
CREATE VIEW admin_member_facts AS
SELECT u.id,u.name,u.email,u.avatar_url AS avatar,u.created_at AS joined_at,u.created_at,u.updated_at,
 u.metadata->>'first_name' AS first_name,u.metadata->>'last_name' AS last_name,
 u.email_verified_at IS NOT NULL AS email_verified,u.last_login_at AS last_login,
 (SELECT COUNT(*) FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status='active' AND (s.expires_at IS NULL OR s.expires_at>NOW())) AS active_subscriptions_count,
 (SELECT p.name FROM user_memberships s JOIN membership_plans p ON p.id=s.plan_id WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status IN ('active','trial','trialing') AND (s.expires_at IS NULL OR s.expires_at>NOW()) ORDER BY s.created_at DESC,s.id DESC LIMIT 1) AS current_plan,
 NULL::text AS current_product,
 COALESCE((SELECT jsonb_agg(t.name ORDER BY t.name) FROM user_member_tags assignment JOIN member_tags t ON t.id=assignment.tag_id WHERE assignment.user_id=u.id),'[]'::jsonb) AS tags,
 CASE WHEN EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status IN ('trial','trialing') AND (s.expires_at IS NULL OR s.expires_at>NOW())) THEN 'trial'
      WHEN EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND s.status='active' AND (s.expires_at IS NULL OR s.expires_at>NOW())) THEN 'active'
      WHEN EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL AND (s.status IN ('cancelled','canceled','expired') OR s.expires_at<=NOW())) THEN 'churned'
      ELSE 'never_subscribed' END AS status,
 COALESCE((SELECT s.status FROM user_memberships s WHERE s.user_id=u.id AND s.deleted_at IS NULL ORDER BY s.created_at DESC,s.id DESC LIMIT 1),'none') AS subscription_status,
 COALESCE((SELECT SUM(GREATEST(o.total-COALESCE(o.refund_amount,0),0)) FROM orders o WHERE o.user_id=u.id AND o.currency='USD' AND o.status IN ('completed','paid','refunded','partially_refunded')),0) AS total_spent,
 FLOOR(EXTRACT(EPOCH FROM (NOW()-u.created_at))/86400)::bigint AS days_since_signup,
 (SELECT FLOOR(EXTRACT(EPOCH FROM (NOW()-MAX(COALESCE(o.completed_at,o.created_at))))/86400)::bigint FROM orders o WHERE o.user_id=u.id AND o.status IN ('completed','paid','refunded','partially_refunded')) AS days_since_last_order,
 CASE WHEN EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE LOWER(d.email)=LOWER(u.email) AND d.opened_at>=NOW()-INTERVAL '7 days') OR EXISTS(SELECT 1 FROM email_logs e WHERE LOWER(e.to_email)=LOWER(u.email) AND e.opened_at>=NOW()-INTERVAL '7 days') THEN 'engaged' ELSE 'not_engaged' END AS email_engagement
FROM users u WHERE u.deleted_at IS NULL;

ALTER TABLE member_segments ADD COLUMN is_system BOOLEAN NOT NULL DEFAULT false;
INSERT INTO member_segments(name,slug,description,is_dynamic,is_system,filter_criteria,is_active)
VALUES ('Active Subscribers','active-subscribers','Members with a current active subscription',true,true,'[{"field":"subscription_status","operator":"eq","value":"active"}]',true),
       ('Trial Users','trial-users','Members currently on trial',true,true,'[{"field":"status","operator":"eq","value":"trial"}]',true)
ON CONFLICT(slug) DO NOTHING;
