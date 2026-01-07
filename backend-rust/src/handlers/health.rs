//! Health Check Handlers
//!
//! ICT 11+ Principal Engineer Grade
//! Kubernetes-compatible health probes

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub database: &'static str,
    pub redis: &'static str,
}

/// Liveness probe - is the application alive?
pub async fn liveness() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        message: "Application is running",
    })
}

/// Readiness probe - is the application ready to serve traffic?
pub async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<ReadinessResponse>) {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    // Check Redis connection
    let redis_status = match &state.redis {
        Some(client) => match client.get_connection() {
            Ok(_) => "connected",
            Err(_) => "disconnected",
        },
        None => "not_configured",
    };

    let all_healthy = db_status == "connected";
    let status_code = if all_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(ReadinessResponse {
            status: if all_healthy { "ready" } else { "not_ready" },
            database: db_status,
            redis: redis_status,
        }),
    )
}

/// Optimization status endpoint
pub async fn optimization() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "rust_version": env!("CARGO_PKG_VERSION"),
        "optimization_level": if cfg!(debug_assertions) { "debug" } else { "release" },
    }))
}
