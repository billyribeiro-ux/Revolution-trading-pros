//! Durable, explicit QA samples; replays report receipts and never resubmit.
use super::{authorized, db, invalid, lock_authorized, model, record};
use crate::{
    AppState,
    routes::crm::manager_permissions::CrmUser as AdminUser,
    services::{
        campaign_mailer::{CampaignMailer, Identity, Message},
        campaign_personalization::{self, Recipient},
        email::EmailService,
    },
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    email: String,
    email_id: Uuid,
    request_id: Uuid,
    expected_version: i64,
}
impl Input {
    fn recipient(&self) -> Result<String, ApiError> {
        if self.email_id.is_nil() || self.request_id.is_nil() || self.expected_version < 1 {
            return Err(invalid(
                "Provide nonzero request and email UUIDs and the current positive sequence version",
            ));
        }
        let email = self.email.trim().to_lowercase();
        model::text(&email, 255, true, true)?;
        errors::validate_email(&email).map_err(|_| invalid("Provide a valid test recipient"))?;
        Ok(email)
    }
}
#[derive(sqlx::FromRow)]
struct Intent {
    request_id: Uuid,
    sequence_id: i64,
    email_id: Uuid,
    created_by: Option<i64>,
    authoring_version: i64,
    recipient: String,
    created_at: DateTime<Utc>,
}
async fn intent(pool: &PgPool, request: Uuid) -> Result<Option<Intent>, ApiError> {
    sqlx::query_as("SELECT request_id,sequence_id,email_id,created_by,authoring_version,recipient,created_at FROM crm_sequence_test_requests WHERE request_id=$1")
        .bind(request).fetch_optional(pool).await.map_err(db)
}
async fn authorize_receipt(
    pool: &PgPool,
    actor: &AdminUser,
    saved: &Intent,
) -> Result<(), ApiError> {
    let mut tx = crate::routes::crm::manager_sources::read_transaction(pool).await?;
    let current: Value = sqlx::query_scalar("SELECT to_jsonb(s) FROM crm_sequences s WHERE id=$1")
        .bind(saved.sequence_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Sequence"))?;
    crate::routes::crm::manager_sources::sequence_on(&mut tx, actor, saved.sequence_id, &current)
        .await?;
    let frozen:Value=sqlx::query_scalar("SELECT r.definition FROM crm_sequence_test_requests q JOIN crm_sequence_revisions r ON r.id=q.revision_id WHERE q.request_id=$1")
        .bind(saved.request_id).fetch_optional(&mut *tx).await.map_err(db)?.ok_or_else(||errors::service_unavailable("Test source revision is unavailable"))?;
    actor.record(&frozen)?;
    tx.commit().await.map_err(db)?;
    Ok(())
}
fn same_intent(
    saved: &Intent,
    id: i64,
    actor: i64,
    input: &Input,
    email: &str,
) -> Result<(), ApiError> {
    if saved.sequence_id != id
        || saved.created_by != Some(actor)
        || saved.email_id != input.email_id
        || saved.recipient != email
        || saved.authoring_version != input.expected_version
    {
        return Err(errors::conflict(
            "This test request ID already belongs to a different intent",
        ));
    }
    Ok(())
}
fn alias(id: i64, email: Uuid) -> String {
    format!("sequence-test:{id}:{email}")
}
async fn receipt(pool: &PgPool, saved: &Intent) -> Result<Value, ApiError> {
    let submissions: Vec<Value> = sqlx::query_scalar("SELECT jsonb_build_object('id',id,'recipient',to_email,'provider',provider,'provider_message_id',provider_message_id,'status',CASE WHEN status IN ('sent','delivered','opened','clicked','bounced') AND NULLIF(provider_message_id,'') IS NOT NULL AND sent_at IS NOT NULL THEN 'accepted' WHEN status='failed' THEN 'rejected' WHEN status='unavailable' THEN 'unavailable' WHEN status='queued' AND queued_at>NOW()-INTERVAL '90 seconds' THEN 'processing' ELSE 'unknown' END,'error',COALESCE(error,error_message),'queued_at',queued_at,'accepted_at',sent_at) FROM email_logs WHERE template_type='sequence_test' AND template_alias=$1 AND model->>'request_id'=$2 ORDER BY id")
        .bind(alias(saved.sequence_id,saved.email_id)).bind(saved.request_id.to_string()).fetch_all(pool).await.map_err(db)?;
    if submissions.len() != 1 || submissions[0]["recipient"] != saved.recipient {
        return Err(errors::service_unavailable(
            "Test receipt is incomplete; reconcile this request before sending another sample",
        ));
    }
    Ok(
        json!({"sequence_id":saved.sequence_id.to_string(),"email_id":saved.email_id,"request_id":saved.request_id,"authoring_version":saved.authoring_version,"metrics_scope":"sequence_test","created_at":saved.created_at,"submissions":submissions}),
    )
}
async fn response(pool: &PgPool, saved: &Intent) -> Result<(StatusCode, Json<Value>), ApiError> {
    let data = receipt(pool, saved).await?;
    let status = match data["submissions"][0]["status"].as_str() {
        Some("accepted") => StatusCode::OK,
        Some("processing") => StatusCode::ACCEPTED,
        Some("unavailable") => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::BAD_GATEWAY,
    };
    Ok((
        status,
        Json(json!({"success":status==StatusCode::OK,"data":data})),
    ))
}
fn source_hash(source: &Value) -> Result<String, ApiError> {
    let definition = json!({"steps":source["steps"],"settings":source["settings"],"design_template":source["design_template"]});
    let bytes =
        serde_json::to_vec(&definition).map_err(|_| invalid("Sequence content is invalid"))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}
#[derive(sqlx::FromRow, Default)]
struct Contact {
    first_name: String,
    last_name: String,
    company: String,
}
fn identity(settings: &model::Mailer, reply: bool) -> Option<Identity> {
    let (email, name) = if reply {
        (&settings.reply_to_email, &settings.reply_to_name)
    } else {
        (&settings.from_email, &settings.from_name)
    };
    (!email.is_empty()).then(|| Identity {
        email: email.clone(),
        name: name.clone(),
    })
}
async fn submit(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let email = input.recipient()?;
    // Resolve persisted intent before reading edited/archived creative or
    // provider configuration. An earlier submission cannot become another send.
    if let Some(saved) = intent(&state.db.pool, input.request_id).await? {
        authorize_receipt(&state.db.pool, &admin, &saved).await?;
        same_intent(&saved, id, admin.0.id, &input, &email)?;
        return response(&state.db.pool, &saved).await;
    }
    let source = authorized(&state.db.pool, &admin, id).await?;
    if source["authoring_version"].as_i64() != Some(input.expected_version) {
        return Err(errors::conflict(
            "Sequence changed; reload before sending a test",
        ));
    }
    let mail = model::mails(source["steps"].clone(), id)?
        .into_iter()
        .find(|mail| mail.id == input.email_id)
        .ok_or_else(|| ApiError::not_found("Sequence email"))?;
    mail.validate(true)?;
    let parent: model::Settings = serde_json::from_value(source["settings"].clone())
        .map_err(|_| invalid("Review the stored sequence sender settings"))?;
    let settings = if mail.settings.is_custom {
        &mail.settings
    } else {
        &parent.mailer_settings
    };
    settings.validate()?;
    let kind = source["design_template"]
        .as_str()
        .filter(|kind| matches!(*kind, "raw_html" | "plain_text"))
        .ok_or_else(|| invalid("Review the stored sequence content type"))?;
    let mailer = CampaignMailer::from_state(&state).await.map_err(|error| {
        tracing::warn!(%error,"Sequence test provider unavailable");
        errors::service_unavailable("Configure a broadcast email provider before sending a test")
    })?;
    crate::services::campaign_delivery::callback_url(&state.config.app_url).map_err(|_| {
        errors::service_unavailable("Configure a valid APP_URL before sending a test")
    })?;
    let mut contacts: Vec<Contact> = sqlx::query_as("SELECT COALESCE(first_name,'') AS first_name,COALESCE(last_name,'') AS last_name,COALESCE(company,'') AS company FROM contacts WHERE LOWER(email)=$1 ORDER BY id LIMIT 2")
        .bind(&email).fetch_all(&state.db.pool).await.map_err(db)?;
    if contacts.len() > 1 {
        return Err(invalid(
            "Resolve contacts sharing the test address before personalizing a sample",
        ));
    }
    let contact = contacts.pop().unwrap_or_default();
    let name = format!("{} {}", contact.first_name, contact.last_name)
        .trim()
        .to_string();
    let name = if name.is_empty() { &email } else { &name };
    let snapshot = json!({"first_name":contact.first_name,"last_name":contact.last_name,"company":contact.company});
    let unsubscribe_url = format!(
        "{}/newsletter/unsubscribe?preview=sequence-test",
        state.config.app_url.trim_end_matches('/')
    );
    let definition = json!({"sequence_id":id,"type":kind,"email_body":mail.email_body,"text_body":mail.text_body});
    let rendered = campaign_personalization::render(
        &definition,
        Recipient {
            email: &email,
            name,
            snapshot: &snapshot,
            app_url: &state.config.app_url,
            unsubscribe_url: &unsubscribe_url,
            brand: &mailer.sender().name,
        },
        &mail.email_subject,
    )
    .map_err(|_| {
        invalid("Sequence email has unsupported variables, invalid syntax or excessive output")
    })?;
    let plain = kind == "plain_text";
    let notice =
        "Test message. Unsubscribe actions are inactive; no sequence enrollment was submitted.";
    let content = rendered["email_body"].as_str().unwrap_or("");
    let body = if plain {
        format!("{notice}\n\n{content}")
    } else {
        format!("<p>{notice}</p>{content}")
    };
    let message = Message {
        to: email.clone(),
        subject: format!(
            "[Test] {}",
            rendered["email_subject"].as_str().unwrap_or("")
        ),
        body,
        plain_text: plain,
        text_body: (!plain)
            .then(|| {
                rendered["text_body"]
                    .as_str()
                    .map(|text| format!("{notice}\n\n{text}"))
            })
            .flatten(),
        from: settings
            .is_custom
            .then(|| identity(settings, false))
            .flatten(),
        reply_to: identity(settings, true),
        unsubscribe_url,
        metadata: BTreeMap::from([
            ("sequence_test".into(), id.to_string()),
            ("sequence_email_id".into(), mail.id.to_string()),
            ("test_request_id".into(), input.request_id.to_string()),
        ]),
    };
    mailer
        .validate(&message)
        .map_err(|error| invalid(&error.to_string()))?;
    let hash = source_hash(&source)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query(
        "SELECT pg_advisory_xact_lock(hashtextextended('crm-sequence-test:'||$1::uuid::text,0))",
    )
    .bind(input.request_id)
    .execute(&mut *tx)
    .await
    .map_err(db)?;
    let replay: Option<Intent> = sqlx::query_as("SELECT request_id,sequence_id,email_id,created_by,authoring_version,recipient,created_at FROM crm_sequence_test_requests WHERE request_id=$1")
        .bind(input.request_id).fetch_optional(&mut *tx).await.map_err(db)?;
    if let Some(saved) = replay {
        tx.rollback().await.map_err(db)?;
        authorize_receipt(&state.db.pool, &admin, &saved).await?;
        same_intent(&saved, id, admin.0.id, &input, &email)?;
        return response(&state.db.pool, &saved).await;
    }
    let saved = lock_authorized(&mut tx, &admin, id).await?;
    if saved["authoring_version"].as_i64() != Some(input.expected_version)
        || source_hash(&saved)? != hash
    {
        return Err(errors::conflict(
            "Sequence changed while preparing this test; reload before submitting",
        ));
    }
    let revision: Uuid = serde_json::from_value(saved["revision_id"].clone())
        .map_err(|_| errors::service_unavailable("Sequence revision is unavailable"))?;
    let frozen: Value =
        sqlx::query_scalar("SELECT definition FROM crm_sequence_revisions WHERE id=$1")
            .bind(revision)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
    if source_hash(&frozen)? != hash {
        return Err(errors::conflict(
            "Stored sequence revision differs from its saved definition; review it before testing",
        ));
    }
    let inserted = sqlx::query("INSERT INTO crm_sequence_test_requests(request_id,sequence_id,revision_id,email_id,created_by,authoring_version,recipient,source_hash) VALUES($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT(request_id) DO NOTHING")
        .bind(input.request_id).bind(id).bind(revision).bind(input.email_id).bind(admin.0.id).bind(input.expected_version).bind(&email).bind(hash).execute(&mut *tx).await.map_err(db)?.rows_affected()==1;
    if !inserted {
        tx.rollback().await.map_err(db)?;
        let saved = intent(&state.db.pool, input.request_id)
            .await?
            .ok_or_else(|| errors::service_unavailable("Test request is unavailable"))?;
        authorize_receipt(&state.db.pool, &admin, &saved).await?;
        same_intent(&saved, id, admin.0.id, &input, &email)?;
        return response(&state.db.pool, &saved).await;
    }
    let metadata = json!({"request_id":input.request_id,"sequence_id":id,"email_id":input.email_id,"revision_id":revision,"authoring_version":input.expected_version,"actor_id":admin.0.id});
    let log:i64=sqlx::query_scalar("INSERT INTO email_logs(email,to_email,subject,template_type,template_alias,model,metadata,status,provider,queued_at,created_at) VALUES($1,$1,$2,'sequence_test',$3,$4,$4,'queued',$5,NOW(),NOW() AT TIME ZONE 'UTC') RETURNING id")
        .bind(&email).bind(&message.subject).bind(alias(id,input.email_id)).bind(metadata).bind(mailer.provider()).fetch_one(&mut *tx).await.map_err(db)?;
    record(&mut tx, admin.0.id, id, "Sequence test staged").await?;
    tx.commit().await.map_err(db)?;
    // Recheck persisted provider configuration after staging, without retrying
    // an intent if configuration changed or a receipt cannot be persisted.
    let current = CampaignMailer::from_state(&state)
        .await
        .ok()
        .filter(|current| {
            current.provider() == mailer.provider()
                && current.stream() == mailer.stream()
                && current.sender().email == mailer.sender().email
                && current.sender().name == mailer.sender().name
        });
    if let Some(current) = current {
        let outcome = current.submit(message).await;
        if let Err(error) = EmailService::record_outcome(&state.db.pool, log, outcome).await {
            tracing::warn!(log,%error,"Sequence test submission not confirmed");
        }
    } else {
        sqlx::query("UPDATE email_logs SET status='unavailable',error='Provider configuration changed before submission',error_message='Provider configuration changed before submission' WHERE id=$1 AND status='queued'")
            .bind(log).execute(&state.db.pool).await.map_err(db)?;
    }
    let saved = intent(&state.db.pool, input.request_id)
        .await?
        .ok_or_else(|| {
            errors::service_unavailable(
                "Test request receipt is unavailable; reconcile before retrying",
            )
        })?;
    response(&state.db.pool, &saved).await
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct History {
    request_id: Option<Uuid>,
}
async fn history(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<History>,
) -> Result<Json<Value>, ApiError> {
    if query.request_id.is_some_and(|request| request.is_nil()) {
        return Err(invalid("Provide a nonzero request UUID"));
    }
    let mut tx = crate::routes::crm::manager_sources::read_transaction(&state.db.pool).await?;
    let source: Value = sqlx::query_scalar("SELECT to_jsonb(s) FROM crm_sequences s WHERE id=$1")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Sequence"))?;
    crate::routes::crm::manager_sources::sequence_on(&mut tx, &admin, id, &source).await?;
    tx.commit().await.map_err(db)?;
    let requests:Vec<Intent>=sqlx::query_as("SELECT request_id,sequence_id,email_id,created_by,authoring_version,recipient,created_at FROM crm_sequence_test_requests WHERE sequence_id=$1 AND ($2::uuid IS NULL OR request_id=$2) ORDER BY created_at DESC,request_id DESC LIMIT 20")
        .bind(id).bind(query.request_id).fetch_all(&state.db.pool).await.map_err(db)?;
    let mut data = Vec::with_capacity(requests.len());
    for saved in requests {
        authorize_receipt(&state.db.pool, &admin, &saved).await?;
        data.push(receipt(&state.db.pool, &saved).await?);
    }
    Ok(Json(
        json!({"success":true,"data":{"sequence_id":id.to_string(),"metrics_scope":"sequence_test","requests":data}}),
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/sequences/{id}/test", post(submit))
        .route("/sequences/{id}/tests", get(history))
}
