//! Admin Reconciliation Routes
//!
//! GET  /api/admin/reconciliation/log   — paginated log of past runs
//! POST /api/admin/reconciliation/run   — trigger a manual run immediately

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{jobs::reconcile_stripe, middleware::admin::AdminUser, AppState};

#[derive(Debug, Deserialize)]
pub struct LogQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn list_log(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(q): Query<LogQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = q.offset.unwrap_or(0);

    #[derive(sqlx::FromRow, serde::Serialize)]
    struct LogRow {
        id: i64,
        run_at: chrono::NaiveDateTime,
        discrepancies_found: i32,
        log: serde_json::Value,
    }

    let rows: Vec<LogRow> = sqlx::query_as(
        r"SELECT id, run_at, discrepancies_found, log
           FROM reconciliation_log
           ORDER BY run_at DESC
           LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "reconcile", error = %e, "Failed to query reconciliation_log");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM reconciliation_log")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "total": total.0,
        "limit": limit,
        "offset": offset,
        "data": rows,
    })))
}

async fn trigger_run(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match reconcile_stripe::run_once(&state).await {
        Ok(discrepancies) => Ok(Json(json!({
            "success": true,
            "discrepancies_found": discrepancies,
        }))),
        Err(e) => {
            tracing::error!(target: "reconcile", error = %e, "Manual reconciliation run failed");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Reconciliation failed: {}", e)})),
            ))
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/log", get(list_log))
        .route("/run", post(trigger_run))
}
