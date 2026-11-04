//! Campaign submission freezes eligible recipients and content before any send.
use super::{campaign_audience, campaigns};
use crate::{
    AppState,
    middleware::admin::AdminUser,
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Campaign submission persistence failed");
    ApiError::database_error("Unable to save campaign delivery")
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Schedule {
    scheduled_at: DateTime<Utc>,
}

async fn submit(
    pool: &PgPool,
    id: i64,
    actor: i64,
    due: DateTime<Utc>,
    resume: bool,
) -> Result<Value, ApiError> {
    // Resolve references before holding a transaction connection. The row version
    // check below prevents concurrent authoring from changing the frozen content.
    let campaign = campaigns::row(pool, id).await?;
    let version = campaign["authoring_version"]
        .as_i64()
        .ok_or_else(|| ApiError::database_error("Missing campaign revision"))?;
    let subject = campaign["email_subject"].as_str().unwrap_or("");
    let body = campaign["email_body"].as_str().unwrap_or("");
    if subject.trim().is_empty() || subject.contains(['\r', '\n']) || body.trim().is_empty() {
        return Err(ApiError::validation_error(
            "Provide a single-line subject and email body before sending",
        ));
    }
    let audience =
        campaign_audience::resolve(pool, campaign.get("subscriber_settings").cloned()).await?;
    let mut tx = pool.begin().await.map_err(db)?;
    let (status, current): (String, i64) =
        sqlx::query_as("SELECT status,authoring_version FROM crm_campaigns WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
    if current != version {
        return Err(errors::conflict(
            "Campaign changed; reload before submitting",
        ));
    }
    if if resume {
        !["paused", "failed"].contains(&status.as_str())
    } else {
        status != "draft"
    } {
        return Err(errors::conflict(
            "Campaign cannot be submitted from its current state",
        ));
    }
    let active:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='processing')").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    if active {
        return Err(errors::conflict(
            "A submission is still in flight; refresh after it finishes",
        ));
    }
    let unknown:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='unknown')").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    if unknown {
        return Err(errors::conflict(
            "An email submission has an unknown outcome; reconcile it with the provider before resuming",
        ));
    }
    let mut query = audience.query(
        "SELECT COUNT(DISTINCT LOWER(c.email)) FROM contacts c",
        true,
    )?;
    let count: i64 = query
        .build_query_scalar()
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
    if count == 0 || count > 100_000 {
        return Err(ApiError::validation_error(
            "Campaign audience must contain 1–100,000 confirmed recipients",
        ));
    }
    let revision:uuid::Uuid=sqlx::query_scalar("INSERT INTO crm_campaign_revisions(campaign_id,authoring_version,definition,created_by) VALUES($1,$2,$3,$4) ON CONFLICT(campaign_id,authoring_version) DO UPDATE SET authoring_version=EXCLUDED.authoring_version RETURNING id").bind(id).bind(version).bind(&campaign).bind(actor).fetch_one(&mut *tx).await.map_err(db)?;
    // Previously accepted deliveries survive edits and resume. Removed recipients
    // retain their audit row as skipped; eligible unsent rows get the new snapshot.
    sqlx::query("UPDATE crm_campaign_deliveries SET status='skipped',last_error='Removed from the submitted audience',updated_at=NOW() WHERE campaign_id=$1 AND status IN ('queued','failed','skipped')").bind(id).execute(&mut *tx).await.map_err(db)?;
    let mut recipients=audience.query("WITH selected AS (SELECT DISTINCT ON(LOWER(c.email)) c.id,LOWER(c.email) AS email,COALESCE(NULLIF(trim(concat_ws(' ',c.first_name,c.last_name)),''),c.email) AS name,jsonb_build_object('first_name',c.first_name,'last_name',c.last_name,'company',c.company) AS snapshot FROM contacts c",true)?;
    recipients.push(" ORDER BY LOWER(c.email),c.id) INSERT INTO crm_campaign_deliveries(campaign_id,revision_id,contact_id,email,display_name,contact_snapshot,due_at) SELECT ").push_bind(id).push(",").push_bind(revision).push(if audience.is_newsletter() { ",NULL::bigint,email,name,snapshot," } else { ",id,email,name,snapshot," }).push_bind(due).push(" FROM selected ON CONFLICT(campaign_id,email) DO UPDATE SET revision_id=EXCLUDED.revision_id,contact_id=EXCLUDED.contact_id,display_name=EXCLUDED.display_name,contact_snapshot=EXCLUDED.contact_snapshot,due_at=EXCLUDED.due_at,status='queued',attempts_remaining=3,last_error=NULL,updated_at=NOW() WHERE crm_campaign_deliveries.status IN ('queued','failed','skipped')");
    recipients.build().execute(&mut *tx).await.map_err(db)?;
    sqlx::query("UPDATE crm_campaigns SET status=CASE WHEN $2>NOW() THEN 'scheduled' ELSE 'sending' END,scheduled_at=$2,pause_reason=NULL,last_error=NULL,audience_snapshot=$3,updated_at=NOW() WHERE id=$1").bind(id).bind(due).bind(json!({"revision_id":revision,"eligible_at_submission":count,"submitted_by":actor,"submitted_at":Utc::now()})).execute(&mut *tx).await.map_err(db)?;
    crate::services::campaign_delivery::refresh(&mut tx, id)
        .await
        .map_err(|e| {
            tracing::error!(%e,"Unable to refresh campaign delivery counts");
            ApiError::database_error("Unable to refresh campaign delivery counts")
        })?;
    crate::services::crm_logs::record(
        &mut tx,
        "info",
        "email",
        if resume {
            "Campaign resumed"
        } else {
            "Campaign submitted"
        },
        json!({"campaign_id":id,"eligible_recipients":count,"scheduled_at":due}),
        Some(actor),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    campaigns::row(pool, id).await
}
async fn send(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        submit(&state.db.pool, id, admin.0.id, Utc::now(), false).await?,
    ))
}
async fn schedule(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Schedule>,
) -> Result<Json<Value>, ApiError> {
    if input.scheduled_at <= Utc::now() {
        return Err(ApiError::validation_error("Schedule must be in the future"));
    }
    Ok(Json(
        submit(&state.db.pool, id, admin.0.id, input.scheduled_at, false).await?,
    ))
}
async fn resume(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let due: Option<DateTime<Utc>> =
        sqlx::query_scalar("SELECT scheduled_at FROM crm_campaigns WHERE id=$1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(db)?
            .ok_or_else(|| ApiError::not_found("Campaign"))?;
    Ok(Json(
        submit(
            &state.db.pool,
            id,
            admin.0.id,
            due.unwrap_or_else(Utc::now).max(Utc::now()),
            true,
        )
        .await?,
    ))
}
async fn pause(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let result=sqlx::query("UPDATE crm_campaigns SET status='paused',pause_reason='manual',updated_at=NOW() WHERE id=$1 AND status IN ('scheduled','sending')").bind(id).execute(&state.db.pool).await.map_err(db)?;
    if result.rows_affected() == 0 {
        return Err(errors::conflict(
            "Only a scheduled or sending campaign can be paused",
        ));
    }
    Ok(Json(campaigns::row(&state.db.pool, id).await?))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/campaigns/{id}/send", post(send))
        .route("/campaigns/{id}/schedule", post(schedule))
        .route("/campaigns/{id}/pause", post(pause))
        .route("/campaigns/{id}/resume", post(resume))
        .route("/campaigns/{id}/deliveries", get(deliveries))
}

#[derive(Deserialize)]
struct DeliveryPage {
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
}
async fn deliveries(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(filters): Query<DeliveryPage>,
) -> Result<Json<Value>, ApiError> {
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(25);
    if page < 1
        || !(1..=100).contains(&size)
        || filters.status.as_deref().is_some_and(|v| {
            ![
                "queued",
                "processing",
                "accepted",
                "failed",
                "unknown",
                "skipped",
            ]
            .contains(&v)
        })
    {
        return Err(ApiError::validation_error(
            "Invalid delivery page or status",
        ));
    }
    campaigns::row(&state.db.pool, id).await?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaign_deliveries WHERE campaign_id=$1 AND ($2::text IS NULL OR status=$2)").bind(id).bind(&filters.status).fetch_one(&state.db.pool).await.map_err(db)?;
    let data:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(receipt) FROM (SELECT id,email,display_name,status,attempt_count,attempts_remaining,due_at,provider_message_id,last_error,sender_identity,provider_stream,accepted_at,delivered_at,opened_at,clicked_at,bounced_at,complained_at,unsubscribed_at,created_at,updated_at FROM crm_campaign_deliveries WHERE campaign_id=$1 AND ($2::text IS NULL OR status=$2) ORDER BY created_at,id LIMIT $3 OFFSET $4) receipt").bind(id).bind(filters.status).bind(size).bind(page.saturating_sub(1).saturating_mul(size)).fetch_all(&state.db.pool).await.map_err(db)?;
    Ok(Json(
        json!({"data":data,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":(total+size-1)/size}}),
    ))
}
