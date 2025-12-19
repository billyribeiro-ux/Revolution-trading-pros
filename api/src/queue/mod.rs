//! PostgreSQL-based job queue

pub mod worker;

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::models::Job;

/// Enqueue a new job
#[allow(dead_code)]
pub async fn enqueue(
    pool: &PgPool,
    queue: &str,
    job_type: &str,
    payload: serde_json::Value,
) -> Result<i64> {
    let job = Job::new(queue, job_type, payload);

    let result = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO jobs (queue, job_type, payload, status, attempts, max_attempts, available_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind(&job.queue)
    .bind(&job.job_type)
    .bind(&job.payload)
    .bind(&job.status)
    .bind(job.attempts)
    .bind(job.max_attempts)
    .bind(job.available_at)
    .bind(job.created_at)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

/// Enqueue a delayed job
#[allow(dead_code)]
pub async fn enqueue_delayed(
    pool: &PgPool,
    queue: &str,
    job_type: &str,
    payload: serde_json::Value,
    run_at: DateTime<Utc>,
) -> Result<i64> {
    let mut job = Job::new(queue, job_type, payload);
    job.available_at = run_at;

    let result = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO jobs (queue, job_type, payload, status, attempts, max_attempts, available_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind(&job.queue)
    .bind(&job.job_type)
    .bind(&job.payload)
    .bind(&job.status)
    .bind(job.attempts)
    .bind(job.max_attempts)
    .bind(job.available_at)
    .bind(job.created_at)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
