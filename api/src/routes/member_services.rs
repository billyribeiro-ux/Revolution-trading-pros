//! Membership-plan catalog and plan-local member cohorts. Product IDs remain independent.
use crate::{AppState, utils::errors::ApiError};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Postgres, QueryBuilder};

pub(super) fn storage(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Member statistics/service persistence failed");
    ApiError::database_error("Unable to read recorded member facts")
}
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogQuery {
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub selected_id: Option<i64>,
}
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServiceMemberQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
fn pagination(page: Option<i64>, size: Option<i64>) -> Result<(i64, i64), ApiError> {
    let (page, size) = (page.unwrap_or(1), size.unwrap_or(25));
    if !(1..=10_000).contains(&page) || !(1..=100).contains(&size) {
        return Err(ApiError::validation_error(
            "Invalid member page or page size",
        ));
    }
    Ok((page, size))
}
fn search(value: Option<String>) -> Result<Option<String>, ApiError> {
    if value
        .as_ref()
        .is_some_and(|v| v.chars().count() > 100 || v.contains('\0'))
    {
        return Err(ApiError::validation_error(
            "Search must contain at most 100 valid characters",
        ));
    }
    Ok(value.filter(|v| !v.is_empty()).map(|v| format!("%{v}%")))
}
const PLAN_SQL: &str = "SELECT jsonb_build_object('id',p.id,'name',p.name,'type','membership_plan','price_units',p.price,'currency',NULL,'billing_cycle',p.billing_cycle,'interval_count',p.interval_count,'is_active',p.is_active,'members_count',(SELECT COUNT(DISTINCT s.user_id) FROM user_memberships s JOIN admin_member_facts m ON m.id=s.user_id WHERE s.plan_id=p.id AND s.deleted_at IS NULL AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND (s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC')) AND (s.status='active' OR(s.status IN('trial','trialing') AND(s.trial_ends_at IS NULL OR s.trial_ends_at>(NOW() AT TIME ZONE 'UTC')))))) FROM membership_plans p";
fn plan_query(id: i64) -> Result<QueryBuilder<Postgres>, ApiError> {
    if id <= 0 {
        return Err(ApiError::validation_error(
            "Membership plan identifier must be positive",
        ));
    }
    let mut query = QueryBuilder::new(PLAN_SQL);
    query.push(" WHERE p.id=").push_bind(id);
    Ok(query)
}
pub(super) async fn catalog(state: &AppState, params: CatalogQuery) -> Result<Value, ApiError> {
    let (page, size) = pagination(params.page, params.per_page)?;
    let pattern = search(params.search)?;
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    let selected: Option<Value> = if let Some(id) = params.selected_id {
        Some(
            plan_query(id)?
                .build_query_scalar()
                .fetch_optional(&mut *tx)
                .await
                .map_err(storage)?
                .ok_or_else(|| ApiError::not_found("Membership plan not found"))?,
        )
    } else {
        None
    };
    let mut count =
        QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM membership_plans p WHERE true");
    let mut rows = QueryBuilder::<Postgres>::new(PLAN_SQL);
    rows.push(" WHERE true");
    for q in [&mut count, &mut rows] {
        if let Some(pattern) = &pattern {
            q.push(" AND(p.name ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR p.slug ILIKE ")
                .push_bind(pattern.clone())
                .push(")");
        }
    }
    let total: i64 = count
        .build_query_scalar()
        .fetch_one(&mut *tx)
        .await
        .map_err(storage)?;
    rows.push(" ORDER BY p.name,p.id LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind((page - 1) * size);
    let services: Vec<Value> = rows
        .build_query_scalar()
        .fetch_all(&mut *tx)
        .await
        .map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    Ok(
        json!({"schema_version":1,"scope":"membership_plan_catalog","services":services,"selected":selected,"pagination":{"total":total,"per_page":size,"current_page":page,"last_page":((total+size-1)/size).max(1)}}),
    )
}
// One local status per member: current active > current trial > latest effective record.
// Historical rows remain in the audience; future grants are scheduled, not current active.
const COHORT_SQL: &str = r#"WITH ranked AS(SELECT s.*,CASE
 WHEN s.status='active' AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND(s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC')) THEN 'active'
 WHEN s.status IN('trial','trialing') AND s.starts_at<=(NOW() AT TIME ZONE 'UTC') AND(s.expires_at IS NULL OR s.expires_at>(NOW() AT TIME ZONE 'UTC')) AND(s.trial_ends_at IS NULL OR s.trial_ends_at>(NOW() AT TIME ZONE 'UTC')) THEN 'trial'
 WHEN s.status IN('cancelled','canceled') THEN 'cancelled'
 WHEN s.status='expired' OR s.expires_at<=(NOW() AT TIME ZONE 'UTC') OR(s.status IN('trial','trialing') AND s.trial_ends_at<=(NOW() AT TIME ZONE 'UTC')) THEN 'expired'
 WHEN s.starts_at>(NOW() AT TIME ZONE 'UTC') THEN 'scheduled'
 WHEN s.status IN('pending','past_due','paused') THEN s.status ELSE 'other' END AS local_status
 FROM user_memberships s WHERE s.plan_id=$1 AND s.deleted_at IS NULL),
 local_members AS(SELECT DISTINCT ON(user_id) user_id,local_status,id AS membership_id FROM ranked ORDER BY user_id,CASE local_status WHEN 'active' THEN 0 WHEN 'trial' THEN 1 ELSE 2 END,created_at DESC NULLS LAST,id DESC),
 audience AS(SELECT m.*,l.local_status AS service_status,l.membership_id AS service_membership_id FROM admin_member_facts m JOIN local_members l ON l.user_id=m.id WHERE($2::text IS NULL OR m.name ILIKE $2 OR m.email ILIKE $2) AND($3::text IS NULL OR l.local_status=$3))"#;

pub(super) async fn members(
    state: &AppState,
    id: i64,
    params: ServiceMemberQuery,
) -> Result<Value, ApiError> {
    let (page, size) = pagination(params.page, params.per_page)?;
    let pattern = search(params.search)?;
    let status = params.status.filter(|v| !v.is_empty());
    if status.as_deref().is_some_and(|s| {
        !matches!(
            s,
            "active"
                | "trial"
                | "cancelled"
                | "expired"
                | "pending"
                | "past_due"
                | "paused"
                | "scheduled"
                | "other"
        )
    }) {
        return Err(ApiError::validation_error(
            "Invalid plan-local membership status",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    let service: Value = plan_query(id)?
        .build_query_scalar()
        .fetch_optional(&mut *tx)
        .await
        .map_err(storage)?
        .ok_or_else(|| ApiError::not_found("Membership plan not found"))?;
    let mut stats_query = QueryBuilder::<Postgres>::new(COHORT_SQL);
    stats_query.push(r#" SELECT jsonb_build_object('total_members',COUNT(*),'active_members',COUNT(*) FILTER(WHERE service_status='active'),'trial_members',COUNT(*) FILTER(WHERE service_status='trial'),'cancelled_members',COUNT(*) FILTER(WHERE service_status='cancelled'),'expired_members',COUNT(*) FILTER(WHERE service_status='expired'),'other_members',COUNT(*) FILTER(WHERE service_status NOT IN('active','trial','cancelled','expired')),'whole_member_recorded_net_usd_order_value',"#).push(super::member_statistics::ORDER_VALUE_SQL).push(r#") FROM audience"#);
    let stats: Value = stats_query
        .build_query_scalar()
        .bind(id)
        .bind(&pattern)
        .bind(&status)
        .fetch_one(&mut *tx)
        .await
        .map_err(storage)?;
    let mut rows_query = QueryBuilder::<Postgres>::new(COHORT_SQL);
    rows_query.push(r#" SELECT to_jsonb(m)||jsonb_build_object('status_label',initcap(replace(m.status,'_',' '))) FROM audience m ORDER BY m.created_at DESC NULLS LAST,m.id LIMIT $4 OFFSET $5"#);
    let members: Vec<Value> = rows_query
        .build_query_scalar()
        .bind(id)
        .bind(&pattern)
        .bind(&status)
        .bind(size)
        .bind((page - 1) * size)
        .fetch_all(&mut *tx)
        .await
        .map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    let total = stats["total_members"]
        .as_i64()
        .ok_or_else(|| ApiError::database_error("Invalid member count"))?;
    Ok(
        json!({"schema_version":1,"scope":"plan_local_membership_cohort","cohort_rule":"Current active, then current trial, otherwise latest effective nondeleted membership; global member status is independent. Naive membership dates use operational UTC; historical timezone provenance is unverified.","service":service,"stats":stats,"members":members,"pagination":{"total":total,"per_page":size,"current_page":page,"last_page":((total+size-1)/size).max(1)}}),
    )
}
