//! Cron job endpoints (internal use).
//!
//! Two admin-gated endpoints that the scheduler invokes on a periodic
//! basis to actually fire pending schedules / releases:
//! - `process_pending_schedules` — execute individual schedules whose
//!   `scheduled_at` is past due (per-row retry + status transition).
//! - `process_pending_releases` — execute release bundles whose
//!   `scheduled_at` is past due (per-item ordered execution with
//!   `stop_on_error` behaviour).
//!
//! Both share the `execute_schedule_action` helper which maps a
//! `ScheduleAction` onto a `cms_content` row update.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{models::User, AppState};

use super::{
    api_error, require_cms_admin, ApiResult, CmsSchedule, CronExecutionResponse, ScheduleAction,
};

/// Process pending schedules (cron job endpoint)
#[utoipa::path(
    post,
    path = "/api/cms/schedules/process",
    responses(
        (status = 200, description = "Processing complete", body = CronExecutionResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required")
    ),
    tag = "CMS Scheduling (Internal)",
    security(("bearer_auth" = []))
)]
pub(super) async fn process_pending_schedules(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<CronExecutionResponse> {
    require_cms_admin(&user)?;

    let mut processed = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    // Get pending schedules that are due
    let pending: Vec<(Uuid, Uuid, ScheduleAction, Option<JsonValue>)> = sqlx::query_as(
        r"
        SELECT id, content_id, action, staged_data
        FROM cms_schedules
        WHERE status = 'pending' AND scheduled_at <= NOW()
        ORDER BY scheduled_at ASC
        LIMIT 100
        FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    for (schedule_id, content_id, action, staged_data) in pending {
        processed += 1;

        // Mark as processing
        let _ = sqlx::query("UPDATE cms_schedules SET status = 'processing' WHERE id = $1")
            .bind(schedule_id)
            .execute(&state.db.pool)
            .await;

        // Execute the action
        let result =
            execute_schedule_action(&state, content_id, &action, staged_data.as_ref()).await;

        match result {
            Ok(_) => {
                successful += 1;
                let _ = sqlx::query(
                    "UPDATE cms_schedules SET status = 'completed', executed_at = NOW() WHERE id = $1",
                )
                .bind(schedule_id)
                .execute(&state.db.pool)
                .await;

                // Log success
                let _ = sqlx::query(
                    r"
                    INSERT INTO cms_schedule_history (
                        schedule_id, content_id, event_type, previous_status, new_status
                    ) VALUES ($1, $2, 'schedule_executed', 'processing', 'completed')
                    ",
                )
                .bind(schedule_id)
                .bind(content_id)
                .execute(&state.db.pool)
                .await;
            }
            Err(e) => {
                failed += 1;
                let error_msg = e.to_string();
                errors.push(format!("Schedule {schedule_id}: {error_msg}"));

                // Check retry count
                let schedule: Option<CmsSchedule> =
                    sqlx::query_as("SELECT * FROM cms_schedules WHERE id = $1")
                        .bind(schedule_id)
                        .fetch_optional(&state.db.pool)
                        .await
                        .ok()
                        .flatten();

                if let Some(s) = schedule {
                    if s.retry_count < s.max_retries {
                        let _ = sqlx::query(
                            r"
                            UPDATE cms_schedules SET
                                status = 'pending',
                                retry_count = retry_count + 1,
                                error_message = $2
                            WHERE id = $1
                            ",
                        )
                        .bind(schedule_id)
                        .bind(&error_msg)
                        .execute(&state.db.pool)
                        .await;
                    } else {
                        let _ = sqlx::query(
                            r"
                            UPDATE cms_schedules SET
                                status = 'failed',
                                executed_at = NOW(),
                                error_message = $2
                            WHERE id = $1
                            ",
                        )
                        .bind(schedule_id)
                        .bind(&error_msg)
                        .execute(&state.db.pool)
                        .await;
                    }
                }
            }
        }
    }

    Ok(Json(CronExecutionResponse {
        processed,
        successful,
        failed,
        errors,
    }))
}

/// Execute a schedule action on content
async fn execute_schedule_action(
    state: &AppState,
    content_id: Uuid,
    action: &ScheduleAction,
    staged_data: Option<&JsonValue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match action {
        ScheduleAction::Publish => {
            sqlx::query(
                "UPDATE cms_content SET status = 'published', published_at = NOW(), updated_at = NOW() WHERE id = $1",
            )
            .bind(content_id)
            .execute(&state.db.pool)
            .await?;
        }
        ScheduleAction::Unpublish => {
            sqlx::query(
                "UPDATE cms_content SET status = 'draft', updated_at = NOW() WHERE id = $1",
            )
            .bind(content_id)
            .execute(&state.db.pool)
            .await?;
        }
        ScheduleAction::Archive => {
            sqlx::query(
                "UPDATE cms_content SET status = 'archived', updated_at = NOW() WHERE id = $1",
            )
            .bind(content_id)
            .execute(&state.db.pool)
            .await?;
        }
        ScheduleAction::Update => {
            if let Some(data) = staged_data {
                // Apply staged changes
                sqlx::query(
                    r"
                    UPDATE cms_content SET
                        title = COALESCE($2->>'title', title),
                        content = COALESCE($2->>'content', content),
                        data = COALESCE($2->'data', data),
                        custom_fields = COALESCE($2->'custom_fields', custom_fields),
                        updated_at = NOW(),
                        version = version + 1
                    WHERE id = $1
                    ",
                )
                .bind(content_id)
                .bind(data)
                .execute(&state.db.pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Process pending releases (cron job endpoint)
#[utoipa::path(
    post,
    path = "/api/cms/releases/process",
    responses(
        (status = 200, description = "Processing complete", body = CronExecutionResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required")
    ),
    tag = "CMS Releases (Internal)",
    security(("bearer_auth" = []))
)]
pub(super) async fn process_pending_releases(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<CronExecutionResponse> {
    require_cms_admin(&user)?;

    let mut processed = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    // Get pending releases
    let pending: Vec<(Uuid, bool)> = sqlx::query_as(
        r"
        SELECT id, stop_on_error
        FROM cms_releases
        WHERE status = 'scheduled' AND scheduled_at <= NOW()
        ORDER BY scheduled_at ASC
        LIMIT 10
        FOR UPDATE SKIP LOCKED
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    for (release_id, stop_on_error) in pending {
        processed += 1;

        // Mark as processing
        let _ = sqlx::query("UPDATE cms_releases SET status = 'processing' WHERE id = $1")
            .bind(release_id)
            .execute(&state.db.pool)
            .await;

        // Get release items
        let items: Vec<(Uuid, Uuid, ScheduleAction, Option<JsonValue>)> = sqlx::query_as(
            r"
            SELECT id, content_id, action, staged_data
            FROM cms_release_items
            WHERE release_id = $1 AND status = 'pending'
            ORDER BY order_index ASC
            ",
        )
        .bind(release_id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        let mut release_failed = false;

        for (item_id, content_id, action, staged_data) in items {
            let result =
                execute_schedule_action(&state, content_id, &action, staged_data.as_ref()).await;

            match result {
                Ok(_) => {
                    let _ = sqlx::query(
                        "UPDATE cms_release_items SET status = 'completed', executed_at = NOW() WHERE id = $1",
                    )
                    .bind(item_id)
                    .execute(&state.db.pool)
                    .await;
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    let _ = sqlx::query(
                        "UPDATE cms_release_items SET status = 'failed', executed_at = NOW(), error_message = $2 WHERE id = $1",
                    )
                    .bind(item_id)
                    .bind(&error_msg)
                    .execute(&state.db.pool)
                    .await;

                    release_failed = true;
                    errors.push(format!("Release {release_id} item {item_id}: {error_msg}"));

                    if stop_on_error {
                        break;
                    }
                }
            }
        }

        if release_failed {
            failed += 1;
            let _ = sqlx::query(
                "UPDATE cms_releases SET status = 'failed', executed_at = NOW() WHERE id = $1",
            )
            .bind(release_id)
            .execute(&state.db.pool)
            .await;
        } else {
            successful += 1;
            let _ = sqlx::query(
                "UPDATE cms_releases SET status = 'completed', executed_at = NOW() WHERE id = $1",
            )
            .bind(release_id)
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(CronExecutionResponse {
        processed,
        successful,
        failed,
        errors,
    }))
}
