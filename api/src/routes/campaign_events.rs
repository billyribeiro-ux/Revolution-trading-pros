//! Authenticated Postmark facts for durable CRM broadcast receipts.
use crate::{AppState, utils::errors::ApiError};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::HeaderMap,
    routing::post,
};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

fn bad(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Unable to persist campaign provider event");
    ApiError::database_error("Unable to save provider event")
}
fn field<'a>(body: &'a Value, key: &str, max: usize) -> Result<&'a str, ApiError> {
    body[key]
        .as_str()
        .filter(|v| !v.is_empty() && v.len() <= max && !v.contains('\0'))
        .ok_or_else(|| bad("Provider event contains a missing or invalid field"))
}
async fn receive(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let secret = std::env::var("POSTMARK_WEBHOOK_TOKEN").unwrap_or_default();
    if secret.len() < 32 || secret.len() > 256 || !secret.bytes().all(|v| v.is_ascii_graphic()) {
        return Err(ApiError::new(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Provider webhook authentication is not configured",
        ));
    }
    let provided = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");
    if !crate::utils::constant_time_compare(provided, &secret) {
        return Err(ApiError::new(
            axum::http::StatusCode::UNAUTHORIZED,
            "Invalid provider credentials",
        ));
    }
    let record = field(&body, "RecordType", 50)?;
    let time_field = match record {
        "Delivery" => "DeliveredAt",
        "Open" | "Click" => "ReceivedAt",
        "Bounce" | "SpamComplaint" => "BouncedAt",
        "SubscriptionChange" => "ChangedAt",
        _ => return Ok(Json(json!({"status":"ignored"}))),
    };
    let stream = field(&body, "MessageStream", 100)?;
    let expected = std::env::var("POSTMARK_BROADCAST_STREAM").unwrap_or_default();
    if expected.trim().is_empty() {
        return Err(ApiError::new(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Broadcast stream is not configured",
        ));
    }
    if stream != expected.trim() {
        return Ok(Json(
            json!({"status":"ignored","reason":"different_message_stream"}),
        ));
    }
    let email = field(
        &body,
        if matches!(record, "Bounce" | "SpamComplaint") {
            "Email"
        } else {
            "Recipient"
        },
        255,
    )?
    .to_lowercase();
    crate::utils::errors::validate_email(&email).map_err(|_| bad("Invalid event recipient"))?;
    let occurred: DateTime<Utc> = DateTime::parse_from_rfc3339(field(&body, time_field, 100)?)
        .map_err(|_| bad("Provider event requires its actual timestamp"))?
        .with_timezone(&Utc);
    let message = body
        .get("MessageID")
        .and_then(Value::as_str)
        .filter(|v| !v.is_empty());
    if message.is_some_and(|v| v.len() > 255 || v.contains('\0')) {
        return Err(bad("Invalid provider message ID"));
    }
    if record != "SubscriptionChange" && message.is_none() {
        return Err(bad("Provider message ID is required"));
    }
    let suppress = match record {
        "SubscriptionChange" => body["SuppressSending"]
            .as_bool()
            .ok_or_else(|| bad("Subscription event requires SuppressSending"))?,
        "SpamComplaint" => true,
        "Bounce" => body["Inactive"].as_bool().unwrap_or(false),
        _ => false,
    };
    let metadata_id = body["Metadata"]["delivery_id"]
        .as_str()
        .map(Uuid::parse_str)
        .transpose()
        .map_err(|_| bad("Invalid delivery metadata"))?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let delivery:Option<(Uuid,i64,String,Option<String>)>=sqlx::query_as("SELECT id,campaign_id,email,provider_message_id FROM crm_campaign_deliveries WHERE provider IS DISTINCT FROM 'smtp' AND (($1::uuid IS NOT NULL AND id=$1) OR ($1::uuid IS NULL AND provider_message_id=$2)) LIMIT 1").bind(metadata_id).bind(message).fetch_optional(&mut *tx).await.map_err(db)?;
    if let Some((_, _, recipient, known)) = &delivery
        && (*recipient != email || known.as_deref().is_some_and(|known| Some(known) != message))
    {
        return Err(bad("Provider event does not match the recipient receipt"));
    }
    let mut campaigns: Vec<i64> = if suppress {
        sqlx::query_scalar("SELECT DISTINCT campaign_id FROM crm_campaign_deliveries WHERE email=$1 ORDER BY campaign_id").bind(&email).fetch_all(&mut *tx).await.map_err(db)?
    } else {
        delivery.as_ref().map(|v| vec![v.1]).unwrap_or_default()
    };
    campaigns.sort_unstable();
    campaigns.dedup();
    for id in &campaigns {
        sqlx::query("SELECT id FROM crm_campaigns WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
    }
    if let Some((id, _, _, _)) = &delivery {
        let (recipient, known): (String, Option<String>) = sqlx::query_as(
            "SELECT email,provider_message_id FROM crm_campaign_deliveries WHERE id=$1 FOR UPDATE",
        )
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
        if recipient != email || known.as_deref().is_some_and(|known| Some(known) != message) {
            return Err(bad(
                "Provider event does not match the current recipient receipt",
            ));
        }
    }
    let fingerprint = hex::encode(Sha256::digest(
        serde_json::to_vec(&json!([
            record,
            stream,
            email,
            message,
            occurred,
            body["ID"],
            body["OriginalLink"],
            body["SuppressSending"],
            body["SuppressionReason"]
        ]))
        .map_err(|_| bad("Invalid provider event"))?,
    ));
    // Keep only bounded provider observations needed by the established report.
    // No raw IP address, coordinates or user agent is retained here.
    let observed = |value: &Value, limit: usize| {
        value
            .as_str()
            .filter(|v| !v.is_empty() && v.len() <= limit && !v.chars().any(char::is_control))
            .map(str::to_owned)
    };
    let original_link = observed(&body["OriginalLink"], 4096).filter(|value| {
        reqwest::Url::parse(value).is_ok_and(|url| {
            matches!(url.scheme(), "http" | "https")
                && url.host_str().is_some()
                && url.username().is_empty()
                && url.password().is_none()
        })
    });
    let country_code = observed(&body["Geo"]["CountryISOCode"], 2)
        .filter(|v| v.len() == 2 && v.bytes().all(|b| b.is_ascii_alphabetic()))
        .map(|v| v.to_ascii_uppercase());
    let details = json!({"stream":stream,"suppressed":suppress,
        "reason":observed(&body["SuppressionReason"],100),
        "original_link":original_link,"platform":observed(&body["Platform"],50),
        "client":observed(&body["Client"]["Name"],200),
        "country":observed(&body["Geo"]["Country"],100),"country_code":country_code,
        "bounce_type":if record == "Bounce" {observed(&body["Type"],100)} else {None}
    });
    let inserted=sqlx::query("INSERT INTO crm_campaign_events(fingerprint,delivery_id,recipient_email,provider_message_id,record_type,occurred_at,details) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(fingerprint) DO NOTHING").bind(fingerprint).bind(delivery.as_ref().map(|v|v.0)).bind(&email).bind(message).bind(record).bind(occurred).bind(details).execute(&mut *tx).await.map_err(db)?;
    if inserted.rows_affected() == 0 {
        return Ok(Json(json!({"status":"duplicate"})));
    }
    if let Some((id, _, _, _)) = delivery {
        // An authenticated provider event can reconcile a lost response. Merely
        // queuing a campaign never creates a provider message or acceptance time.
        sqlx::query("UPDATE crm_campaign_deliveries SET provider='postmark',provider_stream=COALESCE(provider_stream,$5),status=CASE WHEN $2 IS NOT NULL THEN 'accepted' ELSE status END,provider_message_id=COALESCE(provider_message_id,$2),accepted_at=CASE WHEN $2 IS NOT NULL THEN COALESCE(accepted_at,NOW()) ELSE accepted_at END,delivered_at=CASE WHEN $3='Delivery' THEN LEAST(delivered_at,$4) ELSE delivered_at END,opened_at=CASE WHEN $3='Open' THEN LEAST(opened_at,$4) ELSE opened_at END,clicked_at=CASE WHEN $3='Click' THEN LEAST(clicked_at,$4) ELSE clicked_at END,bounced_at=CASE WHEN $3='Bounce' THEN LEAST(bounced_at,$4) ELSE bounced_at END,complained_at=CASE WHEN $3='SpamComplaint' THEN LEAST(complained_at,$4) ELSE complained_at END,last_error=CASE WHEN $2 IS NOT NULL THEN NULL ELSE last_error END,lease_token=CASE WHEN $2 IS NOT NULL THEN NULL ELSE lease_token END,lease_expires_at=CASE WHEN $2 IS NOT NULL THEN NULL ELSE lease_expires_at END,updated_at=NOW() WHERE id=$1 AND attempt_count>0").bind(id).bind(if record=="SubscriptionChange" {None} else {message}).bind(record).bind(occurred).bind(stream).execute(&mut *tx).await.map_err(db)?;
    }
    let newer_confirmation:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM newsletter_subscribers WHERE LOWER(email)=$1 AND status='confirmed' AND unsubscribed_at IS NULL AND confirmed_at > $2::timestamptz AT TIME ZONE 'UTC')").bind(&email).bind(occurred).fetch_one(&mut *tx).await.map_err(db)?;
    if suppress && !newer_confirmation {
        // Out-of-order suppression must not override a later explicit confirmed
        // subscription. Provider reactivation alone never fabricates consent.
        sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=COALESCE(unsubscribed_at,$2),updated_at=NOW() WHERE LOWER(email)=$1 AND (confirmed_at IS NULL OR confirmed_at <= $2::timestamptz AT TIME ZONE 'UTC')").bind(&email).bind(occurred).execute(&mut *tx).await.map_err(db)?;
        sqlx::query("UPDATE crm_campaign_deliveries SET status=CASE WHEN status='queued' THEN 'skipped' ELSE status END,unsubscribed_at=COALESCE(unsubscribed_at,$2),updated_at=NOW() WHERE email=$1").bind(email).bind(occurred).execute(&mut *tx).await.map_err(db)?;
    }
    for id in campaigns {
        crate::services::campaign_delivery::refresh(&mut tx, id)
            .await
            .map_err(|error| {
                tracing::error!(%error,"Unable to refresh provider counters");
                ApiError::database_error("Unable to update provider counters")
            })?;
    }
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"status":"recorded"})))
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/postmark/campaigns", post(receive))
        .layer(DefaultBodyLimit::max(65536))
}
