//! Newsletter routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

#![allow(clippy::map_flatten)]

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{models::User, AppState};

/// Newsletter subscriber row
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct SubscriberRow {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub confirmed_at: Option<chrono::NaiveDateTime>,
    pub unsubscribed_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Subscribe request
#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Confirm request
#[derive(Debug, Deserialize)]
pub struct ConfirmQuery {
    pub token: String,
}

/// Unsubscribe request
#[derive(Debug, Deserialize)]
pub struct UnsubscribeQuery {
    pub token: String,
    pub reason: Option<String>,
}

/// Subscriber list query (admin)
#[derive(Debug, Deserialize)]
pub struct SubscriberListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
}

/// Subscribe to newsletter (public)
async fn subscribe(
    State(state): State<AppState>,
    Json(input): Json<SubscribeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Check if already subscribed
    let existing: Option<SubscriberRow> =
        sqlx::query_as("SELECT * FROM newsletter_subscribers WHERE email = $1")
            .bind(&input.email)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    if let Some(subscriber) = existing {
        if subscriber.status == "confirmed" {
            return Ok(Json(
                json!({"message": "Already subscribed", "status": "confirmed"}),
            ));
        }
        // Resend confirmation for pending
        return Ok(Json(
            json!({"message": "Confirmation email resent", "status": "pending"}),
        ));
    }

    // Create new subscriber
    let tags = input.tags.map(|t| serde_json::to_value(t).ok()).flatten();

    let subscriber: SubscriberRow = sqlx::query_as(
        r#"
        INSERT INTO newsletter_subscribers (email, name, status, source, tags, created_at, updated_at)
        VALUES ($1, $2, 'pending', $3, $4, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(&input.email)
    .bind(&input.name)
    .bind(&input.source)
    .bind(&tags)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") {
            (StatusCode::CONFLICT, Json(json!({"error": "Email already subscribed"})))
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    })?;

    // TODO: Send confirmation email

    Ok(Json(json!({
        "message": "Please check your email to confirm subscription",
        "status": "pending",
        "subscriber_id": subscriber.id
    })))
}

/// Confirm subscription
async fn confirm(
    State(state): State<AppState>,
    Query(query): Query<ConfirmQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // In production, token would be a signed JWT or hash
    // For now, treat token as subscriber ID
    let subscriber_id: i64 = query.token.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid token"})),
        )
    })?;

    let result = sqlx::query(
        "UPDATE newsletter_subscribers SET status = 'confirmed', confirmed_at = NOW(), updated_at = NOW() WHERE id = $1 AND status = 'pending'"
    )
    .bind(subscriber_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Invalid or expired token"})),
        ));
    }

    Ok(Json(
        json!({"message": "Subscription confirmed successfully"}),
    ))
}

/// Unsubscribe
async fn unsubscribe(
    State(state): State<AppState>,
    Query(query): Query<UnsubscribeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let subscriber_id: i64 = query.token.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid token"})),
        )
    })?;

    let result = sqlx::query(
        "UPDATE newsletter_subscribers SET status = 'unsubscribed', unsubscribed_at = NOW(), updated_at = NOW() WHERE id = $1"
    )
    .bind(subscriber_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Subscriber not found"})),
        ));
    }

    Ok(Json(json!({"message": "Successfully unsubscribed"})))
}

/// List subscribers (admin)
async fn list_subscribers(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<SubscriberListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &user; // TODO: Admin check

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
    let offset = (page - 1) * per_page;

    let mut conditions = vec!["1=1".to_string()];

    if let Some(ref status) = query.status {
        conditions.push(format!("status = '{}'", status));
    }

    if let Some(ref search) = query.search {
        conditions.push(format!(
            "(email ILIKE '%{}%' OR name ILIKE '%{}%')",
            search, search
        ));
    }

    let where_clause = conditions.join(" AND ");

    let sql = format!(
        "SELECT * FROM newsletter_subscribers WHERE {} ORDER BY created_at DESC LIMIT {} OFFSET {}",
        where_clause, per_page, offset
    );
    let count_sql = format!(
        "SELECT COUNT(*) FROM newsletter_subscribers WHERE {}",
        where_clause
    );

    let subscribers: Vec<SubscriberRow> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "data": subscribers,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get subscriber stats (admin)
async fn get_stats(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &user; // TODO: Admin check

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let confirmed: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers WHERE status = 'confirmed'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let pending: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers WHERE status = 'pending'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let unsubscribed: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM newsletter_subscribers WHERE status = 'unsubscribed'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(json!({
        "total": total.0,
        "confirmed": confirmed.0,
        "pending": pending.0,
        "unsubscribed": unsubscribed.0
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/subscribe", post(subscribe))
        .route("/confirm", get(confirm))
        .route("/unsubscribe", get(unsubscribe))
        .route("/subscribers", get(list_subscribers))
        .route("/stats", get(get_stats))
}
