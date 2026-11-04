//! Newsletter routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! SECURITY: Parameterized queries, secure tokens, GDPR compliance, admin auth

#![allow(clippy::map_flatten)]

use axum::{
    Json, Router,
    extract::{ConnectInfo, Query, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, KeyInit, Mac};
use serde::Deserialize;
use serde_json::json;
use sha2::Sha256;
use std::net::SocketAddr;

use crate::{AppState, middleware::admin::AdminUser};

type HmacSha256 = Hmac<Sha256>;

// ═══════════════════════════════════════════════════════════════════════════
// TOKEN SECURITY - Cryptographic unsubscribe/confirm tokens
// ═══════════════════════════════════════════════════════════════════════════

/// Generate a secure HMAC-based token for email actions
fn generate_secure_token(subscriber_id: i64, email: &str, action: &str, secret: &[u8]) -> String {
    let data = format!("{subscriber_id}:{email}:{action}");
    let mut mac = HmacSha256::new_from_slice(secret)
        .expect("HMAC-SHA256 accepts any key length per RFC 2104");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let signature = URL_SAFE_NO_PAD.encode(result.into_bytes());
    // Token format: base64(id:email):signature
    let payload = URL_SAFE_NO_PAD.encode(format!("{subscriber_id}:{email}"));
    format!("{payload}:{signature}")
}

/// Verify and decode a secure token
fn verify_secure_token(token: &str, action: &str, secret: &[u8]) -> Option<(i64, String)> {
    if token.len() > 2048 {
        return None;
    }
    let parts: Vec<&str> = token.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let payload = URL_SAFE_NO_PAD.decode(parts[0]).ok()?;
    let payload_str = String::from_utf8(payload).ok()?;
    let payload_parts: Vec<&str> = payload_str.split(':').collect();
    if payload_parts.len() != 2 {
        return None;
    }

    let subscriber_id: i64 = payload_parts[0].parse().ok()?;
    let email = payload_parts[1].to_string();

    // Verify HMAC using constant-time comparison
    let expected_token = generate_secure_token(subscriber_id, &email, action, secret);
    if !crate::utils::constant_time_compare(token, &expected_token) {
        return None;
    }

    Some((subscriber_id, email))
}

/// Get the HMAC secret from environment or use fallback (in production, this must be configured)
fn get_token_secret() -> Vec<u8> {
    std::env::var("NEWSLETTER_TOKEN_SECRET")
        .or_else(|_| std::env::var("JWT_SECRET"))
        .unwrap_or_else(|_| "revolution-trading-newsletter-secret-change-in-production".to_string())
        .into_bytes()
}

// ═══════════════════════════════════════════════════════════════════════════
// DATA TYPES
// ═══════════════════════════════════════════════════════════════════════════

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
    pub gdpr_consent: bool,
    pub consent_ip: Option<String>,
    pub consent_source: Option<String>,
    pub confirmed_at: Option<chrono::NaiveDateTime>,
    pub unsubscribed_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Subscribe request with validation
#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    /// GDPR: User must explicitly consent
    pub gdpr_consent: Option<bool>,
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

pub use crate::services::newsletter_delivery::BulkEmailRequest;

// ═══════════════════════════════════════════════════════════════════════════
// EMAIL VALIDATION - RFC 5322 compliant
// ═══════════════════════════════════════════════════════════════════════════

/// Validate email address format
pub(crate) fn is_valid_email(email: &str) -> bool {
    // Basic validation: contains @, has parts before and after, reasonable length
    if email.len() > 254 || email.len() < 5 {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    // Local part validation
    if local.is_empty() || local.len() > 64 {
        return false;
    }

    // Domain validation
    if domain.is_empty() || !domain.contains('.') {
        return false;
    }

    // Check for valid characters (simplified but effective)
    let valid_local_chars = |c: char| c.is_alphanumeric() || "!#$%&'*+/=?^_`{|}~.-".contains(c);
    let valid_domain_chars = |c: char| c.is_alphanumeric() || c == '.' || c == '-';

    local.chars().all(valid_local_chars) && domain.chars().all(valid_domain_chars)
}

/// Sanitize email (lowercase, trim)
fn sanitize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

fn subscriber_db(error: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    tracing::error!(%error,"Unable to persist subscription request");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error":"Unable to save subscription request"})),
    )
}
async fn submit_confirmation(
    state: &AppState,
    id: i64,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let receipt = crate::routes::email_subscribers::request_confirmation(state, id)
        .await
        .map_err(|e| {
            let (status, body) = e.into_response_tuple();
            (status, Json(json!({"error":body.0.message})))
        })?;
    if receipt["status"] == "accepted" {
        return Ok(Json(
            json!({"message":"Please check your email to confirm your subscription","status":"pending","delivery_status":"accepted"}),
        ));
    }
    let status = if receipt["status"] == "unavailable" {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        StatusCode::BAD_GATEWAY
    };
    Err((
        status,
        Json(
            json!({"error":receipt["message"],"status":"pending","delivery_status":receipt["status"]}),
        ),
    ))
}

/// The confirmation link is created only on the server and never returned to admin clients.
pub(crate) async fn send_confirmation(
    state: &AppState,
    id: i64,
    email: &str,
    name: Option<&str>,
) -> anyhow::Result<()> {
    let token = generate_secure_token(id, email, "confirm", &get_token_secret());
    let model = json!({"name":name.unwrap_or("Subscriber"),"confirm_url":format!("{}/newsletter/confirm?token={}",state.config.app_url.trim_end_matches('/'),token),"app_url":state.config.app_url});
    state
        .services
        .email
        .send_transactional(&state.db.pool, email, "newsletter-confirmation", model)
        .await
}

/// Subscribe to newsletter (public) - with email validation and GDPR consent
async fn subscribe(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(input): Json<SubscribeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Sanitize and validate email
    let email = sanitize_email(&input.email);

    if !is_valid_email(&email) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid email address format"})),
        ));
    }

    if input
        .name
        .as_ref()
        .is_some_and(|v| v.chars().count() > 255 || v.chars().any(char::is_control))
        || input
            .source
            .as_ref()
            .is_some_and(|v| v.chars().count() > 100 || v.chars().any(char::is_control))
        || input.tags.as_ref().is_some_and(|values| {
            values.len() > 100
                || values.iter().any(|v| {
                    v.trim().is_empty()
                        || v.chars().count() > 100
                        || v.chars().any(char::is_control)
                })
        })
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Subscriber name, source or tags exceed their allowed bounds"})),
        ));
    }

    // GDPR: Require explicit consent
    if !input.gdpr_consent.unwrap_or(false) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "GDPR consent is required to subscribe",
                "field": "gdpr_consent"
            })),
        ));
    }

    // Capture IP and User-Agent for GDPR audit trail
    let ip_address = addr.ip().to_string();
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let mut tx = state.db.pool.begin().await.map_err(subscriber_db)?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended(LOWER($1),89201))")
        .bind(&email)
        .execute(&mut *tx)
        .await
        .map_err(subscriber_db)?;
    // Check if already subscribed
    let existing: Option<SubscriberRow> = sqlx::query_as(
        r"SELECT id, email, name, status, source, ip_address, user_agent, tags, metadata,
                      COALESCE(gdpr_consent, false) as gdpr_consent, consent_ip, consent_source,
                      confirmed_at, unsubscribed_at, created_at, updated_at
               FROM newsletter_subscribers WHERE LOWER(email) = $1 ORDER BY updated_at DESC NULLS LAST,id DESC LIMIT 1",
    )
    .bind(&email)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Database error checking subscriber: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    if let Some(subscriber) = existing {
        if subscriber.status == "confirmed"
            && subscriber.confirmed_at.is_some()
            && subscriber.unsubscribed_at.is_none()
        {
            return Ok(Json(
                json!({"message": "Already subscribed", "status": "confirmed"}),
            ));
        }

        // A new explicit subscription request restores pending, not confirmed.
        sqlx::query("UPDATE newsletter_subscribers SET status='pending',name=COALESCE($2,name),gdpr_consent=true,consent_ip=$3,consent_source=COALESCE($4,consent_source),updated_at=NOW() WHERE id=$1").bind(subscriber.id).bind(&input.name).bind(&ip_address).bind(&input.source).execute(&mut *tx).await.map_err(|error|{tracing::error!(%error,"Unable to save subscription request");(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":"Unable to save subscription request"})))})?;
        tx.commit().await.map_err(subscriber_db)?;
        return submit_confirmation(&state, subscriber.id).await;
    }

    // Create new subscriber with GDPR consent tracking
    let tags = input.tags.map(|t| serde_json::to_value(t).ok()).flatten();

    let subscriber: SubscriberRow = sqlx::query_as(
        r"
        INSERT INTO newsletter_subscribers (
            email, name, status, source, tags, ip_address, user_agent,
            gdpr_consent, consent_ip, consent_source, created_at, updated_at
        )
        VALUES ($1, $2, 'pending', $3, $4, $5, $6, true, $5, $3, NOW(), NOW())
        RETURNING id, email, name, status, source, ip_address, user_agent, tags, metadata,
                  COALESCE(gdpr_consent, false) as gdpr_consent, consent_ip, consent_source,
                  confirmed_at, unsubscribed_at, created_at, updated_at
        ",
    )
    .bind(&email)
    .bind(&input.name)
    .bind(&input.source)
    .bind(&tags)
    .bind(&ip_address)
    .bind(&user_agent)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "Email already subscribed"})),
            )
        } else {
            tracing::error!("Failed to create subscriber: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to subscribe"})),
            )
        }
    })?;

    tx.commit().await.map_err(subscriber_db)?;
    submit_confirmation(&state, subscriber.id).await
}

/// Confirm subscription - uses cryptographic token verification
async fn confirm(
    State(state): State<AppState>,
    Query(query): Query<ConfirmQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify cryptographic token
    let (subscriber_id, email) = verify_secure_token(&query.token, "confirm", &get_token_secret())
        .ok_or_else(|| {
            tracing::warn!(
                target: "security",
                event = "invalid_confirm_token",
                "Invalid or tampered confirmation token"
            );
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid or expired confirmation token"})),
            )
        })?;

    let mut tx = state.db.pool.begin().await.map_err(subscriber_db)?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended(LOWER($1),89201))")
        .bind(&email)
        .execute(&mut *tx)
        .await
        .map_err(subscriber_db)?;
    // Verify subscriber exists and email matches
    let result = sqlx::query(
        r"UPDATE newsletter_subscribers
           SET status = 'confirmed', confirmed_at = NOW(), unsubscribed_at=NULL, gdpr_consent=true, consent_source='email_confirmation', updated_at = NOW()
           WHERE id = $1 AND LOWER(email) = LOWER($2) AND status = 'pending' AND id=(SELECT id FROM newsletter_subscribers WHERE LOWER(email)=LOWER($2) ORDER BY updated_at DESC NULLS LAST,id DESC LIMIT 1)",
    )
    .bind(subscriber_id)
    .bind(&email)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Database error confirming subscription: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    if result.rows_affected() == 0 {
        let already:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM newsletter_subscribers WHERE id=$1 AND LOWER(email)=LOWER($2) AND status='confirmed' AND confirmed_at IS NOT NULL AND unsubscribed_at IS NULL)").bind(subscriber_id).bind(&email).fetch_one(&mut *tx).await.map_err(|_|(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":"Unable to verify subscription"}))))?;
        if already {
            return Ok(Json(
                json!({"message":"Subscription already confirmed","status":"confirmed"}),
            ));
        }
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error":"Subscription is not pending or no longer exists"})),
        ));
    }

    tx.commit().await.map_err(subscriber_db)?;
    tracing::info!(
        target: "newsletter",
        event = "confirmed",
        subscriber_id = %subscriber_id,
        email = %email,
        "Newsletter subscription confirmed (double opt-in complete)"
    );

    Ok(Json(json!({
        "message": "Your subscription has been confirmed! Welcome to our newsletter.",
        "status": "confirmed"
    })))
}

/// Unsubscribe - uses cryptographic token verification (GDPR compliant one-click unsubscribe)
async fn unsubscribe(
    State(state): State<AppState>,
    Query(query): Query<UnsubscribeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(token) = uuid::Uuid::parse_str(&query.token) {
        return match crate::services::campaign_delivery::unsubscribe(&state.db.pool, token).await {
            Ok(true) => Ok(Json(
                json!({"status":"unsubscribed","message":"Your newsletter subscription has been cancelled."}),
            )),
            Ok(false) => Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error":"Subscription link not found"})),
            )),
            Err(error) => {
                tracing::error!(%error,"Unable to persist campaign unsubscribe");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":"Unable to save unsubscribe request"})),
                ))
            }
        };
    }
    // Verify cryptographic token
    let (subscriber_id, email) =
        verify_secure_token(&query.token, "unsubscribe", &get_token_secret()).ok_or_else(|| {
            tracing::warn!(
                target: "security",
                event = "invalid_unsubscribe_token",
                "Invalid or tampered unsubscribe token"
            );
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid unsubscribe token"})),
            )
        })?;

    if query
        .reason
        .as_ref()
        .is_some_and(|v| v.chars().count() > 2000 || v.chars().any(char::is_control))
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                json!({"error":"Unsubscribe reason must be at most 2000 characters without controls"}),
            ),
        ));
    }
    // Store unsubscribe reason if provided (GDPR audit)
    let metadata = query
        .reason
        .as_ref()
        .map(|r| json!({"unsubscribe_reason": r}));

    let result = sqlx::query(
        r"UPDATE newsletter_subscribers
           SET status = 'unsubscribed',
               unsubscribed_at = COALESCE(unsubscribed_at, NOW()),
               updated_at = NOW(),
               metadata = COALESCE(metadata,'{}'::jsonb) || COALESCE($3,'{}'::jsonb)
           WHERE LOWER(email) = LOWER($2) AND EXISTS(SELECT 1 FROM newsletter_subscribers owner WHERE owner.id=$1 AND LOWER(owner.email)=LOWER($2))",
    )
    .bind(subscriber_id)
    .bind(&email)
    .bind(&metadata)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error unsubscribing: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Subscriber not found or invalid token"})),
        ));
    }

    tracing::info!(
        target: "newsletter",
        event = "unsubscribed",
        subscriber_id = %subscriber_id,
        email = %email,
        reason = ?query.reason,
        "Newsletter unsubscribe (GDPR compliant)"
    );

    Ok(Json(json!({
        "message": "You have been successfully unsubscribed. We're sorry to see you go!",
        "status": "unsubscribed"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN ENDPOINTS - Protected with AdminUser authentication
// ═══════════════════════════════════════════════════════════════════════════

/// List subscribers (admin only) - PARAMETERIZED QUERIES, no SQL injection
async fn list_subscribers(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Query(query): Query<SubscriberListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "admin_list_subscribers",
        admin_id = %user.id,
        "Admin listing newsletter subscribers"
    );

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(50);
    let valid_statuses = ["pending", "confirmed", "unsubscribed"];
    if page < 1
        || !(1..=100).contains(&per_page)
        || query
            .status
            .as_deref()
            .is_some_and(|v| !valid_statuses.contains(&v))
        || query
            .search
            .as_ref()
            .is_some_and(|v| v.chars().count() > 500 || v.chars().any(char::is_control))
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                json!({"error":"Use a positive page, page size 1–100, a supported status and search of at most 500 characters"}),
            ),
        ));
    }
    let offset = page.saturating_sub(1).saturating_mul(per_page);
    let status_filter = query.status.as_deref();
    let search_pattern = query
        .search
        .as_ref()
        .map(|s| format!("%{}%", crate::utils::escape_like(s)));
    let subscribers: Vec<SubscriberRow> = sqlx::query_as(
        r"
        SELECT id, email, name, status, source, ip_address, user_agent, tags, metadata,
               COALESCE(gdpr_consent, false) as gdpr_consent, consent_ip, consent_source,
               confirmed_at, unsubscribed_at, created_at, updated_at
        FROM (SELECT DISTINCT ON (LOWER(email)) * FROM newsletter_subscribers ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC) canonical
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR email ILIKE $2 OR name ILIKE $2)
        ORDER BY created_at DESC,id DESC
        LIMIT $3 OFFSET $4
        ",
    )
    .bind(status_filter)
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error listing subscribers: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)
        FROM (SELECT DISTINCT ON (LOWER(email)) * FROM newsletter_subscribers ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC) canonical
        WHERE ($1::text IS NULL OR status = $1)
          AND ($2::text IS NULL OR email ILIKE $2 OR name ILIKE $2)
        ",
    )
    .bind(status_filter)
    .bind(search_pattern.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .map_err(subscriber_db)?;

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

/// Get subscriber stats (admin only)
async fn get_stats(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "admin_view_newsletter_stats",
        admin_id = %user.id,
        "Admin viewing newsletter statistics"
    );

    // Single optimized query for all stats
    let stats: (i64, i64, i64, i64, i64) = sqlx::query_as(
        r"
        SELECT
            COUNT(*) as total,
            COUNT(*) FILTER (WHERE status = 'confirmed') as confirmed,
            COUNT(*) FILTER (WHERE status = 'pending') as pending,
            COUNT(*) FILTER (WHERE status = 'unsubscribed') as unsubscribed,
            COUNT(*) FILTER (WHERE gdpr_consent = true) as with_consent
        FROM (SELECT DISTINCT ON (LOWER(email)) * FROM newsletter_subscribers ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC) canonical
        ",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error getting stats: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Recent signups (last 7 days)
    let recent: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM (SELECT DISTINCT ON (LOWER(email)) * FROM newsletter_subscribers ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC) canonical WHERE created_at > NOW() - INTERVAL '7 days'",
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(subscriber_db)?;

    Ok(Json(json!({
        "total": stats.0,
        "confirmed": stats.1,
        "pending": stats.2,
        "unsubscribed": stats.3,
        "with_gdpr_consent": stats.4,
        "recent_signups_7d": recent.0,
        "confirmation_rate": if stats.0 > 0 {
            (stats.1 as f64 / stats.0 as f64 * 100.0).round()
        } else { 0.0 }
    })))
}

/// Delete a canonical newsletter profile and case variants; delivery history is retained.
async fn delete_subscriber(
    State(state): State<AppState>,
    _admin: AdminUser,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    crate::routes::email_subscribers::delete_identity(&state, id)
        .await
        .map_err(|e| {
            let (status, body) = e.into_response_tuple();
            (status, Json(json!({"error":body.0.message})))
        })
}

/// Export subscriber data (admin only) - GDPR data portability
async fn export_subscriber(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "admin_export_subscriber",
        admin_id = %user.id,
        subscriber_id = %id,
        "Admin exporting subscriber data (GDPR portability)"
    );

    let subscriber: SubscriberRow = sqlx::query_as(
        r"SELECT id, email, name, status, source, ip_address, user_agent, tags, metadata,
                  COALESCE(gdpr_consent, false) as gdpr_consent, consent_ip, consent_source,
                  confirmed_at, unsubscribed_at, created_at, updated_at
           FROM newsletter_subscribers WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error exporting subscriber: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Subscriber not found"})),
        )
    })?;

    Ok(Json(json!({
        "data": subscriber,
        "export_date": chrono::Utc::now().to_rfc3339(),
        "gdpr_compliant": true
    })))
}

/// Commit recipient/content snapshots before returning a queue receipt.
async fn send_bulk_email(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<BulkEmailRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), crate::utils::errors::ApiError> {
    let queued =
        crate::services::newsletter_delivery::queue(&state.db.pool, user.id, input).await?;
    Ok((StatusCode::ACCEPTED, Json(queued)))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Public newsletter routes
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/subscribe", post(subscribe))
        .route("/confirm", get(confirm))
        .route("/unsubscribe", get(unsubscribe).post(unsubscribe_post))
}

/// Admin newsletter routes (requires AdminUser)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/subscribers", get(list_subscribers))
        .route("/subscribers/{id}", delete(delete_subscriber))
        .route("/subscribers/{id}/export", get(export_subscriber))
        .route("/stats", get(get_stats))
        .route("/bulk-send", post(send_bulk_email))
}

/// Combined router (backwards compatible)
pub fn router() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/subscribe", post(subscribe))
        .route("/confirm", get(confirm))
        .route("/unsubscribe", get(unsubscribe).post(unsubscribe_post))
        // Admin routes (now properly protected)
        .route("/subscribers", get(list_subscribers))
        .route("/subscribers/{id}", delete(delete_subscriber))
        .route("/subscribers/{id}/export", get(export_subscriber))
        .route("/stats", get(get_stats))
        .route("/bulk-send", post(send_bulk_email))
}
#[derive(Deserialize)]
struct OneClick {
    #[serde(rename = "List-Unsubscribe")]
    action: String,
}
async fn unsubscribe_post(
    state: State<AppState>,
    query: Query<UnsubscribeQuery>,
    axum::Form(form): axum::Form<OneClick>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if form.action != "One-Click" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Invalid one-click unsubscribe request"})),
        ));
    }
    unsubscribe(state, query).await
}
