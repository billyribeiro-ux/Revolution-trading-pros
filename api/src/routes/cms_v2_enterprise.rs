//! CMS v2 Enterprise Routes - Apple ICT 7 Principal Engineer Grade
//!
//! API endpoints for enterprise CMS features:
//! - Audit logging
//! - Workflow management
//! - Preview tokens
//! - Webhook management
//!
//! @version 1.0.0 - January 27, 2026

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{
    models::User,
    services::{cms_audit, cms_preview, cms_workflow},
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUTHORIZATION HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════════

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;

fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (status, Json(json!({ "error": message })))
}

fn require_admin(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin" | "developer") {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Admin access required"))
    }
}

fn require_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing"
    ) {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Editor access required"))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUDIT LOGGING ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get audit logs with filtering
async fn get_audit_logs(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<cms_audit::AuditLogQuery>,
) -> ApiResult<JsonValue> {
    require_admin(&user)?;

    let logs = cms_audit::get_audit_logs(&state.db.pool, &query)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let total = cms_audit::get_audit_log_count(&state.db.pool, &query)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(json!({
        "data": logs,
        "meta": {
            "total": total,
            "limit": query.limit.unwrap_or(100),
            "offset": query.offset.unwrap_or(0)
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// WORKFLOW MANAGEMENT ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct TransitionWorkflowRequest {
    to_stage: String,
    comment: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AssignForReviewRequest {
    assigned_to: i64,
    due_date: Option<chrono::DateTime<chrono::Utc>>,
    priority: Option<String>,
    notes: Option<String>,
}

/// Get workflow status for content
async fn get_workflow_status(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
) -> ApiResult<cms_workflow::WorkflowStatus> {
    require_editor(&user)?;

    let status = cms_workflow::get_or_create_workflow_status(&state.db.pool, content_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(status))
}

/// Transition workflow to new stage
async fn transition_workflow(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Json(input): Json<TransitionWorkflowRequest>,
) -> ApiResult<cms_workflow::WorkflowStatus> {
    require_editor(&user)?;

    let to_stage = match input.to_stage.as_str() {
        "draft" => cms_workflow::WorkflowStage::Draft,
        "in_review" => cms_workflow::WorkflowStage::InReview,
        "approved" => cms_workflow::WorkflowStage::Approved,
        "published" => cms_workflow::WorkflowStage::Published,
        "archived" => cms_workflow::WorkflowStage::Archived,
        _ => return Err(api_error(StatusCode::BAD_REQUEST, "Invalid workflow stage")),
    };

    let status = cms_workflow::transition_workflow(
        &state.db.pool,
        content_id,
        to_stage,
        user.id,
        input.comment,
    )
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(status))
}

/// Assign content for review
async fn assign_for_review(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Json(input): Json<AssignForReviewRequest>,
) -> ApiResult<cms_workflow::WorkflowStatus> {
    require_editor(&user)?;

    let priority = input.priority.as_deref().and_then(|p| match p {
        "low" => Some(cms_workflow::WorkflowPriority::Low),
        "normal" => Some(cms_workflow::WorkflowPriority::Normal),
        "high" => Some(cms_workflow::WorkflowPriority::High),
        "urgent" => Some(cms_workflow::WorkflowPriority::Urgent),
        _ => None,
    });

    let status = cms_workflow::assign_for_review(
        &state.db.pool,
        content_id,
        input.assigned_to,
        user.id,
        input.due_date,
        priority,
        input.notes,
    )
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(status))
}

/// Get user's assignments
async fn get_my_assignments(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<Vec<cms_workflow::WorkflowStatus>> {
    let assignments = cms_workflow::get_user_assignments(&state.db.pool, user.id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(assignments))
}

/// Get workflow history for content
async fn get_workflow_history(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<Vec<cms_workflow::WorkflowHistory>> {
    require_editor(&user)?;

    let limit = query.limit.unwrap_or(50).min(200);

    let history = cms_workflow::get_workflow_history(&state.db.pool, content_id, limit)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(history))
}

#[derive(Debug, Deserialize)]
struct HistoryQuery {
    limit: Option<i64>,
}

/// Unassign content
async fn unassign_content(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
) -> ApiResult<cms_workflow::WorkflowStatus> {
    require_editor(&user)?;

    let status = cms_workflow::unassign_content(&state.db.pool, content_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(status))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// PREVIEW TOKEN ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct CreatePreviewTokenRequest {
    expires_in_hours: Option<i64>,
    max_views: Option<i32>,
}

/// Generate preview token for content
async fn create_preview_token(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
    Json(input): Json<CreatePreviewTokenRequest>,
) -> ApiResult<cms_preview::PreviewToken> {
    require_editor(&user)?;

    let token = cms_preview::generate_preview_token(
        &state.db.pool,
        content_id,
        user.id,
        input.expires_in_hours.unwrap_or(24),
        input.max_views,
    )
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(token))
}

/// Validate preview token (public endpoint)
async fn validate_preview_token(
    State(state): State<AppState>,
    Path(token): Path<Uuid>,
) -> ApiResult<JsonValue> {
    let preview = cms_preview::validate_preview_token(&state.db.pool, token)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Invalid or expired preview token"))?;

    Ok(Json(json!({
        "valid": true,
        "content_id": preview.content_id,
        "expires_at": preview.expires_at,
        "views_remaining": preview.max_views.map(|m| m - preview.view_count)
    })))
}

/// Get preview tokens for content
async fn get_content_preview_tokens(
    State(state): State<AppState>,
    user: User,
    Path(content_id): Path<Uuid>,
) -> ApiResult<Vec<cms_preview::PreviewToken>> {
    require_editor(&user)?;

    let tokens = cms_preview::get_content_preview_tokens(&state.db.pool, content_id)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(tokens))
}

/// Revoke preview token
async fn revoke_preview_token(
    State(state): State<AppState>,
    user: User,
    Path(token): Path<Uuid>,
) -> ApiResult<JsonValue> {
    require_editor(&user)?;

    let revoked = cms_preview::revoke_preview_token(&state.db.pool, token)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    if revoked {
        Ok(Json(json!({ "message": "Preview token revoked" })))
    } else {
        Err(api_error(StatusCode::NOT_FOUND, "Preview token not found"))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Enterprise features router
pub fn router() -> Router<AppState> {
    Router::new()
        // Audit Logs
        .route("/audit-logs", get(get_audit_logs))
        // Workflow Management
        .route("/workflow/:content_id", get(get_workflow_status))
        .route(
            "/workflow/:content_id/transition",
            post(transition_workflow),
        )
        .route("/workflow/:content_id/assign", post(assign_for_review))
        .route("/workflow/:content_id/unassign", post(unassign_content))
        .route("/workflow/:content_id/history", get(get_workflow_history))
        .route("/workflow/my-assignments", get(get_my_assignments))
        // Preview Tokens
        .route(
            "/preview/:content_id/tokens",
            get(get_content_preview_tokens).post(create_preview_token),
        )
        .route("/preview/token/:token", delete(revoke_preview_token))
}

/// Public preview validation endpoint
pub fn public_router() -> Router<AppState> {
    Router::new().route("/preview/:token", get(validate_preview_token))
}
