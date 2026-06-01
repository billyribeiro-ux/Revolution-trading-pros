//! Member activity log handler (paginated, optional action filter).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::{ActivityQuery, MemberActivity};

/// GET /admin/member-management/:id/activity - Get member activity log
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn get_activity(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<ActivityQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(25).min(100);
    let offset = (page - 1) * per_page;

    let activity: Vec<MemberActivity> = if let Some(ref action) = query.action {
        sqlx::query_as(
            r"
            SELECT id, user_id, action, description, metadata, ip_address, user_agent, created_at
            FROM user_activity_log
            WHERE user_id = $1 AND action = $2
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            ",
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
            r"
            SELECT id, user_id, action, description, metadata, ip_address, user_agent, created_at
            FROM user_activity_log
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            ",
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
