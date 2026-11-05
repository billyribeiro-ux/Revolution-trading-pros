//! Explicit QA samples: commit intent before submission and never resend a replay.
use super::{Config, db, invalid, row};
use crate::{
    AppState,
    middleware::admin::AdminUser,
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
use futures::{StreamExt, stream};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Input {
    email: Option<String>,
    emails: Option<Vec<String>>,
    request_id: Option<Uuid>,
    expected_version: Option<i64>,
    variant: Option<String>,
}
impl Input {
    fn recipients(&self) -> Result<Vec<String>, ApiError> {
        let values = match (&self.email, &self.emails) {
            (Some(email), None) => vec![email.clone()],
            (None, Some(emails)) => emails.clone(),
            _ => {
                return Err(invalid(
                    "Provide email or emails, with one to five test recipients",
                ));
            }
        };
        if !(1..=5).contains(&values.len()) {
            return Err(invalid("Provide one to five test recipients"));
        }
        let mut normalized = Vec::with_capacity(values.len());
        for email in values {
            let email = email.trim().to_lowercase();
            super::text(&email, 255, true, true)?;
            errors::validate_email(&email).map_err(|_| invalid("Provide valid test recipients"))?;
            if normalized.contains(&email) {
                return Err(invalid("Test recipients must be distinct"));
            }
            normalized.push(email);
        }
        normalized.sort();
        Ok(normalized)
    }
}
#[derive(sqlx::FromRow)]
struct Intent {
    request_id: Uuid,
    original_campaign_id: i64,
    created_by: Option<i64>,
    authoring_version: i64,
    recipients: Vec<String>,
    variant: String,
    created_at: DateTime<Utc>,
}
async fn intent(pool: &PgPool, id: Uuid) -> Result<Option<Intent>, ApiError> {
    sqlx::query_as("SELECT request_id,original_campaign_id,created_by,authoring_version,recipients,variant,created_at FROM email_campaign_test_requests WHERE request_id=$1")
        .bind(id).fetch_optional(pool).await.map_err(db)
}
fn same_intent(
    saved: &Intent,
    campaign: i64,
    actor: i64,
    recipients: &[String],
    variant: &str,
    version: Option<i64>,
) -> Result<(), ApiError> {
    if saved.original_campaign_id != campaign
        || saved.created_by != Some(actor)
        || saved.recipients != recipients
        || saved.variant != variant
        || version.is_some_and(|version| version != saved.authoring_version)
    {
        return Err(errors::conflict(
            "This test request ID already belongs to a different intent",
        ));
    }
    Ok(())
}
async fn receipt(pool: &PgPool, saved: &Intent) -> Result<Value, ApiError> {
    let alias = format!("native-campaign-test:{}", saved.original_campaign_id);
    let submissions: Vec<Value> = sqlx::query_scalar("SELECT jsonb_build_object('id',id,'recipient',to_email,'provider',provider,'provider_message_id',provider_message_id,'status',CASE WHEN status IN ('sent','delivered','opened','clicked','bounced') AND NULLIF(provider_message_id,'') IS NOT NULL THEN 'accepted' WHEN status='failed' THEN 'rejected' WHEN status='unavailable' THEN 'unavailable' WHEN status='queued' AND queued_at>NOW()-INTERVAL '90 seconds' THEN 'processing' ELSE 'unknown' END,'error',COALESCE(error,error_message),'queued_at',queued_at,'accepted_at',sent_at) FROM email_logs WHERE template_type='native_campaign_test' AND template_alias=$1 AND model->>'request_id'=$2 ORDER BY to_email,id")
        .bind(alias).bind(saved.request_id.to_string()).fetch_all(pool).await.map_err(db)?;
    if submissions.len() != saved.recipients.len()
        || !submissions
            .iter()
            .zip(&saved.recipients)
            .all(|(receipt, email)| receipt["recipient"] == *email)
    {
        return Err(errors::service_unavailable(
            "Test receipts are incomplete; reconcile this request before sending another sample",
        ));
    }
    Ok(
        json!({"campaign_id":saved.original_campaign_id,"metrics_scope":"native_campaign_test","request_id":saved.request_id,"authoring_version":saved.authoring_version,"variant":saved.variant,"created_at":saved.created_at,"submissions":submissions}),
    )
}
async fn response(pool: &PgPool, saved: &Intent) -> Result<(StatusCode, Json<Value>), ApiError> {
    let data = receipt(pool, saved).await?;
    let submissions = data["submissions"]
        .as_array()
        .ok_or_else(|| errors::service_unavailable("Test receipts are unavailable"))?;
    let status = if submissions
        .iter()
        .all(|value| value["status"] == "accepted")
    {
        StatusCode::OK
    } else if submissions
        .iter()
        .any(|value| value["status"] == "unknown" || value["status"] == "rejected")
    {
        StatusCode::BAD_GATEWAY
    } else if submissions
        .iter()
        .any(|value| value["status"] == "unavailable")
    {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        StatusCode::ACCEPTED
    };
    Ok((
        status,
        Json(json!({"success":status==StatusCode::OK,"data":data})),
    ))
}
#[derive(sqlx::FromRow)]
struct Source {
    subject: String,
    html_content: Option<String>,
    text_content: Option<String>,
    template_id: Option<i64>,
    authoring_config: Value,
    authoring_version: i64,
}
struct Prepared {
    recipient: String,
    message: Message,
}
async fn submit(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let recipients = input.recipients()?;
    let variant = input.variant.as_deref().unwrap_or("a");
    if !matches!(variant, "a" | "b")
        || input.expected_version.is_some_and(|value| value < 1)
        || input.request_id.is_some_and(|value| value.is_nil())
    {
        return Err(invalid(
            "Provide variant a or b, a positive version and a nonzero request UUID",
        ));
    }
    let request_id = input.request_id.unwrap_or_else(Uuid::new_v4);
    // Resolve a replay before configuration or creative reads. Revocation or a
    // later edit cannot turn an existing request into another submission.
    if let Some(saved) = intent(&state.db.pool, request_id).await? {
        same_intent(
            &saved,
            id,
            admin.0.id,
            &recipients,
            variant,
            input.expected_version,
        )?;
        return response(&state.db.pool, &saved).await;
    }
    let source: Source = sqlx::query_as("SELECT subject,html_content,text_content,template_id,authoring_config,authoring_version FROM email_campaigns WHERE id=$1")
        .bind(id).fetch_optional(&state.db.pool).await.map_err(db)?.ok_or_else(||ApiError::not_found("Campaign"))?;
    if input
        .expected_version
        .is_some_and(|version| version != source.authoring_version)
    {
        return Err(errors::conflict(
            "Campaign changed; reload before sending a test",
        ));
    }
    let config: Config = serde_json::from_value(source.authoring_config.clone())
        .map_err(|_| errors::service_unavailable("Stored campaign configuration is invalid"))?;
    config.validate()?;
    if !config.from_name.is_empty() && config.from_email.is_empty() {
        return Err(invalid("Specify a sender email when overriding its name"));
    }
    let subject = if variant == "b" {
        config
            .ab_test_config
            .as_ref()
            .filter(|ab| ab.enabled)
            .and_then(|ab| ab.subject_b.as_deref())
            .ok_or_else(|| invalid("Subject B requires an enabled A/B test"))?
    } else {
        &source.subject
    };
    let html = source.html_content.as_deref().unwrap_or("");
    let plain = html.trim().is_empty();
    let body = if plain {
        source.text_content.as_deref().unwrap_or("")
    } else {
        html
    };
    if body.trim().is_empty() || subject.trim().is_empty() {
        return Err(invalid(
            "Provide saved campaign subject and content before sending a test",
        ));
    }
    let mailer = CampaignMailer::from_state(&state).await.map_err(|error| {
        tracing::warn!(%error,"Campaign test mailer unavailable");
        errors::service_unavailable("Configure a broadcast email provider before sending a test")
    })?;
    let provider = mailer.provider().to_string();
    let sender = mailer.sender().clone();
    let delivery_stream = mailer.stream().to_string();
    let unsubscribe_url = format!(
        "{}/newsletter/unsubscribe?preview=campaign-test",
        state.config.app_url.trim_end_matches('/')
    );
    let definition = json!({"native_email_campaign_id":id,"type":if plain {"plain_text"} else {"raw_html"},"email_body":body,"text_body":source.text_content});
    let mut prepared = Vec::with_capacity(recipients.len());
    for email in &recipients {
        let name: Option<String> =
            sqlx::query_scalar("SELECT name FROM users WHERE LOWER(email)=$1 ORDER BY id LIMIT 1")
                .bind(email)
                .fetch_optional(&state.db.pool)
                .await
                .map_err(db)?;
        let name = name.as_deref().unwrap_or(email);
        let snapshot = json!({"first_name":"","last_name":"","company":""});
        let resolved = campaign_personalization::render(
            &definition,
            Recipient {
                email,
                name,
                snapshot: &snapshot,
                app_url: &state.config.app_url,
                unsubscribe_url: &unsubscribe_url,
                brand: &sender.name,
            },
            subject,
        )
        .map_err(|_| {
            invalid(
                "Campaign template has unsupported variables, invalid syntax or excessive output",
            )
        })?;
        let subject = format!(
            "[Test] {}",
            resolved["email_subject"].as_str().unwrap_or("")
        );
        let notice =
            "Test message. Unsubscribe actions are inactive; no campaign audience was submitted.";
        let content = resolved["email_body"].as_str().unwrap_or("");
        let body = if plain {
            format!("{notice}\n\n{content}")
        } else {
            format!("<p>{notice}</p>{content}")
        };
        let text_body = (!plain)
            .then(|| {
                resolved["text_body"]
                    .as_str()
                    .map(|text| format!("{notice}\n\n{text}"))
            })
            .flatten();
        let message = Message {
            to: email.clone(),
            subject,
            body,
            plain_text: plain,
            text_body,
            from: (!config.from_email.is_empty()).then(|| Identity {
                name: config.from_name.clone(),
                email: config.from_email.clone(),
            }),
            reply_to: config.reply_to.as_ref().map(|email| Identity {
                name: String::new(),
                email: email.clone(),
            }),
            unsubscribe_url: unsubscribe_url.clone(),
            metadata: BTreeMap::from([
                ("native_campaign_test".into(), id.to_string()),
                ("test_request_id".into(), request_id.to_string()),
                ("variant".into(), variant.into()),
            ]),
        };
        mailer
            .validate(&message)
            .map_err(|error| invalid(&error.to_string()))?;
        prepared.push(Prepared {
            recipient: email.clone(),
            message,
        });
    }
    let source_hash = hex::encode(Sha256::digest(serde_json::to_vec(&json!({"subject":subject,"body":body,"text":source.text_content,"config":source.authoring_config})).map_err(|_|invalid("Campaign content is invalid"))?));
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let version: Option<i64> =
        sqlx::query_scalar("SELECT authoring_version FROM email_campaigns WHERE id=$1 FOR SHARE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?;
    let version = version.ok_or_else(|| ApiError::not_found("Campaign"))?;
    let inserted = sqlx::query("INSERT INTO email_campaign_test_requests(request_id,campaign_id,original_campaign_id,created_by,authoring_version,recipients,variant,source_hash) VALUES($1,$2,$2,$3,$4,$5,$6,$7) ON CONFLICT(request_id) DO NOTHING")
        .bind(request_id).bind(id).bind(admin.0.id).bind(source.authoring_version).bind(&recipients).bind(variant).bind(source_hash).execute(&mut *tx).await.map_err(db)?.rows_affected()==1;
    if !inserted {
        tx.rollback().await.map_err(db)?;
        let saved = intent(&state.db.pool, request_id)
            .await?
            .ok_or_else(|| errors::service_unavailable("Test request is unavailable"))?;
        same_intent(
            &saved,
            id,
            admin.0.id,
            &recipients,
            variant,
            input.expected_version,
        )?;
        return response(&state.db.pool, &saved).await;
    }
    if version != source.authoring_version {
        return Err(errors::conflict(
            "Campaign changed while preparing this test; reload before submitting",
        ));
    }
    let alias = format!("native-campaign-test:{id}");
    let mut submissions = Vec::with_capacity(prepared.len());
    for sample in prepared {
        let model = json!({"request_id":request_id,"authoring_version":version,"variant":variant,"actor_id":admin.0.id});
        let receipt: i64 = sqlx::query_scalar("INSERT INTO email_logs(email,to_email,subject,template_type,template_alias,model,metadata,status,provider,queued_at,created_at,email_template_id) VALUES($1,$1,$2,'native_campaign_test',$3,$4,$4,'queued',$5,NOW(),NOW() AT TIME ZONE 'UTC',$6) RETURNING id")
            .bind(&sample.recipient).bind(&sample.message.subject).bind(&alias).bind(model).bind(&provider).bind(source.template_id).fetch_one(&mut *tx).await.map_err(db)?;
        submissions.push((receipt, sample.message));
    }
    super::record(&mut tx, admin.0.id, id, "Email campaign test staged").await?;
    tx.commit().await.map_err(db)?;
    stream::iter(submissions).map(|(receipt,message)| {
        let state=&state;
        let provider=&provider;
        let sender=&sender;
        let delivery_stream=&delivery_stream;
        async move {
            // Honor revocation after staging too. No stored intent is retried.
            let current = CampaignMailer::from_state(state).await;
            let current = current.ok().filter(|current|current.provider()==provider && current.stream()==delivery_stream && current.sender().email==sender.email && current.sender().name==sender.name);
            if let Some(current)=current {
                let outcome = current.submit(message).await;
                if let Err(error)=EmailService::record_outcome(&state.db.pool,receipt,outcome).await { tracing::warn!(receipt,%error,"Campaign test submission not confirmed"); }
            } else if let Err(error)=sqlx::query("UPDATE email_logs SET status='unavailable',error='Provider configuration changed before submission',error_message='Provider configuration changed before submission' WHERE id=$1 AND status='queued'").bind(receipt).execute(&state.db.pool).await {
                tracing::error!(receipt,%error,"Campaign test configuration failure receipt could not be persisted");
            }
        }
    }).buffer_unordered(5).collect::<Vec<_>>().await;
    let saved = intent(&state.db.pool, request_id).await?.ok_or_else(|| {
        errors::service_unavailable("Test request receipt unavailable; reconcile before retrying")
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
    _: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<History>,
) -> Result<Json<Value>, ApiError> {
    row(&state.db.pool, id).await?;
    let requests:Vec<Intent>=sqlx::query_as("SELECT request_id,original_campaign_id,created_by,authoring_version,recipients,variant,created_at FROM email_campaign_test_requests WHERE original_campaign_id=$1 AND ($2::uuid IS NULL OR request_id=$2) ORDER BY created_at DESC,request_id DESC LIMIT 20")
        .bind(id).bind(query.request_id).fetch_all(&state.db.pool).await.map_err(db)?;
    let mut data = Vec::with_capacity(requests.len());
    for saved in requests {
        data.push(receipt(&state.db.pool, &saved).await?);
    }
    Ok(Json(
        json!({"success":true,"data":{"campaign_id":id,"metrics_scope":"native_campaign_test","requests":data}}),
    ))
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{id}/test", post(submit))
        .route("/{id}/tests", get(history))
}
