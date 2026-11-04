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
use hmac::{Hmac, KeyInit, Mac};
use reqwest::Client;
use serde::Serialize;
use sha2::Sha256;
use sqlx::PgPool;
use std::sync::LazyLock;
use tokio::time::{Duration, sleep};

use crate::{db::Database, models::Job};

type HmacSha256 = Hmac<Sha256>;

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("static reqwest Client must build")
});

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
    let postmark_token = std::env::var("POSTMARK_TOKEN")
        .or_else(|_| std::env::var("POSTMARK_API_KEY"))
        .unwrap_or_default();
    let from_email = std::env::var("POSTMARK_FROM_EMAIL")
        .or_else(|_| std::env::var("EMAIL_FROM_ADDRESS"))
        .or_else(|_| std::env::var("FROM_EMAIL"))
        .unwrap_or_default();

    if postmark_token.is_empty() {
        anyhow::bail!("Postmark is unavailable: configure POSTMARK_TOKEN or POSTMARK_API_KEY");
    }

    if from_email.trim().is_empty() {
        anyhow::bail!("Configure POSTMARK_FROM_EMAIL, EMAIL_FROM_ADDRESS or FROM_EMAIL");
    }
    send_email_request(
        &HTTP_CLIENT,
        "https://api.postmarkapp.com/email",
        &postmark_token,
        PostmarkEmail {
            from: from_email,
            to: to.to_string(),
            subject: subject.to_string(),
            html_body: body.to_string(),
            message_stream: "outbound".to_string(),
        },
    )
    .await
}

async fn send_email_request(
    client: &Client,
    endpoint: &str,
    token: &str,
    email: PostmarkEmail,
) -> Result<()> {
    let response: serde_json::Value = client
        .post(endpoint)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("X-Postmark-Server-Token", token)
        .json(&email)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    if response["ErrorCode"].as_i64() != Some(0)
        || response["MessageID"].as_str().is_none_or(str::is_empty)
    {
        anyhow::bail!(
            "Postmark did not acknowledge email acceptance: {}",
            response["ErrorCode"]
        );
    }
    Ok(())
}

/// Generate HMAC-SHA256 signature for webhook payload
fn generate_webhook_signature(payload: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC-SHA256 accepts any key length per RFC 2104");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Claim one delivery at a time with a recoverable lease and bounded retries.
async fn process_webhook_deliveries(pool: &PgPool) -> Result<u32> {
    #[derive(sqlx::FromRow)]
    struct Delivery {
        id: i64,
        event_type: String,
        payload: serde_json::Value,
        attempt_count: i32,
        url: String,
        secret: Option<String>,
        timeout_seconds: i32,
        retry_count: i32,
        headers: Option<serde_json::Value>,
    }
    sqlx::query("UPDATE webhook_deliveries d SET status='failed',error_message='Delivery lease expired after final attempt',next_retry_at=NULL FROM webhooks w WHERE d.webhook_id=w.id AND d.status='processing' AND d.next_retry_at<=NOW() AND d.attempt_count>=GREATEST(1,w.retry_count)")
        .execute(pool).await?;
    let mut processed = 0;
    for _ in 0..10 {
        let delivery:Option<Delivery>=sqlx::query_as("WITH next AS (SELECT d.id,w.url,w.secret,w.timeout_seconds,w.retry_count,w.headers FROM webhook_deliveries d JOIN webhooks w ON w.id=d.webhook_id WHERE w.is_active=true AND d.status IN ('pending','processing') AND (d.next_retry_at IS NULL OR d.next_retry_at<=NOW()) AND d.attempt_count<GREATEST(1,w.retry_count) ORDER BY d.created_at,d.id LIMIT 1 FOR UPDATE OF d SKIP LOCKED) UPDATE webhook_deliveries d SET status='processing',attempt_count=attempt_count+1,attempts=attempt_count+1,next_retry_at=NOW()+INTERVAL '2 minutes' FROM next WHERE d.id=next.id RETURNING d.id,d.event_type,d.payload,d.attempt_count,next.url,next.secret,next.timeout_seconds,next.retry_count,next.headers")
            .fetch_optional(pool).await?;
        let Some(delivery) = delivery else {
            break;
        };
        let url = delivery.url.clone();
        let validated = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::task::spawn_blocking(move || {
                crate::services::cms_webhooks::validate_webhook_url(&url)
            }),
        )
        .await;
        let mut permanent = false;
        let outcome = match validated {
            Ok(Ok(Ok(url))) => {
                let payload = serde_json::to_string(
                    &serde_json::json!({"event":delivery.event_type,"timestamp":Utc::now().to_rfc3339(),"data":delivery.payload}),
                )?;
                let mut request = HTTP_CLIENT
                    .post(url)
                    .timeout(Duration::from_secs(
                        delivery.timeout_seconds.clamp(1, 30) as u64
                    ))
                    .header("Content-Type", "application/json")
                    .header("X-Webhook-Event", &delivery.event_type)
                    .header("X-Webhook-Delivery-Id", delivery.id.to_string());
                if let Some(secret) = &delivery.secret {
                    request = request.header(
                        "X-Webhook-Signature",
                        format!("sha256={}", generate_webhook_signature(&payload, secret)),
                    );
                }
                if let Some(headers) = delivery
                    .headers
                    .as_ref()
                    .and_then(serde_json::Value::as_object)
                {
                    for (key, value) in headers {
                        if !matches!(
                            key.to_ascii_lowercase().as_str(),
                            "host"
                                | "content-length"
                                | "transfer-encoding"
                                | "connection"
                                | "x-webhook-signature"
                                | "x-webhook-event"
                                | "x-webhook-delivery-id"
                        ) && let Some(value) = value.as_str()
                        {
                            request = request.header(key, value);
                        }
                    }
                }
                match request.body(payload).send().await {
                    Ok(mut response) => {
                        let status = response.status().as_u16() as i32;
                        let mut bytes = Vec::new();
                        let mut read_error = None;
                        while bytes.len() < 1000 {
                            match response.chunk().await {
                                Ok(Some(chunk)) => {
                                    let take = chunk.len().min(1000 - bytes.len());
                                    bytes.extend_from_slice(&chunk[..take]);
                                }
                                Ok(None) => break,
                                Err(error) => {
                                    read_error = Some(error.to_string());
                                    break;
                                }
                            }
                        }
                        let error = read_error.or_else(|| {
                            (!(200..300).contains(&status))
                                .then(|| format!("Webhook returned HTTP {status}"))
                        });
                        (
                            Some(status),
                            String::from_utf8_lossy(&bytes).into_owned(),
                            error,
                        )
                    }
                    Err(error) => (None, String::new(), Some(error.to_string())),
                }
            }
            Ok(Ok(Err(error))) => {
                permanent = !matches!(
                    error,
                    crate::services::cms_webhooks::WebhookEgressError::ResolutionFailed
                );
                (
                    None,
                    String::new(),
                    Some(format!("Blocked by SSRF egress policy: {error}")),
                )
            }
            Ok(Err(error)) => (
                None,
                String::new(),
                Some(format!("Webhook validation worker failed: {error}")),
            ),
            Err(_) => (
                None,
                String::new(),
                Some("Webhook URL validation timed out".to_owned()),
            ),
        };
        let (http_status, body, error) = outcome;
        let status = if error.is_none() {
            "delivered"
        } else if permanent || delivery.attempt_count >= delivery.retry_count.max(1) {
            "failed"
        } else {
            "pending"
        };
        let backoff = match delivery.attempt_count {
            1 => 60,
            2 => 300,
            3 => 900,
            4 => 3600,
            _ => 14400,
        };
        sqlx::query("UPDATE webhook_deliveries SET status=$2,response_status=$3,response_body=$4,error_message=$5,delivered_at=CASE WHEN $2='delivered' THEN NOW() ELSE NULL END,next_retry_at=CASE WHEN $2='pending' THEN NOW()+make_interval(secs => $6) ELSE NULL END WHERE id=$1 AND status='processing' AND attempt_count=$7")
            .bind(delivery.id).bind(status).bind(http_status).bind(body).bind(error).bind(backoff).bind(delivery.attempt_count).execute(pool).await?;
        processed += 1;
    }
    Ok(processed)
}

/// Execute legacy numeric-ID content schedules atomically with their status.
async fn process_scheduled_content(pool: &PgPool) -> Result<u32> {
    use sqlx::Acquire;
    let mut processed = 0;
    for _ in 0..10 {
        let mut tx = pool.begin().await?;
        let item:Option<(i64,String,i64,String,Option<serde_json::Value>)>=sqlx::query_as("SELECT id,content_type,content_id,scheduled_action,payload FROM scheduled_content WHERE status='scheduled' AND scheduled_at<=NOW() ORDER BY scheduled_at,id LIMIT 1 FOR UPDATE SKIP LOCKED")
            .fetch_optional(&mut *tx).await?;
        let Some((id, content_type, content_id, action, payload)) = item else {
            break;
        };
        let mut effect = tx.begin().await?;
        let result = apply_legacy_content(
            &mut effect,
            &content_type,
            content_id,
            &action,
            payload.as_ref(),
        )
        .await;
        let error = match result {
            Ok(()) => {
                effect.commit().await?;
                None
            }
            Err(error) => {
                effect.rollback().await?;
                Some(error.to_string())
            }
        };
        sqlx::query("UPDATE scheduled_content SET status=$2,executed_at=CASE WHEN $2='executed' THEN NOW() ELSE NULL END,error_message=$3,updated_at=NOW() WHERE id=$1")
            .bind(id).bind(if error.is_none() {"executed"} else {"failed"}).bind(&error).execute(&mut *tx).await?;
        tx.commit().await?;
        processed += 1;
    }
    Ok(processed)
}

async fn apply_legacy_content(
    connection: &mut sqlx::PgConnection,
    content_type: &str,
    id: i64,
    action: &str,
    payload: Option<&serde_json::Value>,
) -> Result<()> {
    let published = match action {
        "publish" => true,
        "unpublish" => false,
        "update" => false,
        _ => anyhow::bail!("Unsupported scheduled action: {action}"),
    };
    let result = if action == "update" {
        let data = payload
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| anyhow::anyhow!("Scheduled update requires an object payload"))?;
        if data.is_empty()
            || data.len() > 8
            || payload.is_some_and(|v| v.to_string().len() > 2_000_000)
        {
            anyhow::bail!("Invalid scheduled update payload");
        }
        let allowed = match content_type {
            "post" | "posts" => &["title", "excerpt", "content_blocks"][..],
            "product" | "products" => &["name", "description", "long_description"][..],
            "course" | "courses" | "video" | "videos" => &["title", "description"][..],
            _ => anyhow::bail!("Unsupported content type: {content_type}"),
        };
        if data.iter().any(|(key, value)| {
            !allowed.contains(&key.as_str())
                || if key == "content_blocks" {
                    !value.is_array()
                } else {
                    value.as_str().is_none_or(|s| {
                        s.contains('\0')
                            || ((key == "title" || key == "name")
                                && (s.trim().is_empty() || s.chars().count() > 255))
                    })
                }
        }) {
            anyhow::bail!("Invalid staged content fields");
        }
        let query = match content_type {
            "post" | "posts" => {
                "UPDATE posts SET title=COALESCE($2->>'title',title),excerpt=COALESCE($2->>'excerpt',excerpt),content_blocks=COALESCE($2->'content_blocks',content_blocks),updated_at=NOW() WHERE id=$1"
            }
            "product" | "products" => {
                "UPDATE products SET name=COALESCE($2->>'name',name),description=COALESCE($2->>'description',description),long_description=COALESCE($2->>'long_description',long_description),updated_at=NOW() WHERE id=$1"
            }
            "course" | "courses" => {
                "UPDATE courses_enhanced SET title=COALESCE($2->>'title',title),description=COALESCE($2->>'description',description),updated_at=NOW() WHERE id=$1 AND deleted_at IS NULL"
            }
            "video" | "videos" => {
                "UPDATE videos SET title=COALESCE($2->>'title',title),description=COALESCE($2->>'description',description),updated_at=NOW() WHERE id=$1"
            }
            _ => unreachable!(),
        };
        sqlx::query(query)
            .bind(id)
            .bind(payload)
            .execute(&mut *connection)
            .await?
    } else {
        let query = match content_type {
            "post" | "posts" => {
                "UPDATE posts SET status=CASE WHEN $2 THEN 'published' ELSE 'draft' END,published_at=CASE WHEN $2 THEN COALESCE(published_at,NOW()) ELSE published_at END,updated_at=NOW() WHERE id=$1"
            }
            "product" | "products" => {
                "UPDATE products SET is_active=$2,updated_at=NOW() WHERE id=$1"
            }
            "course" | "courses" => {
                "UPDATE courses_enhanced SET is_published=$2,published_at=CASE WHEN $2 THEN COALESCE(published_at,NOW()) ELSE published_at END,updated_at=NOW() WHERE id=$1 AND deleted_at IS NULL"
            }
            "video" | "videos" => "UPDATE videos SET is_public=$2,updated_at=NOW() WHERE id=$1",
            _ => anyhow::bail!("Unsupported content type: {content_type}"),
        };
        sqlx::query(query)
            .bind(id)
            .bind(published)
            .execute(&mut *connection)
            .await?
    };
    if result.rows_affected() != 1 {
        anyhow::bail!("Scheduled content was not found");
    }
    Ok(())
}

/// Clean up expired preview tokens
async fn cleanup_expired_tokens(pool: &PgPool) -> Result<u32> {
    let result = sqlx::query(
        r"
        DELETE FROM preview_tokens
        WHERE expires_at < NOW() - INTERVAL '7 days'
        ",
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

/// Claim work atomically; a crashed worker's expired lease can be retried.
pub async fn process_next_job(db: &Database) -> anyhow::Result<Option<uuid::Uuid>> {
    sqlx::query("UPDATE jobs SET status='failed',error='Worker lease expired after final attempt',reserved_at=NULL WHERE status='processing' AND reserved_at<NOW()-INTERVAL '5 minutes' AND attempts>=max_attempts")
        .execute(&db.pool).await?;
    let job: Option<Job> = sqlx::query_as(
        "WITH next AS (SELECT id FROM jobs WHERE ((status='pending' AND available_at<=NOW()) OR (status='processing' AND reserved_at<NOW()-INTERVAL '5 minutes')) AND attempts<max_attempts ORDER BY available_at,id LIMIT 1 FOR UPDATE SKIP LOCKED) UPDATE jobs j SET status='processing',started_at=NOW(),reserved_at=NOW(),attempts=attempts+1 FROM next WHERE j.id=next.id RETURNING j.*"
    ).fetch_optional(&db.pool).await?;
    let Some(job) = job else {
        return Ok(None);
    };
    match process_job(&job).await {
        Ok(()) => {
            sqlx::query("UPDATE jobs SET status='completed',completed_at=NOW(),reserved_at=NULL,error=NULL WHERE id=$1 AND attempts=$2 AND status='processing'")
                .bind(job.id).bind(job.attempts).execute(&db.pool).await?;
        }
        Err(error) => {
            sqlx::query("UPDATE jobs SET status=CASE WHEN attempts>=max_attempts THEN 'failed' ELSE 'pending' END,error=$2,reserved_at=NULL,available_at=NOW()+make_interval(secs => (10*power(2,LEAST(attempts,6)))::int) WHERE id=$1 AND attempts=$3 AND status='processing'")
                .bind(job.id).bind(error.to_string()).bind(job.attempts).execute(&db.pool).await?;
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
        "process_payment" | "generate_report" | "sync_data" | "reindex_search"
        | "send_notification" => {
            anyhow::bail!(
                "Job handler '{}' is not configured or implemented",
                job.job_type
            )
        }
        _ => anyhow::bail!("Unknown job type: {}", job.job_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[tokio::test]
    async fn postmark_adapter_requires_provider_acknowledgement() {
        async fn provider(
            State(calls): State<Arc<AtomicUsize>>,
            headers: HeaderMap,
            Json(body): Json<serde_json::Value>,
        ) -> Json<serde_json::Value> {
            assert_eq!(
                headers["X-Postmark-Server-Token"],
                "controlled-adapter-token"
            );
            assert_eq!(body["To"], "fixture@completion.test");
            assert_eq!(body["HtmlBody"], "<p>Fixture</p>");
            Json(match calls.fetch_add(1, Ordering::SeqCst) {
                0 => serde_json::json!({"ErrorCode":10,"MessageID":"not-accepted"}),
                1 => serde_json::json!({"ErrorCode":0}),
                _ => serde_json::json!({"ErrorCode":0,"MessageID":"controlled-adapter-message"}),
            })
        }
        let calls = Arc::new(AtomicUsize::new(0));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/email", listener.local_addr().unwrap());
        let router = Router::new()
            .route("/email", post(provider))
            .with_state(calls.clone());
        let server = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        for accepted in [false, false, true] {
            let result = send_email_request(
                &HTTP_CLIENT,
                &endpoint,
                "controlled-adapter-token",
                PostmarkEmail {
                    from: "sender@completion.test".into(),
                    to: "fixture@completion.test".into(),
                    subject: "Fixture".into(),
                    html_body: "<p>Fixture</p>".into(),
                    message_stream: "outbound".into(),
                },
            )
            .await;
            assert_eq!(result.is_ok(), accepted, "{result:?}");
        }
        assert_eq!(calls.load(Ordering::SeqCst), 3);
        server.abort();
    }
}
