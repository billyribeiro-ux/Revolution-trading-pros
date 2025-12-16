//! Admin subscriptions routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::subscription::{SubscriptionPlan, SubscriptionWithPlan, SubscriptionMetrics};
use crate::models::post::{PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;

/// GET /api/admin/subscriptions - List all subscriptions
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: SubscriptionListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from(
        r#"
        SELECT us.*, 
               json_build_object('id', sp.id, 'name', sp.name, 'price', sp.price) as plan,
               p.name as product_name,
               u.email as user_email, u.name as user_name
        FROM user_subscriptions us
        JOIN subscription_plans sp ON sp.id = us.plan_id
        JOIN products p ON p.id = us.product_id
        JOIN users u ON u.id = us.user_id
        WHERE 1=1
        "#
    );

    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND us.status = ${}", param_idx));
        params.push(serde_json::json!(status));
        param_idx += 1;
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(" AND (u.email ILIKE ${} OR u.name ILIKE ${})", param_idx, param_idx));
        params.push(serde_json::json!(format!("%{}%", search)));
        param_idx += 1;
    }

    // Count total
    let count_sql = format!(
        "SELECT COUNT(*) as count FROM user_subscriptions us JOIN users u ON u.id = us.user_id WHERE 1=1 {}",
        if query.status.is_some() { "AND us.status = $1" } else { "" }
    );
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    sql.push_str(" ORDER BY us.created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let subscriptions: Vec<SubscriptionWithUser> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: subscriptions,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// GET /api/admin/subscriptions/plans - List all subscription plans
pub async fn plans(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let plans: Vec<SubscriptionPlanWithStats> = ctx.data.db.query(
        r#"
        SELECT sp.*,
               p.name as product_name,
               (SELECT COUNT(*) FROM user_subscriptions us WHERE us.plan_id = sp.id AND us.status = 'active') as active_count
        FROM subscription_plans sp
        JOIN products p ON p.id = sp.product_id
        ORDER BY sp.sort_order ASC
        "#,
        vec![]
    ).await?;

    Response::from_json(&plans)
}

/// GET /api/admin/subscriptions/metrics - Get subscription metrics
pub async fn metrics(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let now = crate::utils::now();
    let month_start = now.format("%Y-%m-01").to_string();

    // Active subscriptions
    let active: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM user_subscriptions WHERE status = 'active'",
        vec![]
    ).await?.map(|r| r.count).unwrap_or(0);

    // Trialing
    let trialing: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM user_subscriptions WHERE status = 'trialing'",
        vec![]
    ).await?.map(|r| r.count).unwrap_or(0);

    // Cancelled
    let cancelled: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM user_subscriptions WHERE status = 'cancelled'",
        vec![]
    ).await?.map(|r| r.count).unwrap_or(0);

    // MRR (Monthly Recurring Revenue)
    let mrr: i64 = ctx.data.db.query_one::<SumResult>(
        r#"
        SELECT COALESCE(SUM(sp.price), 0) as total
        FROM user_subscriptions us
        JOIN subscription_plans sp ON sp.id = us.plan_id
        WHERE us.status IN ('active', 'trialing')
        "#,
        vec![]
    ).await?.map(|r| r.total).unwrap_or(0);

    // New this month
    let new_this_month: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM user_subscriptions WHERE created_at >= $1",
        vec![serde_json::json!(month_start)]
    ).await?.map(|r| r.count).unwrap_or(0);

    // Cancelled this month
    let cancelled_this_month: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM user_subscriptions WHERE cancelled_at >= $1",
        vec![serde_json::json!(month_start)]
    ).await?.map(|r| r.count).unwrap_or(0);

    let metrics = SubscriptionMetrics {
        total_active: active,
        total_trialing: trialing,
        total_cancelled: cancelled,
        mrr,
        arr: mrr * 12,
        churn_rate: if active > 0 { (cancelled_this_month as f32 / active as f32) * 100.0 } else { 0.0 },
        average_lifetime_value: 0, // Would need historical data
        new_this_month,
        cancelled_this_month,
    };

    Response::from_json(&metrics)
}

#[derive(serde::Deserialize, Default)]
struct SubscriptionListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
    search: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SubscriptionWithUser {
    #[serde(flatten)]
    subscription: crate::models::subscription::UserSubscription,
    plan: serde_json::Value,
    product_name: String,
    user_email: String,
    user_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct SubscriptionPlanWithStats {
    #[serde(flatten)]
    plan: SubscriptionPlan,
    product_name: String,
    active_count: i64,
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}

#[derive(serde::Deserialize)]
struct SumResult {
    total: i64,
}
