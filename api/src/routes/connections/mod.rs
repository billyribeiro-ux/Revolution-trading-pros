//! Connections Routes - ICT Level 7 Principal Engineer Grade
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! SECURITY: All endpoints require AdminUser authentication
//! Features:
//! - Third-party service connections with database storage
//! - API keys encrypted at rest
//! - Stripe connection management
//! - Email provider settings
//! - Integration webhooks
//! - Connection health monitoring
//! - Audit logging for all changes
//!
//! Split from a single 1559-LOC `connections.rs` file (R20-B
//! maintainability pass, 2026-05-20). Public API unchanged:
//! external callers still import `routes::connections::admin_router`
//! and the test-visible DTOs (`ServiceConnection`,
//! `IntegrationWebhook`, `ConnectServiceRequest`, etc.) at the
//! same path.

use axum::{
    routing::{delete, get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

use crate::AppState;

mod audit;
mod audit_log;
mod crypto;
mod definitions;
mod handlers;
mod webhooks;

// ═══════════════════════════════════════════════════════════════════════════
// SHARED TYPES (DTOs surfaced to integration tests + re-exported below)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ServiceConnection {
    pub id: i64,
    pub service_key: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub status: String,
    pub health_score: i32,
    pub health_status: Option<String>,
    pub environment: Option<String>,
    pub credentials_encrypted: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub webhook_url: Option<String>,
    pub webhook_secret: Option<String>,
    pub api_calls_today: i32,
    pub api_calls_total: i64,
    pub last_error: Option<String>,
    pub last_verified_at: Option<DateTime<Utc>>,
    pub connected_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IntegrationWebhook {
    pub id: i64,
    pub connection_id: i64,
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub events: Option<serde_json::Value>,
    pub is_active: bool,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub last_status_code: Option<i32>,
    pub failure_count: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectServiceRequest {
    pub credentials: HashMap<String, String>,
    pub environment: Option<String>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConnectionRequest {
    pub credentials: Option<HashMap<String, String>>,
    pub environment: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub url: String,
    pub events: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWebhookRequest {
    pub name: Option<String>,
    pub url: Option<String>,
    pub events: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub category: Option<String>,
    pub status: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER (preserves the pre-split public API exactly)
// ═══════════════════════════════════════════════════════════════════════════

/// Build the connections admin router
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Main connection routes
        .route("/", get(handlers::get_connections_status))
        .route("/status", get(handlers::get_connections_status))
        .route("/summary", get(handlers::get_connections_summary))
        .route("/categories", get(handlers::get_categories_handler))
        .route("/audit-logs", get(audit_log::get_audit_logs))
        // Individual connection routes
        .route(
            "/{key}",
            get(handlers::get_connection).delete(handlers::delete_connection),
        )
        .route("/{key}/connect", post(handlers::connect_service))
        .route("/{key}/test", post(handlers::test_connection))
        .route("/{key}/disconnect", post(handlers::disconnect_service))
        // Webhook routes
        .route(
            "/{key}/webhooks",
            get(webhooks::list_webhooks).post(webhooks::create_webhook),
        )
        .route("/{key}/webhooks/{id}", delete(webhooks::delete_webhook))
}
