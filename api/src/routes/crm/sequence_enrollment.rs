//! Explicit, consented enrollment; a repeated request cannot restart a cohort.
use super::{
    sequence_model::{self as model, invalid, text},
    sequences,
};
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
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(untagged)]
enum RecordId {
    Number(i64),
    Text(String),
}
impl RecordId {
    fn value(&self) -> Result<i64, ApiError> {
        let value = match self {
            Self::Number(value) => Some(*value),
            Self::Text(value) if value.bytes().all(|byte| byte.is_ascii_digit()) => {
                value.parse().ok()
            }
            _ => None,
        };
        value
            .filter(|value| *value > 0)
            .ok_or_else(|| invalid("Contact identities must be positive record IDs"))
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    contact_ids: Vec<RecordId>,
    start_position: Option<usize>,
    expected_version: Option<i64>,
    reason: Option<String>,
}
impl Input {
    fn ids(&self) -> Result<Vec<i64>, ApiError> {
        if !(1..=1000).contains(&self.contact_ids.len()) {
            return Err(invalid("Select1–1000contacts"));
        }
        if self.expected_version.is_some_and(|value| value < 1) {
            return Err(invalid("Sequence version must be positive"));
        }
        let mut ids = self
            .contact_ids
            .iter()
            .map(RecordId::value)
            .collect::<Result<Vec<_>, _>>()?;
        ids.sort_unstable();
        if ids.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(invalid("Contact identities must be distinct"));
        }
        if let Some(reason) = &self.reason {
            text(reason, 4000, false, false)?;
        }
        Ok(ids)
    }
}
#[derive(sqlx::FromRow)]
struct Contact {
    id: i64,
    email: String,
}
async fn enroll(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    let ids = input.ids()?;
    if input.reason.is_some() {
        return Err(invalid("Enrollment does not accept an unsubscribe reason"));
    }
    // Resolve referenced filters before holding the transaction connection.
    let current = sequences::row(&state.db.pool, id).await?;
    let audience = super::resolve_campaign_audience(
        &state.db.pool,
        Some(current["subscriber_settings"].clone()),
    )
    .await?;
    let mut tx = state.db.pool.begin().await.map_err(sequences::db)?;
    let saved = sequences::lock(&mut tx, id).await?;
    let version = saved["authoring_version"]
        .as_i64()
        .ok_or_else(|| invalid("Stored sequence version is invalid"))?;
    if current["authoring_version"].as_i64() != Some(version)
        || input
            .expected_version
            .is_some_and(|expected| expected != version)
    {
        return Err(errors::conflict(
            "Sequence changed; reload before enrolling contacts",
        ));
    }
    if !matches!(saved["status"].as_str(), Some("active" | "paused")) {
        return Err(errors::conflict(
            "Activate this sequence before enrolling contacts",
        ));
    }
    sequences::activation(&saved, &state.config.app_url)?;
    let mails = model::mails(saved["steps"].clone(), id)?;
    let active = mails
        .iter()
        .filter(|mail| mail.status == "active")
        .collect::<Vec<_>>();
    let start = input.start_position.unwrap_or(0);
    let (completed, first) = active
        .iter()
        .enumerate()
        .find(|(_, mail)| mail.position >= start)
        .ok_or_else(|| invalid("Choose an active starting email position"))?;
    let contacts: Vec<Contact> = sqlx::query_as(
        "SELECT id,LOWER(email) AS email FROM contacts WHERE id=ANY($1) ORDER BY id FOR SHARE",
    )
    .bind(&ids)
    .fetch_all(&mut *tx)
    .await
    .map_err(sequences::db)?;
    if contacts.len() != ids.len() {
        return Err(invalid("Some requested contacts no longer exist"));
    }
    let emails = contacts
        .iter()
        .map(|contact| contact.email.clone())
        .collect::<Vec<_>>();
    let distinct = emails.iter().collect::<std::collections::BTreeSet<_>>();
    if distinct.len() != emails.len() {
        return Err(invalid(
            "Resolve contacts sharing the same email before enrolling this batch",
        ));
    }
    audience.lock_member_definition(&mut tx).await?;
    let mut selection = audience.query("SELECT DISTINCT LOWER(c.email) FROM contacts c", true)?;
    selection
        .push(" AND LOWER(c.email)=ANY(")
        .push_bind(emails)
        .push(")");
    let eligible: Vec<String> = selection
        .build_query_scalar()
        .fetch_all(&mut *tx)
        .await
        .map_err(sequences::db)?;
    if contacts
        .iter()
        .any(|contact| !eligible.contains(&contact.email))
    {
        return Err(invalid(
            "Every enrolled contact must currently match the sequence filters and have a confirmed newsletter subscription",
        ));
    }
    let revision: Uuid = serde_json::from_value(saved["revision_id"].clone())
        .map_err(|_| errors::service_unavailable("Sequence revision is unavailable"))?;
    let mut enrolled = 0_u64;
    for contact in contacts {
        enrolled+=sqlx::query("INSERT INTO crm_sequence_trackers(sequence_id,revision_id,contact_id,contact_email,status,next_sequence_mail_id,next_execution_at,completed_steps,total_steps,notes) VALUES($1,$2,$3,$4,'active',$5,NOW()+make_interval(secs=>$6::double precision),$7,$8,$9) ON CONFLICT DO NOTHING")
            .bind(id).bind(revision).bind(contact.id).bind(&contact.email).bind(first.id).bind(first.delay as f64).bind(completed as i32).bind(active.len() as i32).bind(json!([{"action":"enrolled","at":chrono::Utc::now(),"actor_id":admin.0.id,"revision_id":revision,"start_position":first.position}])).execute(&mut *tx).await.map_err(sequences::db)?.rows_affected();
    }
    sequences::record(&mut tx, admin.0.id, id, "Contacts enrolled in sequence").await?;
    tx.commit().await.map_err(sequences::db)?;
    Ok(Json(
        json!({"subscribed_count":enrolled,"already_enrolled":ids.len() as u64-enrolled,"requested":ids.len(),"sequence_id":id.to_string(),"authoring_version":version,"execution_status":if saved["status"]=="active"{"scheduled"}else{"sequence_paused"}}),
    ))
}
pub(super) async fn refresh_engines(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
) -> Result<(), ApiError> {
    let engines: Vec<i64> = sqlx::query_scalar(
        "SELECT campaign_id FROM crm_sequence_engines WHERE sequence_id=$1 ORDER BY campaign_id",
    )
    .bind(id)
    .fetch_all(&mut **tx)
    .await
    .map_err(sequences::db)?;
    for engine in engines {
        crate::services::campaign_delivery::refresh(tx, engine)
            .await
            .map_err(|error| {
                tracing::error!(%error,"Sequence delivery refresh failed");
                errors::service_unavailable("Sequence delivery metrics could not be refreshed")
            })?;
    }
    Ok(())
}
async fn unsubscribe(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    let ids = input.ids()?;
    if input.start_position.is_some() {
        return Err(invalid("Unsubscribe does not accept a starting position"));
    }
    let mut tx = state.db.pool.begin().await.map_err(sequences::db)?;
    let saved = sequences::lock(&mut tx, id).await?;
    if input
        .expected_version
        .is_some_and(|version| saved["authoring_version"].as_i64() != Some(version))
    {
        return Err(errors::conflict(
            "Sequence changed; reload before unsubscribing contacts",
        ));
    }
    let changed=sqlx::query("UPDATE crm_sequence_trackers SET status='cancelled',next_execution_at=NULL,last_error=$3,notes=notes||$4,updated_at=NOW() WHERE sequence_id=$1 AND contact_id=ANY($2) AND status IN ('active','paused','failed')")
        .bind(id).bind(&ids).bind(input.reason.as_deref().unwrap_or("Removed from sequence by an administrator")).bind(json!([{"action":"unsubscribed","at":chrono::Utc::now(),"actor_id":admin.0.id}])).execute(&mut *tx).await.map_err(sequences::db)?.rows_affected();
    // A provider call already in flight retains its actual receipt. No further
    // queued step belonging to these canceled enrollments can be claimed.
    sqlx::query("UPDATE crm_campaign_deliveries d SET status='skipped',last_error='Contact removed from sequence before submission',updated_at=NOW() FROM crm_sequence_trackers t WHERE t.id=d.sequence_tracker_id AND t.sequence_id=$1 AND t.contact_id=ANY($2) AND t.status='cancelled' AND d.status='queued'")
        .bind(id).bind(&ids).execute(&mut *tx).await.map_err(sequences::db)?;
    refresh_engines(&mut tx, id).await?;
    sequences::record(&mut tx, admin.0.id, id, "Contacts removed from sequence").await?;
    tx.commit().await.map_err(sequences::db)?;
    Ok(Json(
        json!({"unsubscribed_count":changed,"sequence_id":id.to_string()}),
    ))
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
    search: Option<String>,
}
async fn subscribers(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    sequences::row(&state.db.pool, id).await?;
    let page = query.page.unwrap_or(1);
    let size = query.per_page.unwrap_or(20);
    if page < 1 || !(1..=100).contains(&size) {
        return Err(invalid("Use a positive page and page size1–100"));
    }
    if query.status.as_deref().is_some_and(|status| {
        !matches!(
            status,
            "active" | "paused" | "completed" | "cancelled" | "failed"
        )
    }) {
        return Err(invalid("Invalid enrollment status"));
    }
    if let Some(search) = &query.search {
        text(search, 255, false, false)?;
    }
    let search = query.search.map(|search| format!("%{search}%"));
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',t.id::text,'contact_id',t.contact_id::text,'sequence_id',t.sequence_id::text,'revision_id',t.revision_id,'contact_email',t.contact_email,'status',t.status,'last_sequence_mail_id',t.last_sequence_mail_id,'next_sequence_mail_id',t.next_sequence_mail_id,'started_at',t.started_at,'last_executed_at',t.last_executed_at,'next_execution_at',t.next_execution_at,'completed_at',t.completed_at,'notes',t.notes,'last_error',t.last_error,'created_at',t.created_at,'updated_at',t.updated_at,'emails_sent',(SELECT COUNT(*) FROM crm_campaign_deliveries d WHERE d.sequence_tracker_id=t.id AND d.accepted_at IS NOT NULL),'emails_opened',(SELECT COUNT(*) FROM crm_campaign_deliveries d WHERE d.sequence_tracker_id=t.id AND d.opened_at IS NOT NULL),'emails_clicked',(SELECT COUNT(*) FROM crm_campaign_deliveries d WHERE d.sequence_tracker_id=t.id AND d.clicked_at IS NOT NULL),'progress_percentage',ROUND(100.0*t.completed_steps/NULLIF(t.total_steps,0),2),'contact',CASE WHEN c.id IS NULL THEN NULL ELSE jsonb_build_object('id',c.id::text,'email',c.email,'first_name',c.first_name,'last_name',c.last_name) END) FROM crm_sequence_trackers t LEFT JOIN contacts c ON c.id=t.contact_id WHERE t.sequence_id=$1 AND ($2::text IS NULL OR t.status=$2) AND ($3::text IS NULL OR t.contact_email ILIKE $3 OR concat_ws(' ',c.first_name,c.last_name) ILIKE $3) ORDER BY t.created_at DESC,t.id DESC LIMIT $4 OFFSET $5")
        .bind(id).bind(&query.status).bind(&search).bind(size).bind(page.saturating_sub(1).saturating_mul(size)).fetch_all(&state.db.pool).await.map_err(sequences::db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_trackers t LEFT JOIN contacts c ON c.id=t.contact_id WHERE t.sequence_id=$1 AND ($2::text IS NULL OR t.status=$2) AND ($3::text IS NULL OR t.contact_email ILIKE $3 OR concat_ws(' ',c.first_name,c.last_name) ILIKE $3)")
        .bind(id).bind(query.status).bind(search).fetch_one(&state.db.pool).await.map_err(sequences::db)?;
    Ok(Json(
        json!({"data":rows,"meta":{"total":total,"current_page":page,"per_page":size,"last_page":((total+size-1)/size).max(1)}}),
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/sequences/{id}/subscribe", post(enroll))
        .route("/sequences/{id}/unsubscribe", post(unsubscribe))
        .route("/sequences/{id}/subscribers", get(subscribers))
}
