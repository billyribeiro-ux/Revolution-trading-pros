//! ICT 7 Grade — Download analytics endpoint for an indicator.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

/// ICT 7: Get download analytics for an indicator
pub(super) async fn get_download_analytics(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(indicator_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Total downloads
    let total_downloads: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(COALESCE(download_count, 0)), 0) FROM indicator_files WHERE indicator_id = $1",
    )
    .bind(indicator_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Downloads by platform
    let by_platform: Vec<(String, i64)> = sqlx::query_as(
        r"
        SELECT platform, COALESCE(SUM(COALESCE(download_count, 0)), 0) as count
        FROM indicator_files
        WHERE indicator_id = $1
        GROUP BY platform
        ORDER BY count DESC
        ",
    )
    .bind(indicator_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicator_id": indicator_id,
            "total_downloads": total_downloads.0,
            "by_platform": by_platform.into_iter().map(|(p, c)| json!({"platform": p, "downloads": c})).collect::<Vec<_>>()
        }
    })))
}
