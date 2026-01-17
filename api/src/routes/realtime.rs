//! Real-time Updates - Apple ICT 11+ Principal Engineer Grade
//! January 2026
//!
//! Server-Sent Events (SSE) for real-time CMS updates:
//! - Content changes (created, updated, deleted)
//! - Workflow transitions
//! - Publishing events
//! - User notifications

#![allow(clippy::unnecessary_map_or)]

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::StreamExt;

use crate::{models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// EVENT TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Real-time event types for CMS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum CmsEvent {
    /// Content was created
    ContentCreated {
        content_type: String,
        content_id: i64,
        title: Option<String>,
        created_by: i64,
        created_at: DateTime<Utc>,
    },
    /// Content was updated
    ContentUpdated {
        content_type: String,
        content_id: i64,
        title: Option<String>,
        updated_by: i64,
        updated_at: DateTime<Utc>,
        changed_fields: Option<Vec<String>>,
    },
    /// Content was deleted
    ContentDeleted {
        content_type: String,
        content_id: i64,
        deleted_by: i64,
        deleted_at: DateTime<Utc>,
    },
    /// Content was published
    ContentPublished {
        content_type: String,
        content_id: i64,
        title: Option<String>,
        published_by: i64,
        published_at: DateTime<Utc>,
    },
    /// Content was unpublished/reverted to draft
    ContentUnpublished {
        content_type: String,
        content_id: i64,
        unpublished_by: i64,
        unpublished_at: DateTime<Utc>,
    },
    /// Workflow stage changed
    WorkflowTransition {
        content_type: String,
        content_id: i64,
        from_stage: String,
        to_stage: String,
        transitioned_by: i64,
        transitioned_at: DateTime<Utc>,
        comment: Option<String>,
    },
    /// Content assigned for review
    ReviewAssigned {
        content_type: String,
        content_id: i64,
        assigned_to: i64,
        assigned_by: i64,
        due_date: Option<DateTime<Utc>>,
        priority: Option<String>,
    },
    /// Scheduled content published automatically
    ScheduledPublish {
        content_type: String,
        content_id: i64,
        published_at: DateTime<Utc>,
    },
    /// User notification
    Notification {
        user_id: i64,
        title: String,
        message: String,
        link: Option<String>,
        notification_type: String,
    },
    /// Heartbeat/keep-alive
    Heartbeat { timestamp: DateTime<Utc> },
}

// ═══════════════════════════════════════════════════════════════════════════
// EVENT BROADCASTER
// ═══════════════════════════════════════════════════════════════════════════

/// Thread-safe event broadcaster using tokio broadcast channel
#[derive(Debug, Clone)]
pub struct EventBroadcaster {
    sender: broadcast::Sender<CmsEvent>,
}

impl EventBroadcaster {
    /// Create a new event broadcaster with capacity for 1000 events
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { sender }
    }

    /// Broadcast an event to all connected clients
    pub fn broadcast(&self, event: CmsEvent) {
        // It's okay if there are no receivers
        let _ = self.sender.send(event);
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<CmsEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Send a content created event
pub fn emit_content_created(
    broadcaster: &EventBroadcaster,
    content_type: &str,
    content_id: i64,
    title: Option<String>,
    created_by: i64,
) {
    broadcaster.broadcast(CmsEvent::ContentCreated {
        content_type: content_type.to_string(),
        content_id,
        title,
        created_by,
        created_at: Utc::now(),
    });
}

/// Send a content updated event
pub fn emit_content_updated(
    broadcaster: &EventBroadcaster,
    content_type: &str,
    content_id: i64,
    title: Option<String>,
    updated_by: i64,
    changed_fields: Option<Vec<String>>,
) {
    broadcaster.broadcast(CmsEvent::ContentUpdated {
        content_type: content_type.to_string(),
        content_id,
        title,
        updated_by,
        updated_at: Utc::now(),
        changed_fields,
    });
}

/// Send a content deleted event
pub fn emit_content_deleted(
    broadcaster: &EventBroadcaster,
    content_type: &str,
    content_id: i64,
    deleted_by: i64,
) {
    broadcaster.broadcast(CmsEvent::ContentDeleted {
        content_type: content_type.to_string(),
        content_id,
        deleted_by,
        deleted_at: Utc::now(),
    });
}

/// Send a content published event
pub fn emit_content_published(
    broadcaster: &EventBroadcaster,
    content_type: &str,
    content_id: i64,
    title: Option<String>,
    published_by: i64,
) {
    broadcaster.broadcast(CmsEvent::ContentPublished {
        content_type: content_type.to_string(),
        content_id,
        title,
        published_by,
        published_at: Utc::now(),
    });
}

/// Send a workflow transition event
pub fn emit_workflow_transition(
    broadcaster: &EventBroadcaster,
    content_type: &str,
    content_id: i64,
    from_stage: &str,
    to_stage: &str,
    transitioned_by: i64,
    comment: Option<String>,
) {
    broadcaster.broadcast(CmsEvent::WorkflowTransition {
        content_type: content_type.to_string(),
        content_id,
        from_stage: from_stage.to_string(),
        to_stage: to_stage.to_string(),
        transitioned_by,
        transitioned_at: Utc::now(),
        comment,
    });
}

/// Send a review assigned event
pub fn emit_review_assigned(
    broadcaster: &EventBroadcaster,
    content_type: &str,
    content_id: i64,
    assigned_to: i64,
    assigned_by: i64,
    due_date: Option<DateTime<Utc>>,
    priority: Option<String>,
) {
    broadcaster.broadcast(CmsEvent::ReviewAssigned {
        content_type: content_type.to_string(),
        content_id,
        assigned_to,
        assigned_by,
        due_date,
        priority,
    });
}

/// Send a notification to a specific user
pub fn emit_notification(
    broadcaster: &EventBroadcaster,
    user_id: i64,
    title: &str,
    message: &str,
    link: Option<String>,
    notification_type: &str,
) {
    broadcaster.broadcast(CmsEvent::Notification {
        user_id,
        title: title.to_string(),
        message: message.to_string(),
        link,
        notification_type: notification_type.to_string(),
    });
}

// ═══════════════════════════════════════════════════════════════════════════
// SSE ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct SseQuery {
    /// Filter by content types (comma-separated)
    pub content_types: Option<String>,
    /// Filter by event types (comma-separated)
    pub event_types: Option<String>,
}

/// SSE endpoint for real-time CMS updates
/// GET /realtime/events
async fn sse_events(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<SseQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let user_id = user.id;

    // Parse filter options
    let content_type_filter: Option<Vec<String>> = query
        .content_types
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect());

    let event_type_filter: Option<Vec<String>> = query
        .event_types
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect());

    // Subscribe to broadcast channel
    let mut receiver = state.event_broadcaster.subscribe();

    // Create the SSE stream
    let stream = async_stream::stream! {
        // Send initial connection event
        yield Ok(Event::default()
            .event("connected")
            .data(json!({
                "user_id": user_id,
                "connected_at": Utc::now(),
                "filters": {
                    "content_types": content_type_filter,
                    "event_types": event_type_filter
                }
            }).to_string()));

        // Listen for events
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    // Apply filters
                    let should_send = match &event {
                        CmsEvent::ContentCreated { content_type, .. } |
                        CmsEvent::ContentUpdated { content_type, .. } |
                        CmsEvent::ContentDeleted { content_type, .. } |
                        CmsEvent::ContentPublished { content_type, .. } |
                        CmsEvent::ContentUnpublished { content_type, .. } |
                        CmsEvent::WorkflowTransition { content_type, .. } |
                        CmsEvent::ReviewAssigned { content_type, .. } |
                        CmsEvent::ScheduledPublish { content_type, .. } => {
                            content_type_filter.as_ref().map_or(true, |f| f.contains(content_type))
                        }
                        CmsEvent::Notification { user_id: target_user, .. } => {
                            // Only send notifications to the target user
                            *target_user == user_id
                        }
                        CmsEvent::Heartbeat { .. } => true,
                    };

                    if should_send {
                        let event_type = match &event {
                            CmsEvent::ContentCreated { .. } => "content.created",
                            CmsEvent::ContentUpdated { .. } => "content.updated",
                            CmsEvent::ContentDeleted { .. } => "content.deleted",
                            CmsEvent::ContentPublished { .. } => "content.published",
                            CmsEvent::ContentUnpublished { .. } => "content.unpublished",
                            CmsEvent::WorkflowTransition { .. } => "workflow.transition",
                            CmsEvent::ReviewAssigned { .. } => "workflow.assigned",
                            CmsEvent::ScheduledPublish { .. } => "content.scheduled_publish",
                            CmsEvent::Notification { .. } => "notification",
                            CmsEvent::Heartbeat { .. } => "heartbeat",
                        };

                        let event_data = serde_json::to_string(&event).unwrap_or_default();
                        yield Ok(Event::default()
                            .event(event_type)
                            .data(event_data));
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    // Client fell behind, send warning
                    tracing::warn!("SSE client lagged {} events", n);
                    yield Ok(Event::default()
                        .event("warning")
                        .data(json!({ "message": format!("Missed {} events due to lag", n) }).to_string()));
                }
                Err(broadcast::error::RecvError::Closed) => {
                    // Channel closed, end stream
                    break;
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("heartbeat"),
    )
}

/// Get current SSE connection stats
/// GET /realtime/stats
async fn sse_stats(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role != "admin" && role != "super-admin" && role != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Admin access required"
            })),
        ));
    }

    let receiver_count = state.event_broadcaster.sender.receiver_count();

    Ok(Json(json!({
        "active_connections": receiver_count,
        "status": "operational"
    })))
}

/// Test endpoint to broadcast a test event (admin only)
/// POST /realtime/test
#[derive(Debug, Deserialize)]
pub struct TestEventRequest {
    pub message: String,
}

async fn broadcast_test_event(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<TestEventRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if role != "admin" && role != "super-admin" && role != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Admin access required"
            })),
        ));
    }

    emit_notification(
        &state.event_broadcaster,
        user.id,
        "Test Event",
        &input.message,
        None,
        "test",
    );

    Ok(Json(json!({
        "message": "Test event broadcast successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/events", get(sse_events))
        .route("/stats", get(sse_stats))
        .route("/test", axum::routing::post(broadcast_test_event))
}
