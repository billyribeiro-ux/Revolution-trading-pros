//! Member CRUD handlers: full-detail fetch, create, update, soft-delete,
//! plus engagement-score and timeline helpers used by the detail endpoint.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{NaiveDateTime, Utc};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::{
    CreateMemberRequest, MemberActivity, MemberDetail, MemberNote, MemberOrder, MemberSubscription,
    UpdateMemberRequest,
};

// ===============================================================================
// GET MEMBER FULL DETAILS
// ===============================================================================

/// GET /admin/member-management/:id - Get member with full details
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn get_member_full(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

    // Get member basic info
    let member: MemberDetail = sqlx::query_as(
        r"
        SELECT id, name, email, role, avatar_url, email_verified_at, mfa_enabled, created_at, updated_at
        FROM users WHERE id = $1
        ",
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

    // Subscriptions — joined with membership_plans for canonical price/name (cents)
    let subscriptions: Vec<MemberSubscription> = sqlx::query_as(
        r"
        SELECT um.id,
               mp.name AS product_name,
               um.status,
               (mp.price * 100)::BIGINT AS price_cents,
               mp.billing_cycle AS billing_period,
               um.starts_at AS started_at,
               um.expires_at,
               um.cancelled_at,
               um.created_at
        FROM user_memberships um
        LEFT JOIN membership_plans mp ON mp.id = um.plan_id
        WHERE um.user_id = $1
        ORDER BY um.created_at DESC
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Orders for the user — read from `orders` table (canonical), in cents
    let orders: Vec<MemberOrder> = sqlx::query_as(
        r"
        SELECT id,
               order_number,
               (total * 100)::BIGINT AS total_cents,
               status,
               created_at
        FROM orders
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get recent activity
    let activity: Vec<MemberActivity> = sqlx::query_as(
        r"
        SELECT id, user_id, action, description, metadata, ip_address, user_agent, created_at
        FROM user_activity_log
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 20
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get notes
    let notes: Vec<MemberNote> = sqlx::query_as(
        r"
        SELECT mn.id, mn.user_id, mn.content, mn.created_by, u.name as created_by_name, mn.created_at
        FROM member_notes mn
        LEFT JOIN users u ON u.id = mn.created_by
        WHERE mn.user_id = $1
        ORDER BY mn.created_at DESC
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get ban status
    let ban_info: Option<(String, Option<NaiveDateTime>, Option<String>)> = sqlx::query_as(
        r"
        SELECT status, banned_until, ban_reason
        FROM user_status
        WHERE user_id = $1
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    // Calculate stats
    let total_spent_cents: i64 = subscriptions.iter().filter_map(|s| s.price_cents).sum();
    let active_subs = subscriptions
        .iter()
        .filter(|s| s.status == "active")
        .count();

    // Get tags
    let tags: Vec<String> = sqlx::query_scalar(
        r"
        SELECT mt.name
        FROM member_tags mt
        JOIN user_member_tags umt ON umt.tag_id = mt.id
        WHERE umt.user_id = $1
        ",
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
            "total_spent_cents": total_spent_cents,
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
                "price_cents": sub.price_cents
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
                "total_cents": order.total_cents,
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
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn create_member(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

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
        use rand::distr::Alphanumeric;
        use rand::RngExt;
        let mut rng = rand::rng();
        (0..16).map(|_| rng.sample(Alphanumeric) as char).collect()
    });

    // FIX-2026-04-26 (Priority 7): use Argon2id (crate::utils::hash_password) instead of
    // bcrypt to standardize on the same hashing algorithm as auth.rs::register.
    // Original line:
    // let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|e| {
    //     (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to hash password: {}", e)})))
    // })?;
    let password_hash = crate::utils::hash_password(&password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to hash password: {}", e)})),
        )
    })?;

    // FIX-2026-04-26 (Priority 5): wrap user-insert + activity-log-insert in a tx.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

    // Create user
    // Original pool reference: .fetch_one(&state.db.pool)
    let member: MemberDetail = sqlx::query_as(
        r"
        INSERT INTO users (name, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING id, name, email, role, avatar_url, email_verified_at, mfa_enabled, created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(&input.email)
    .bind(&password_hash)
    .bind(input.role.as_deref().unwrap_or("user"))
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Log activity
    // Original pool reference: .execute(&state.db.pool)
    let _ = sqlx::query(
        r"
        INSERT INTO user_activity_log (user_id, action, description, created_at)
        VALUES ($1, 'account_created', 'Account created by admin', NOW())
        ",
    )
    .bind(member.id)
    .execute(&mut *tx)
    .await;

    // FIX-2026-04-26 (Priority 5): commit the create-member transaction.
    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?;

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
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn update_member(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

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
        updates.push(format!("name = ${param_idx}"));
    }
    if input.email.is_some() {
        param_idx += 1;
        updates.push(format!("email = ${param_idx}"));
    }
    if input.role.is_some() {
        param_idx += 1;
        updates.push(format!("role = ${param_idx}"));
    }
    if input.avatar_url.is_some() {
        param_idx += 1;
        updates.push(format!("avatar_url = ${param_idx}"));
    }
    if input.password.is_some() {
        param_idx += 1;
        updates.push(format!("password_hash = ${param_idx}"));
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

    let mut query = sqlx::query_as::<_, MemberDetail>(sqlx::AssertSqlSafe(sql.as_str())).bind(id);

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
        // FIX-2026-04-26 (Priority 7): use Argon2id instead of bcrypt — same standardization.
        // Original line:
        // let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        //     (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to hash password: {}", e)})))
        // })?;
        let hash = crate::utils::hash_password(password).map_err(|e| {
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
        r"
        INSERT INTO user_activity_log (user_id, action, description, created_at)
        VALUES ($1, 'profile_updated_by_admin', 'Profile updated by admin', NOW())
        ",
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
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn delete_member(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Prevent self-deletion
    if admin.0.id == id {
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
    let deleted_email = format!("deleted_{id}@removed.local");
    let result = sqlx::query(
        r"
        UPDATE users
        SET email = $2,
            name = 'Deleted User',
            password_hash = 'DELETED',
            role = 'deleted',
            avatar_url = NULL,
            updated_at = NOW()
        WHERE id = $1
        ",
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
        r"
        INSERT INTO user_activity_log (user_id, action, description, created_at)
        VALUES ($1, 'account_deleted', 'Account deleted by admin', NOW())
        ",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "message": "Member deleted successfully"
    })))
}
