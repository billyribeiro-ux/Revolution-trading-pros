//! Background job model for PostgreSQL queue

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub queue: String,
    pub job_type: String,
    pub payload: serde_json::Value,
    pub status: String, // "pending", "processing", "completed", "failed"
    pub attempts: i32,
    pub max_attempts: i32,
    pub error: Option<String>,
    pub run_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    SendEmail { to: String, subject: String, body: String },
    ProcessPayment { user_id: Uuid, amount_cents: i32 },
    GenerateReport { report_type: String, user_id: Uuid },
    SyncData { source: String },
    SendNotification { user_id: Uuid, message: String },
}

impl Job {
    pub fn new(queue: &str, job_type: &str, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            queue: queue.to_string(),
            job_type: job_type.to_string(),
            payload,
            status: "pending".to_string(),
            attempts: 0,
            max_attempts: 3,
            error: None,
            run_at: Utc::now(),
            started_at: None,
            completed_at: None,
            created_at: Utc::now(),
        }
    }
}
