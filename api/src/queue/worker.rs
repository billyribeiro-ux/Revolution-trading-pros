//! Background job worker

use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use tokio::time::{sleep, Duration};

use crate::{db::Database, models::Job};

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
    let postmark_token = std::env::var("POSTMARK_API_KEY")
        .unwrap_or_else(|_| "".to_string());
    let from_email = std::env::var("FROM_EMAIL")
        .unwrap_or_else(|_| "noreply@revolutiontradingpros.com".to_string());

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
