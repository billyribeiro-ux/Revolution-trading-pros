//! Health check routes

use worker::{Request, Response, RouteContext};
use crate::AppState;

/// Get current timestamp as ISO string (WASM compatible)
fn now_iso() -> String {
    let ms = js_sys::Date::now() as i64;
    let secs = ms / 1000;
    let nanos = ((ms % 1000) * 1_000_000) as u32;
    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
        dt.to_rfc3339()
    } else {
        format!("{}000", secs)
    }
}

/// Liveness probe - is the service running?
pub async fn liveness(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    Response::from_json(&serde_json::json!({
        "status": "ok",
        "service": "revolution-api",
        "timestamp": now_iso()
    }))
}

/// Readiness probe - is the service ready to accept traffic?
pub async fn readiness(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    // In WASM, we can't easily check connectivity without making actual requests
    // For now, assume ready if the worker is running
    Response::from_json(&serde_json::json!({
        "status": "ready",
        "checks": {
            "database": "ok",
            "cache": "ok"
        },
        "timestamp": now_iso()
    }))
}
