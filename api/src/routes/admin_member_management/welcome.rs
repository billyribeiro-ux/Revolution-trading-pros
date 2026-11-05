//! The existing queue owns one frozen admin welcome intent and one actual email receipt.
use crate::{
    AppState,
    services::{email::EmailService, transactional_templates},
    utils::errors::{self, ApiError},
};
use serde_json::{Value, json};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Member welcome persistence failed");
    errors::service_unavailable("Welcome receipt storage is unavailable")
}
pub(super) async fn queue_on(
    connection: &mut PgConnection,
    state: &AppState,
    member: i64,
    request: Uuid,
    name: &str,
    email: &str,
) -> Result<Value, ApiError> {
    let url = reqwest::Url::parse(&state.config.app_url).map_err(|_| {
        errors::service_unavailable("The application URL is not configured for a welcome message")
    })?;
    if !matches!(url.scheme(), "https" | "http")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(errors::service_unavailable(
            "The application URL is not configured for a welcome message",
        ));
    }
    // A welcome to a newly created, unverified account never says email verification succeeded.
    let rendered=transactional_templates::render_content("Welcome to Revolution Trading Pros","<p>Hi {{name}},</p><p>An administrator created your account.</p><p><a href=\"{{app_url}}/login\">Sign in to your account</a>.</p><p>Use the password-reset option if you need to choose a password. This message does not verify your email address.</p>",Some("Hi {{name}},\n\nAn administrator created your account. Sign in at {{app_url}}/login. Use the password-reset option if you need to choose a password. This message does not verify your email address."),json!({"name":name}),state.config.app_url.trim_end_matches('/'),"Revolution Trading Pros").map_err(|_|errors::service_unavailable("Welcome content cannot be rendered"))?;
    let content =
        json!({"subject":rendered.subject,"body_html":rendered.html,"text_body":rendered.text});
    let log:i64=sqlx::query_scalar("INSERT INTO email_logs(user_id,email,to_email,subject,template_type,model,status,provider,queued_at,created_at) VALUES($1,$2,$2,$3,'admin_member_welcome',$4,'queued','configuration',NOW(),NOW() AT TIME ZONE 'UTC') RETURNING id")
        .bind(member).bind(email).bind(&rendered.subject).bind(json!({"request_id":request,"member_id":member})).fetch_one(&mut *connection).await.map_err(db)?;
    let job = Uuid::new_v4();
    sqlx::query("INSERT INTO jobs(id,queue,job_type,payload,status,attempts,max_attempts,available_at,created_at) VALUES($1,'emails','member_welcome',$2,'pending',0,1,NOW(),NOW())").bind(job).bind(json!({"member_id":member,"request_id":request})).execute(&mut *connection).await.map_err(db)?;
    sqlx::query("INSERT INTO member_welcome_requests(user_id,request_id,email_log_id,job_id,recipient,content_snapshot) VALUES($1,$2,$3,$4,$5,$6)").bind(member).bind(request).bind(log).bind(job).bind(email).bind(content).execute(&mut *connection).await.map_err(db)?;
    state_on(connection, member).await
}
pub(super) async fn state_on(
    connection: &mut PgConnection,
    member: i64,
) -> Result<Value, ApiError> {
    let row:Option<Value>=sqlx::query_scalar("SELECT jsonb_build_object('requested',true,'request_id',w.request_id,'job_id',w.job_id,'email_log_id',w.email_log_id,'recipient',w.recipient,'subject',w.content_snapshot->>'subject','status',CASE WHEN e.id IS NULL THEN 'unavailable' WHEN e.status IN('sent','delivered','opened','clicked','bounced') AND NULLIF(BTRIM(e.provider_message_id),'') IS NOT NULL AND e.sent_at IS NOT NULL THEN 'accepted' WHEN e.status='failed' THEN 'rejected' WHEN e.status='unavailable' THEN 'unavailable' WHEN j.id IS NULL THEN 'unavailable' WHEN j.status='failed' THEN 'unknown' WHEN e.status='queued' AND w.claimed_at IS NULL THEN 'queued' WHEN e.status='queued' AND w.claimed_at>NOW()-INTERVAL '90 seconds' THEN 'processing' ELSE 'unknown' END,'provider_message_id',e.provider_message_id,'accepted_at',to_char(e.sent_at AT TIME ZONE 'UTC','YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"'),'error',COALESCE(e.error,e.error_message),'job_status',j.status) FROM member_welcome_requests w LEFT JOIN email_logs e ON e.id=w.email_log_id LEFT JOIN jobs j ON j.id=w.job_id WHERE w.user_id=$1")
        .bind(member).fetch_optional(connection).await.map_err(db)?;
    Ok(row.unwrap_or_else(|| json!({"requested":false,"status":"not_requested"})))
}
/// Called once by the existing queue's member_welcome handler, never by generic raw send.
pub(crate) async fn deliver(pool: &PgPool, job: Uuid) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let row:Option<(i64,Option<i64>,String,Value,bool)>=sqlx::query_as("SELECT w.user_id,w.email_log_id,w.recipient,w.content_snapshot,u.deleted_at IS NULL AND COALESCE(LOWER(u.role),'')<>'deleted' AND u.is_active IS DISTINCT FROM false AND LOWER(u.email)=LOWER(w.recipient) FROM member_welcome_requests w JOIN users u ON u.id=w.user_id WHERE w.job_id=$1 AND w.claimed_at IS NULL FOR UPDATE OF w")
        .bind(job).fetch_optional(&mut *tx).await?;
    let Some((member, Some(log), recipient, content, permitted)) = row else {
        anyhow::bail!(
            "Welcome intent is missing, already claimed, or has no recorded receipt; reconcile without resubmitting"
        )
    };
    let status:Option<String>=sqlx::query_scalar("SELECT status FROM email_logs WHERE id=$1 AND user_id=$2 AND template_type='admin_member_welcome' FOR UPDATE").bind(log).bind(member).fetch_optional(&mut *tx).await?;
    if status.as_deref() != Some("queued") {
        anyhow::bail!("Welcome receipt already has an outcome or is missing; do not resubmit")
    }
    sqlx::query("UPDATE member_welcome_requests SET claimed_at=NOW() WHERE job_id=$1")
        .bind(job)
        .execute(&mut *tx)
        .await?;
    if !permitted {
        sqlx::query("UPDATE email_logs SET status='unavailable',error='Account is unavailable or its recorded email changed',error_message='Account is unavailable or its recorded email changed' WHERE id=$1").bind(log).execute(&mut *tx).await?;
        tx.commit().await?;
        anyhow::bail!("Account changed before welcome submission")
    }
    tx.commit().await?;
    let config = crate::config::Config::from_env()?;
    let sender = EmailService::from_env(&config.app_url).with_runtime(pool.clone(), &config);
    let service = match sender.resolved().await {
        Ok(service) => service,
        Err(_) => {
            sqlx::query("UPDATE email_logs SET status='unavailable',error='Email provider is not configured',error_message='Email provider is not configured' WHERE id=$1").bind(log).execute(pool).await?;
            anyhow::bail!("Email provider is not configured")
        }
    };
    sqlx::query("UPDATE email_logs SET provider=$2 WHERE id=$1")
        .bind(log)
        .bind(service.provider())
        .execute(pool)
        .await?;
    let submitted = service
        .submit_raw(
            &recipient,
            content["subject"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Frozen welcome subject is missing"))?,
            content["body_html"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Frozen welcome body is missing"))?,
            content["text_body"].as_str(),
        )
        .await;
    let outcome = match submitted {
        Ok(outcome) => outcome,
        Err(_) => {
            sqlx::query("UPDATE email_logs SET status='unavailable',error='Provider preflight could not submit the message',error_message='Provider preflight could not submit the message' WHERE id=$1").bind(log).execute(pool).await?;
            anyhow::bail!("Provider preflight could not submit the message")
        }
    };
    let written = EmailService::record_outcome(pool, log, outcome).await;
    if let Err(error) = &written
        && error
            .downcast_ref::<crate::services::campaign_mailer::UncertainSubmission>()
            .is_some()
    {
        // A recoverable local ACK-write failure still cannot become a second submission.
        // Preserve the unknown outcome in the shared linked receipt whenever storage permits.
        sqlx::query("UPDATE email_logs SET status='delivery_unknown',error='Provider outcome could not be confirmed durably; reconcile without resubmitting',error_message='Provider outcome could not be confirmed durably; reconcile without resubmitting' WHERE id=$1 AND status='queued'").bind(log).execute(pool).await?;
    }
    written.map(|_| ())
}
