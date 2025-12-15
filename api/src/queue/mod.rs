//! PostgreSQL-based job queue

pub mod worker;

use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Job;

/// Enqueue a new job
pub async fn enqueue(
    pool: &PgPool,
    queue: &str,
    job_type: &str,
    payload: serde_json::Value,
) -> Result<Uuid> {
    let job = Job::new(queue, job_type, payload);

    sqlx::query(
        r#"
        INSERT INTO jobs (id, queue, job_type, payload, status, attempts, max_attempts, run_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(job.id)
    .bind(&job.queue)
    .bind(&job.job_type)
    .bind(&job.payload)
    .bind(&job.status)
    .bind(job.attempts)
    .bind(job.max_attempts)
    .bind(job.run_at)
    .bind(job.created_at)
    .execute(pool)
    .await?;

    Ok(job.id)
}

/// Enqueue a delayed job
pub async fn enqueue_delayed(
    pool: &PgPool,
    queue: &str,
    job_type: &str,
    payload: serde_json::Value,
    run_at: DateTime<Utc>,
) -> Result<Uuid> {
    let mut job = Job::new(queue, job_type, payload);
    job.run_at = run_at;

    sqlx::query(
        r#"
        INSERT INTO jobs (id, queue, job_type, payload, status, attempts, max_attempts, run_at, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(job.id)
    .bind(&job.queue)
    .bind(&job.job_type)
    .bind(&job.payload)
    .bind(&job.status)
    .bind(job.attempts)
    .bind(job.max_attempts)
    .bind(job.run_at)
    .bind(job.created_at)
    .execute(pool)
    .await?;

    Ok(job.id)
}
