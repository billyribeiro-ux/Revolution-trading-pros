//! CMS Scheduled Job Processor - Apple ICT 7+ Principal Engineer Grade
//!
//! Background worker for processing scheduled CMS jobs:
//! - Scheduled publishing
//! - Scheduled unpublishing
//! - Content archival
//! - Webhook delivery
//!
//! @version 2.0.0 - January 2026

use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

use crate::services::cms_webhooks;

/// Scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Interval between processing runs (in seconds)
    pub poll_interval_secs: u64,
    /// Maximum jobs to process per batch
    pub batch_size: i32,
    /// Enable webhook processing
    pub process_webhooks: bool,
    /// Enable content scheduling
    pub process_content: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            poll_interval_secs: 30,
            batch_size: 100,
            process_webhooks: true,
            process_content: true,
        }
    }
}

/// Start the scheduler as a background task
pub fn start_scheduler(pool: Arc<PgPool>, config: SchedulerConfig) {
    tokio::spawn(async move {
        info!("CMS scheduler started with {}s poll interval", config.poll_interval_secs);

        let mut ticker = interval(Duration::from_secs(config.poll_interval_secs));

        loop {
            ticker.tick().await;

            // Process content scheduling jobs
            if config.process_content {
                match process_scheduled_content(&pool, config.batch_size).await {
                    Ok(count) if count > 0 => {
                        info!("Processed {} scheduled content jobs", count);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error processing scheduled content: {}", e);
                    }
                }
            }

            // Process webhook deliveries
            if config.process_webhooks {
                let http_client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(30))
                    .build()
                    .expect("Failed to create HTTP client");

                match cms_webhooks::process_pending_deliveries(&pool, &http_client, config.batch_size).await
                {
                    Ok(count) if count > 0 => {
                        info!("Processed {} webhook deliveries", count);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error processing webhook deliveries: {}", e);
                    }
                }
            }

            // Process scheduled unpublishing
            match process_scheduled_unpublish(&pool).await {
                Ok(count) if count > 0 => {
                    info!("Unpublished {} scheduled content items", count);
                }
                Ok(_) => {}
                Err(e) => {
                    error!("Error processing scheduled unpublish: {}", e);
                }
            }

            // Clean up old audit logs (optional, can be run less frequently)
            // This would be better as a daily cron job
        }
    });
}

/// Process scheduled content jobs using database function
async fn process_scheduled_content(pool: &PgPool, batch_size: i32) -> Result<i32> {
    let count: i32 = sqlx::query_scalar("SELECT cms_process_scheduled_jobs($1)")
        .bind(batch_size)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

/// Process content that should be auto-published
async fn process_auto_publish(pool: &PgPool) -> Result<i32> {
    let result = sqlx::query(
        r#"
        UPDATE cms_content
        SET status = 'published',
            published_at = NOW(),
            updated_at = NOW()
        WHERE status = 'scheduled'
          AND scheduled_publish_at <= NOW()
          AND scheduled_publish_at IS NOT NULL
          AND deleted_at IS NULL
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i32)
}

/// Process content that should be auto-unpublished
async fn process_scheduled_unpublish(pool: &PgPool) -> Result<i32> {
    let result = sqlx::query(
        r#"
        UPDATE cms_content
        SET status = 'archived',
            updated_at = NOW()
        WHERE status = 'published'
          AND scheduled_unpublish_at <= NOW()
          AND scheduled_unpublish_at IS NOT NULL
          AND deleted_at IS NULL
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i32)
}

/// Get pending job count for monitoring
pub async fn get_pending_job_count(pool: &PgPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'pending'"
    )
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

/// Get failed job count for monitoring
pub async fn get_failed_job_count(pool: &PgPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_scheduled_jobs WHERE status = 'failed'"
    )
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

/// Get pending webhook delivery count
pub async fn get_pending_webhook_count(pool: &PgPool) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM cms_webhook_deliveries WHERE status IN ('pending', 'retrying')"
    )
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

/// Retry all failed jobs
pub async fn retry_all_failed_jobs(pool: &PgPool) -> Result<i32> {
    let result = sqlx::query(
        r#"
        UPDATE cms_scheduled_jobs
        SET status = 'pending',
            attempts = 0,
            error_message = NULL,
            locked_by = NULL,
            locked_at = NULL
        WHERE status = 'failed'
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i32)
}

/// Cancel a scheduled job
pub async fn cancel_job(pool: &PgPool, job_id: uuid::Uuid) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE cms_scheduled_jobs
        SET status = 'cancelled',
            updated_at = NOW()
        WHERE id = $1 AND status = 'pending'
        "#,
    )
    .bind(job_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Schedule a new job
pub async fn schedule_job(
    pool: &PgPool,
    job_type: &str,
    content_id: Option<uuid::Uuid>,
    scheduled_at: chrono::DateTime<chrono::Utc>,
    payload: Option<serde_json::Value>,
    created_by: Option<uuid::Uuid>,
) -> Result<uuid::Uuid> {
    let job_id: uuid::Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO cms_scheduled_jobs (job_type, content_id, scheduled_at, payload, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(job_type)
    .bind(content_id)
    .bind(scheduled_at)
    .bind(&payload)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(job_id)
}

/// Get scheduler health status
pub async fn get_scheduler_health(pool: &PgPool) -> Result<serde_json::Value> {
    let pending_jobs = get_pending_job_count(pool).await.unwrap_or(0);
    let failed_jobs = get_failed_job_count(pool).await.unwrap_or(0);
    let pending_webhooks = get_pending_webhook_count(pool).await.unwrap_or(0);

    // Get oldest pending job age
    let oldest_pending: Option<(chrono::DateTime<chrono::Utc>,)> = sqlx::query_as(
        "SELECT scheduled_at FROM cms_scheduled_jobs WHERE status = 'pending' ORDER BY scheduled_at LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    let oldest_age_secs = oldest_pending
        .map(|(scheduled_at,)| (chrono::Utc::now() - scheduled_at).num_seconds())
        .unwrap_or(0);

    Ok(serde_json::json!({
        "healthy": failed_jobs < 100 && oldest_age_secs < 3600,
        "pendingJobs": pending_jobs,
        "failedJobs": failed_jobs,
        "pendingWebhooks": pending_webhooks,
        "oldestPendingAgeSecs": oldest_age_secs.max(0)
    }))
}
