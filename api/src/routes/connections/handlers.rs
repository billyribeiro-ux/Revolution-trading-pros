//! Connection CRUD handlers (split from `connections.rs` lines
//! 365-1232, R20-B maintainability pass, 2026-05-20). Behavior
//! preserved verbatim — only the module boundary changed.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::collections::HashMap;

use crate::{middleware::admin::AdminUser, AppState};

use super::audit::log_connection_audit;
use super::crypto::{decrypt_credentials, encrypt_credentials, mask_credentials};
use super::definitions::{get_categories, get_service_definitions};
use super::{ConnectServiceRequest, ListQuery, ServiceConnection};

/// GET /admin/connections - Get all connections with status
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_connections_status(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin fetching connections"
    );

    let now = chrono::Utc::now();

    // Get database connections
    let mut sql = String::from(
        r"
        SELECT id, service_key, name, category, description, status, health_score,
               health_status, environment, credentials_encrypted, settings,
               webhook_url, webhook_secret, api_calls_today, api_calls_total,
               last_error, last_verified_at, connected_at, created_at, updated_at
        FROM service_connections
        WHERE 1=1
        ",
    );

    let mut bind_count = 0;

    if query.category.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND category = ${bind_count}"));
    }

    if query.status.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND status = ${bind_count}"));
    }

    sql.push_str(" ORDER BY category, name");

    let mut query_builder = sqlx::query_as::<_, ServiceConnection>(&sql);

    if let Some(ref category) = query.category {
        query_builder = query_builder.bind(category);
    }
    if let Some(ref status) = query.status {
        query_builder = query_builder.bind(status);
    }

    let db_connections: Vec<ServiceConnection> = query_builder
        .fetch_all(state.db.pool())
        .await
        .unwrap_or_default();

    // Build a map of existing connections by service_key
    let existing_keys: std::collections::HashSet<String> = db_connections
        .iter()
        .map(|c| c.service_key.clone())
        .collect();

    // Get service definitions and merge with database connections
    let definitions = get_service_definitions();
    let mut connections: Vec<serde_json::Value> = Vec::new();

    for def in definitions {
        let key = def["key"].as_str().unwrap_or("");
        let is_builtin = def["is_builtin"].as_bool().unwrap_or(false);

        if let Some(db_conn) = db_connections.iter().find(|c| c.service_key == key) {
            // Merge database connection with definition
            let masked_creds = db_conn
                .credentials_encrypted
                .as_ref()
                .map(|e| mask_credentials(&decrypt_credentials(e)));

            connections.push(json!({
                "key": db_conn.service_key,
                "name": db_conn.name,
                "category": db_conn.category,
                "description": db_conn.description,
                "icon": def["icon"],
                "color": def["color"],
                "docs_url": def["docs_url"],
                "signup_url": def["signup_url"],
                "is_oauth": def["is_oauth"],
                "is_builtin": def["is_builtin"],
                "is_connected": db_conn.status == "connected",
                "status": db_conn.status,
                "health_score": db_conn.health_score,
                "environments": def["environments"],
                "fields": def["fields"],
                "connection": {
                    "id": db_conn.id,
                    "service_key": db_conn.service_key,
                    "status": db_conn.status,
                    "health_score": db_conn.health_score,
                    "health_status": db_conn.health_status,
                    "environment": db_conn.environment,
                    "credentials": masked_creds,
                    "settings": db_conn.settings,
                    "webhook_url": db_conn.webhook_url,
                    "connected_at": db_conn.connected_at,
                    "last_verified_at": db_conn.last_verified_at,
                    "api_calls_today": db_conn.api_calls_today,
                    "api_calls_total": db_conn.api_calls_total,
                    "last_error": db_conn.last_error
                }
            }));
        } else if is_builtin {
            // Built-in services are always "connected"
            connections.push(json!({
                "key": key,
                "name": def["name"],
                "category": def["category"],
                "description": def["description"],
                "icon": def["icon"],
                "color": def["color"],
                "is_oauth": false,
                "is_builtin": true,
                "is_connected": true,
                "status": "connected",
                "health_score": 100,
                "fields": [],
                "connection": {
                    "id": format!("builtin-{}", key),
                    "service_key": key,
                    "status": "connected",
                    "health_score": 100,
                    "health_status": "healthy",
                    "connected_at": now,
                    "last_verified_at": now,
                    "api_calls_today": 0,
                    "api_calls_total": 0,
                    "last_error": null
                }
            }));
        } else {
            // Service not connected
            connections.push(json!({
                "key": key,
                "name": def["name"],
                "category": def["category"],
                "description": def["description"],
                "icon": def["icon"],
                "color": def["color"],
                "docs_url": def["docs_url"],
                "signup_url": def["signup_url"],
                "is_oauth": def["is_oauth"],
                "is_builtin": false,
                "is_connected": false,
                "status": "pending",
                "health_score": 0,
                "environments": def["environments"],
                "fields": def["fields"],
                "connection": null
            }));
        }
    }

    Ok(Json(json!({
        "success": true,
        "connections": connections,
        "categories": get_categories()
    })))
}

/// GET /admin/connections/summary - Get connection summary for dashboard
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_connections_summary(
    State(state): State<AppState>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(admin_id = admin.0.id, "Admin fetching connections summary");

    let connected: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM service_connections WHERE status = 'connected'")
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    let pending: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM service_connections WHERE status = 'pending'")
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    let error: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM service_connections WHERE status = 'error'")
            .fetch_one(state.db.pool())
            .await
            .unwrap_or(0);

    let avg_health: f64 = sqlx::query_scalar(
        "SELECT COALESCE(AVG(health_score), 0) FROM service_connections WHERE status = 'connected'",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0.0);

    // Add builtin services count
    let builtin_count = 3; // fluent_crm_pro, fluent_forms_pro, fluent_smtp

    Ok(Json(json!({
        "success": true,
        "connections": {
            "total": connected + pending + error + builtin_count,
            "connected": connected + builtin_count,
            "pending": pending,
            "error": error
        },
        "health_score": avg_health.round() as i32
    })))
}

/// GET /admin/connections/:key - Get single connection details
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_connection(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Admin fetching connection"
    );

    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid service key"})),
        ));
    }

    let connection: Option<ServiceConnection> = sqlx::query_as(
        r"
        SELECT id, service_key, name, category, description, status, health_score,
               health_status, environment, credentials_encrypted, settings,
               webhook_url, webhook_secret, api_calls_today, api_calls_total,
               last_error, last_verified_at, connected_at, created_at, updated_at
        FROM service_connections
        WHERE service_key = $1
        ",
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Get service definition
    let definitions = get_service_definitions();
    let def = definitions.iter().find(|d| d["key"].as_str() == Some(&key));

    match (connection, def) {
        (Some(conn), Some(def)) => {
            let masked_creds = conn
                .credentials_encrypted
                .as_ref()
                .map(|e| mask_credentials(&decrypt_credentials(e)));

            Ok(Json(json!({
                "success": true,
                "data": {
                    "key": conn.service_key,
                    "name": conn.name,
                    "category": conn.category,
                    "description": conn.description,
                    "icon": def["icon"],
                    "color": def["color"],
                    "is_connected": conn.status == "connected",
                    "status": conn.status,
                    "health_score": conn.health_score,
                    "fields": def["fields"],
                    "connection": {
                        "id": conn.id,
                        "environment": conn.environment,
                        "credentials": masked_creds,
                        "settings": conn.settings,
                        "webhook_url": conn.webhook_url,
                        "connected_at": conn.connected_at,
                        "last_verified_at": conn.last_verified_at,
                        "api_calls_today": conn.api_calls_today,
                        "api_calls_total": conn.api_calls_total,
                        "last_error": conn.last_error
                    }
                }
            })))
        }
        (None, Some(def)) => {
            // Service exists in definitions but not connected
            Ok(Json(json!({
                "success": true,
                "data": {
                    "key": key,
                    "name": def["name"],
                    "category": def["category"],
                    "description": def["description"],
                    "icon": def["icon"],
                    "color": def["color"],
                    "is_connected": false,
                    "status": "pending",
                    "health_score": 0,
                    "fields": def["fields"],
                    "connection": null
                }
            })))
        }
        _ => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Service not found"})),
        )),
    }
}

/// POST /admin/connections/:key/connect - Connect a service
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn connect_service(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
    Json(input): Json<ConnectServiceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid service key"})),
        ));
    }

    // Get service definition
    let definitions = get_service_definitions();
    let def = definitions.iter().find(|d| d["key"].as_str() == Some(&key));

    let def = def.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Service not found"})),
    ))?;

    // Validate required fields
    if let Some(fields) = def["fields"].as_array() {
        for field in fields {
            let field_key = field["key"].as_str().unwrap_or("");
            let required = field["required"].as_bool().unwrap_or(false);

            if required && !input.credentials.contains_key(field_key) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": format!("Missing required field: {}", field_key)})),
                ));
            }
        }
    }

    // Encrypt credentials
    let encrypted_creds = encrypt_credentials(&input.credentials);

    // Check if connection already exists
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM service_connections WHERE service_key = $1")
            .bind(&key)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let connection: ServiceConnection = if let Some(id) = existing {
        // Update existing connection
        sqlx::query_as(
            r"
            UPDATE service_connections
            SET credentials_encrypted = $2,
                environment = $3,
                settings = $4,
                status = 'connected',
                health_score = 100,
                health_status = 'healthy',
                connected_at = NOW(),
                last_verified_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, service_key, name, category, description, status, health_score,
                      health_status, environment, credentials_encrypted, settings,
                      webhook_url, webhook_secret, api_calls_today, api_calls_total,
                      last_error, last_verified_at, connected_at, created_at, updated_at
            ",
        )
        .bind(id)
        .bind(&encrypted_creds)
        .bind(&input.environment)
        .bind(&input.settings)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
    } else {
        // Create new connection
        sqlx::query_as(
            r"
            INSERT INTO service_connections (
                service_key, name, category, description, credentials_encrypted,
                environment, settings, status, health_score, health_status,
                connected_at, last_verified_at, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, 'connected', 100, 'healthy',
                NOW(), NOW(), NOW(), NOW()
            )
            RETURNING id, service_key, name, category, description, status, health_score,
                      health_status, environment, credentials_encrypted, settings,
                      webhook_url, webhook_secret, api_calls_today, api_calls_total,
                      last_error, last_verified_at, connected_at, created_at, updated_at
            ",
        )
        .bind(&key)
        .bind(def["name"].as_str().unwrap_or(&key))
        .bind(def["category"].as_str().unwrap_or("Other"))
        .bind(def["description"].as_str())
        .bind(&encrypted_creds)
        .bind(&input.environment)
        .bind(&input.settings)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
    };

    // Audit log
    log_connection_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "connection.connected",
        "service_connection",
        Some(connection.id),
        None,
        Some(json!({
            "service_key": key,
            "environment": input.environment
        })),
        None,
    )
    .await;

    // PE7 invariant 2A: drop any cached creds so the next request reads the
    // freshly-pasted keys without waiting on the 60s TTL.
    state
        .services
        .credentials
        .invalidate(&key, input.environment.as_deref().unwrap_or(""))
        .await;

    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Service connected"
    );

    Ok(Json(json!({
        "success": true,
        "message": format!("Service {} connected successfully", key),
        "connection": {
            "id": connection.id,
            "service_key": connection.service_key,
            "status": connection.status,
            "health_score": connection.health_score
        }
    })))
}

/// POST /admin/connections/:key/test - Test a service connection
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn test_connection(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
    Json(input): Json<Option<HashMap<String, String>>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Admin testing connection"
    );

    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid service key"})),
        ));
    }

    // Get existing connection or use provided credentials
    let credentials: HashMap<String, String> = if let Some(creds) = input {
        creds
    } else {
        // Get from database
        let encrypted: Option<String> = sqlx::query_scalar(
            "SELECT credentials_encrypted FROM service_connections WHERE service_key = $1",
        )
        .bind(&key)
        .fetch_optional(state.db.pool())
        .await
        .ok()
        .flatten();

        encrypted
            .map(|e| decrypt_credentials(&e))
            .unwrap_or_default()
    };

    if credentials.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No credentials provided"})),
        ));
    }

    // Perform connection test based on service type
    let (success, latency_ms, error_msg) = match key.as_str() {
        "stripe" => {
            // Test Stripe API
            if let Some(secret_key) = credentials.get("secret_key") {
                if secret_key.starts_with("sk_") {
                    (true, 45, None)
                } else {
                    (
                        false,
                        0,
                        Some("Invalid Stripe secret key format".to_string()),
                    )
                }
            } else {
                (false, 0, Some("Missing secret_key".to_string()))
            }
        }
        "bunny_cdn" => {
            // Test Bunny API
            if credentials.contains_key("api_key") && credentials.contains_key("storage_zone") {
                (true, 120, None)
            } else {
                (false, 0, Some("Missing required credentials".to_string()))
            }
        }
        "sendgrid" => {
            // Test SendGrid API
            if let Some(api_key) = credentials.get("api_key") {
                if api_key.starts_with("SG.") {
                    (true, 85, None)
                } else {
                    (
                        false,
                        0,
                        Some("Invalid SendGrid API key format".to_string()),
                    )
                }
            } else {
                (false, 0, Some("Missing api_key".to_string()))
            }
        }
        "openai" => {
            // Test OpenAI API
            if let Some(api_key) = credentials.get("api_key") {
                if api_key.starts_with("sk-") {
                    (true, 250, None)
                } else {
                    (false, 0, Some("Invalid OpenAI API key format".to_string()))
                }
            } else {
                (false, 0, Some("Missing api_key".to_string()))
            }
        }
        _ => {
            // Generic test - just check credentials exist
            if !credentials.is_empty() {
                (true, 50, None)
            } else {
                (false, 0, Some("No credentials provided".to_string()))
            }
        }
    };

    // Update connection status in database
    if success {
        let _ = sqlx::query(
            r"
            UPDATE service_connections
            SET health_score = 100,
                health_status = 'healthy',
                last_verified_at = NOW(),
                last_error = NULL,
                updated_at = NOW()
            WHERE service_key = $1
            ",
        )
        .bind(&key)
        .execute(state.db.pool())
        .await;
    } else {
        let _ = sqlx::query(
            r"
            UPDATE service_connections
            SET health_score = 0,
                health_status = 'error',
                last_verified_at = NOW(),
                last_error = $2,
                updated_at = NOW()
            WHERE service_key = $1
            ",
        )
        .bind(&key)
        .bind(&error_msg)
        .execute(state.db.pool())
        .await;
    }

    if success {
        Ok(Json(json!({
            "success": true,
            "message": format!("Connection test for {} passed", key),
            "latency_ms": latency_ms,
            "status": "healthy"
        })))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": format!("Connection test for {} failed", key),
                "error": error_msg,
                "status": "error"
            })),
        ))
    }
}

/// POST /admin/connections/:key/disconnect - Disconnect a service
#[tracing::instrument(skip(state, admin))]
pub(super) async fn disconnect_service(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid service key"})),
        ));
    }

    // Get connection for audit
    let connection: Option<ServiceConnection> =
        sqlx::query_as("SELECT * FROM service_connections WHERE service_key = $1")
            .bind(&key)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let Some(connection) = connection else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Connection not found"})),
        ));
    };

    // Clear credentials and update status
    sqlx::query(
        r"
        UPDATE service_connections
        SET credentials_encrypted = NULL,
            status = 'disconnected',
            health_score = 0,
            health_status = NULL,
            webhook_url = NULL,
            webhook_secret = NULL,
            updated_at = NOW()
        WHERE service_key = $1
        ",
    )
    .bind(&key)
    .execute(state.db.pool())
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
        "connection.disconnected",
        "service_connection",
        Some(connection.id),
        Some(json!({"service_key": key, "was_connected": true})),
        None,
        None,
    )
    .await;

    // PE7 invariant 2A: drop cached creds. Disconnecting an environment must
    // immediately stop the runtime from using its keys.
    state
        .services
        .credentials
        .invalidate(&key, connection.environment.as_deref().unwrap_or(""))
        .await;

    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Service disconnected"
    );

    Ok(Json(json!({
        "success": true,
        "message": format!("Service {} disconnected", key)
    })))
}

/// DELETE /admin/connections/:key - Delete a connection entirely
#[tracing::instrument(skip(state, admin))]
pub(super) async fn delete_connection(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid service key"})),
        ));
    }

    // Get connection for audit
    let connection: Option<ServiceConnection> =
        sqlx::query_as("SELECT * FROM service_connections WHERE service_key = $1")
            .bind(&key)
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let Some(connection) = connection else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Connection not found"})),
        ));
    };

    // ICT 7 SAFETY: wrap multi-table mutation in a transaction so a crash mid-flow
    // can't leave webhook rows orphaned with their parent connection already gone
    // (or vice-versa).
    let mut tx = state.db.pool().begin().await.map_err(|e| {
        tracing::error!("tx start (delete_connection): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Delete associated webhooks first
    sqlx::query("DELETE FROM integration_webhooks WHERE connection_id = $1")
        .bind(connection.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete integration_webhooks: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete webhooks"})),
            )
        })?;

    // Delete connection
    sqlx::query("DELETE FROM service_connections WHERE service_key = $1")
        .bind(&key)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete service_connection: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("tx commit (delete_connection): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Audit log (best-effort, fire-and-forget after the tx is durably committed)
    log_connection_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "connection.deleted",
        "service_connection",
        Some(connection.id),
        Some(json!({"service_key": key, "name": connection.name})),
        None,
        None,
    )
    .await;

    // PE7 invariant 2A: invalidate cached creds.
    state
        .services
        .credentials
        .invalidate(&key, connection.environment.as_deref().unwrap_or(""))
        .await;

    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Service connection deleted"
    );

    Ok(Json(json!({
        "success": true,
        "message": format!("Connection {} deleted", key)
    })))
}

/// GET /admin/connections/categories - Get connection categories
#[tracing::instrument(skip(_state, admin))]
pub(super) async fn get_categories_handler(
    State(_state): State<AppState>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        "Admin fetching connection categories"
    );

    Ok(Json(json!({
        "success": true,
        "categories": get_categories()
    })))
}
