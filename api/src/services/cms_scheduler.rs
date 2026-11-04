//! Transactional execution shared by the CMS worker and admin processing endpoints.
use crate::routes::cms_scheduling::{CronExecutionResponse, ScheduleAction};
use anyhow::{Result, anyhow, bail};
use serde_json::Value;
use sqlx::{Acquire, PgConnection, PgPool};
use uuid::Uuid;

fn summary() -> CronExecutionResponse {
    CronExecutionResponse {
        processed: 0,
        successful: 0,
        failed: 0,
        errors: Vec::new(),
    }
}

async fn apply(
    connection: &mut PgConnection,
    content_id: Uuid,
    action: &ScheduleAction,
    staged: Option<&Value>,
) -> Result<()> {
    let affected = match action {
        ScheduleAction::Publish => sqlx::query("UPDATE cms_content SET status='published',published_at=COALESCE(published_at,NOW()),updated_at=NOW(),version=version+1 WHERE id=$1 AND deleted_at IS NULL")
            .bind(content_id).execute(&mut *connection).await?,
        ScheduleAction::Unpublish => sqlx::query("UPDATE cms_content SET status='draft',updated_at=NOW(),version=version+1 WHERE id=$1 AND deleted_at IS NULL")
            .bind(content_id).execute(&mut *connection).await?,
        ScheduleAction::Archive => sqlx::query("UPDATE cms_content SET status='archived',updated_at=NOW(),version=version+1 WHERE id=$1 AND deleted_at IS NULL")
            .bind(content_id).execute(&mut *connection).await?,
        ScheduleAction::Update => {
            let data=staged.and_then(Value::as_object).ok_or_else(|| anyhow!("Update requires staged content fields"))?;
            if data.is_empty() || data.keys().any(|key| !matches!(key.as_str(),"title"|"content"|"content_blocks"|"custom_fields")) {
                bail!("Staged updates support title, content, content_blocks and custom_fields");
            }
            if data.get("title").is_some_and(|v| v.as_str().is_none_or(|s| s.trim().is_empty() || s.chars().count()>500 || s.contains('\0')))
                || data.get("content").is_some_and(|v| v.as_str().is_none_or(|s| s.len()>2_000_000 || s.contains('\0')))
                || data.get("content_blocks").is_some_and(|v| !v.is_array())
                || data.get("custom_fields").is_some_and(|v| !v.is_object()) {
                bail!("Staged content has invalid field types or lengths");
            }
            sqlx::query("UPDATE cms_content SET title=COALESCE($2->>'title',title),content=COALESCE($2->>'content',content),content_blocks=COALESCE($2->'content_blocks',content_blocks),custom_fields=COALESCE($2->'custom_fields',custom_fields),updated_at=NOW(),version=version+1 WHERE id=$1 AND deleted_at IS NULL")
                .bind(content_id).bind(staged).execute(&mut *connection).await?
        }
    };
    if affected.rows_affected() != 1 {
        bail!("Content is missing or deleted");
    }
    Ok(())
}

pub async fn process_schedules(pool: &PgPool) -> Result<CronExecutionResponse> {
    let mut result = summary();
    for _ in 0..50 {
        let mut tx = pool.begin().await?;
        let item: Option<(Uuid,Uuid,ScheduleAction,Option<Value>,i32,i32)>=sqlx::query_as("SELECT id,content_id,action,staged_data,retry_count,max_retries FROM cms_schedules WHERE status='pending' AND scheduled_at<=NOW() ORDER BY scheduled_at,id LIMIT 1 FOR UPDATE SKIP LOCKED")
            .fetch_optional(&mut *tx).await?;
        let Some((id, content_id, action, staged, retries, max_retries)) = item else {
            break;
        };
        let mut effect = tx.begin().await?;
        let applied = apply(&mut effect, content_id, &action, staged.as_ref()).await;
        let (status, error) = match applied {
            Ok(()) => {
                effect.commit().await?;
                result.successful += 1;
                ("completed", None)
            }
            Err(error) => {
                effect.rollback().await?;
                result.failed += 1;
                let message = error.to_string();
                result.errors.push(format!("Schedule {id}: {message}"));
                (
                    if retries < max_retries {
                        "pending"
                    } else {
                        "failed"
                    },
                    Some(message),
                )
            }
        };
        sqlx::query("UPDATE cms_schedules SET status=$2::cms_schedule_status,error_message=$3,retry_count=CASE WHEN $2='pending' THEN retry_count+1 ELSE retry_count END,scheduled_at=CASE WHEN $2='pending' THEN NOW()+make_interval(secs => (30*power(2,LEAST(retry_count,8)))::int) ELSE scheduled_at END,executed_at=CASE WHEN $2='pending' THEN NULL ELSE NOW() END,updated_at=NOW() WHERE id=$1")
            .bind(id).bind(status).bind(&error).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO cms_schedule_history(schedule_id,content_id,event_type,previous_status,new_status,event_data) VALUES($1,$2,$3,'pending',$4,$5)")
            .bind(id).bind(content_id).bind(if error.is_some() {"schedule_failed"} else {"schedule_executed"}).bind(status).bind(serde_json::json!({"error":error})).execute(&mut *tx).await?;
        tx.commit().await?;
        result.processed += 1;
    }
    Ok(result)
}

pub async fn process_releases(pool: &PgPool) -> Result<CronExecutionResponse> {
    let mut result = summary();
    for _ in 0..10 {
        let mut tx = pool.begin().await?;
        let release: Option<(Uuid,bool)>=sqlx::query_as("SELECT id,stop_on_error FROM cms_releases WHERE status='scheduled' AND scheduled_at<=NOW() ORDER BY scheduled_at,id LIMIT 1 FOR UPDATE SKIP LOCKED")
            .fetch_optional(&mut *tx).await?;
        let Some((id, stop_on_error)) = release else {
            break;
        };
        let items: Vec<(Uuid,Uuid,ScheduleAction,Option<Value>)>=sqlx::query_as("SELECT id,content_id,action,staged_data FROM cms_release_items WHERE release_id=$1 AND status='pending' ORDER BY order_index,id FOR UPDATE")
            .bind(id).fetch_all(&mut *tx).await?;
        let mut errors = Vec::new();
        if items.is_empty() {
            errors.push("Release has no pending items".to_owned());
        }
        for (item_id, content_id, action, staged) in items {
            let mut effect = tx.begin().await?;
            let applied = apply(&mut effect, content_id, &action, staged.as_ref()).await;
            let error = match applied {
                Ok(()) => {
                    effect.commit().await?;
                    None
                }
                Err(error) => {
                    effect.rollback().await?;
                    Some(error.to_string())
                }
            };
            sqlx::query("UPDATE cms_release_items SET status=$2::cms_schedule_status,executed_at=NOW(),error_message=$3,updated_at=NOW() WHERE id=$1")
                .bind(item_id).bind(if error.is_some() {"failed"} else {"completed"}).bind(&error).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO cms_schedule_history(release_id,content_id,event_type,new_status,event_data) VALUES($1,$2,'release_item_executed',$3,$4)")
                .bind(id).bind(content_id).bind(if error.is_some() {"failed"} else {"completed"}).bind(serde_json::json!({"item_id":item_id,"error":error})).execute(&mut *tx).await?;
            if let Some(error) = error {
                errors.push(format!("Item {item_id}: {error}"));
                if stop_on_error {
                    break;
                }
            }
        }
        let failed = !errors.is_empty();
        sqlx::query("UPDATE cms_releases SET status=$2::cms_release_status,executed_at=NOW(),error_message=$3,completed_items=(SELECT COUNT(*)::int FROM cms_release_items WHERE release_id=$1 AND status='completed'),failed_items=(SELECT COUNT(*)::int FROM cms_release_items WHERE release_id=$1 AND status='failed'),updated_at=NOW() WHERE id=$1")
            .bind(id).bind(if failed {"failed"} else {"completed"}).bind(if failed {Some(errors.join("; "))} else {None}).execute(&mut *tx).await?;
        tx.commit().await?;
        result.processed += 1;
        if failed {
            result.failed += 1;
            result
                .errors
                .extend(errors.into_iter().map(|e| format!("Release {id}: {e}")));
        } else {
            result.successful += 1;
        }
    }
    Ok(result)
}

/// Each effect and status/history transition commits together. A killed worker
/// releases its row locks, so the next worker can resume pending work safely.
pub async fn run(pool: PgPool) {
    let mut timer = tokio::time::interval(std::time::Duration::from_secs(5));
    timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        timer.tick().await;
        for result in [
            process_schedules(&pool).await,
            process_releases(&pool).await,
        ] {
            match result {
                Ok(result) if result.failed > 0 => {
                    tracing::warn!(errors=?result.errors,"CMS scheduled actions failed")
                }
                Err(error) => tracing::error!(%error,"CMS scheduler failed"),
                _ => {}
            }
        }
    }
}
