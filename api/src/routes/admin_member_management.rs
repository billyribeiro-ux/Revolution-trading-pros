//! Admin Member Management Controller - ICT 11+ Principal Engineer
//! Apple ICT 7 Grade - January 2026
//!
//! Enterprise-grade member management: CRUD, ban/suspend, activity log,
//! notes, Excel/PDF export. All routes require admin privileges.

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{models::User, AppState};

// ===============================================================================
// AUTHORIZATION
// ===============================================================================

fn require_admin(user: &User) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role == "admin" || role == "super-admin" || role == "super_admin" || role == "developer" {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Access denied",
                "message": "This action requires admin privileges"
            })),
        ))
    }
}

// ===============================================================================
// TYPES
// ===============================================================================

#[derive(Debug, Serialize, FromRow)]
pub struct MemberDetail {
    pub id: i64,
    pub name: Option<String>,
    pub email: String,
    pub role: Option<String>,
    pub avatar_url: Option<String>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub mfa_enabled: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberSubscription {
    pub id: i64,
    pub product_name: Option<String>,
    pub status: String,
    pub price: Option<f64>,
    pub billing_period: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberOrder {
    pub id: i64,
    pub order_number: Option<String>,
    pub total: Option<f64>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberActivity {
    pub id: i64,
    pub user_id: i64,
    pub action: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberNote {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_by: Option<i64>,
    pub created_by_name: Option<String>,
    pub created_at: NaiveDateTime,
}

// ===============================================================================
// REQUEST TYPES
// ===============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: Option<String>,
    pub send_welcome_email: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub avatar_url: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BanMemberRequest {
    pub reason: Option<String>,
    pub duration_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>, // csv, xlsx, pdf
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub action: Option<String>,
}

// ===============================================================================
// GET MEMBER FULL DETAILS
// ===============================================================================

/// GET /admin/member-management/:id - Get member with full details
async fn get_member_full(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Get member basic info
    let member: MemberDetail = sqlx::query_as(
        r#"
        SELECT id, name, email, role, avatar_url, email_verified_at, mfa_enabled, created_at, updated_at
        FROM users WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Member not found"}))))?;

    // Get subscriptions
    let subscriptions: Vec<MemberSubscription> = sqlx::query_as(
        r#"
        SELECT id, product_name, status, price, billing_period,
               started_at, expires_at, cancelled_at, created_at
        FROM user_memberships
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get orders (from stripe_orders if exists, otherwise user_memberships as proxy)
    let orders: Vec<MemberOrder> = sqlx::query_as(
        r#"
        SELECT id, stripe_invoice_id as order_number, price as total, status, created_at
        FROM user_memberships
        WHERE user_id = $1 AND price > 0
        ORDER BY created_at DESC
        LIMIT 50
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get recent activity
    let activity: Vec<MemberActivity> = sqlx::query_as(
        r#"
        SELECT id, user_id, action, description, metadata, ip_address, user_agent, created_at
        FROM user_activity_log
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 20
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get notes
    let notes: Vec<MemberNote> = sqlx::query_as(
        r#"
        SELECT mn.id, mn.user_id, mn.content, mn.created_by, u.name as created_by_name, mn.created_at
        FROM member_notes mn
        LEFT JOIN users u ON u.id = mn.created_by
        WHERE mn.user_id = $1
        ORDER BY mn.created_at DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get ban status
    let ban_info: Option<(String, Option<NaiveDateTime>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT status, banned_until, ban_reason
        FROM user_status
        WHERE user_id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    // Calculate stats
    let total_spent: f64 = subscriptions.iter().filter_map(|s| s.price).sum();
    let active_subs = subscriptions
        .iter()
        .filter(|s| s.status == "active")
        .count();

    // Get tags
    let tags: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT mt.name
        FROM member_tags mt
        JOIN user_member_tags umt ON umt.tag_id = mt.id
        WHERE umt.user_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Engagement score calculation (simplified)
    let engagement_score = calculate_engagement_score(&subscriptions, &activity, &orders);

    // Build timeline
    let timeline = build_member_timeline(&subscriptions, &activity, &orders);

    Ok(Json(json!({
        "member": {
            "id": member.id,
            "name": member.name,
            "email": member.email,
            "role": member.role,
            "avatar_url": member.avatar_url,
            "email_verified": member.email_verified_at.is_some(),
            "mfa_enabled": member.mfa_enabled.unwrap_or(false),
            "created_at": member.created_at,
            "updated_at": member.updated_at,
            "tags": tags,
            "status": ban_info.as_ref().map(|b| b.0.clone()).unwrap_or_else(|| "active".to_string()),
            "banned_until": ban_info.as_ref().and_then(|b| b.1),
            "ban_reason": ban_info.as_ref().and_then(|b| b.2.clone())
        },
        "subscriptions": subscriptions,
        "orders": orders,
        "activity": activity,
        "notes": notes,
        "stats": {
            "total_spent": (total_spent * 100.0).round() / 100.0,
            "active_subscriptions": active_subs,
            "total_orders": orders.len(),
            "member_since_days": (Utc::now().naive_utc() - member.created_at).num_days()
        },
        "engagement_score": engagement_score,
        "timeline": timeline
    })))
}

fn calculate_engagement_score(
    subscriptions: &[MemberSubscription],
    activity: &[MemberActivity],
    orders: &[MemberOrder],
) -> i32 {
    let mut score = 0;

    // Active subscriptions boost
    let active_count = subscriptions
        .iter()
        .filter(|s| s.status == "active")
        .count();
    score += (active_count * 20).min(40) as i32;

    // Activity recency
    if let Some(last_activity) = activity.first() {
        let days_ago = (Utc::now().naive_utc() - last_activity.created_at).num_days();
        if days_ago < 7 {
            score += 30;
        } else if days_ago < 30 {
            score += 20;
        } else if days_ago < 90 {
            score += 10;
        }
    }

    // Orders/spending
    score += (orders.len() * 5).min(30) as i32;

    score.min(100)
}

fn build_member_timeline(
    subscriptions: &[MemberSubscription],
    activity: &[MemberActivity],
    orders: &[MemberOrder],
) -> Vec<serde_json::Value> {
    let mut timeline: Vec<serde_json::Value> = Vec::new();

    // Add subscription events
    for sub in subscriptions.iter().take(10) {
        timeline.push(json!({
            "type": "subscription",
            "title": format!("{} - {}", sub.product_name.as_deref().unwrap_or("Subscription"), sub.status),
            "date": sub.created_at.to_string(),
            "icon": "subscription",
            "meta": {
                "product": sub.product_name,
                "status": sub.status,
                "price": sub.price
            }
        }));
    }

    // Add activity events
    for act in activity.iter().take(10) {
        timeline.push(json!({
            "type": "activity",
            "title": act.action.clone(),
            "date": act.created_at.to_string(),
            "icon": "activity",
            "meta": {
                "description": act.description,
                "ip": act.ip_address
            }
        }));
    }

    // Add order events
    for order in orders.iter().take(10) {
        timeline.push(json!({
            "type": "order",
            "title": format!("Order #{}", order.order_number.as_deref().unwrap_or(&order.id.to_string())),
            "date": order.created_at.to_string(),
            "icon": "order",
            "meta": {
                "total": order.total,
                "status": order.status
            }
        }));
    }

    // Sort by date descending
    timeline.sort_by(|a, b| {
        let date_a = a["date"].as_str().unwrap_or("");
        let date_b = b["date"].as_str().unwrap_or("");
        date_b.cmp(date_a)
    });

    timeline.into_iter().take(20).collect()
}

// ===============================================================================
// CREATE MEMBER
// ===============================================================================

/// POST /admin/member-management - Create new member
async fn create_member(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Validate email
    if !input.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid email address"})),
        ));
    }

    // Check if email exists
    let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if exists.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "A member with this email already exists"})),
        ));
    }

    // Generate password hash
    let password_was_generated = input.password.is_none();
    let password = input.password.unwrap_or_else(|| {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..16)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect()
    });

    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to hash password: {}", e)})),
        )
    })?;

    // Create user
    let member: MemberDetail = sqlx::query_as(
        r#"
        INSERT INTO users (name, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING id, name, email, role, avatar_url, email_verified_at, mfa_enabled, created_at, updated_at
        "#,
    )
    .bind(&input.name)
    .bind(&input.email)
    .bind(&password_hash)
    .bind(input.role.as_deref().unwrap_or("user"))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO user_activity_log (user_id, action, description, created_at)
        VALUES ($1, 'account_created', 'Account created by admin', NOW())
        "#,
    )
    .bind(member.id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member created successfully",
        "member": member,
        "temporary_password": if password_was_generated { Some(password) } else { None }
    })))
}

// ===============================================================================
// UPDATE MEMBER
// ===============================================================================

/// PUT /admin/member-management/:id - Update member
async fn update_member(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Check member exists
    let existing: Option<(i64, String)> =
        sqlx::query_as("SELECT id, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    if existing.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Member not found"})),
        ));
    }

    // Check email uniqueness if changing
    if let Some(ref email) = input.email {
        let email_exists: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM users WHERE email = $1 AND id != $2")
                .bind(email)
                .bind(id)
                .fetch_optional(&state.db.pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    )
                })?;

        if email_exists.is_some() {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({"error": "Email already in use by another member"})),
            ));
        }
    }

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if input.name.is_some() {
        param_idx += 1;
        updates.push(format!("name = ${}", param_idx));
    }
    if input.email.is_some() {
        param_idx += 1;
        updates.push(format!("email = ${}", param_idx));
    }
    if input.role.is_some() {
        param_idx += 1;
        updates.push(format!("role = ${}", param_idx));
    }
    if input.avatar_url.is_some() {
        param_idx += 1;
        updates.push(format!("avatar_url = ${}", param_idx));
    }
    if input.password.is_some() {
        param_idx += 1;
        updates.push(format!("password_hash = ${}", param_idx));
    }

    updates.push("updated_at = NOW()".to_string());

    if updates.len() == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let sql = format!(
        "UPDATE users SET {} WHERE id = $1 RETURNING id, name, email, role, avatar_url, email_verified_at, mfa_enabled, created_at, updated_at",
        updates.join(", ")
    );

    let mut query = sqlx::query_as::<_, MemberDetail>(&sql).bind(id);

    if let Some(ref name) = input.name {
        query = query.bind(name);
    }
    if let Some(ref email) = input.email {
        query = query.bind(email);
    }
    if let Some(ref role) = input.role {
        query = query.bind(role);
    }
    if let Some(ref avatar_url) = input.avatar_url {
        query = query.bind(avatar_url);
    }
    if let Some(ref password) = input.password {
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to hash password: {}", e)})),
            )
        })?;
        query = query.bind(hash);
    }

    let member = query.fetch_one(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO user_activity_log (user_id, action, description, created_at)
        VALUES ($1, 'profile_updated_by_admin', 'Profile updated by admin', NOW())
        "#,
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member updated successfully",
        "member": member
    })))
}

// ===============================================================================
// DELETE MEMBER
// ===============================================================================

/// DELETE /admin/member-management/:id - Delete member (soft delete)
async fn delete_member(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Prevent self-deletion
    if user.id == id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot delete your own account"})),
        ));
    }

    // Check member exists
    let existing: Option<(i64, String)> =
        sqlx::query_as("SELECT id, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    if existing.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Member not found"})),
        ));
    }

    // Soft delete: anonymize data but keep record for audit
    let deleted_email = format!("deleted_{}@removed.local", id);
    let result = sqlx::query(
        r#"
        UPDATE users
        SET email = $2,
            name = 'Deleted User',
            password_hash = 'DELETED',
            role = 'deleted',
            avatar_url = NULL,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(&deleted_email)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Member not found"})),
        ));
    }

    // Cancel active subscriptions
    let _ = sqlx::query(
        "UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW() WHERE user_id = $1 AND status = 'active'",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO user_activity_log (user_id, action, description, created_at)
        VALUES ($1, 'account_deleted', 'Account deleted by admin', NOW())
        "#,
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member deleted successfully"
    })))
}

// ===============================================================================
// BAN/SUSPEND/UNBAN MEMBER
// ===============================================================================

/// POST /admin/member-management/:id/ban - Ban member
async fn ban_member(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<BanMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    // Prevent self-ban
    if user.id == id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot ban your own account"})),
        ));
    }

    // Calculate ban expiry
    let banned_until = input
        .duration_days
        .map(|days| Utc::now().naive_utc() + chrono::Duration::days(days as i64));

    // Upsert user status
    sqlx::query(
        r#"
        INSERT INTO user_status (user_id, status, banned_until, ban_reason, updated_at)
        VALUES ($1, 'banned', $2, $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            status = 'banned',
            banned_until = $2,
            ban_reason = $3,
            updated_at = NOW()
        "#,
    )
    .bind(id)
    .bind(banned_until)
    .bind(&input.reason)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Log activity
    let _ = sqlx::query(
        r#"
        INSERT INTO user_activity_log (user_id, action, description, metadata, created_at)
        VALUES ($1, 'account_banned', $2, $3, NOW())
        "#,
    )
    .bind(id)
    .bind(format!(
        "Account banned{}",
        input
            .duration_days
            .map(|d| format!(" for {} days", d))
            .unwrap_or_default()
    ))
    .bind(json!({
        "reason": input.reason,
        "duration_days": input.duration_days,
        "banned_by": user.id
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member banned successfully",
        "banned_until": banned_until
    })))
}

/// POST /admin/member-management/:id/suspend - Suspend member (temporary)
async fn suspend_member(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<BanMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    if user.id == id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "You cannot suspend your own account"})),
        ));
    }

    let suspended_until = input
        .duration_days
        .map(|days| Utc::now().naive_utc() + chrono::Duration::days(days as i64));

    sqlx::query(
        r#"
        INSERT INTO user_status (user_id, status, banned_until, ban_reason, updated_at)
        VALUES ($1, 'suspended', $2, $3, NOW())
        ON CONFLICT (user_id) DO UPDATE SET
            status = 'suspended',
            banned_until = $2,
            ban_reason = $3,
            updated_at = NOW()
        "#,
    )
    .bind(id)
    .bind(suspended_until)
    .bind(&input.reason)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let _ = sqlx::query(
        r#"
        INSERT INTO user_activity_log (user_id, action, description, metadata, created_at)
        VALUES ($1, 'account_suspended', $2, $3, NOW())
        "#,
    )
    .bind(id)
    .bind(format!(
        "Account suspended{}",
        input
            .duration_days
            .map(|d| format!(" for {} days", d))
            .unwrap_or_default()
    ))
    .bind(json!({
        "reason": input.reason,
        "duration_days": input.duration_days,
        "suspended_by": user.id
    }))
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member suspended successfully",
        "suspended_until": suspended_until
    })))
}

/// POST /admin/member-management/:id/unban - Unban/unsuspend member
async fn unban_member(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    sqlx::query(
        r#"
        UPDATE user_status
        SET status = 'active', banned_until = NULL, ban_reason = NULL, updated_at = NOW()
        WHERE user_id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let _ = sqlx::query(
        r#"
        INSERT INTO user_activity_log (user_id, action, description, metadata, created_at)
        VALUES ($1, 'account_unbanned', 'Account unbanned by admin', $2, NOW())
        "#,
    )
    .bind(id)
    .bind(json!({"unbanned_by": user.id}))
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member unbanned successfully"
    })))
}

// ===============================================================================
// MEMBER NOTES
// ===============================================================================

/// GET /admin/member-management/:id/notes - Get member notes
async fn get_notes(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<Vec<MemberNote>>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let notes: Vec<MemberNote> = sqlx::query_as(
        r#"
        SELECT mn.id, mn.user_id, mn.content, mn.created_by, u.name as created_by_name, mn.created_at
        FROM member_notes mn
        LEFT JOIN users u ON u.id = mn.created_by
        WHERE mn.user_id = $1
        ORDER BY mn.created_at DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(notes))
}

/// POST /admin/member-management/:id/notes - Create note
async fn create_note(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<CreateNoteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    if input.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Note content cannot be empty"})),
        ));
    }

    let note: MemberNote = sqlx::query_as(
        r#"
        INSERT INTO member_notes (user_id, content, created_by, created_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id, user_id, content, created_by, NULL::text as created_by_name, created_at
        "#,
    )
    .bind(id)
    .bind(&input.content)
    .bind(user.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "message": "Note created successfully",
        "note": note
    })))
}

/// DELETE /admin/member-management/:id/notes/:note_id - Delete note
async fn delete_note(
    State(state): State<AppState>,
    user: User,
    Path((member_id, note_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let result = sqlx::query("DELETE FROM member_notes WHERE id = $1 AND user_id = $2")
        .bind(note_id)
        .bind(member_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Note not found"})),
        ));
    }

    Ok(Json(json!({
        "message": "Note deleted successfully"
    })))
}

// ===============================================================================
// ACTIVITY LOG
// ===============================================================================

/// GET /admin/member-management/:id/activity - Get member activity log
async fn get_activity(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Query(query): Query<ActivityQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;

    let activity: Vec<MemberActivity> = if let Some(ref action) = query.action {
        sqlx::query_as(
            r#"
            SELECT id, user_id, action, description, metadata, ip_address, user_agent, created_at
            FROM user_activity_log
            WHERE user_id = $1 AND action = $2
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(id)
        .bind(action)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as(
            r#"
            SELECT id, user_id, action, description, metadata, ip_address, user_agent, created_at
            FROM user_activity_log
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
    };

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM user_activity_log WHERE user_id = $1")
            .bind(id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(0);

    Ok(Json(json!({
        "activity": activity,
        "pagination": {
            "total": total,
            "per_page": per_page,
            "current_page": page,
            "last_page": ((total as f64) / (per_page as f64)).ceil() as i64
        }
    })))
}

// ===============================================================================
// EXPORT - EXCEL & PDF
// ===============================================================================

/// GET /admin/member-management/export - Export members
async fn export_members(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ExportQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    require_admin(&user)?;

    let format = query.format.as_deref().unwrap_or("csv");

    // Build query conditions
    let mut conditions = Vec::new();

    if let Some(ref status) = query.status {
        match status.as_str() {
            "active" => conditions.push("u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')".to_string()),
            "trial" => conditions.push("u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'trial')".to_string()),
            "churned" => conditions.push("u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status IN ('cancelled', 'expired')) AND u.id NOT IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')".to_string()),
            _ => {}
        }
    }

    if let Some(ref date_from) = query.date_from {
        if is_valid_date(date_from) {
            conditions.push(format!("u.created_at >= '{}'", date_from));
        }
    }
    if let Some(ref date_to) = query.date_to {
        if is_valid_date(date_to) {
            conditions.push(format!("u.created_at <= '{}'", date_to));
        }
    }

    let where_clause = if conditions.is_empty() {
        "1=1".to_string()
    } else {
        conditions.join(" AND ")
    };

    // Fetch members
    #[allow(clippy::type_complexity)]
    let members: Vec<(i64, Option<String>, String, Option<String>, NaiveDateTime)> =
        sqlx::query_as(&format!(
            r#"
        SELECT u.id, u.name, u.email, u.role, u.created_at
        FROM users u
        WHERE {}
        ORDER BY u.created_at DESC
        LIMIT 10000
        "#,
            where_clause
        ))
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    match format {
        "xlsx" => {
            // Generate Excel file
            let xlsx_content = generate_xlsx(&members);
            Ok((
                [
                    (
                        header::CONTENT_TYPE,
                        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    ),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"members.xlsx\"",
                    ),
                ],
                xlsx_content,
            )
                .into_response())
        }
        "pdf" => {
            // Generate PDF (simplified - in production use a proper PDF library)
            let pdf_content = generate_pdf(&members);
            Ok((
                [
                    (header::CONTENT_TYPE, "application/pdf"),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"members.pdf\"",
                    ),
                ],
                pdf_content,
            )
                .into_response())
        }
        _ => {
            // CSV (default)
            let mut csv = String::from("ID,Name,Email,Role,Created At\n");
            for (id, name, email, role, created_at) in &members {
                csv.push_str(&format!(
                    "{},{},{},{},{}\n",
                    id,
                    name.as_deref().unwrap_or("").replace(',', ";"),
                    email.replace(',', ";"),
                    role.as_deref().unwrap_or("user"),
                    created_at
                ));
            }
            Ok((
                [
                    (header::CONTENT_TYPE, "text/csv"),
                    (
                        header::CONTENT_DISPOSITION,
                        "attachment; filename=\"members.csv\"",
                    ),
                ],
                csv,
            )
                .into_response())
        }
    }
}

fn is_valid_date(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();
    parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

#[allow(clippy::type_complexity)]
fn generate_xlsx(
    members: &[(i64, Option<String>, String, Option<String>, NaiveDateTime)],
) -> Vec<u8> {
    // Simple XLSX generation using xlsx-writer pattern
    // In production, use the `rust_xlsxwriter` or `calamine` crate
    // For now, return a simple XML-based spreadsheet
    let mut content = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Workbook xmlns="urn:schemas-microsoft-com:office:spreadsheet"
 xmlns:ss="urn:schemas-microsoft-com:office:spreadsheet">
 <Worksheet ss:Name="Members">
  <Table>
   <Row>
    <Cell><Data ss:Type="String">ID</Data></Cell>
    <Cell><Data ss:Type="String">Name</Data></Cell>
    <Cell><Data ss:Type="String">Email</Data></Cell>
    <Cell><Data ss:Type="String">Role</Data></Cell>
    <Cell><Data ss:Type="String">Created At</Data></Cell>
   </Row>
"#,
    );

    for (id, name, email, role, created_at) in members {
        content.push_str(&format!(
            r#"   <Row>
    <Cell><Data ss:Type="Number">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
    <Cell><Data ss:Type="String">{}</Data></Cell>
   </Row>
"#,
            id,
            escape_xml(name.as_deref().unwrap_or("")),
            escape_xml(email),
            escape_xml(role.as_deref().unwrap_or("user")),
            created_at
        ));
    }

    content.push_str(
        r#"  </Table>
 </Worksheet>
</Workbook>"#,
    );

    content.into_bytes()
}

#[allow(clippy::type_complexity)]
fn generate_pdf(
    members: &[(i64, Option<String>, String, Option<String>, NaiveDateTime)],
) -> Vec<u8> {
    // Simple PDF generation
    // In production, use a proper PDF library like `printpdf` or `genpdf`
    // For now, generate a text-based PDF structure
    let mut lines = vec![
        "Revolution Trading Pros - Members Export".to_string(),
        format!("Generated: {}", Utc::now().format("%Y-%m-%d %H:%M:%S")),
        format!("Total Members: {}", members.len()),
        "".to_string(),
        "ID | Name | Email | Role | Created At".to_string(),
        "-".repeat(80),
    ];

    for (id, name, email, role, created_at) in members.iter().take(1000) {
        lines.push(format!(
            "{} | {} | {} | {} | {}",
            id,
            name.as_deref().unwrap_or("N/A"),
            email,
            role.as_deref().unwrap_or("user"),
            created_at.format("%Y-%m-%d")
        ));
    }

    // Create a minimal valid PDF
    let content = lines.join("\n");
    let pdf = format!(
        r#"%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>
endobj
4 0 obj
<< /Length {} >>
stream
BT
/F1 10 Tf
50 750 Td
12 TL
{}
ET
endstream
endobj
5 0 obj
<< /Type /Font /Subtype /Type1 /BaseFont /Courier >>
endobj
xref
0 6
0000000000 65535 f
0000000009 00000 n
0000000058 00000 n
0000000115 00000 n
0000000268 00000 n
trailer
<< /Size 6 /Root 1 0 R >>
startxref
{}
%%EOF"#,
        content.len() + 50,
        content
            .lines()
            .map(|l| format!("({}) Tj T*", escape_pdf(l)))
            .collect::<Vec<_>>()
            .join("\n"),
        500 + content.len()
    );

    pdf.into_bytes()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn escape_pdf(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

// ===============================================================================
// ROUTER
// ===============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        // CRUD
        .route("/", post(create_member))
        .route("/export", get(export_members))
        .route(
            "/:id",
            get(get_member_full)
                .put(update_member)
                .delete(delete_member),
        )
        // Ban/Suspend/Unban
        .route("/:id/ban", post(ban_member))
        .route("/:id/suspend", post(suspend_member))
        .route("/:id/unban", post(unban_member))
        // Notes
        .route("/:id/notes", get(get_notes).post(create_note))
        .route("/:id/notes/:note_id", delete(delete_note))
        // Activity
        .route("/:id/activity", get(get_activity))
}
