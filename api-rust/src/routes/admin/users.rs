//! Admin users routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::user::{User, UserPublic};
use crate::models::post::{PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;

/// GET /api/admin/users - List all users
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: UserListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM users WHERE 1=1");
    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(role) = &query.role {
        sql.push_str(&format!(" AND role = ${}", param_idx));
        params.push(serde_json::json!(role));
        param_idx += 1;
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(" AND (email ILIKE ${} OR name ILIKE ${})", param_idx, param_idx));
        params.push(serde_json::json!(format!("%{}%", search)));
        param_idx += 1;
    }

    // Count total
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*) as count");
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    // Add sorting and pagination
    sql.push_str(" ORDER BY created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let users: Vec<User> = ctx.data.db.query(&sql, params).await?;
    let public_users: Vec<UserPublic> = users.into_iter().map(UserPublic::from).collect();

    let response = PaginatedResponse {
        data: public_users,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// GET /api/admin/users/:id - Get a single user
pub async fn show(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing user id".to_string()))?;

    let user: Option<User> = ctx.data.db.query_one(
        "SELECT * FROM users WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    match user {
        Some(u) => {
            // Get additional user data
            let memberships: Vec<serde_json::Value> = ctx.data.db.query(
                r#"
                SELECT pu.*, p.name as product_name, p.product_type
                FROM product_user pu
                JOIN products p ON p.id = pu.product_id
                WHERE pu.user_id = $1
                "#,
                vec![serde_json::json!(id)]
            ).await?;

            let subscriptions: Vec<serde_json::Value> = ctx.data.db.query(
                r#"
                SELECT us.*, sp.name as plan_name
                FROM user_subscriptions us
                JOIN subscription_plans sp ON sp.id = us.plan_id
                WHERE us.user_id = $1
                "#,
                vec![serde_json::json!(id)]
            ).await?;

            Response::from_json(&serde_json::json!({
                "user": UserPublic::from(u),
                "memberships": memberships,
                "subscriptions": subscriptions
            }))
        }
        None => Err(ApiError::NotFound("User not found".to_string()).into())
    }
}

/// POST /api/admin/users/:id/ban - Ban a user
pub async fn ban(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let admin = require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing user id".to_string()))?;

    #[derive(serde::Deserialize)]
    struct BanRequest {
        reason: Option<String>,
    }

    let body: BanRequest = req.json().await.unwrap_or(BanRequest { reason: None });

    // Can't ban yourself
    if id == admin.id.to_string() {
        return Err(ApiError::BadRequest("You cannot ban yourself".to_string()).into());
    }

    // Can't ban super admins
    let target_user: Option<User> = ctx.data.db.query_one(
        "SELECT * FROM users WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    if let Some(user) = &target_user {
        if user.role.is_super_admin() {
            return Err(ApiError::Forbidden("Cannot ban a super admin".to_string()).into());
        }
    }

    let now = chrono::Utc::now();

    ctx.data.db.execute(
        "UPDATE users SET banned_at = $1, ban_reason = $2, updated_at = $1 WHERE id = $3",
        vec![
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(body.reason),
            serde_json::json!(id),
        ]
    ).await?;

    // Invalidate all user sessions
    ctx.data.db.execute(
        "DELETE FROM sessions WHERE user_id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "User banned successfully"
    }))
}

/// POST /api/admin/users/:id/unban - Unban a user
pub async fn unban(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing user id".to_string()))?;

    let now = chrono::Utc::now();

    ctx.data.db.execute(
        "UPDATE users SET banned_at = NULL, ban_reason = NULL, updated_at = $1 WHERE id = $2",
        vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(id)]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "User unbanned successfully"
    }))
}

#[derive(serde::Deserialize, Default)]
struct UserListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    role: Option<String>,
    search: Option<String>,
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}
