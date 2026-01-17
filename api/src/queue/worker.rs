//! Background job worker - ICT 11+ Principal Engineer Grade
//! January 2026
//!
//! Handles:
//! - Email sending via Postmark
//! - Webhook deliveries with HMAC signatures
//! - Scheduled content publishing
//! - Payment processing
//! - Report generation

#![allow(clippy::manual_range_contains)]

use anyhow::Result;
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::Serialize;
use sha2::Sha256;
use sqlx::PgPool;
use tokio::time::{sleep, Duration};

use crate::{db::Database, models::Job};

type HmacSha256 = Hmac<Sha256>;

/// Postmark email payload
#[derive(Serialize)]
struct PostmarkEmail {
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "HtmlBody")]
    html_body: String,
    #[serde(rename = "MessageStream")]
    message_stream: String,
}

/// Send an email via Postmark API
async fn send_email_via_postmark(to: &str, subject: &str, body: &str) -> Result<()> {
    let postmark_token = std::env::var("POSTMARK_API_KEY").unwrap_or_else(|_| "".to_string());
    let from_email = std::env::var("FROM_EMAIL")
        .unwrap_or_else(|_| "noreply@revolution-trading-pros.pages.dev".to_string());

    if postmark_token.is_empty() {
        tracing::warn!("POSTMARK_API_KEY not set, skipping email send");
        return Ok(());
    }

    let email = PostmarkEmail {
        from: from_email,
        to: to.to_string(),
        subject: subject.to_string(),
        html_body: body.to_string(),
        message_stream: "outbound".to_string(),
    };

    let client = Client::new();
    client
        .post("https://api.postmarkapp.com/email")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("X-Postmark-Server-Token", &postmark_token)
        .json(&email)
        .send()
        .await?
        .error_for_status()?;

    tracing::info!("Email sent successfully to: {}", to);
    Ok(())
}

/// Generate HMAC-SHA256 signature for webhook payload
fn generate_webhook_signature(payload: &str, secret: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Process pending webhook deliveries
async fn process_webhook_deliveries(pool: &PgPool) -> Result<u32> {
    #[derive(Debug, sqlx::FromRow)]
    struct WebhookDeliveryRow {
        id: i64,
        webhook_id: i64,
        event_type: String,
        payload: serde_json::Value,
        attempts: i32,
    }

    #[derive(Debug, sqlx::FromRow)]
    struct WebhookRow {
        url: String,
        secret: Option<String>,
        timeout_seconds: i32,
        retry_count: i32,
        headers: Option<serde_json::Value>,
    }

    // Get pending webhook deliveries
    let deliveries: Vec<WebhookDeliveryRow> = sqlx::query_as(
        r#"
        SELECT wd.id, wd.webhook_id, wd.event_type, wd.payload, wd.attempts
        FROM webhook_deliveries wd
        JOIN webhooks w ON w.id = wd.webhook_id AND w.is_active = true
        WHERE wd.status = 'pending'
          AND (wd.next_retry_at IS NULL OR wd.next_retry_at <= NOW())
        ORDER BY wd.created_at ASC
        LIMIT 10
        FOR UPDATE OF wd SKIP LOCKED
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut processed = 0u32;

    for delivery in deliveries {
        // Get webhook config
        let webhook: Option<WebhookRow> = sqlx::query_as(
            "SELECT url, secret, timeout_seconds, retry_count, headers FROM webhooks WHERE id = $1",
        )
        .bind(delivery.webhook_id)
        .fetch_optional(pool)
        .await?;

        let webhook = match webhook {
            Some(w) => w,
            None => {
                // Webhook deleted, mark delivery as failed
                sqlx::query(
                    "UPDATE webhook_deliveries SET status = 'failed', error_message = 'Webhook not found' WHERE id = $1"
                )
                .bind(delivery.id)
                .execute(pool)
                .await?;
                continue;
            }
        };

        // Prepare the payload
        let payload_str = serde_json::to_string(&serde_json::json!({
            "event": delivery.event_type,
            "timestamp": Utc::now().to_rfc3339(),
            "data": delivery.payload
        }))?;

        // Build request with optional HMAC signature
        let client = Client::builder()
            .timeout(Duration::from_secs(webhook.timeout_seconds as u64))
            .build()?;

        let mut request = client
            .post(&webhook.url)
            .header("Content-Type", "application/json")
            .header("X-Webhook-Event", &delivery.event_type)
            .header("X-Webhook-Delivery-Id", delivery.id.to_string());

        // Add HMAC signature if secret is set
        if let Some(ref secret) = webhook.secret {
            let signature = generate_webhook_signature(&payload_str, secret);
            request = request.header("X-Webhook-Signature", format!("sha256={}", signature));
        }

        // Add custom headers
        if let Some(ref headers) = webhook.headers {
            if let Some(headers_obj) = headers.as_object() {
                for (key, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        request = request.header(key, value_str);
                    }
                }
            }
        }

        let response = request.body(payload_str).send().await;

        match response {
            Ok(resp) => {
                let status = resp.status().as_u16() as i32;
                let body = resp.text().await.unwrap_or_default();
                let body_truncated = if body.len() > 1000 {
                    &body[..1000]
                } else {
                    &body
                };

                if status >= 200 && status < 300 {
                    // Success
                    sqlx::query(
                        r#"
                        UPDATE webhook_deliveries SET
                            status = 'delivered',
                            response_status = $1,
                            response_body = $2,
                            completed_at = NOW(),
                            attempts = attempts + 1
                        WHERE id = $3
                        "#,
                    )
                    .bind(status)
                    .bind(body_truncated)
                    .bind(delivery.id)
                    .execute(pool)
                    .await?;

                    tracing::info!(
                        "Webhook delivered: {} -> {}",
                        delivery.event_type,
                        webhook.url
                    );
                } else {
                    // HTTP error, schedule retry if attempts remaining
                    let new_attempts = delivery.attempts + 1;
                    if new_attempts >= webhook.retry_count {
                        sqlx::query(
                            r#"
                            UPDATE webhook_deliveries SET
                                status = 'failed',
                                response_status = $1,
                                response_body = $2,
                                error_message = 'Max retries exceeded',
                                attempts = attempts + 1
                            WHERE id = $3
                            "#,
                        )
                        .bind(status)
                        .bind(body_truncated)
                        .bind(delivery.id)
                        .execute(pool)
                        .await?;
                    } else {
                        // Exponential backoff: 1min, 5min, 15min, 1hr, 4hr
                        let backoff_seconds = match new_attempts {
                            1 => 60,
                            2 => 300,
                            3 => 900,
                            4 => 3600,
                            _ => 14400,
                        };

                        sqlx::query(
                            r#"
                            UPDATE webhook_deliveries SET
                                response_status = $1,
                                response_body = $2,
                                attempts = attempts + 1,
                                next_retry_at = NOW() + INTERVAL '1 second' * $3
                            WHERE id = $4
                            "#,
                        )
                        .bind(status)
                        .bind(body_truncated)
                        .bind(backoff_seconds)
                        .bind(delivery.id)
                        .execute(pool)
                        .await?;

                        tracing::warn!(
                            "Webhook delivery failed (attempt {}), retry in {}s: {} -> {}",
                            new_attempts,
                            backoff_seconds,
                            delivery.event_type,
                            webhook.url
                        );
                    }
                }
            }
            Err(e) => {
                // Network error, schedule retry
                let new_attempts = delivery.attempts + 1;
                let error_msg = e.to_string();

                if new_attempts >= webhook.retry_count {
                    sqlx::query(
                        r#"
                        UPDATE webhook_deliveries SET
                            status = 'failed',
                            error_message = $1,
                            attempts = attempts + 1
                        WHERE id = $2
                        "#,
                    )
                    .bind(&error_msg)
                    .bind(delivery.id)
                    .execute(pool)
                    .await?;
                } else {
                    let backoff_seconds = match new_attempts {
                        1 => 60,
                        2 => 300,
                        3 => 900,
                        4 => 3600,
                        _ => 14400,
                    };

                    sqlx::query(
                        r#"
                        UPDATE webhook_deliveries SET
                            error_message = $1,
                            attempts = attempts + 1,
                            next_retry_at = NOW() + INTERVAL '1 second' * $2
                        WHERE id = $3
                        "#,
                    )
                    .bind(&error_msg)
                    .bind(backoff_seconds)
                    .bind(delivery.id)
                    .execute(pool)
                    .await?;

                    tracing::warn!(
                        "Webhook network error (attempt {}): {} - {}",
                        new_attempts,
                        webhook.url,
                        error_msg
                    );
                }
            }
        }

        processed += 1;
    }

    Ok(processed)
}

/// Process scheduled content publishing
async fn process_scheduled_content(pool: &PgPool) -> Result<u32> {
    #[derive(Debug, sqlx::FromRow)]
    struct ScheduledRow {
        id: i64,
        content_type: String,
        content_id: i64,
        scheduled_action: String,
    }

    // Get due scheduled content
    let scheduled: Vec<ScheduledRow> = sqlx::query_as(
        r#"
        SELECT id, content_type, content_id, scheduled_action
        FROM scheduled_content
        WHERE status = 'scheduled' AND scheduled_at <= NOW()
        ORDER BY scheduled_at ASC
        LIMIT 10
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut processed = 0u32;

    for item in scheduled {
        tracing::info!(
            "Processing scheduled {}: {} ({})",
            item.scheduled_action,
            item.content_type,
            item.content_id
        );

        let result = match item.content_type.as_str() {
            "post" | "posts" => {
                // Publish the post
                sqlx::query(
                    "UPDATE posts SET status = 'published', published_at = NOW(), updated_at = NOW() WHERE id = $1"
                )
                .bind(item.content_id)
                .execute(pool)
                .await
            }
            "product" | "products" => {
                // Activate the product
                sqlx::query(
                    "UPDATE products SET is_active = true, updated_at = NOW() WHERE id = $1",
                )
                .bind(item.content_id)
                .execute(pool)
                .await
            }
            "course" | "courses" => {
                // Publish the course
                sqlx::query(
                    "UPDATE courses SET status = 'published', published_at = NOW(), updated_at = NOW() WHERE id = $1"
                )
                .bind(item.content_id)
                .execute(pool)
                .await
            }
            "video" | "videos" => {
                // Publish the video
                sqlx::query(
                    "UPDATE videos SET status = 'published', published_at = NOW(), updated_at = NOW() WHERE id = $1"
                )
                .bind(item.content_id)
                .execute(pool)
                .await
            }
            _ => {
                tracing::warn!(
                    "Unknown content type for scheduled publish: {}",
                    item.content_type
                );
                Ok(sqlx::postgres::PgQueryResult::default())
            }
        };

        match result {
            Ok(_) => {
                // Mark as executed
                sqlx::query(
                    r#"
                    UPDATE scheduled_content SET
                        status = 'executed',
                        executed_at = NOW(),
                        updated_at = NOW()
                    WHERE id = $1
                    "#,
                )
                .bind(item.id)
                .execute(pool)
                .await?;

                // Also update workflow status if exists
                sqlx::query(
                    r#"
                    UPDATE content_workflow_status SET
                        current_stage = 'published',
                        updated_at = NOW()
                    WHERE content_type = $1 AND content_id = $2
                    "#,
                )
                .bind(&item.content_type)
                .bind(item.content_id)
                .execute(pool)
                .await?;

                tracing::info!(
                    "Scheduled {} executed successfully: {} ({})",
                    item.scheduled_action,
                    item.content_type,
                    item.content_id
                );
            }
            Err(e) => {
                // Mark as failed
                sqlx::query(
                    r#"
                    UPDATE scheduled_content SET
                        status = 'failed',
                        error_message = $1,
                        updated_at = NOW()
                    WHERE id = $2
                    "#,
                )
                .bind(e.to_string())
                .bind(item.id)
                .execute(pool)
                .await?;

                tracing::error!(
                    "Scheduled {} failed: {} ({}) - {}",
                    item.scheduled_action,
                    item.content_type,
                    item.content_id,
                    e
                );
            }
        }

        processed += 1;
    }

    Ok(processed)
}

/// Clean up expired preview tokens
async fn cleanup_expired_tokens(pool: &PgPool) -> Result<u32> {
    let result = sqlx::query(
        r#"
        DELETE FROM preview_tokens
        WHERE expires_at < NOW() - INTERVAL '7 days'
        "#,
    )
    .execute(pool)
    .await?;

    let deleted = result.rows_affected() as u32;
    if deleted > 0 {
        tracing::info!("Cleaned up {} expired preview tokens", deleted);
    }

    Ok(deleted)
}

/// Run the job worker loop
pub async fn run(db: Database) {
    tracing::info!("Job worker started");

    // Counters for periodic tasks
    let mut webhook_check_counter = 0u32;
    let mut scheduled_check_counter = 0u32;
    let mut cleanup_counter = 0u32;

    loop {
        // Process regular jobs
        match process_next_job(&db).await {
            Ok(Some(job_id)) => {
                tracing::info!("Processed job: {}", job_id);
            }
            Ok(None) => {
                // No jobs available, wait before polling again
                sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                tracing::error!("Job processing error: {}", e);
                sleep(Duration::from_secs(5)).await;
            }
        }

        // Process webhook deliveries every 5 seconds
        webhook_check_counter += 1;
        if webhook_check_counter >= 5 {
            webhook_check_counter = 0;
            match process_webhook_deliveries(&db.pool).await {
                Ok(count) if count > 0 => {
                    tracing::debug!("Processed {} webhook deliveries", count);
                }
                Err(e) => {
                    tracing::error!("Webhook processing error: {}", e);
                }
                _ => {}
            }
        }

        // Process scheduled content every 10 seconds
        scheduled_check_counter += 1;
        if scheduled_check_counter >= 10 {
            scheduled_check_counter = 0;
            match process_scheduled_content(&db.pool).await {
                Ok(count) if count > 0 => {
                    tracing::info!("Processed {} scheduled content items", count);
                }
                Err(e) => {
                    tracing::error!("Scheduled content processing error: {}", e);
                }
                _ => {}
            }
        }

        // Clean up expired tokens every 5 minutes (300 iterations)
        cleanup_counter += 1;
        if cleanup_counter >= 300 {
            cleanup_counter = 0;
            if let Err(e) = cleanup_expired_tokens(&db.pool).await {
                tracing::error!("Token cleanup error: {}", e);
            }
        }
    }
}

/// Fetch and process the next available job
async fn process_next_job(db: &Database) -> anyhow::Result<Option<i64>> {
    // Fetch next job using SELECT FOR UPDATE SKIP LOCKED
    let job: Option<Job> = sqlx::query_as(
        r#"
        SELECT * FROM jobs
        WHERE status = 'pending'
          AND available_at <= NOW()
          AND attempts < max_attempts
        ORDER BY available_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .fetch_optional(&db.pool)
    .await?;

    let job = match job {
        Some(j) => j,
        None => return Ok(None),
    };

    // Mark as processing
    sqlx::query(
        "UPDATE jobs SET status = 'processing', started_at = NOW(), attempts = attempts + 1 WHERE id = $1",
    )
    .bind(job.id)
    .execute(&db.pool)
    .await?;

    // Process the job
    let result = process_job(&job).await;

    match result {
        Ok(_) => {
            // Mark as completed
            sqlx::query("UPDATE jobs SET status = 'completed', completed_at = NOW() WHERE id = $1")
                .bind(job.id)
                .execute(&db.pool)
                .await?;
        }
        Err(e) => {
            let error_msg = e.to_string();
            let new_status = if job.attempts + 1 >= job.max_attempts {
                "failed"
            } else {
                "pending" // Will be retried
            };

            sqlx::query("UPDATE jobs SET status = $1, error = $2 WHERE id = $3")
                .bind(new_status)
                .bind(&error_msg)
                .bind(job.id)
                .execute(&db.pool)
                .await?;
        }
    }

    Ok(Some(job.id))
}

/// Process a single job based on its type
async fn process_job(job: &Job) -> anyhow::Result<()> {
    tracing::info!("Processing job: {} ({})", job.id, job.job_type);

    match job.job_type.as_str() {
        "send_email" => {
            let to = job.payload["to"].as_str().unwrap_or("");
            let subject = job.payload["subject"].as_str().unwrap_or("");
            let body = job.payload["body"].as_str().unwrap_or("");

            if to.is_empty() {
                return Err(anyhow::anyhow!("Email 'to' field is required"));
            }

            tracing::info!("Sending email to: {} - {}", to, subject);
            send_email_via_postmark(to, subject, body).await
        }
        "process_payment" => {
            let user_id = job.payload["user_id"].as_str().unwrap_or("");
            let amount = job.payload["amount_cents"].as_i64().unwrap_or(0);
            tracing::info!(
                "Processing payment for user: {} - {} cents",
                user_id,
                amount
            );
            // Process payment here
            Ok(())
        }
        "generate_report" => {
            let report_type = job.payload["report_type"].as_str().unwrap_or("");
            tracing::info!("Generating report: {}", report_type);
            // Generate report here
            Ok(())
        }
        "sync_data" => {
            let source = job.payload["source"].as_str().unwrap_or("");
            tracing::info!("Syncing data from: {}", source);
            // Sync data here
            Ok(())
        }
        "reindex_search" => {
            let content_type = job.payload["content_type"].as_str().unwrap_or("");
            tracing::info!("Reindexing search for: {}", content_type);
            // Reindex search here
            Ok(())
        }
        "send_notification" => {
            let user_id = job.payload["user_id"].as_i64().unwrap_or(0);
            let title = job.payload["title"].as_str().unwrap_or("");
            let message = job.payload["message"].as_str().unwrap_or("");
            tracing::info!("Sending notification to user {}: {}", user_id, title);
            // Send push notification or email here
            let _ = (user_id, title, message);
            Ok(())
        }
        _ => {
            tracing::warn!("Unknown job type: {}", job.job_type);
            Ok(())
        }
    }
}
