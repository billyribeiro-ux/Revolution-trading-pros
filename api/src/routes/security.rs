//! Security Events & Audit Log Routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Security event tracking and audit logging for compliance and monitoring

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SecurityEvent {
    pub id: i64,
    pub user_id: Option<i64>,
    pub event_type: String,
    pub event_category: String,
    pub severity: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct SecurityEventsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub user_id: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Handlers
// ═══════════════════════════════════════════════════════════════════════════

/// Get security events (admin only)
/// GET /api/security/events
async fn get_security_events(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<SecurityEventsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Admin only
    let role = user.role.as_deref().unwrap_or("user");
    if role != "admin" && role != "super-admin" && role != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Admin access required"})),
        ));
    }

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    // Build query with filters
    let mut sql = String::from(
        "SELECT id, user_id, event_type, event_category, severity, ip_address, user_agent, metadata, created_at 
         FROM security_events WHERE 1=1"
    );
    let mut count_sql = String::from("SELECT COUNT(*) FROM security_events WHERE 1=1");

    if query.event_type.is_some() {
        sql.push_str(" AND event_type = $1");
        count_sql.push_str(" AND event_type = $1");
    }
    if query.severity.is_some() {
        let param_num = if query.event_type.is_some() {
            "$2"
        } else {
            "$1"
        };
        sql.push_str(&format!(" AND severity = {}", param_num));
        count_sql.push_str(&format!(" AND severity = {}", param_num));
    }
    if query.user_id.is_some() {
        let param_num = match (query.event_type.is_some(), query.severity.is_some()) {
            (true, true) => "$3",
            (true, false) | (false, true) => "$2",
            (false, false) => "$1",
        };
        sql.push_str(&format!(" AND user_id = {}", param_num));
        count_sql.push_str(&format!(" AND user_id = {}", param_num));
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT $");
    sql.push_str(
        &(1 + [
            query.event_type.is_some(),
            query.severity.is_some(),
            query.user_id.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count())
        .to_string(),
    );
    sql.push_str(" OFFSET $");
    sql.push_str(
        &(2 + [
            query.event_type.is_some(),
            query.severity.is_some(),
            query.user_id.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count())
        .to_string(),
    );

    // Build query with parameters
    let mut query_builder = sqlx::query_as::<_, SecurityEvent>(&sql);
    let mut count_builder = sqlx::query_as::<_, (i64,)>(&count_sql);

    if let Some(ref event_type) = query.event_type {
        query_builder = query_builder.bind(event_type);
        count_builder = count_builder.bind(event_type);
    }
    if let Some(ref severity) = query.severity {
        query_builder = query_builder.bind(severity);
        count_builder = count_builder.bind(severity);
    }
    if let Some(user_id) = query.user_id {
        query_builder = query_builder.bind(user_id);
        count_builder = count_builder.bind(user_id);
    }

    query_builder = query_builder.bind(per_page).bind(offset);

    let events = query_builder
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    let total = count_builder
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "data": events,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get security event statistics
/// GET /api/security/stats
async fn get_security_stats(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Admin only
    let role = user.role.as_deref().unwrap_or("user");
    if role != "admin" && role != "super-admin" && role != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Admin access required"})),
        ));
    }

    let total_events: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM security_events")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let critical_events: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM security_events WHERE severity = 'critical'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let recent_logins: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM security_events WHERE event_type = 'login_success' AND created_at > NOW() - INTERVAL '24 hours'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let failed_logins: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM security_events WHERE event_type = 'login_failed' AND created_at > NOW() - INTERVAL '24 hours'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "total_events": total_events.0,
        "critical_events": critical_events.0,
        "recent_logins_24h": recent_logins.0,
        "failed_logins_24h": failed_logins.0
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/events",
            get(get_security_events).post(get_security_events),
        )
        .route("/stats", get(get_security_stats))
}
