//! Durable event capture and consent-checked enrollment, separate from delivery.
use super::{sequence_model as model, sequences};
use crate::{
    AppState,
    routes::crm::manager_permissions::CrmUser as AdminUser,
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Configuration {
    tag: Option<String>,
    event_name: Option<String>,
    product_id: Option<i64>,
}
pub(super) fn validate(kind: &str, value: &Value, activation: bool) -> Result<(), ApiError> {
    if !matches!(
        kind,
        "manual" | "signup" | "purchase" | "tag_added" | "custom_event"
    ) {
        return Err(model::invalid(
            "Review the unsupported stored sequence trigger",
        ));
    }
    model::object(value)?;
    let config: Configuration = serde_json::from_value(value.clone())
        .map_err(|_| model::invalid("Unsupported sequence trigger settings"))?;
    if config.tag.is_some() && kind != "tag_added"
        || config.event_name.is_some() && kind != "custom_event"
        || config.product_id.is_some() && kind != "purchase"
    {
        return Err(model::invalid(
            "Trigger settings do not match the selected trigger",
        ));
    }
    if let Some(tag) = &config.tag {
        model::text(tag, 100, true, true)?;
    }
    if let Some(name) = &config.event_name {
        model::text(name, 128, true, true)?;
    }
    if config.product_id.is_some_and(|id| id <= 0) {
        return Err(model::invalid(
            "Trigger product_id must be a positive record ID",
        ));
    }
    if activation
        && (kind == "tag_added" && config.tag.is_none()
            || kind == "custom_event" && config.event_name.is_none())
    {
        return Err(model::invalid(
            "Tag triggers require tag; custom triggers require event_name",
        ));
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EventInput {
    event_id: Uuid,
    contact_id: i64,
    event_name: String,
    #[serde(default = "empty_object")]
    properties: Value,
}
fn empty_object() -> Value {
    json!({})
}
async fn capture(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<EventInput>,
) -> Result<Json<Value>, ApiError> {
    if input.contact_id <= 0 || input.event_id.is_nil() {
        return Err(model::invalid("Provide a contact ID and nonnil event UUID"));
    }
    model::text(&input.event_name, 128, true, true)?;
    model::object(&input.properties)?;
    let source_key = format!("custom:{}", input.event_id);
    let payload = json!({"event_name":input.event_name,"properties":input.properties,"contact_id":input.contact_id});
    let mut tx = state.db.pool.begin().await.map_err(sequences::db)?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))")
        .bind(&source_key)
        .execute(&mut *tx)
        .await
        .map_err(sequences::db)?;
    let existing: Option<(i64, Value, Option<i64>)> =
        sqlx::query_as("SELECT id,payload,actor_id FROM crm_trigger_events WHERE source_key=$1")
            .bind(&source_key)
            .fetch_optional(&mut *tx)
            .await
            .map_err(sequences::db)?;
    let id = if let Some((id, saved, actor)) = existing {
        if saved != payload || actor != Some(admin.0.id) {
            return Err(errors::conflict(
                "Event UUID already records different event details",
            ));
        }
        id
    } else {
        let email: String =
            sqlx::query_scalar("SELECT LOWER(email) FROM contacts WHERE id=$1 FOR SHARE")
                .bind(input.contact_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(sequences::db)?
                .ok_or_else(|| errors::not_found("Contact"))?;
        let id: i64 = sqlx::query_scalar("INSERT INTO crm_trigger_events(source_key,kind,contact_id,email,payload,actor_id) VALUES($1,'custom_event',$2,$3,$4,$5) RETURNING id")
            .bind(source_key).bind(input.contact_id).bind(email).bind(payload).bind(admin.0.id)
            .fetch_one(&mut *tx).await.map_err(sequences::db)?;
        crate::services::crm_logs::record(
            &mut tx,
            "info",
            "email",
            "CRM trigger event recorded",
            json!({"event_id":id,"contact_id":input.contact_id}),
            Some(admin.0.id),
        )
        .await
        .map_err(sequences::db)?;
        id
    };
    let matched: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_trigger_matches WHERE event_id=$1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(sequences::db)?;
    tx.commit().await.map_err(sequences::db)?;
    Ok(Json(
        json!({"event_id":input.event_id,"record_id":id.to_string(),"event_recorded":true,"matched_sequences":matched,"execution_status":"recorded","metrics_scope":"sequence_trigger"}),
    ))
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
}
async fn history(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(20);
    if page < 1 || !(1..=100).contains(&size) {
        return Err(model::invalid(
            "Choose a positive page and 1–100 trigger records",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(sequences::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(sequences::db)?;
    sequences::authorized_on(&mut tx, &admin, id).await?;
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_sequence_trigger_matches WHERE sequence_id=$1",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .map_err(sequences::db)?;
    let data:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',m.id::text,'event_id',m.event_id::text,'sequence_id',m.sequence_id::text,'revision_id',m.revision_id,'kind',e.kind,'happened_at',e.happened_at,'status',m.status,'contact_id',m.contact_id::text,'tracker_id',m.tracker_id::text,'attempts',m.attempts,'last_error',m.last_error,'processed_at',m.processed_at) FROM crm_sequence_trigger_matches m JOIN crm_trigger_events e ON e.id=m.event_id WHERE m.sequence_id=$1 ORDER BY m.id DESC LIMIT $2 OFFSET $3")
        .bind(id).bind(size).bind(page.saturating_sub(1).saturating_mul(size)).fetch_all(&mut *tx).await.map_err(sequences::db)?;
    tx.commit().await.map_err(sequences::db)?;
    Ok(Json(
        json!({"sequence_id":id.to_string(),"metrics_scope":"sequence_trigger","data":data,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":((total+size-1)/size).max(1)}}),
    ))
}

#[derive(sqlx::FromRow)]
struct Candidate {
    id: i64,
    sequence_id: i64,
    revision_id: Uuid,
    event_id: i64,
    kind: String,
    contact_id: Option<i64>,
    email: String,
    actor_id: Option<i64>,
    payload: Value,
    definition: Value,
}
#[derive(sqlx::FromRow)]
struct Identity {
    id: i64,
    email: String,
}
async fn finish(
    tx: &mut Transaction<'_, Postgres>,
    candidate: &Candidate,
    status: &str,
    contact: Option<i64>,
    tracker: Option<i64>,
    reason: Option<&str>,
) -> Result<(), ApiError> {
    sqlx::query("UPDATE crm_sequence_trigger_matches SET status=$2,contact_id=$3,tracker_id=$4,last_error=$5,processed_at=NOW() WHERE id=$1")
        .bind(candidate.id).bind(status).bind(contact).bind(tracker).bind(reason).execute(&mut **tx).await.map_err(sequences::db)?;
    crate::services::crm_logs::record(tx,"info","email","Sequence trigger processed",json!({"sequence_id":candidate.sequence_id,"event_id":candidate.event_id,"status":status,"tracker_id":tracker,"reason":reason}),candidate.actor_id)
        .await.map_err(sequences::db)
}
async fn enroll(pool: &PgPool, candidate: &Candidate) -> Result<bool, ApiError> {
    validate(
        candidate.definition["trigger_type"].as_str().unwrap_or(""),
        &candidate.definition["trigger_config"],
        true,
    )?;
    // Resolve referenced definitions before acquiring the transaction connection.
    let audience = super::resolve_campaign_audience(
        pool,
        Some(candidate.definition["subscriber_settings"].clone()),
    )
    .await?;
    let mut tx = pool.begin().await.map_err(sequences::db)?;
    let parent = sequences::lock(&mut tx, candidate.sequence_id).await?;
    if parent["status"] != "active" {
        return Ok(false);
    }
    let locked:Option<i64>=sqlx::query_scalar("SELECT id FROM crm_sequence_trigger_matches WHERE id=$1 AND status='pending' AND available_at<=NOW() FOR UPDATE SKIP LOCKED")
        .bind(candidate.id).fetch_optional(&mut *tx).await.map_err(sequences::db)?;
    if locked.is_none() {
        return Ok(false);
    }
    let source_current = match candidate.kind.as_str() {
        "signup" if candidate.payload["source"] == "member" => {
            sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM users WHERE id=$1 AND LOWER(email)=$2 AND is_active=true AND deleted_at IS NULL)")
                .bind(candidate.payload["user_id"].as_i64()).bind(&candidate.email).fetch_one(&mut *tx).await.map_err(sequences::db)?
        }
        "signup" => {
            sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM newsletter_subscribers WHERE id=$1 AND LOWER(email)=$2)")
                .bind(candidate.payload["subscriber_id"].as_i64()).bind(&candidate.email).fetch_one(&mut *tx).await.map_err(sequences::db)?
        }
        "purchase" => {
            sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM orders o JOIN users u ON u.id=o.user_id WHERE o.id=$1 AND o.status='completed' AND LOWER(COALESCE(NULLIF(o.billing_email,''),u.email))=$2)")
                .bind(candidate.payload["order_id"].as_i64()).bind(&candidate.email).fetch_one(&mut *tx).await.map_err(sequences::db)?
        }
        _ => true,
    };
    if !source_current {
        finish(
            &mut tx,
            candidate,
            "skipped",
            None,
            None,
            Some("Trigger source was deleted, changed identity or is no longer eligible"),
        )
        .await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    let confirmed:bool=sqlx::query_scalar("SELECT COALESCE((SELECT status='confirmed' AND confirmed_at IS NOT NULL AND unsubscribed_at IS NULL FROM newsletter_subscribers WHERE LOWER(email)=$1 ORDER BY updated_at DESC NULLS LAST,id DESC LIMIT 1),false)")
        .bind(&candidate.email).fetch_one(&mut *tx).await.map_err(sequences::db)?;
    if !confirmed {
        finish(
            &mut tx,
            candidate,
            "skipped",
            None,
            None,
            Some("No current confirmed newsletter consent"),
        )
        .await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    // Address-based sources must have an unambiguous actual CRM identity.
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('crm-trigger-contact:'||$1,0))")
        .bind(&candidate.email)
        .execute(&mut *tx)
        .await
        .map_err(sequences::db)?;
    let mut contacts: Vec<Identity> = if let Some(id) = candidate.contact_id {
        sqlx::query_as("SELECT id,LOWER(email) AS email FROM contacts WHERE id=$1 AND LOWER(email)=$2 FOR SHARE")
            .bind(id).bind(&candidate.email).fetch_all(&mut *tx).await.map_err(sequences::db)?
    } else if matches!(candidate.kind.as_str(), "signup" | "purchase") {
        sqlx::query_as("SELECT id,LOWER(email) AS email FROM contacts WHERE LOWER(email)=$1 ORDER BY id FOR SHARE")
            .bind(&candidate.email).fetch_all(&mut *tx).await.map_err(sequences::db)?
    } else {
        vec![]
    };
    if contacts.is_empty() && matches!(candidate.kind.as_str(), "signup" | "purchase") {
        let contact:Identity=sqlx::query_as("INSERT INTO contacts(email,source,status) VALUES($1,'sequence_trigger','lead') RETURNING id,LOWER(email) AS email")
            .bind(&candidate.email).fetch_one(&mut *tx).await.map_err(sequences::db)?;
        contacts.push(contact);
    }
    if contacts.len() != 1 {
        finish(
            &mut tx,
            candidate,
            "skipped",
            None,
            None,
            Some("Contact was deleted, changed address or has ambiguous CRM identities"),
        )
        .await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    let contact = &contacts[0];
    audience.lock_member_definition(&mut tx).await?;
    let eligible = if audience.is_member() || audience.is_newsletter() {
        let mut query = audience.query("SELECT LOWER(c.email) FROM contacts c", true)?;
        query.push(" AND LOWER(c.email)=").push_bind(&contact.email);
        query
            .build_query_scalar::<String>()
            .fetch_optional(&mut *tx)
            .await
            .map_err(sequences::db)?
            .is_some()
    } else {
        let mut query = audience.query("SELECT c.id FROM contacts c", true)?;
        query.push(" AND c.id=").push_bind(contact.id);
        query
            .build_query_scalar::<i64>()
            .fetch_optional(&mut *tx)
            .await
            .map_err(sequences::db)?
            .is_some()
    };
    if !eligible {
        finish(
            &mut tx,
            candidate,
            "skipped",
            Some(contact.id),
            None,
            Some("Contact does not currently match the saved sequence audience"),
        )
        .await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    let mails = model::mails(candidate.definition["steps"].clone(), candidate.sequence_id)?;
    let active = mails
        .iter()
        .filter(|mail| mail.status == "active")
        .collect::<Vec<_>>();
    let first = active
        .first()
        .ok_or_else(|| model::invalid("Captured sequence revision has no active email"))?;
    let tracker:Option<i64>=sqlx::query_scalar("INSERT INTO crm_sequence_trackers(sequence_id,revision_id,contact_id,contact_email,status,next_sequence_mail_id,next_execution_at,total_steps,notes) VALUES($1,$2,$3,$4,'active',$5,NOW()+make_interval(secs=>$6::double precision),$7,$8) ON CONFLICT DO NOTHING RETURNING id")
        .bind(candidate.sequence_id).bind(candidate.revision_id).bind(contact.id).bind(&contact.email).bind(first.id).bind(first.delay as f64).bind(active.len() as i32)
        .bind(json!([{"action":"trigger_enrolled","at":chrono::Utc::now(),"event_id":candidate.event_id,"kind":candidate.kind,"revision_id":candidate.revision_id}]))
        .fetch_optional(&mut *tx).await.map_err(sequences::db)?;
    let (status, tracker) = if let Some(id) = tracker {
        ("enrolled", id)
    } else {
        let id:i64=sqlx::query_scalar("SELECT id FROM crm_sequence_trackers WHERE sequence_id=$1 AND (contact_id=$2 OR LOWER(contact_email)=$3) ORDER BY id LIMIT 1")
            .bind(candidate.sequence_id).bind(contact.id).bind(&contact.email).fetch_one(&mut *tx).await.map_err(sequences::db)?;
        ("already_enrolled", id)
    };
    finish(
        &mut tx,
        candidate,
        status,
        Some(contact.id),
        Some(tracker),
        None,
    )
    .await?;
    tx.commit().await.map_err(sequences::db)?;
    Ok(true)
}
pub async fn process_one(pool: &PgPool) -> Result<bool, ApiError> {
    let cancelled=sqlx::query("UPDATE crm_sequence_trigger_matches m SET status='cancelled',last_error='Sequence archived or stopped before enrollment',processed_at=NOW() FROM crm_sequences s WHERE s.id=m.sequence_id AND m.status='pending' AND (s.deleted_at IS NOT NULL OR s.status IN ('draft','completed'))")
        .execute(pool).await.map_err(sequences::db)?.rows_affected();
    let candidate:Option<Candidate>=sqlx::query_as("SELECT m.id,m.sequence_id,m.revision_id,m.event_id,e.kind,e.contact_id,e.email,e.actor_id,e.payload,r.definition FROM crm_sequence_trigger_matches m JOIN crm_sequences s ON s.id=m.sequence_id JOIN crm_trigger_events e ON e.id=m.event_id JOIN crm_sequence_revisions r ON r.id=m.revision_id WHERE m.status='pending' AND m.available_at<=NOW() AND s.status='active' AND s.deleted_at IS NULL ORDER BY m.id LIMIT 1")
        .fetch_optional(pool).await.map_err(sequences::db)?;
    let Some(candidate) = candidate else {
        return Ok(cancelled > 0);
    };
    let result = enroll(pool, &candidate).await;
    if let Err(error) = &result {
        // No provider call occurs here. A failed atomic enrollment can be retried.
        sqlx::query("UPDATE crm_sequence_trigger_matches SET attempts=attempts+1,status=CASE WHEN attempts>=4 THEN 'failed' ELSE 'pending' END,last_error=$2,available_at=NOW()+INTERVAL '60 seconds' WHERE id=$1 AND status='pending'")
            .bind(candidate.id).bind(&error.message).execute(pool).await.map_err(sequences::db)?;
    }
    result
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/sequences/events", post(capture))
        .route("/sequences/{id}/triggers", get(history))
}
