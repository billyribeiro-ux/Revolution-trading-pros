// ═══════════════════════════════════════════════════════════════════════════
// Database Migration Endpoint
// ICT 11+ Pattern: Admin-only endpoint to run migrations remotely
// ═══════════════════════════════════════════════════════════════════════════

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/run-schedule-migration", post(run_schedule_migration))
}

/// POST /api/migrate/run-schedule-migration
/// Run the trading room schedules migration (admin only)
async fn run_schedule_migration(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        "Admin running schedule migration"
    );

    let migration_sql = include_str!("../../migrations/013_trading_room_schedules.sql");

    // Execute the migration
    match sqlx::raw_sql(migration_sql).execute(&state.db.pool).await {
        Ok(_) => {
            tracing::info!("Schedule migration completed successfully");
            Ok(Json(json!({
                "success": true,
                "message": "Schedule migration completed successfully",
                "tables_created": [
                    "trading_room_schedules",
                    "schedule_exceptions"
                ],
                "seed_data": {
                    "day_trading_room": 6,
                    "swing_trading_room": 3,
                    "small_account_mentorship": 3
                }
            })))
        }
        Err(e) => {
            tracing::error!("Migration failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Migration failed: {}", e)
                })),
            ))
        }
    }
}
