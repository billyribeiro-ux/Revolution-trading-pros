//! One explicit personal message has one durable intent and one provider receipt.
use super::{Page, RecordId, activity, contact, read_contact};
use crate::{
    AppState,
    routes::crm::manager_permissions::CrmUser as AdminUser,
    services::{crm_logs, email::EmailService, transactional_templates},
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Input {
    request_id: Uuid,
    expected_email: String,
    subject: String,
    body_html: String,
    text_body: Option<String>,
    template_id: Option<RecordId>,
}
impl Input {
    fn normalized(&self, id: i64) -> Result<(String, Option<i64>, String), ApiError> {
        let email = self.expected_email.trim().to_lowercase();
        errors::validate_email(&email)
            .map_err(|_| ApiError::validation_error("Provide the saved contact email"))?;
        if id <= 0
            || self.request_id.is_nil()
            || email.len() > 255
            || self.subject.trim().is_empty()
            || self.subject.chars().count() > 500
            || self.subject.contains(['\r', '\n', '\0'])
            || self.body_html.len() > 1_000_000
            || self.body_html.contains('\0')
            || self
                .text_body
                .as_ref()
                .is_some_and(|text| text.len() > 1_000_000 || text.contains('\0'))
            || (self.body_html.trim().is_empty()
                && self
                    .text_body
                    .as_ref()
                    .is_none_or(|text| text.trim().is_empty()))
        {
            return Err(ApiError::validation_error(
                "Provide a nonzero request UUID, bounded subject/content and saved recipient without invalid characters",
            ));
        }
        let template = self.template_id.as_ref().map(RecordId::value).transpose()?;
        let bytes=serde_json::to_vec(&json!({"contact_id":id,"recipient":email,"subject":self.subject.trim(),"body_html":self.body_html,"text_body":self.text_body,"template_id":template})).map_err(|_|ApiError::validation_error("Message intent is invalid"))?;
        Ok((email, template, hex::encode(Sha256::digest(bytes))))
    }
}
#[derive(sqlx::FromRow)]
struct Intent {
    request_id: Uuid,
    original_contact_id: i64,
    created_by: Option<i64>,
    recipient: String,
    subject: String,
    source_hash: String,
    source_subject: String,
    original_template_id: Option<i64>,
    email_log_id: Option<i64>,
    created_at: DateTime<Utc>,
}
macro_rules! intent_sql {()=>{"SELECT request_id,original_contact_id,created_by,recipient,subject,source_hash,content_snapshot->>'source_subject' AS source_subject,original_template_id,email_log_id,created_at FROM crm_contact_mail_requests WHERE request_id=$1"};}
async fn intent(pool: &PgPool, key: Uuid) -> Result<Option<Intent>, ApiError> {
    sqlx::query_as(intent_sql!())
        .bind(key)
        .fetch_optional(pool)
        .await
        .map_err(activity::db)
}
fn same(saved: &Intent, id: i64, actor: i64, email: &str, hash: &str) -> Result<(), ApiError> {
    if saved.original_contact_id != id
        || saved.created_by != Some(actor)
        || saved.recipient != email
        || saved.source_hash != hash
    {
        return Err(errors::conflict(
            "Request UUID already belongs to a different contact message intent",
        ));
    }
    Ok(())
}
async fn response(pool: &PgPool, saved: &Intent) -> Result<(StatusCode, Json<Value>), ApiError> {
    let log = saved.email_log_id.ok_or_else(|| {
        errors::service_unavailable(
            "Message receipt is incomplete; reconcile before sending another message",
        )
    })?;
    let receipt:Option<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',id::text,'recipient',to_email,'subject',subject,'provider',provider,'provider_message_id',provider_message_id,'status',CASE WHEN status IN ('sent','delivered','opened','clicked','bounced') AND NULLIF(provider_message_id,'') IS NOT NULL AND sent_at IS NOT NULL THEN 'accepted' WHEN status='failed' THEN 'rejected' WHEN status='unavailable' THEN 'unavailable' WHEN status='queued' AND queued_at>NOW()-INTERVAL '90 seconds' THEN 'processing' ELSE 'unknown' END,'error',COALESCE(error,error_message),'queued_at',queued_at,'accepted_at',CASE WHEN status IN ('sent','delivered','opened','clicked','bounced') AND NULLIF(provider_message_id,'') IS NOT NULL THEN sent_at ELSE NULL END) FROM email_logs WHERE id=$1 AND template_type='crm_contact_mail' AND model->>'request_id'=$2")
        .bind(log).bind(saved.request_id.to_string()).fetch_optional(pool).await.map_err(activity::db)?;
    let receipt = receipt
        .filter(|value| value["recipient"] == saved.recipient && value["subject"] == saved.subject)
        .ok_or_else(|| {
            errors::service_unavailable(
                "Message receipt is incomplete; reconcile before sending another message",
            )
        })?;
    let status = match receipt["status"].as_str() {
        Some("accepted") => StatusCode::OK,
        Some("processing") => StatusCode::ACCEPTED,
        Some("unavailable") => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::BAD_GATEWAY,
    };
    Ok((
        status,
        Json(
            json!({"success":status==StatusCode::OK,"data":{"contact_id":saved.original_contact_id.to_string(),"request_id":saved.request_id,"metrics_scope":"contact_message","created_at":saved.created_at,"source_subject":saved.source_subject,"source_hash":saved.source_hash,"template_id":saved.original_template_id.map(|id|id.to_string()),"submissions":[receipt]}}),
        ),
    ))
}
fn model(source: &Value, email: &str) -> Value {
    let first = source["first_name"].as_str().unwrap_or("");
    let last = source["last_name"].as_str().unwrap_or("");
    let full = format!("{first} {last}").trim().to_string();
    let name = if full.is_empty() { email } else { &full };
    let person = json!({"email":email,"name":name,"full_name":name,"first_name":first,"last_name":last,"company":source["company"].as_str().unwrap_or(""),"phone":source["phone"].as_str().unwrap_or(""),"job_title":source["job_title"].as_str().unwrap_or("")});
    json!({"email":email,"name":name,"full_name":name,"first_name":first,"last_name":last,"company":source["company"].as_str().unwrap_or(""),"contact":person,"user":person,"subscriber":person})
}
async fn send(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    admin.body(
        &serde_json::to_value(&input)
            .map_err(|_| ApiError::validation_error("Invalid personal message intent"))?,
    )?;
    let (email, template, hash) = input.normalized(id)?;
    if let Some(saved) = intent(&state.db.pool, input.request_id).await? {
        same(&saved, id, admin.0.id, &email, &hash)?;
        return response(&state.db.pool, &saved).await;
    }
    let source = read_contact(&state.db.pool, id).await?;
    let actual = source["email"].as_str().unwrap_or("").trim().to_lowercase();
    if actual != email {
        return Err(errors::conflict(
            "Contact email changed; reload and review the actual recipient before sending",
        ));
    }
    let mailer =
        state.services.email.resolved().await.map_err(|_| {
            errors::service_unavailable("Email provider configuration is unavailable")
        })?;
    if !mailer.is_enabled() {
        return Err(errors::service_unavailable(
            "Configure an email provider before sending contact messages",
        ));
    }
    let snapshot = model(&source, &email);
    let rendered = transactional_templates::render_content(
        input.subject.trim(),
        &input.body_html,
        input.text_body.as_deref(),
        snapshot.clone(),
        &state.config.app_url,
        mailer.from_name(),
    )
    .map_err(|_| {
        ApiError::validation_error(
            "Message has unsupported contact variables, invalid syntax or excessive output",
        )
    })?;
    if rendered.html.trim().is_empty()
        && rendered
            .text
            .as_ref()
            .is_none_or(|text| text.trim().is_empty())
    {
        return Err(ApiError::validation_error(
            "Rendered contact message is empty",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query(
        "SELECT pg_advisory_xact_lock(hashtextextended('crm-contact-mail:'||$1::uuid::text,0))",
    )
    .bind(input.request_id)
    .execute(&mut *tx)
    .await
    .map_err(activity::db)?;
    let replay: Option<Intent> = sqlx::query_as(intent_sql!())
        .bind(input.request_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(activity::db)?;
    if let Some(saved) = replay {
        tx.rollback().await.map_err(activity::db)?;
        same(&saved, id, admin.0.id, &email, &hash)?;
        return response(&state.db.pool, &saved).await;
    }
    let current = contact(&mut tx, id, true).await?;
    if current["email"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_lowercase()
        != email
        || model(&current, &email) != snapshot
    {
        return Err(errors::conflict(
            "Contact changed while preparing the message; reload and review before submitting",
        ));
    }
    if let Some(template) = template {
        let exists: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM email_templates WHERE id=$1 AND is_active=true FOR KEY SHARE",
        )
        .bind(template)
        .fetch_optional(&mut *tx)
        .await
        .map_err(activity::db)?;
        if exists.is_none() {
            return Err(ApiError::validation_error(
                "Choose an active native email template or clear its origin reference",
            ));
        }
    }
    let content = json!({"source_subject":input.subject.trim(),"subject":rendered.subject,"html":rendered.html,"text":rendered.text,"recipient_model":snapshot,"provider":mailer.provider(),"sender":{"email":mailer.from_email(),"name":mailer.from_name()}});
    sqlx::query("INSERT INTO crm_contact_mail_requests(request_id,contact_id,original_contact_id,created_by,recipient,subject,source_hash,content_snapshot,template_id,original_template_id) VALUES($1,$2,$2,$3,$4,$5,$6,$7,$8,$8)")
        .bind(input.request_id).bind(id).bind(admin.0.id).bind(&email).bind(&rendered.subject).bind(&hash).bind(content).bind(template).execute(&mut *tx).await.map_err(activity::db)?;
    let log_metadata = json!({"request_id":input.request_id,"contact_id":id,"actor_id":admin.0.id,"source_hash":hash});
    let log:i64=sqlx::query_scalar("INSERT INTO email_logs(email,to_email,user_id,subject,template_type,template_alias,model,metadata,email_template_id,status,provider,queued_at,created_at) VALUES($1,$1,$2,$3,'crm_contact_mail',$4,$5,$5,$6,'queued',$7,NOW(),NOW() AT TIME ZONE 'UTC') RETURNING id")
        .bind(&email).bind(current["user_id"].as_i64()).bind(&rendered.subject).bind(format!("contact-mail:{id}")).bind(log_metadata).bind(template).bind(mailer.provider()).fetch_one(&mut *tx).await.map_err(activity::db)?;
    sqlx::query("UPDATE crm_contact_mail_requests SET email_log_id=$2 WHERE request_id=$1")
        .bind(input.request_id)
        .bind(log)
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "email",
        "Personal contact message staged",
        json!({"contact_id":id,"request_id":input.request_id,"email_log_id":log}),
        Some(admin.0.id),
    )
    .await
    .map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    let current = state
        .services
        .email
        .resolved()
        .await
        .ok()
        .filter(|current| {
            current.is_enabled()
                && current.provider() == mailer.provider()
                && current.from_email() == mailer.from_email()
                && current.from_name() == mailer.from_name()
        });
    if let Some(current) = current {
        let outcome = current
            .submit_raw(
                &email,
                &rendered.subject,
                &rendered.html,
                rendered.text.as_deref(),
            )
            .await;
        match outcome {
            Ok(outcome) => {
                if let Err(error) = EmailService::record_outcome(&state.db.pool, log, outcome).await
                {
                    tracing::warn!(log,%error,"Contact message receipt is not confirmed");
                }
            }
            Err(error) => {
                sqlx::query("UPDATE email_logs SET status='unavailable',error=$2,error_message=$2 WHERE id=$1 AND status='queued'").bind(log).bind(error.to_string()).execute(&state.db.pool).await.map_err(activity::db)?;
            }
        }
    } else {
        sqlx::query("UPDATE email_logs SET status='unavailable',error='Provider configuration changed before submission',error_message='Provider configuration changed before submission' WHERE id=$1 AND status='queued'").bind(log).execute(&state.db.pool).await.map_err(activity::db)?;
    }
    let saved = intent(&state.db.pool, input.request_id)
        .await?
        .ok_or_else(|| {
            errors::service_unavailable(
                "Contact message intent is unavailable; reconcile before resubmitting",
            )
        })?;
    response(&state.db.pool, &saved).await
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct History {
    page: Option<i64>,
    per_page: Option<i64>,
    request_id: Option<Uuid>,
}
macro_rules! history_sql{($tail:literal)=>{concat!(r#"WITH messages AS (
 SELECT 'personal:'||r.request_id AS id,r.original_contact_id::text AS contact_id,r.request_id,COALESCE(e.subject,r.subject) AS subject,
 CASE WHEN e.id IS NULL THEN 'unknown' WHEN e.status IN ('sent','delivered','opened','clicked','bounced') AND NULLIF(e.provider_message_id,'') IS NOT NULL AND e.sent_at IS NOT NULL THEN 'accepted' WHEN e.status='failed' THEN 'rejected' WHEN e.status='unavailable' THEN 'unavailable' WHEN e.status='queued' AND e.queued_at>NOW()-INTERVAL '90 seconds' THEN 'processing' ELSE 'unknown' END AS status,
 e.provider,e.provider_message_id,e.queued_at,e.sent_at,e.sent_at AS accepted_at,e.opened_at AT TIME ZONE 'UTC' AS opened_at,e.clicked_at AT TIME ZONE 'UTC' AS clicked_at,e.bounced_at AT TIME ZONE 'UTC' AS bounced_at,
 CASE WHEN e.id IS NULL THEN 'Message receipt is incomplete; reconciliation is required' ELSE COALESCE(e.error,e.error_message) END AS error,NULL::text AS campaign_name,'contact_message'::text AS scope,r.created_at AS occurred_at,r.content_snapshot->>'source_subject' AS source_subject,r.source_hash,r.original_template_id::text AS template_id,r.recipient,e.id::text AS receipt_id
 FROM crm_contact_mail_requests r LEFT JOIN email_logs e ON e.id=r.email_log_id WHERE r.original_contact_id=$1 AND ($2::uuid IS NULL OR r.request_id=$2)
 UNION ALL SELECT 'delivery:'||d.id,$1::text,NULL::uuid,COALESCE(d.rendered_subject,d.subject_template,rev.definition->>'email_subject',c.subject),CASE d.status WHEN 'accepted' THEN CASE WHEN d.accepted_at IS NOT NULL AND NULLIF(d.provider_message_id,'') IS NOT NULL THEN 'accepted' ELSE 'unknown' END WHEN 'failed' THEN 'rejected' WHEN 'unknown' THEN 'unknown' ELSE d.status END,d.provider,d.provider_message_id,d.created_at,d.accepted_at,d.accepted_at,d.opened_at,d.clicked_at,d.bounced_at,d.last_error,COALESCE(c.title,c.name),'crm_outbox',COALESCE(d.accepted_at,d.created_at),NULL::text,NULL::text,NULL::text,d.email,d.id::text
 FROM crm_campaign_deliveries d JOIN crm_campaigns c ON c.id=d.campaign_id JOIN crm_campaign_revisions rev ON rev.id=d.revision_id WHERE d.contact_id=$1 AND $2::uuid IS NULL
 UNION ALL SELECT 'member-email:'||e.id,$1::text,NULL::uuid,e.subject,CASE WHEN e.status IN ('sent','delivered','opened','clicked','bounced') AND NULLIF(e.provider_message_id,'') IS NOT NULL AND e.sent_at IS NOT NULL THEN 'accepted' WHEN e.status='failed' THEN 'rejected' WHEN e.status='unavailable' THEN 'unavailable' WHEN e.status='queued' THEN 'queued' ELSE 'unknown' END,e.provider,e.provider_message_id,e.queued_at,e.sent_at,e.sent_at,e.opened_at AT TIME ZONE 'UTC',e.clicked_at AT TIME ZONE 'UTC',e.bounced_at AT TIME ZONE 'UTC',COALESCE(e.error,e.error_message),NULL,'linked_member_email_log',COALESCE(e.sent_at,e.queued_at,e.created_at AT TIME ZONE 'UTC'),NULL::text,NULL::text,e.email_template_id::text,COALESCE(e.to_email,e.email),e.id::text
 FROM email_logs e JOIN contacts c ON c.user_id=e.user_id WHERE c.id=$1 AND $2::uuid IS NULL AND COALESCE(e.template_type,'') NOT IN ('crm_contact_mail','sequence_test','native_campaign_test')
) "#,$tail)}}
async fn emails(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<History>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = Page {
        page: query.page,
        per_page: query.per_page,
    }
    .bounds()?;
    if query.request_id.is_some_and(|id| id.is_nil()) {
        return Err(ApiError::validation_error("Provide a nonzero request UUID"));
    }
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM contacts WHERE id=$1) OR EXISTS(SELECT 1 FROM crm_contact_mail_requests WHERE original_contact_id=$1)").bind(id).fetch_one(&mut *tx).await.map_err(activity::db)?;
    if !exists {
        return Err(errors::not_found("Contact"));
    }
    let total: i64 = sqlx::query_scalar(history_sql!("SELECT COUNT(*) FROM messages"))
        .bind(id)
        .bind(query.request_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(activity::db)?;
    let rows:Vec<Value>=sqlx::query_scalar(history_sql!("SELECT to_jsonb(m) FROM messages m ORDER BY occurred_at DESC NULLS LAST,id DESC LIMIT $3 OFFSET $4")).bind(id).bind(query.request_id).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(super::envelope(id, rows, page, size, total))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/contacts/{id}/send-email", post(send))
        .route("/contacts/{id}/emails", get(emails))
}
