//! Current member cohorts and explicitly scoped recorded-order facts.
use crate::{AppState, utils::errors::ApiError};
use serde_json::Value;

pub(crate) const ORDER_VALUE_SQL: &str = r#"(SELECT jsonb_build_object(
 'currency','USD','net_cents',COALESCE(SUM(ROUND(GREATEST(o.total-COALESCE(o.refund_amount,0),0)*100)) FILTER(WHERE o.currency='USD'),0)::bigint,
 'order_count',COUNT(*) FILTER(WHERE o.currency='USD'),
 'excluded_non_usd_orders',COUNT(*) FILTER(WHERE o.currency IS NOT NULL AND o.currency<>'' AND o.currency<>'USD'),
 'excluded_unknown_currency_orders',COUNT(*) FILTER(WHERE o.currency IS NULL OR o.currency=''),
 'scope','recorded_orders_less_recorded_refunds',
 'limitations','Recorded qualifying orders only; excludes unmaterialized renewal invoices, processor fees and settlement. Currency comes from the order record, not independently reconciled processor facts.')
 FROM orders o JOIN audience a ON a.id=o.user_id
 WHERE o.status IN ('completed','paid','refunded','partial_refund','partially_refunded'))"#;

pub(super) const CURRENT_MEMBERSHIPS_SQL: &str = r#"SELECT s.*,p.name AS plan_name,p.price,p.billing_cycle,p.interval_count
 FROM user_memberships s JOIN admin_member_facts m ON m.id=s.user_id JOIN membership_plans p ON p.id=s.plan_id
 WHERE s.deleted_at IS NULL AND s.starts_at<=NOW() AND (s.expires_at IS NULL OR s.expires_at>NOW())
 AND (s.status='active' OR (s.status IN ('trial','trialing') AND (s.trial_ends_at IS NULL OR s.trial_ends_at>NOW())))"#;

pub(super) async fn load(state: &AppState) -> Result<Value, ApiError> {
    let sql = format!(
        r#"WITH audience AS(SELECT * FROM admin_member_facts),
 current_memberships AS({CURRENT_MEMBERSHIPS_SQL}),
 counts AS(SELECT COUNT(*) AS total,COUNT(*) FILTER(WHERE created_at>=date_trunc('month',NOW() AT TIME ZONE 'UTC') AND created_at<=NOW() AT TIME ZONE 'UTC') AS new_current,
 COUNT(*) FILTER(WHERE created_at>=date_trunc('month',NOW() AT TIME ZONE 'UTC')-INTERVAL '1 month' AND created_at<date_trunc('month',NOW() AT TIME ZONE 'UTC')) AS new_previous,
 COUNT(*) FILTER(WHERE status='active') AS active,COUNT(*) FILTER(WHERE status='trial') AS trial,
 COUNT(*) FILTER(WHERE status='churned') AS churned,COUNT(*) FILTER(WHERE status='never_subscribed') AS never_subscribed FROM audience),
 catalog AS(SELECT *,CASE
 WHEN price<0 THEN NULL
 WHEN lower(billing_cycle) IN ('monthly','month') AND interval_count>0 THEN price/interval_count
 WHEN lower(billing_cycle) IN ('quarterly','quarter') AND interval_count IN(1,3) THEN price/3
 WHEN lower(billing_cycle) IN ('annual','annually','yearly','year') AND interval_count IN(1,12) THEN price/12
 ELSE NULL END AS monthly_units FROM current_memberships WHERE status='active'),
 top_plans AS(SELECT plan_id AS id,plan_name AS name,'membership_plan' AS type,COUNT(DISTINCT user_id) AS members_count FROM current_memberships GROUP BY plan_id,plan_name ORDER BY members_count DESC,plan_id LIMIT 6),
 months AS(SELECT generate_series(date_trunc('month',NOW() AT TIME ZONE 'UTC')-INTERVAL '11 months',date_trunc('month',NOW() AT TIME ZONE 'UTC'),INTERVAL '1 month') AS month),
 growth AS(SELECT to_char(month,'YYYY-MM') AS month,(SELECT COUNT(*) FROM audience a WHERE a.created_at<months.month+INTERVAL '1 month' AND a.created_at<=NOW() AT TIME ZONE 'UTC') AS members,(SELECT COUNT(*) FROM audience a WHERE a.created_at>=months.month AND a.created_at<months.month+INTERVAL '1 month' AND a.created_at<=NOW() AT TIME ZONE 'UTC') AS new FROM months ORDER BY months.month)
 SELECT jsonb_build_object('schema_version',1,'scope','current_non_deleted_members','evaluated_at',NOW(),'recorded_timestamp_basis','Legacy created_at values are timestamp without time zone; historical instants have not been verified as UTC. Month grouping uses recorded wall-clock values.',
 'period',jsonb_build_object('timezone','UTC','current_month_start',date_trunc('month',NOW() AT TIME ZONE 'UTC'),'comparison','month_to_date_vs_previous_full_month'),
 'overview',jsonb_build_object('total_members',total,'new_this_month',new_current,'new_last_month',new_previous,'growth_rate',CASE WHEN new_previous>0 THEN ROUND((new_current-new_previous)*100.0/new_previous,1) ELSE NULL END,'growth_rate_reason',CASE WHEN new_previous=0 THEN 'No registrations in the previous full month' ELSE NULL END),
 'subscriptions',jsonb_build_object('active',active,'trial',trial,'churned',churned,'never_subscribed',never_subscribed,'active_membership_records',(SELECT COUNT(*) FROM current_memberships WHERE status='active'),'churn_rate',NULL,'churn_rate_reason','Current former-member cohort is known; historical membership transitions and a period denominator are not recorded as a complete census.'),
 'revenue',jsonb_build_object('recorded_net_usd_order_value',{ORDER_VALUE_SQL},
 'mrr_cents',NULL,'mrr_reason','Current memberships do not preserve a complete charged recurring amount, currency and interval snapshot.',
 'avg_ltv_cents',NULL,'avg_ltv_reason','Lifetime payment coverage and a defined lifetime cohort are not recorded.',
 'catalog_monthly_estimate',(SELECT jsonb_build_object('value',CASE WHEN COUNT(*)=0 THEN 0 ELSE SUM(monthly_units) END,'unit','catalog_price_unit','currency',NULL,'included_membership_records',COUNT(monthly_units),'excluded_membership_records',COUNT(*)-COUNT(monthly_units),'scope','current_active_membership_records_at_mutable_catalog_prices','reason','Catalog currency is unrecorded. This is a nominal price estimate, not charged revenue or measured MRR.') FROM catalog)),
 'top_services',COALESCE((SELECT jsonb_agg(to_jsonb(top_plans)) FROM top_plans),'[]'::jsonb),
 'growth_trend',COALESCE((SELECT jsonb_agg(to_jsonb(growth)) FROM growth),'[]'::jsonb),
 'growth_timestamp_coverage',(SELECT jsonb_build_object('unknown_members',COUNT(*) FILTER(WHERE created_at IS NULL),'future_members',COUNT(*) FILTER(WHERE created_at>NOW() AT TIME ZONE 'UTC')) FROM audience),'growth_scope','Recorded registrations of currently nondeleted members, excluding missing and future recorded dates; not a historical membership census') FROM counts"#
    );
    sqlx::query_scalar(&sql)
        .fetch_one(&state.db.pool)
        .await
        .map_err(super::member_services::storage)
}

/// Shared whole-member money facts; the caller must authorize and verify the selected member.
pub(crate) async fn recorded_order_value_on(
    connection: &mut sqlx::PgConnection,
    member_id: i64,
) -> Result<Value, ApiError> {
    let sql = format!(
        "WITH audience AS(SELECT id FROM users WHERE id=$1 AND deleted_at IS NULL) SELECT {ORDER_VALUE_SQL}"
    );
    sqlx::query_scalar(&sql)
        .bind(member_id)
        .fetch_one(connection)
        .await
        .map_err(super::member_services::storage)
}
