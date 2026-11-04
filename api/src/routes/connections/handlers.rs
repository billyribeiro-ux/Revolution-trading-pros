//! Connection CRUD handlers (split from `connections.rs` lines
//! 365-1232, R20-B maintainability pass, 2026-05-20). Behavior
//! preserved verbatim — only the module boundary changed.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde_json::json;
use std::collections::HashMap;

use crate::{
    AppState,
    middleware::admin::{AdminUser, SuperAdminUser},
};

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

    let environment = query.environment.as_deref().unwrap_or("production");
    if !["production", "sandbox"].contains(&environment) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid environment"})),
        ));
    }
    if query
        .status
        .as_deref()
        .is_some_and(|status| !["connected", "pending", "error", "disconnected"].contains(&status))
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid connection status"})),
        ));
    }
    let db_connections:Vec<ServiceConnection>=sqlx::query_as("SELECT * FROM service_connections WHERE environment=$1 OR environment IS NULL ORDER BY (environment=$1) DESC NULLS LAST,updated_at DESC,id DESC")
        .bind(environment).fetch_all(state.db.pool()).await.map_err(crate::utils::errors::db_error)?;
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

        if let Some(db_conn) = db_connections
            .iter()
            .find(|c| c.service_key == key && !is_builtin)
        {
            let verified = db_conn.status == "connected"
                && db_conn.health_status.as_deref() == Some("verified")
                && db_conn.environment.as_deref() == Some(environment);
            let status = if db_conn.status == "connected" && !verified {
                "pending"
            } else {
                db_conn.status.as_str()
            };
            let health = if verified { db_conn.health_score } else { 0 };
            let last_error = if db_conn.status == "connected" && !verified {
                Some("Legacy connection requires provider verification".to_string())
            } else {
                db_conn.last_error.clone()
            };
            // Merge database connection with definition
            let masked_creds = db_conn.credentials_encrypted.as_ref().map(|e| {
                mask_credentials(&decrypt_credentials(
                    &state.config.credentials_encryption_key,
                    e,
                ))
            });

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
                "is_connected": verified,
                "status": status,
                "health_score": health,
                "environments": def["environments"],
                "fields": def["fields"],
                "connection": {
                    "id": db_conn.id,
                    "service_key": db_conn.service_key,
                    "status": status,
                    "health_score": health,
                    "health_status": if verified {Some("verified")}else{db_conn.health_status.as_deref().filter(|status|*status!="healthy")},
                    "environment": db_conn.environment,
                    "credentials": masked_creds,
                    "settings": db_conn.settings,
                    "webhook_url": db_conn.webhook_url,
                    "connected_at": if verified {db_conn.connected_at}else{None},
                    "last_verified_at": if verified {db_conn.last_verified_at}else{None},
                    "api_calls_today": db_conn.api_calls_today,
                    "api_calls_total": db_conn.api_calls_total,
                    "last_error": last_error
                }
            }));
        } else if is_builtin {
            let (available, checked_at, error) = match key {
                "fluent_crm_pro" | "fluent_forms_pro" => {
                    let sql = if key == "fluent_crm_pro" {
                        "SELECT id FROM contacts LIMIT 1"
                    } else {
                        "SELECT id FROM forms LIMIT 1"
                    };
                    sqlx::query(sqlx::AssertSqlSafe(sql))
                        .fetch_optional(state.db.pool())
                        .await
                        .map_err(crate::utils::errors::db_error)?;
                    (true, Some(chrono::Utc::now()), None)
                }
                _ => {
                    let verified: Option<chrono::DateTime<chrono::Utc>> = if state
                        .services
                        .email
                        .is_enabled()
                    {
                        sqlx::query_scalar("SELECT MAX(sent_at) FROM email_logs WHERE provider='postmark' AND provider_message_id IS NOT NULL AND status='sent' AND sent_at>NOW()-INTERVAL '15 minutes'")
                            .fetch_one(state.db.pool()).await.map_err(crate::utils::errors::db_error)?
                    } else {
                        None
                    };
                    let error = if verified.is_some() {
                        None
                    } else {
                        Some(
                            state
                                .services
                                .email
                                .configuration_error()
                                .unwrap_or_else(|| {
                                    "Email configured; no recent provider-acknowledged receipt"
                                        .into()
                                }),
                        )
                    };
                    (verified.is_some(), verified, error)
                }
            };
            let mut value = def.clone();
            value["is_connected"] = json!(available);
            value["status"] = json!(if available { "connected" } else { "pending" });
            value["health_score"] = json!(if available { 100 } else { 0 });
            value["connection"] = json!({"id":format!("native-{key}"),"service_key":key,"status":value["status"],"health_score":value["health_score"],"health_status":if available{"last_check_passed"}else{"unverified"},"connected_at":null,"last_verified_at":checked_at,"last_error":error,"api_calls_today":0,"api_calls_total":0});
            connections.push(value);
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

    connections.retain(|connection| {
        query
            .category
            .as_ref()
            .is_none_or(|category| connection["category"] == category.as_str())
            && query
                .status
                .as_ref()
                .is_none_or(|status| connection["status"] == status.as_str())
    });
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
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(admin_id = admin.0.id, "Admin fetching connections summary");

    let Json(data) = get_connections_status(State(state), admin, Query(query)).await?;
    let values = data["connections"].as_array().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error":"Invalid connection summary"})),
    ))?;
    let count = |status: &str| {
        values
            .iter()
            .filter(|value| value["status"] == status)
            .count()
    };
    let connected = count("connected");
    let health: Vec<f64> = values
        .iter()
        .filter(|value| value["status"] == "connected")
        .filter_map(|value| value["health_score"].as_f64())
        .collect();
    Ok(Json(
        json!({"success":true,"connections":{"total":values.len(),"connected":connected,"pending":count("pending"),"error":count("error"),"disconnected":count("disconnected")},"health_score":if health.is_empty(){None}else{Some((health.iter().sum::<f64>()/health.len() as f64).round() as i32)}}),
    ))
}

/// GET /admin/connections/:key - Get single connection details
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_connection(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let Json(data) = get_connections_status(State(state), admin, Query(query)).await?;
    let connection = data["connections"]
        .as_array()
        .and_then(|values| values.iter().find(|value| value["key"] == key))
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(json!({"error":"Service not found"})),
        ))?;
    Ok(Json(json!({"success":true,"data":connection})))
}

/// POST /admin/connections/:key/connect - Connect a service
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn connect_service(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Path(key): Path<String>,
    Json(mut input): Json<ConnectServiceRequest>,
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

            if required
                && input
                    .credentials
                    .get(field_key)
                    .is_none_or(|value| value.trim().is_empty())
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": format!("Missing required field: {}", field_key)})),
                ));
            }
        }
    }

    input
        .credentials
        .retain(|_, value| !value.trim().is_empty());
    let environment = input.environment.as_deref().unwrap_or("production");
    if !["production", "sandbox"].contains(&environment) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid environment"})),
        ));
    }
    if input.credentials.is_empty()
        || input.credentials.len() > 32
        || input.credentials.values().any(|value| {
            value.trim().is_empty()
                || value.len() > 8192
                || value.contains(['\r', '\n', '\0'])
                || value.contains("********")
        })
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Missing or invalid credentials"})),
        ));
    }
    if def["is_builtin"] == true {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Built-in services use native configuration"})),
        ));
    }
    let result = super::probe::check(&state.config, &key, &input.credentials, environment).await;
    let encrypted_creds =
        encrypt_credentials(&state.config.credentials_encryption_key, &input.credentials);
    if encrypted_creds.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Credential encryption failed"})),
        ));
    }
    let status = if result.success {
        "connected"
    } else if result.status == "rejected" {
        "error"
    } else {
        "pending"
    };
    let mut tx = state
        .db
        .pool()
        .begin()
        .await
        .map_err(crate::utils::errors::db_error)?;
    let connection: ServiceConnection=sqlx::query_as("INSERT INTO service_connections(service_key,name,category,description,credentials_encrypted,environment,settings,status,health_score,health_status,last_verified_at,last_error,connected_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,CASE WHEN $11 THEN NOW() END,$12,CASE WHEN $11 THEN NOW() END) ON CONFLICT(service_key,(COALESCE(environment,''))) DO UPDATE SET credentials_encrypted=EXCLUDED.credentials_encrypted,settings=EXCLUDED.settings,status=EXCLUDED.status,health_score=EXCLUDED.health_score,health_status=EXCLUDED.health_status,last_verified_at=EXCLUDED.last_verified_at,last_error=EXCLUDED.last_error,connected_at=EXCLUDED.connected_at,updated_at=NOW() RETURNING *")
        .bind(&key).bind(def["name"].as_str()).bind(def["category"].as_str()).bind(def["description"].as_str()).bind(&encrypted_creds).bind(environment).bind(&input.settings).bind(status).bind(if result.success{100}else{0}).bind(result.status).bind(result.success).bind(if result.success{None}else{Some(result.message.as_str())})
        .fetch_one(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
    sqlx::query("INSERT INTO admin_audit_logs(admin_id,admin_email,action,entity_type,entity_id,new_value) VALUES($1,$2,'connection.verification','service_connection',$3,$4)")
        .bind(admin.0.id).bind(&admin.0.email).bind(connection.id).bind(json!({"service_key":key,"environment":environment,"verification":result}).to_string()).execute(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
    tx.commit().await.map_err(crate::utils::errors::db_error)?;

    // PE7 invariant 2A: drop any cached creds so the next request reads the
    // freshly-pasted keys without waiting on the 60s TTL.
    state
        .services
        .credentials
        .invalidate(&key, environment)
        .await;

    tracing::info!(
        admin_id = admin.0.id,
        service_key = %key,
        "Service connected"
    );

    Ok(Json(json!({
        "success": true,
        "message": if result.success {"Credentials saved and read-only provider check passed"} else {"Credentials saved; provider verification did not pass"},
        "verified": result.success,
        "verification": result,
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
    admin: SuperAdminUser,
    Path(key): Path<String>,
    Json(input): Json<Option<TestConnectionInput>>,
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

    let (provided, environment) = match input {
        Some(TestConnectionInput::Wrapped(fields)) => (fields.credentials, fields.environment),
        Some(TestConnectionInput::Flat(credentials)) => (Some(credentials), None),
        None => (None, None),
    };
    let environment = environment.as_deref().unwrap_or("production");
    if !["production", "sandbox"].contains(&environment) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid environment"})),
        ));
    }
    let stored: Option<ServiceConnection>=sqlx::query_as("SELECT * FROM service_connections WHERE service_key=$1 AND (environment=$2 OR environment IS NULL) ORDER BY (environment=$2) DESC NULLS LAST,id DESC LIMIT 1")
        .bind(&key).bind(environment).fetch_optional(state.db.pool()).await.map_err(crate::utils::errors::db_error)?;
    let saved = stored
        .as_ref()
        .and_then(|row| row.credentials_encrypted.as_ref())
        .map(|encrypted| decrypt_credentials(&state.config.credentials_encryption_key, encrypted));
    let credentials = provided.as_ref().or(saved.as_ref()).ok_or((
        StatusCode::BAD_REQUEST,
        Json(json!({"error":"No credentials provided"})),
    ))?;
    if credentials.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"No credentials provided"})),
        ));
    }
    if !get_service_definitions()
        .iter()
        .any(|def| def["key"] == key)
    {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error":"Service not found"})),
        ));
    }
    let result = super::probe::check(&state.config, &key, credentials, environment).await;
    let mut persisted = false;
    if let Some(stored) = stored.filter(|_| saved.as_ref() == Some(credentials)) {
        let mut tx = state
            .db
            .pool()
            .begin()
            .await
            .map_err(crate::utils::errors::db_error)?;
        let changed=sqlx::query("UPDATE service_connections SET status=$2,health_score=$3,health_status=$4,last_verified_at=CASE WHEN $5 THEN NOW() ELSE last_verified_at END,last_error=$6,updated_at=NOW() WHERE id=$1 AND credentials_encrypted=$7 AND status<>'disconnected'")
            .bind(stored.id).bind(if result.success{"connected"}else if result.status=="rejected"{"error"}else{"pending"}).bind(if result.success{100}else{0}).bind(result.status).bind(result.success).bind(if result.success{None}else{Some(result.message.as_str())}).bind(&stored.credentials_encrypted)
            .execute(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
        if changed.rows_affected() != 1 {
            return Err((
                StatusCode::CONFLICT,
                Json(json!({"error":"Connection changed during verification; reload and retry"})),
            ));
        }
        sqlx::query("INSERT INTO admin_audit_logs(admin_id,admin_email,action,entity_type,entity_id,new_value) VALUES($1,$2,'connection.verification','service_connection',$3,$4)")
            .bind(admin.0.id).bind(&admin.0.email).bind(stored.id).bind(json!({"service_key":key,"environment":environment,"verification":result}).to_string()).execute(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
        tx.commit().await.map_err(crate::utils::errors::db_error)?;
        state
            .services
            .credentials
            .invalidate(&key, environment)
            .await;
        persisted = true;
    }
    Ok(Json(
        json!({"success":result.success,"message":result.message,"error":if result.success{None}else{Some(&result.message)},"status":result.status,"latency_ms":result.latency_ms,"status_code":result.status_code,"persisted":persisted}),
    ))
}

/// Credential removal, dependent webhooks and its audit receipt commit together.
async fn revoke_connection(
    state: AppState,
    admin: SuperAdminUser,
    key: String,
    query: ListQuery,
    remove: bool,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let environment = query.environment.as_deref().unwrap_or("production");
    if !["production", "sandbox"].contains(&environment) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid environment"})),
        ));
    }
    let mut tx = state
        .db
        .pool()
        .begin()
        .await
        .map_err(crate::utils::errors::db_error)?;
    let connection:Option<ServiceConnection>=sqlx::query_as("SELECT * FROM service_connections WHERE service_key=$1 AND (environment=$2 OR environment IS NULL) ORDER BY (environment=$2) DESC NULLS LAST,id DESC LIMIT 1 FOR UPDATE")
        .bind(&key).bind(environment).fetch_optional(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
    let connection = connection.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error":"Connection not found"})),
    ))?;
    let action = if remove {
        sqlx::query("DELETE FROM integration_webhooks WHERE connection_id=$1")
            .bind(connection.id)
            .execute(&mut *tx)
            .await
            .map_err(crate::utils::errors::db_error)?;
        sqlx::query("DELETE FROM service_connections WHERE id=$1")
            .bind(connection.id)
            .execute(&mut *tx)
            .await
            .map_err(crate::utils::errors::db_error)?;
        "connection.deleted"
    } else {
        sqlx::query("UPDATE service_connections SET credentials_encrypted=NULL,status='disconnected',health_score=0,health_status=NULL,webhook_url=NULL,webhook_secret=NULL,updated_at=NOW() WHERE id=$1").bind(connection.id).execute(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
        sqlx::query("UPDATE integration_webhooks SET is_active=FALSE,updated_at=NOW() WHERE connection_id=$1").bind(connection.id).execute(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
        "connection.disconnected"
    };
    sqlx::query("INSERT INTO admin_audit_logs(admin_id,admin_email,action,entity_type,entity_id,old_value) VALUES($1,$2,$3,'service_connection',$4,$5)")
        .bind(admin.0.id).bind(&admin.0.email).bind(action).bind(connection.id).bind(json!({"service_key":key,"environment":connection.environment,"status":connection.status}).to_string())
        .execute(&mut *tx).await.map_err(crate::utils::errors::db_error)?;
    tx.commit().await.map_err(crate::utils::errors::db_error)?;
    state
        .services
        .credentials
        .invalidate(&key, environment)
        .await;
    Ok(Json(
        json!({"success":true,"message":if remove{"Connection deleted"}else{"Connection disconnected"}}),
    ))
}
pub(super) async fn disconnect_service(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Path(key): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    revoke_connection(state, admin, key, query, false).await
}
pub(super) async fn delete_connection(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Path(key): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    revoke_connection(state, admin, key, query, true).await
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

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TestConnectionFields {
    credentials: Option<HashMap<String, String>>,
    environment: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(untagged)]
pub(super) enum TestConnectionInput {
    Wrapped(TestConnectionFields),
    Flat(HashMap<String, String>),
}
#[derive(serde::Deserialize)]
pub(super) struct InitiateRequest {
    #[serde(alias = "service_key")]
    key: String,
    #[serde(flatten)]
    input: ConnectServiceRequest,
}
pub(super) async fn initiate_service(
    state: State<AppState>,
    admin: SuperAdminUser,
    Json(request): Json<InitiateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    connect_service(state, admin, Path(request.key), Json(request.input)).await
}
