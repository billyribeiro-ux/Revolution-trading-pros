//! Authenticated CRM webhook authoring and durable delivery history.
use crate::{
    AppState, middleware::admin::AdminUser, services::crm_webhooks, utils::errors::ApiError,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgConnection;

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"CRM webhook persistence failed");
    ApiError::database_error("Unable to access CRM webhooks")
}
fn invalid(message: impl Into<String>) -> ApiError {
    ApiError::validation_error(&message.into())
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    status: Option<String>,
}
impl Filters {
    fn bounds(&self) -> Result<(i64, i64, i64), ApiError> {
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(25);
        if page < 1
            || !(1..=100).contains(&size)
            || self
                .search
                .as_ref()
                .is_some_and(|s| s.len() > 500 || s.contains('\0'))
            || self
                .status
                .as_ref()
                .is_some_and(|s| !matches!(s.as_str(), "all" | "active" | "inactive"))
        {
            return Err(invalid("Invalid webhook page, page size, search or status"));
        }
        Ok((page, size, page.saturating_sub(1).saturating_mul(size)))
    }
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    name: Option<String>,
    url: Option<String>,
    events: Option<Vec<String>>,
    secret: Option<String>,
    headers: Option<Value>,
    is_active: Option<bool>,
}
fn validate(value: &Value) -> Result<(), ApiError> {
    let name = value["name"]
        .as_str()
        .ok_or_else(|| invalid("Name is required"))?;
    if name.trim().is_empty() || name.chars().count() > 255 || name.contains('\0') {
        return Err(invalid("Name must contain 1–255 characters"));
    }
    crm_webhooks::target(
        value["url"]
            .as_str()
            .ok_or_else(|| invalid("URL is required"))?,
    )
    .map_err(|e| invalid(e.to_string()))?;
    let events = value["events"]
        .as_array()
        .ok_or_else(|| invalid("Events must be an array"))?;
    let mut seen = std::collections::HashSet::new();
    if events.is_empty()
        || events.len() > crm_webhooks::EVENTS.len()
        || events.iter().any(|event| {
            event.as_str().is_none_or(|event| {
                !crm_webhooks::EVENTS.iter().any(|(key, _)| key == &event) || !seen.insert(event)
            })
        })
    {
        return Err(invalid("Choose unique supported webhook events"));
    }
    if value["secret"]
        .as_str()
        .is_some_and(|s| s.len() > 255 || s.contains('\0'))
    {
        return Err(invalid("Secret must be at most 255 bytes"));
    }
    crm_webhooks::validate_headers(&value["headers"]).map_err(|e| invalid(e.to_string()))?;
    Ok(())
}
fn merge(mut value: Value, input: Input) -> Value {
    if let Some(name) = input.name {
        value["name"] = json!(name.trim());
    }
    if let Some(url) = input.url {
        value["url"] = json!(url.trim());
    }
    if let Some(events) = input.events {
        value["events"] = json!(events);
    }
    if let Some(secret) = input.secret {
        value["secret"] = if secret.is_empty() {
            Value::Null
        } else {
            json!(secret)
        };
    }
    if let Some(headers) = input.headers {
        value["headers"] = headers;
    }
    if let Some(active) = input.is_active {
        value["is_active"] = json!(active);
    }
    value
}
async fn read(connection: &mut PgConnection, id: i64, lock: bool) -> Result<Value, ApiError> {
    let sql = if lock {
        "SELECT to_jsonb(w)||jsonb_build_object('id',id::text,'created_by',created_by::text) FROM crm_webhooks w WHERE id=$1 FOR UPDATE"
    } else {
        "SELECT to_jsonb(w)||jsonb_build_object('id',id::text,'created_by',created_by::text) FROM crm_webhooks w WHERE id=$1"
    };
    sqlx::query_scalar(sql)
        .bind(id)
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Webhook not found"))
}
async fn list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = filters.bounds()?;
    let search = filters
        .search
        .map(|s| format!("%{}%", crate::utils::escape_like(&s)));
    let active = match filters.status.as_deref() {
        Some("active") => Some(true),
        Some("inactive") => Some(false),
        _ => None,
    };
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_webhooks WHERE ($1::text IS NULL OR name ILIKE $1 OR url ILIKE $1) AND ($2::bool IS NULL OR is_active=$2)").bind(&search).bind(active).fetch_one(&state.db.pool).await.map_err(db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT (to_jsonb(w)-'secret')||jsonb_build_object('id',id::text,'created_by',created_by::text) FROM crm_webhooks w WHERE ($1::text IS NULL OR name ILIKE $1 OR url ILIKE $1) AND ($2::bool IS NULL OR is_active=$2) ORDER BY created_at DESC,id DESC LIMIT $3 OFFSET $4").bind(search).bind(active).bind(size).bind(offset).fetch_all(&state.db.pool).await.map_err(db)?;
    let stats:Value=sqlx::query_scalar("SELECT jsonb_build_object('total',COUNT(*),'active',COUNT(*) FILTER(WHERE is_active),'totalTriggers',COALESCE(SUM(trigger_count),0),'totalFailures',COALESCE(SUM(failure_count),0)) FROM crm_webhooks").fetch_one(&state.db.pool).await.map_err(db)?;
    let pages = ((total + size - 1) / size).max(1);
    Ok(Json(
        json!({"data":rows,"stats":stats,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":pages,"last_page":pages}}),
    ))
}
async fn get_one(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        read(&mut *state.db.pool.acquire().await.map_err(db)?, id, false).await?,
    ))
}
async fn create(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<Input>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let value = merge(
        json!({"events":[],"headers":{},"is_active":true,"secret":null}),
        input,
    );
    validate(&value)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let id:i64=sqlx::query_scalar("INSERT INTO crm_webhooks(name,url,events,secret,headers,is_active,created_by) VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING id").bind(value["name"].as_str()).bind(value["url"].as_str()).bind(&value["events"]).bind(value["secret"].as_str()).bind(&value["headers"]).bind(value["is_active"].as_bool()).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(db)?;
    let row = read(&mut tx, id, false).await?;
    tx.commit().await.map_err(db)?;
    Ok((StatusCode::CREATED, Json(row)))
}
async fn update(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let value = merge(read(&mut tx, id, true).await?, input);
    validate(&value)?;
    sqlx::query("UPDATE crm_webhooks SET name=$2,url=$3,events=$4,secret=$5,headers=$6,is_active=$7,updated_at=NOW() WHERE id=$1").bind(id).bind(value["name"].as_str()).bind(value["url"].as_str()).bind(&value["events"]).bind(value["secret"].as_str()).bind(&value["headers"]).bind(value["is_active"].as_bool()).execute(&mut *tx).await.map_err(db)?;
    if value["is_active"] == false {
        sqlx::query("UPDATE crm_webhook_deliveries SET status='cancelled',error_message='Webhook disabled' WHERE webhook_id=$1 AND status='queued'").bind(id).execute(&mut *tx).await.map_err(db)?;
    }
    let row = read(&mut tx, id, false).await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row))
}
async fn remove(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    if sqlx::query("DELETE FROM crm_webhooks WHERE id=$1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(db)?
        .rows_affected()
        == 0
    {
        return Err(ApiError::not_found("Webhook not found"));
    }
    Ok(Json(json!({"message":"Webhook deleted"})))
}
async fn events(_admin: AdminUser) -> Json<Value> {
    Json(Value::Object(
        crm_webhooks::EVENTS
            .iter()
            .map(|(key, label)| (key.to_string(), json!(label)))
            .collect(),
    ))
}
async fn test_hook(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let row = read(&mut tx, id, true).await?;
    if row["is_active"] != true {
        return Err(invalid("Activate the webhook before sending a test"));
    }
    let payload = json!({"id":uuid::Uuid::new_v4(),"event":"webhook_test","timestamp":chrono::Utc::now(),"data":{"test":true,"webhook_id":id.to_string()}});
    let delivery:uuid::Uuid=sqlx::query_scalar("INSERT INTO crm_webhook_deliveries(webhook_id,event,payload) VALUES($1,'webhook_test',$2) RETURNING id").bind(id).bind(payload).fetch_one(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok((
        StatusCode::ACCEPTED,
        Json(json!({"delivery_id":delivery,"status":"queued","success":null,"response_code":null})),
    ))
}
async fn logs(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = filters.bounds()?;
    read(&mut *state.db.pool.acquire().await.map_err(db)?, id, false).await?;
    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_webhook_deliveries WHERE webhook_id=$1")
            .bind(id)
            .fetch_one(&state.db.pool)
            .await
            .map_err(db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(d)-'lease_token'-'lease_expires_at'||jsonb_build_object('webhook_id',webhook_id::text,'is_success',status='delivered') FROM crm_webhook_deliveries d WHERE webhook_id=$1 ORDER BY triggered_at DESC,id DESC LIMIT $2 OFFSET $3").bind(id).bind(size).bind(offset).fetch_all(&state.db.pool).await.map_err(db)?;
    let pages = ((total + size - 1) / size).max(1);
    Ok(Json(
        json!({"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":pages,"last_page":pages}}),
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/webhooks", get(list).post(create))
        .route("/webhooks/events", get(events))
        .route("/webhooks/{id}", get(get_one).put(update).delete(remove))
        .route("/webhooks/{id}/test", post(test_hook))
        .route("/webhooks/{id}/logs", get(logs))
}
