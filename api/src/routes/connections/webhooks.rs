//! Webhook handlers for service connections (split from `connections.rs`
//! lines 1234-1462, R20-B maintainability pass, 2026-05-20). Behavior
//! preserved verbatim.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

use super::audit::log_connection_audit;
use super::crypto::generate_webhook_secret;
use super::{CreateWebhookRequest, IntegrationWebhook};

/// GET /admin/connections/:key/webhooks - Get webhooks for a connection
#[tracing::instrument(skip(state, admin))]
pub(super) async fn list_webhooks(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Admin listing webhooks"
    );

    // Get connection ID
    let connection_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM service_connections WHERE service_key = $1")
            .bind(&key)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let connection_id = connection_id.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Connection not found"})),
    ))?;

    let webhooks: Vec<IntegrationWebhook> = sqlx::query_as(
        r"
        SELECT id, connection_id, name, url, secret, events, is_active,
               last_triggered_at, last_status_code, failure_count,
               created_at, updated_at
        FROM integration_webhooks
        WHERE connection_id = $1
        ORDER BY name
        ",
    )
    .bind(connection_id)
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    // Mask secrets
    let webhooks_masked: Vec<serde_json::Value> = webhooks
        .into_iter()
        .map(|w| {
            json!({
                "id": w.id,
                "name": w.name,
                "url": w.url,
                "secret_set": w.secret.is_some(),
                "events": w.events,
                "is_active": w.is_active,
                "last_triggered_at": w.last_triggered_at,
                "last_status_code": w.last_status_code,
                "failure_count": w.failure_count
            })
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": webhooks_masked
    })))
}

/// POST /admin/connections/:key/webhooks - Create webhook
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn create_webhook(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
    Json(input): Json<CreateWebhookRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate URL
    if !input.url.starts_with("https://") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Webhook URL must use HTTPS"})),
        ));
    }

    // Get connection ID
    let connection_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM service_connections WHERE service_key = $1")
            .bind(&key)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let connection_id = connection_id.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Connection not found"})),
    ))?;

    // Generate webhook secret
    let secret = generate_webhook_secret();

    let webhook: IntegrationWebhook = sqlx::query_as(
        r"
        INSERT INTO integration_webhooks (
            connection_id, name, url, secret, events, is_active,
            created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, true, NOW(), NOW())
        RETURNING id, connection_id, name, url, secret, events, is_active,
                  last_triggered_at, last_status_code, failure_count,
                  created_at, updated_at
        ",
    )
    .bind(connection_id)
    .bind(&input.name)
    .bind(&input.url)
    .bind(&secret)
    .bind(&input.events)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Audit log
    log_connection_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "webhook.created",
        "integration_webhook",
        Some(webhook.id),
        None,
        Some(json!({"name": input.name, "url": input.url})),
        Some(json!({"connection_id": connection_id})),
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        webhook_id = webhook.id,
        "Webhook created"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Webhook created successfully",
        "data": {
            "id": webhook.id,
            "name": webhook.name,
            "url": webhook.url,
            "secret": secret, // Only returned on creation
            "events": webhook.events,
            "is_active": webhook.is_active
        }
    })))
}

/// DELETE /admin/connections/:key/webhooks/:id - Delete webhook
#[tracing::instrument(skip(state, admin))]
pub(super) async fn delete_webhook(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((key, webhook_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify connection exists
    let connection_id: Option<i64> =
        sqlx::query_scalar("SELECT id FROM service_connections WHERE service_key = $1")
            .bind(&key)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let connection_id = connection_id.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Connection not found"})),
    ))?;

    // Delete webhook
    let result =
        sqlx::query("DELETE FROM integration_webhooks WHERE id = $1 AND connection_id = $2")
            .bind(webhook_id)
            .bind(connection_id)
            .execute(state.db.pool())
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Webhook not found"})),
        ));
    }

    // Audit log
    log_connection_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "webhook.deleted",
        "integration_webhook",
        Some(webhook_id),
        None,
        None,
        Some(json!({"connection_id": connection_id})),
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        webhook_id = webhook_id,
        "Webhook deleted"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Webhook deleted successfully"
    })))
}
