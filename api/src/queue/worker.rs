//! Background job worker

use tokio::time::{sleep, Duration};

use crate::{db::Database, models::Job};

/// Run the job worker loop
pub async fn run(db: Database) {
    tracing::info!("Job worker started");

    loop {
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
    }
}

/// Fetch and process the next available job
async fn process_next_job(db: &Database) -> anyhow::Result<Option<uuid::Uuid>> {
    // Fetch next job using SELECT FOR UPDATE SKIP LOCKED
    let job: Option<Job> = sqlx::query_as(
        r#"
        SELECT * FROM jobs
        WHERE status = 'pending'
          AND run_at <= NOW()
          AND attempts < max_attempts
        ORDER BY run_at ASC
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
            sqlx::query(
                "UPDATE jobs SET status = 'completed', completed_at = NOW() WHERE id = $1",
            )
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

            sqlx::query(
                "UPDATE jobs SET status = $1, error = $2 WHERE id = $3",
            )
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
            let _body = job.payload["body"].as_str().unwrap_or("");
            tracing::info!("Sending email to: {} - {}", to, subject);
            // TODO: Actually send email here using EmailService
            Ok(())
        }
        "process_payment" => {
            let user_id = job.payload["user_id"].as_str().unwrap_or("");
            let amount = job.payload["amount_cents"].as_i64().unwrap_or(0);
            tracing::info!("Processing payment for user: {} - {} cents", user_id, amount);
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
        _ => {
            tracing::warn!("Unknown job type: {}", job.job_type);
            Ok(())
        }
    }
}
