//! Health check routes

use worker::{Request, Response, RouteContext};
use crate::AppState;

/// Liveness probe - is the service running?
pub async fn liveness(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    Response::from_json(&serde_json::json!({
        "status": "ok",
        "service": "revolution-api",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Readiness probe - is the service ready to accept traffic?
pub async fn readiness(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    // Check database connectivity
    let db_ok = ctx.data.db.is_connected().await;
    
    // Check cache connectivity
    let cache_ok = ctx.data.cache.is_connected().await;
    
    let status = if db_ok && cache_ok { "ready" } else { "not_ready" };
    let http_status = if db_ok && cache_ok { 200 } else { 503 };
    
    Response::from_json(&serde_json::json!({
        "status": status,
        "checks": {
            "database": if db_ok { "ok" } else { "failed" },
            "cache": if cache_ok { "ok" } else { "failed" }
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
    .map(|r| r.with_status(http_status))
}
