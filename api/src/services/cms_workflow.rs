//! CMS Workflow Management Service - Apple ICT 7 Principal Engineer Grade
//!
//! Multi-stage editorial workflow with assignments and approvals.
//! Supports:
//! - Draft → Review → Approved → Published workflow
//! - Assignment system with due dates and priorities
//! - Workflow history tracking
//! - Team collaboration
//!
//! @version 1.0.0 - January 27, 2026

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[sqlx(type_name = "cms_workflow_stage", rename_all = "lowercase")]
pub enum WorkflowStage {
    Draft,
    #[sqlx(rename = "in_review")]
    InReview,
    Approved,
    Published,
    Archived,
}

impl WorkflowStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkflowStage::Draft => "draft",
            WorkflowStage::InReview => "in_review",
            WorkflowStage::Approved => "approved",
            WorkflowStage::Published => "published",
            WorkflowStage::Archived => "archived",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "cms_workflow_priority", rename_all = "lowercase")]
pub enum WorkflowPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkflowStatus {
    pub id: Uuid,
    pub content_id: Uuid,
    pub current_stage: WorkflowStage,
    pub previous_stage: Option<WorkflowStage>,
    pub assigned_to: Option<i64>,
    pub assigned_by: Option<i64>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: WorkflowPriority,
    pub notes: Option<String>,
    pub transition_comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkflowHistory {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub content_id: Uuid,
    pub from_stage: Option<WorkflowStage>,
    pub to_stage: WorkflowStage,
    pub comment: Option<String>,
    pub transitioned_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// WORKFLOW MANAGEMENT FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get or create workflow status for content
pub async fn get_or_create_workflow_status(
    pool: &PgPool,
    content_id: Uuid,
) -> Result<WorkflowStatus> {
    // Try to get existing workflow
    if let Some(workflow) = get_workflow_status(pool, content_id).await? {
        return Ok(workflow);
    }

    // Create new workflow if doesn't exist
    let workflow: WorkflowStatus = sqlx::query_as(
        r#"
        INSERT INTO cms_workflow_status (content_id, current_stage, priority)
        VALUES ($1, 'draft', 'normal')
        RETURNING id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
                  assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        "#,
    )
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(workflow)
}

/// Get workflow status for content
pub async fn get_workflow_status(pool: &PgPool, content_id: Uuid) -> Result<Option<WorkflowStatus>> {
    let workflow: Option<WorkflowStatus> = sqlx::query_as(
        r#"
        SELECT id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
               assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        FROM cms_workflow_status
        WHERE content_id = $1
        "#,
    )
    .bind(content_id)
    .fetch_optional(pool)
    .await?;

    Ok(workflow)
}

/// Transition workflow to new stage
pub async fn transition_workflow(
    pool: &PgPool,
    content_id: Uuid,
    to_stage: WorkflowStage,
    transitioned_by: i64,
    comment: Option<String>,
) -> Result<WorkflowStatus> {
    let workflow: WorkflowStatus = sqlx::query_as(
        r#"
        UPDATE cms_workflow_status
        SET previous_stage = current_stage,
            current_stage = $1,
            transition_comment = $2,
            updated_at = NOW()
        WHERE content_id = $3
        RETURNING id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
                  assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        "#,
    )
    .bind(to_stage)
    .bind(&comment)
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(workflow)
}

/// Assign content for review
pub async fn assign_for_review(
    pool: &PgPool,
    content_id: Uuid,
    assigned_to: i64,
    assigned_by: i64,
    due_date: Option<DateTime<Utc>>,
    priority: Option<WorkflowPriority>,
    notes: Option<String>,
) -> Result<WorkflowStatus> {
    let workflow: WorkflowStatus = sqlx::query_as(
        r#"
        UPDATE cms_workflow_status
        SET assigned_to = $1,
            assigned_by = $2,
            assigned_at = NOW(),
            due_date = $3,
            priority = COALESCE($4, priority),
            notes = $5,
            current_stage = 'in_review',
            updated_at = NOW()
        WHERE content_id = $6
        RETURNING id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
                  assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        "#,
    )
    .bind(assigned_to)
    .bind(assigned_by)
    .bind(due_date)
    .bind(priority)
    .bind(&notes)
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(workflow)
}

/// Get all assignments for a user
pub async fn get_user_assignments(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<WorkflowStatus>> {
    let assignments: Vec<WorkflowStatus> = sqlx::query_as(
        r#"
        SELECT id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
               assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        FROM cms_workflow_status
        WHERE assigned_to = $1
          AND current_stage NOT IN ('published', 'archived')
        ORDER BY
            CASE priority
                WHEN 'urgent' THEN 1
                WHEN 'high' THEN 2
                WHEN 'normal' THEN 3
                WHEN 'low' THEN 4
            END,
            due_date NULLS LAST,
            created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(assignments)
}

/// Get workflow history for content
pub async fn get_workflow_history(
    pool: &PgPool,
    content_id: Uuid,
    limit: i64,
) -> Result<Vec<WorkflowHistory>> {
    let history: Vec<WorkflowHistory> = sqlx::query_as(
        r#"
        SELECT id, workflow_id, content_id, from_stage, to_stage, comment, transitioned_by, created_at
        FROM cms_workflow_history
        WHERE content_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
    )
    .bind(content_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(history)
}

/// Get pending review count for user
pub async fn get_pending_review_count(pool: &PgPool, user_id: i64) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_workflow_status
        WHERE assigned_to = $1
          AND current_stage = 'in_review'
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

/// Get overdue assignments for user
pub async fn get_overdue_assignments(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<WorkflowStatus>> {
    let assignments: Vec<WorkflowStatus> = sqlx::query_as(
        r#"
        SELECT id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
               assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        FROM cms_workflow_status
        WHERE assigned_to = $1
          AND current_stage NOT IN ('published', 'archived')
          AND due_date < NOW()
        ORDER BY due_date ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(assignments)
}

/// Unassign content
pub async fn unassign_content(pool: &PgPool, content_id: Uuid) -> Result<WorkflowStatus> {
    let workflow: WorkflowStatus = sqlx::query_as(
        r#"
        UPDATE cms_workflow_status
        SET assigned_to = NULL,
            assigned_by = NULL,
            assigned_at = NULL,
            due_date = NULL,
            notes = NULL,
            updated_at = NOW()
        WHERE content_id = $1
        RETURNING id, content_id, current_stage, previous_stage, assigned_to, assigned_by,
                  assigned_at, due_date, priority, notes, transition_comment, created_at, updated_at
        "#,
    )
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(workflow)
}
