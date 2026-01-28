//! CMS Webhook Delivery Service - Apple ICT 7+ Principal Engineer Grade
//!
//! Handles webhook delivery with:
//! - Async delivery with retries
//! - HMAC signature generation
//! - Exponential backoff
//! - Delivery logging
//!
//! @version 2.0.0 - January 2026

use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sha2::Sha256;
use sqlx::PgPool;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// Webhook delivery status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    Retrying,
}

impl DeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
        }
    }
}

/// Webhook delivery record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebhookDelivery {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub content_id: Option<Uuid>,
    pub payload: JsonValue,
    pub status: String,
    pub attempts: i32,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub headers: Option<JsonValue>,
    pub retry_count: i32,
    pub timeout_seconds: i32,
}

/// Webhook delivery result
#[derive(Debug, Serialize)]
pub struct DeliveryResult {
    pub success: bool,
    pub status_code: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: u64,
    pub error: Option<String>,
}

/// Generate HMAC-SHA256 signature for webhook payload
pub fn generate_signature(payload: &str, secret: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Create webhook payload
pub fn create_webhook_payload(
    event_type: &str,
    content_id: Option<Uuid>,
    data: JsonValue,
) -> JsonValue {
    json!({
        "event": event_type,
        "timestamp": Utc::now().to_rfc3339(),
        "contentId": content_id,
        "data": data
    })
}

/// Deliver webhook with signature
pub async fn deliver_webhook(
    client: &Client,
    config: &WebhookConfig,
    payload: &JsonValue,
) -> DeliveryResult {
    let start = std::time::Instant::now();
    let payload_str = serde_json::to_string(payload).unwrap_or_default();

    // Build request
    let mut request = client
        .post(&config.url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "RevolutionTradingPros-CMS/2.0")
        .header(
            "X-Webhook-Event",
            payload
                .get("event")
                .and_then(|e| e.as_str())
                .unwrap_or("unknown"),
        );

    // Add signature if secret is configured
    if let Some(secret) = &config.secret {
        let signature = generate_signature(&payload_str, secret);
        request = request
            .header("X-Webhook-Signature", format!("sha256={}", signature))
            .header("X-Webhook-Timestamp", Utc::now().timestamp().to_string());
    }

    // Add custom headers if configured
    if let Some(headers) = &config.headers {
        if let Some(headers_obj) = headers.as_object() {
            for (key, value) in headers_obj {
                if let Some(val_str) = value.as_str() {
                    request = request.header(key.as_str(), val_str);
                }
            }
        }
    }

    // Set timeout
    request = request.timeout(std::time::Duration::from_secs(
        config.timeout_seconds as u64,
    ));

    // Execute request
    match request.body(payload_str).send().await {
        Ok(response) => {
            let status = response.status().as_u16() as i32;
            let elapsed = start.elapsed().as_millis() as u64;

            let body = response.text().await.ok();
            let success = (200..300).contains(&(status as u16));

            DeliveryResult {
                success,
                status_code: Some(status),
                response_body: body,
                response_time_ms: elapsed,
                error: if success {
                    None
                } else {
                    Some(format!("HTTP {}", status))
                },
            }
        }
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            DeliveryResult {
                success: false,
                status_code: None,
                response_body: None,
                response_time_ms: elapsed,
                error: Some(e.to_string()),
            }
        }
    }
}

/// Process pending webhook deliveries
pub async fn process_pending_deliveries(
    pool: &PgPool,
    client: &Client,
    batch_size: i32,
) -> Result<i32> {
    let mut processed = 0;

    // Get pending deliveries
    let deliveries: Vec<(Uuid, Uuid, JsonValue, i32)> = sqlx::query_as(
        r#"
        SELECT d.id, d.webhook_id, d.payload, d.attempts
        FROM cms_webhook_deliveries d
        WHERE d.status = 'pending'
           OR (d.status = 'retrying' AND d.next_retry_at <= NOW())
        ORDER BY d.created_at
        LIMIT $1
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .bind(batch_size)
    .fetch_all(pool)
    .await?;

    for (delivery_id, webhook_id, payload, attempts) in deliveries {
        // Get webhook config
        let webhook: Option<WebhookConfig> = sqlx::query_as!(
            WebhookConfig,
            r#"
            SELECT id, name, url, secret, events, headers, retry_count, timeout_seconds
            FROM cms_webhooks
            WHERE id = $1 AND is_active = true
            "#,
            webhook_id
        )
        .fetch_optional(pool)
        .await?;

        let Some(config) = webhook else {
            // Webhook disabled or deleted, mark as failed
            sqlx::query(
                "UPDATE cms_webhook_deliveries SET status = 'failed', error_message = 'Webhook disabled' WHERE id = $1",
            )
            .bind(delivery_id)
            .execute(pool)
            .await?;
            continue;
        };

        // Deliver webhook
        let result = deliver_webhook(client, &config, &payload).await;

        // Update delivery record
        let new_status = if result.success {
            "delivered"
        } else if attempts + 1 >= config.retry_count {
            "failed"
        } else {
            "retrying"
        };

        // Calculate next retry time with exponential backoff
        let next_retry = if new_status == "retrying" {
            let delay_seconds = 2_i64.pow((attempts + 1) as u32).min(3600);
            Some(Utc::now() + Duration::seconds(delay_seconds))
        } else {
            None
        };

        sqlx::query(
            r#"
            UPDATE cms_webhook_deliveries
            SET status = $1,
                attempts = attempts + 1,
                response_status = $2,
                response_body = $3,
                response_time_ms = $4,
                error_message = $5,
                delivered_at = CASE WHEN $1 = 'delivered' THEN NOW() ELSE NULL END,
                next_retry_at = $6
            WHERE id = $7
            "#,
        )
        .bind(new_status)
        .bind(result.status_code)
        .bind(&result.response_body)
        .bind(result.response_time_ms as i32)
        .bind(&result.error)
        .bind(next_retry)
        .bind(delivery_id)
        .execute(pool)
        .await?;

        // Update webhook stats
        if result.success {
            sqlx::query(
                "UPDATE cms_webhooks SET success_count = success_count + 1, last_triggered_at = NOW() WHERE id = $1",
            )
            .bind(webhook_id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query("UPDATE cms_webhooks SET failure_count = failure_count + 1 WHERE id = $1")
                .bind(webhook_id)
                .execute(pool)
                .await?;
        }

        processed += 1;
    }

    Ok(processed)
}

/// Queue a webhook delivery for an event
pub async fn queue_webhook_event(
    pool: &PgPool,
    event_type: &str,
    content_id: Option<Uuid>,
    data: JsonValue,
) -> Result<i32> {
    let payload = create_webhook_payload(event_type, content_id, data);

    // Use database function to queue webhooks
    let count: i32 = sqlx::query_scalar("SELECT cms_queue_webhook_delivery($1, $2, $3)")
        .bind(event_type)
        .bind(content_id)
        .bind(&payload)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

/// Get delivery history for a webhook
pub async fn get_webhook_deliveries(
    pool: &PgPool,
    webhook_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WebhookDelivery>> {
    let deliveries: Vec<WebhookDelivery> = sqlx::query_as!(
        WebhookDelivery,
        r#"
        SELECT id, webhook_id, event_type, content_id, payload,
               status::text as "status!", attempts, response_status, response_body,
               response_time_ms, error_message, created_at,
               delivered_at, next_retry_at
        FROM cms_webhook_deliveries
        WHERE webhook_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        webhook_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(deliveries)
}

/// Retry a failed delivery
pub async fn retry_delivery(pool: &PgPool, delivery_id: Uuid) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE cms_webhook_deliveries
        SET status = 'pending',
            next_retry_at = NULL,
            error_message = NULL
        WHERE id = $1 AND status IN ('failed', 'retrying')
        "#,
    )
    .bind(delivery_id)
    .execute(pool)
    .await?;

    Ok(())
}
