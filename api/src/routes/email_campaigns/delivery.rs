//! Atomically link native campaign identities to the shared immutable outbox.
use super::{Config, db, invalid, lock_row, quiescent, record, row};
use crate::{
    AppState,
    middleware::admin::AdminUser,
    routes::crm,
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    body::Bytes,
    extract::{Path, State},
    routing::post,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Postgres, Transaction};
#[derive(sqlx::FromRow)]
struct Source {
    id: i64,
    name: String,
    subject: String,
    html_content: Option<String>,
    text_content: Option<String>,
    template_id: Option<i64>,
    segment_id: Option<i64>,
    authoring_config: Value,
    authoring_version: i64,
    scheduled_at: Option<DateTime<Utc>>,
    sent_count: Option<i32>,
}
async fn source(pool: &sqlx::PgPool, id: i64) -> Result<Source, ApiError> {
    sqlx::query_as("SELECT id,name,subject,html_content,text_content,template_id,segment_id,authoring_config,authoring_version,scheduled_at AT TIME ZONE 'UTC' AS scheduled_at,(COALESCE(sent_count,0)>0 OR COALESCE(open_count,0)>0 OR COALESCE(click_count,0)>0 OR COALESCE(bounce_count,0)>0 OR COALESCE(unsubscribe_count,0)>0 OR sent_at IS NOT NULL) AS has_legacy_activity FROM email_campaigns WHERE id=$1").bind(id).fetch_optional(pool).await.map_err(db)?.ok_or_else(||ApiError::not_found("Campaign"))
}
fn audience(source: &Source) -> Value {
    json!({"members":true,"member_segment":source.segment_id,"eligibility":"confirmed_only"})
}
fn settings(config: &Config) -> Value {
    json!({"mailer_settings":{"is_custom":!config.from_email.is_empty(),"from_name":config.from_name,"from_email":config.from_email,"reply_to_name":"","reply_to_email":config.reply_to.as_deref().unwrap_or("")}})
}
fn content(source: &Source) -> (String, Option<String>, &'static str) {
    let html = source.html_content.clone().unwrap_or_default();
    if html.trim().is_empty() {
        (
            source.text_content.clone().unwrap_or_default(),
            None,
            "plain_text",
        )
    } else {
        (html, source.text_content.clone(), "raw_html")
    }
}
/// Caller holds engine-before-native locks. This writes only unsent authoring
/// state; previously submitted definitions and accepted recipients stay intact.
pub(super) async fn synchronize(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    engine: i64,
) -> Result<(), ApiError> {
    let source:Source=sqlx::query_as("SELECT id,name,subject,html_content,text_content,template_id,segment_id,authoring_config,authoring_version,scheduled_at AT TIME ZONE 'UTC' AS scheduled_at,(COALESCE(sent_count,0)>0 OR COALESCE(open_count,0)>0 OR COALESCE(click_count,0)>0 OR COALESCE(bounce_count,0)>0 OR COALESCE(unsubscribe_count,0)>0 OR sent_at IS NOT NULL) AS has_legacy_activity FROM email_campaigns WHERE id=$1").bind(id).fetch_one(&mut **tx).await.map_err(db)?;
    let config: Config = serde_json::from_value(source.authoring_config.clone())
        .map_err(|_| ApiError::database_error("Stored campaign configuration is invalid"))?;
    let (body, text, kind) = content(&source);
    sqlx::query("UPDATE crm_campaigns SET name=$2,title=$2,subject=$3,content=$4,text_body=$5,campaign_type=$6,email_pre_header=$7,settings=$8,subscriber_settings=$9,email_template_id=$10,ab_test_config=$11,authoring_version=$12,updated_at=NOW() WHERE id=$1 AND native_email_campaign_id=$13").bind(engine).bind(&source.name).bind(&source.subject).bind(body).bind(text).bind(kind).bind(&config.preview_text).bind(settings(&config)).bind(audience(&source)).bind(source.template_id).bind(config.ab_test_config.as_ref().map(|ab|json!(ab))).bind(source.authoring_version).bind(id).execute(&mut **tx).await.map_err(db)?;
    Ok(())
}
struct Prepared {
    source: Source,
    config: Config,
    audience: crm::ResolvedAudience,
}
async fn prepare(state: &AppState, id: i64) -> Result<Prepared, ApiError> {
    let source = source(&state.db.pool, id).await?;
    let config: Config = serde_json::from_value(source.authoring_config.clone())
        .map_err(|_| ApiError::database_error("Stored campaign configuration is invalid"))?;
    config.validate()?;
    if !config.from_name.is_empty() && config.from_email.is_empty() {
        return Err(invalid("Specify a sender email when overriding its name"));
    }
    let audience = crm::resolve_campaign_audience(&state.db.pool, Some(audience(&source))).await?;
    let (body, text, kind) = content(&source);
    if source.subject.trim().is_empty() || body.trim().is_empty() {
        return Err(invalid(
            "Provide a subject and message content before submitting",
        ));
    }
    let definition = json!({"native_email_campaign_id":id,"type":kind,"email_subject":source.subject,"email_body":body,"text_body":text});
    let snapshot = json!({"first_name":"","last_name":"","company":""});
    let recipient = || crate::services::campaign_personalization::Recipient {
        email: "",
        name: "",
        snapshot: &snapshot,
        app_url: &state.config.app_url,
        unsubscribe_url: "",
        brand: state.services.email.from_name(),
    };
    crate::services::campaign_personalization::render(&definition, recipient(), &source.subject)
        .map_err(|_| {
            invalid(
                "Campaign template has unsupported variables, invalid syntax or excessive output",
            )
        })?;
    if let Some(ab) = &config.ab_test_config {
        if ab.enabled {
            crate::services::campaign_personalization::render(
                &definition,
                recipient(),
                ab.subject_b.as_deref().unwrap_or(""),
            )
            .map_err(|_| {
                invalid("Subject B has unsupported variables, invalid syntax or excessive output")
            })?;
        }
    }
    Ok(Prepared {
        source,
        config,
        audience,
    })
}
#[derive(Clone, Copy)]
enum Submission {
    Send,
    Schedule(DateTime<Utc>),
    Resume,
}
async fn submit(
    state: &AppState,
    id: i64,
    actor: i64,
    mode: Submission,
) -> Result<Value, ApiError> {
    let prepared = prepare(state, id).await?;
    let due = match mode {
        Submission::Schedule(due) => {
            if due <= Utc::now() {
                return Err(invalid("Schedule must be in the future"));
            }
            due
        }
        Submission::Resume => prepared
            .config
            .planned_at
            .or(prepared.source.scheduled_at)
            .unwrap_or_else(Utc::now)
            .max(Utc::now()),
        Submission::Send => Utc::now(),
    };
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (_, existing, status) = lock_row(&mut tx, id).await?;
    let version: i64 =
        sqlx::query_scalar("SELECT authoring_version FROM email_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
    if version != prepared.source.authoring_version {
        return Err(errors::conflict(
            "Campaign changed; reload before submitting",
        ));
    }
    let resume = matches!(mode, Submission::Resume);
    if if resume {
        !matches!(status.as_str(), "paused" | "failed")
    } else {
        status != "draft"
    } {
        return Err(errors::conflict(
            "Campaign cannot be submitted from its current state",
        ));
    }
    if existing.is_none() && (resume || prepared.source.sent_count.is_some_and(|v| v > 0)) {
        return Err(errors::conflict(
            "Historical campaign has no verified recipient ledger; duplicate it for a new submission",
        ));
    }
    quiescent(&mut tx, existing).await?;
    let engine = if let Some(engine) = existing {
        synchronize(&mut tx, id, engine).await?;
        engine
    } else {
        let engine:i64=sqlx::query_scalar("INSERT INTO crm_campaigns(name,title,status,campaign_type,design_template,native_email_campaign_id,created_by,failed_count) VALUES($1,$1,'draft','raw_html','raw',$2,$3,0) RETURNING id").bind(&prepared.source.name).bind(id).bind(actor).fetch_one(&mut *tx).await.map_err(db)?;
        sqlx::query("UPDATE email_campaigns SET delivery_campaign_id=$2 WHERE id=$1 AND delivery_campaign_id IS NULL").bind(id).bind(engine).execute(&mut *tx).await.map_err(db)?;
        synchronize(&mut tx, id, engine).await?;
        engine
    };
    let definition = crm::campaign_definition_on(&mut tx, engine).await?;
    crm::freeze_on(
        &mut tx,
        engine,
        actor,
        due,
        resume,
        &definition,
        &prepared.audience,
    )
    .await?;
    record(
        &mut tx,
        actor,
        id,
        if resume {
            "Email campaign resumed"
        } else {
            "Email campaign submitted"
        },
    )
    .await?;
    tx.commit().await.map_err(db)?;
    row(&state.db.pool, id).await
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct SendInput {
    #[serde(alias = "send_at")]
    scheduled_at: Option<DateTime<Utc>>,
}
fn input(body: Bytes) -> Result<SendInput, ApiError> {
    if body.len() > 4096 {
        return Err(invalid("Campaign action request exceeds4KiB"));
    }
    if body.is_empty() {
        Ok(SendInput::default())
    } else {
        serde_json::from_slice(&body).map_err(|_| invalid("Provide a valid scheduled_at timestamp"))
    }
}
async fn send(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    body: Bytes,
) -> Result<Json<Value>, ApiError> {
    let mode = input(body)?
        .scheduled_at
        .map_or(Submission::Send, Submission::Schedule);
    Ok(Json(
        json!({"success":true,"data":submit(&state,id,admin.0.id,mode).await?}),
    ))
}
async fn schedule(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    body: Bytes,
) -> Result<Json<Value>, ApiError> {
    let due = input(body)?
        .scheduled_at
        .ok_or_else(|| invalid("Provide scheduled_at"))?;
    Ok(Json(
        json!({"success":true,"data":submit(&state,id,admin.0.id,Submission::Schedule(due)).await?}),
    ))
}
async fn resume(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        json!({"success":true,"data":submit(&state,id,admin.0.id,Submission::Resume).await?}),
    ))
}
async fn pause(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (_, engine, status) = lock_row(&mut tx, id).await?;
    if !matches!(status.as_str(), "scheduled" | "sending") {
        return Err(errors::conflict(
            "Only scheduled or sending campaigns can be paused",
        ));
    }
    let engine = engine
        .ok_or_else(|| errors::conflict("Historical campaign has no verified delivery queue"))?;
    sqlx::query("UPDATE crm_campaigns SET status='paused',pause_reason='manual',updated_at=NOW() WHERE id=$1").bind(engine).execute(&mut *tx).await.map_err(db)?;
    crate::services::campaign_delivery::refresh(&mut tx, engine)
        .await
        .map_err(|_| ApiError::database_error("Unable to update delivery status"))?;
    record(&mut tx, admin.0.id, id, "Email campaign paused").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":row(&state.db.pool,id).await?}),
    ))
}
async fn cancel(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (_, engine, status) = lock_row(&mut tx, id).await?;
    if !matches!(
        status.as_str(),
        "scheduled" | "sending" | "paused" | "failed"
    ) {
        return Err(errors::conflict(
            "Campaign cannot be cancelled from its current state",
        ));
    }
    quiescent(&mut tx, engine).await?;
    if let Some(engine) = engine {
        sqlx::query("UPDATE crm_campaign_deliveries SET status='skipped',last_error='Campaign cancelled before submission',updated_at=NOW() WHERE campaign_id=$1 AND status IN ('queued','failed')").bind(engine).execute(&mut *tx).await.map_err(db)?;
        sqlx::query("UPDATE crm_campaigns SET status='cancelled',pause_reason='manual',updated_at=NOW() WHERE id=$1").bind(engine).execute(&mut *tx).await.map_err(db)?;
        crate::services::campaign_delivery::refresh(&mut tx, engine)
            .await
            .map_err(|_| ApiError::database_error("Unable to update delivery status"))?;
    } else {
        sqlx::query("UPDATE email_campaigns SET status='cancelled',updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
    }
    record(&mut tx, admin.0.id, id, "Email campaign cancelled").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":row(&state.db.pool,id).await?}),
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}/send", post(send))
        .route("/{id}/schedule", post(schedule))
        .route("/{id}/pause", post(pause))
        .route("/{id}/resume", post(resume))
        .route("/{id}/cancel", post(cancel))
}
