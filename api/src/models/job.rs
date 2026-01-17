//! Background job model for PostgreSQL queue

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: i64,
    pub queue: String,
    pub job_type: String,
    pub payload: serde_json::Value,
    pub status: String, // "pending", "processing", "completed", "failed"
    pub attempts: i32,
    pub max_attempts: i32,
    pub error: Option<String>,
    pub available_at: DateTime<Utc>,
    pub reserved_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum JobType {
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },
    ProcessPayment {
        user_id: Uuid,
        amount_cents: i32,
    },
    GenerateReport {
        report_type: String,
        user_id: Uuid,
    },
    SyncData {
        source: String,
    },
    SendNotification {
        user_id: Uuid,
        message: String,
    },
}

impl Job {
    #[allow(dead_code)]
    pub fn new(queue: &str, job_type: &str, payload: serde_json::Value) -> Self {
        Self {
            id: 0, // Will be assigned by database
            queue: queue.to_string(),
            job_type: job_type.to_string(),
            payload,
            status: "pending".to_string(),
            attempts: 0,
            max_attempts: 3,
            error: None,
            available_at: Utc::now(),
            reserved_at: None,
            started_at: None,
            completed_at: None,
            created_at: Utc::now(),
        }
    }
}
