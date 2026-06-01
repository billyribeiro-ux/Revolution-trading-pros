//! Monitoring and Observability Module
//! ICT 11+ Principal Engineer Grade - Production Monitoring

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use subtle::ConstantTimeEq;

/// Application metrics
#[derive(Clone)]
pub struct Metrics {
    pub requests_total: Arc<AtomicU64>,
    pub requests_success: Arc<AtomicU64>,
    pub requests_error: Arc<AtomicU64>,
    pub auth_attempts: Arc<AtomicU64>,
    pub auth_success: Arc<AtomicU64>,
    pub auth_failures: Arc<AtomicU64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            requests_total: Arc::new(AtomicU64::new(0)),
            requests_success: Arc::new(AtomicU64::new(0)),
            requests_error: Arc::new(AtomicU64::new(0)),
            auth_attempts: Arc::new(AtomicU64::new(0)),
            auth_success: Arc::new(AtomicU64::new(0)),
            auth_failures: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl Metrics {
    /// Increment request counter
    pub fn record_request(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    /// Record successful request
    pub fn record_success(&self) {
        self.requests_success.fetch_add(1, Ordering::Relaxed);
    }

    /// Record error
    pub fn record_error(&self) {
        self.requests_error.fetch_add(1, Ordering::Relaxed);
    }

    /// Record auth attempt
    pub fn record_auth_attempt(&self) {
        self.auth_attempts.fetch_add(1, Ordering::Relaxed);
    }

    /// Record successful auth
    pub fn record_auth_success(&self) {
        self.auth_success.fetch_add(1, Ordering::Relaxed);
    }

    /// Record failed auth
    pub fn record_auth_failure(&self) {
        self.auth_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            requests_total: self.requests_total.load(Ordering::Relaxed),
            requests_success: self.requests_success.load(Ordering::Relaxed),
            requests_error: self.requests_error.load(Ordering::Relaxed),
            auth_attempts: self.auth_attempts.load(Ordering::Relaxed),
            auth_success: self.auth_success.load(Ordering::Relaxed),
            auth_failures: self.auth_failures.load(Ordering::Relaxed),
        }
    }
}

/// Metrics snapshot for serialization
#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    pub requests_total: u64,
    pub requests_success: u64,
    pub requests_error: u64,
    pub auth_attempts: u64,
    pub auth_success: u64,
    pub auth_failures: u64,
}

/// Prometheus-compatible metrics endpoint
/// GET /metrics
async fn metrics_endpoint(State(metrics): State<Metrics>) -> String {
    let snapshot = metrics.snapshot();

    format!(
        r"# HELP requests_total Total number of requests
# TYPE requests_total counter
requests_total {}

# HELP requests_success Total number of successful requests
# TYPE requests_success counter
requests_success {}

# HELP requests_error Total number of error requests
# TYPE requests_error counter
requests_error {}

# HELP auth_attempts Total number of authentication attempts
# TYPE auth_attempts counter
auth_attempts {}

# HELP auth_success Total number of successful authentications
# TYPE auth_success counter
auth_success {}

# HELP auth_failures Total number of failed authentications
# TYPE auth_failures counter
auth_failures {}
",
        snapshot.requests_total,
        snapshot.requests_success,
        snapshot.requests_error,
        snapshot.auth_attempts,
        snapshot.auth_success,
        snapshot.auth_failures,
    )
}

/// JSON metrics endpoint for dashboards
/// GET /metrics/json
async fn metrics_json(State(metrics): State<Metrics>) -> Json<MetricsSnapshot> {
    Json(metrics.snapshot())
}

/// Health check with detailed status
/// GET /health/detailed
async fn health_detailed() -> (StatusCode, Json<serde_json::Value>) {
    // ICT 11+ Fix: Use unwrap_or_default to prevent panic on system clock issues
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "uptime_seconds": uptime,
            "version": env!("CARGO_PKG_VERSION"),
            "rust_version": env!("CARGO_PKG_RUST_VERSION"),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
    )
}

/// Authorization guard for the monitoring surface.
///
/// SECURITY FIX (FULL_REPO_AUDIT_2026-05-17 P1-4 #3): `/monitoring/metrics`,
/// `/monitoring/metrics/json`, and `/monitoring/health/detailed` were
/// completely unauthenticated. They expose request/auth-failure counters
/// (an attacker's brute-force feedback channel), process uptime, and exact
/// crate/rustc versions — a recon + info-disclosure surface.
///
/// ROOT-CAUSE / DESIGN NOTE: this sub-router is structurally `Router<Metrics>`
/// — `main.rs` finalizes it with `monitoring::router().with_state(metrics)`
/// before nesting, so `AppState` is *not* in scope here and the cookie/JWT
/// `AdminUser` extractor (which is `FromRequestParts<AppState>`) cannot be
/// used at this layer without editing `main.rs` (out of ownership). User-
/// identity auth is also the wrong primitive for a machine-scraped Prometheus
/// target, which presents no JWT. The durable fix is the industry-standard
/// pattern: a shared-secret scrape token validated in constant time.
///
/// Behaviour (fail-closed):
///   - `MONITORING_TOKEN` set  -> require `Authorization: Bearer <token>`,
///     compared in constant time. Missing/wrong token -> 401.
///   - `MONITORING_TOKEN` unset + ENVIRONMENT=development/dev -> allow,
///     mirroring the dev-fallback house style used by `setup_db`/`init_db`
///     and `config::required_or_dev` so the local stack can still scrape.
///   - `MONITORING_TOKEN` unset + any other ENVIRONMENT -> 503. The endpoint
///     refuses to serve recon data rather than falling open.
async fn require_monitoring_token(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let configured = std::env::var("MONITORING_TOKEN")
        .ok()
        .filter(|t| !t.is_empty());

    match configured {
        Some(expected) => {
            let presented = request
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
                .map(str::trim)
                .unwrap_or("");

            // Constant-time comparison (subtle) to deny a timing oracle on
            // the scrape token. ct_eq over equal-length byte slices; unequal
            // lengths can never match so we short-circuit without leaking the
            // expected length through control flow timing on the hot path.
            let ok = presented.len() == expected.len()
                && bool::from(presented.as_bytes().ct_eq(expected.as_bytes()));

            if ok {
                next.run(request).await
            } else {
                tracing::warn!(
                    target: "security",
                    event = "unauthorized_monitoring_access",
                    "Rejected monitoring scrape with missing/invalid token"
                );
                (StatusCode::UNAUTHORIZED, "Invalid monitoring token").into_response()
            }
        }
        None => {
            let environment =
                std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
            if environment == "development" || environment == "dev" {
                next.run(request).await
            } else {
                tracing::error!(
                    target: "security",
                    event = "monitoring_token_unconfigured",
                    "MONITORING_TOKEN is not set outside development; refusing to expose \
                     monitoring endpoints (fail-closed)"
                );
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Monitoring endpoints are disabled: MONITORING_TOKEN is not configured",
                )
                    .into_response()
            }
        }
    }
}

/// Create monitoring router
///
/// All three routes are sensitive (counters / uptime / version recon) and are
/// gated by [`require_monitoring_token`]. There is no basic-liveness route in
/// this module — the public `/health` and load-balancer `/ready` probes live
/// in `routes::health` and are unaffected — so nothing here is left public.
pub fn router() -> Router<Metrics> {
    Router::new()
        .route("/metrics", get(metrics_endpoint))
        .route("/metrics/json", get(metrics_json))
        .route("/health/detailed", get(health_detailed))
        .layer(axum::middleware::from_fn(require_monitoring_token))
}

/// ICT 11+ Metrics Middleware Layer
/// Tracks all requests and their success/failure status
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn metrics_middleware(
    State(metrics): State<Metrics>,
    request: Request,
    next: Next,
) -> Response {
    // Record request
    metrics.record_request();

    // Process request
    let response = next.run(request).await;

    // Record success/error based on status
    if response.status().is_success() {
        metrics.record_success();
    } else if response.status().is_client_error() || response.status().is_server_error() {
        metrics.record_error();
    }

    response
}
