//! Durable broadcast delivery. A committed lease precedes every provider call.
use super::campaign_mailer::{CampaignMailer, Identity, Message, Submission};
use anyhow::{Result, anyhow};
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::BTreeMap;
use uuid::Uuid;

/// Call while holding the campaign row lock. Counters describe provider facts,
/// never assumed inbox delivery or fabricated tracking activity.
pub async fn refresh(tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<()> {
    sqlx::query("UPDATE crm_campaigns c SET status=CASE WHEN EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=c.id AND status IN ('queued','processing')) THEN 'paused' WHEN EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=c.id AND status='failed') THEN 'failed' ELSE 'sent' END,pause_reason=CASE WHEN EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=c.id AND status IN ('queued','processing')) THEN 'manual' ELSE NULL END,last_error=NULL WHERE c.id=$1 AND c.pause_reason='uncertain_submission' AND NOT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=c.id AND status='unknown')").bind(id).execute(&mut **tx).await?;
    sqlx::query("WITH counts AS (SELECT COUNT(*)::bigint total,COUNT(*) FILTER(WHERE status='queued') queued,COUNT(*) FILTER(WHERE status='processing') processing,COUNT(*) FILTER(WHERE status='accepted') accepted,COUNT(*) FILTER(WHERE status='failed') failed,COUNT(*) FILTER(WHERE status='unknown') unknown,COUNT(*) FILTER(WHERE status='skipped') skipped,COUNT(*) FILTER(WHERE delivered_at IS NOT NULL) delivered,COUNT(*) FILTER(WHERE opened_at IS NOT NULL) opened,COUNT(*) FILTER(WHERE clicked_at IS NOT NULL) clicked,COUNT(*) FILTER(WHERE bounced_at IS NOT NULL) bounced,COUNT(*) FILTER(WHERE complained_at IS NOT NULL) complained,COUNT(*) FILTER(WHERE unsubscribed_at IS NOT NULL) unsubscribed FROM crm_campaign_deliveries WHERE campaign_id=$1) UPDATE crm_campaigns c SET recipients_count=n.total,emails_sent=n.accepted,failed_count=n.failed,opens=n.opened,clicks=n.clicked,open_rate=CASE WHEN n.accepted>0 THEN 100.0*n.opened/n.accepted ELSE 0 END,click_rate=CASE WHEN n.accepted>0 THEN 100.0*n.clicked/n.accepted ELSE 0 END,delivery_stats=to_jsonb(n),status=CASE WHEN n.unknown>0 AND c.status IN ('sending','scheduled') THEN 'paused' WHEN n.queued+n.processing=0 AND c.status IN ('sending','scheduled') THEN CASE WHEN n.failed>0 THEN 'failed' ELSE 'sent' END ELSE c.status END,pause_reason=CASE WHEN n.unknown>0 AND c.status IN ('sending','scheduled') THEN 'uncertain_submission' ELSE c.pause_reason END,last_error=CASE WHEN n.unknown>0 THEN 'A submission outcome is unknown; provider reconciliation is required before retrying' ELSE c.last_error END,sent_at=CASE WHEN n.queued+n.processing+n.unknown=0 AND n.accepted>0 THEN COALESCE(c.sent_at,NOW()) ELSE c.sent_at END,updated_at=NOW() FROM counts n WHERE c.id=$1").bind(id).execute(&mut **tx).await?;
    Ok(())
}
#[derive(sqlx::FromRow)]
struct Claimed {
    id: Uuid,
    campaign_id: i64,
    email: String,
    unsubscribe_token: Uuid,
    definition: Value,
    lease: Uuid,
}

async fn claim(pool: &PgPool) -> Result<Option<Claimed>> {
    let mut tx = pool.begin().await?;
    // Lock campaign before delivery everywhere, including lease recovery.
    let campaign:Option<i64>=sqlx::query_scalar("SELECT c.id FROM crm_campaigns c WHERE EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE d.campaign_id=c.id AND ((d.status='processing' AND d.lease_expires_at<NOW()) OR (d.status='queued' AND d.due_at<=NOW() AND c.status IN ('scheduled','sending') AND (c.next_delivery_at IS NULL OR c.next_delivery_at<=NOW())))) ORDER BY c.scheduled_at NULLS FIRST,c.id FOR UPDATE SKIP LOCKED LIMIT 1").fetch_optional(&mut *tx).await?;
    let Some(campaign) = campaign else {
        return Ok(None);
    };
    let expired=sqlx::query("UPDATE crm_campaign_deliveries SET status='unknown',last_error='Worker lease expired after a possible provider submission; reconcile before retrying',lease_token=NULL,lease_expires_at=NULL,updated_at=NOW() WHERE campaign_id=$1 AND status='processing' AND lease_expires_at<NOW()").bind(campaign).execute(&mut *tx).await?;
    if expired.rows_affected() > 0 {
        refresh(&mut tx, campaign).await?;
        tx.commit().await?;
        return Ok(None);
    }
    let next:Option<Uuid>=sqlx::query_scalar("SELECT d.id FROM crm_campaign_deliveries d JOIN crm_campaigns c ON c.id=d.campaign_id WHERE d.campaign_id=$1 AND d.status='queued' AND d.due_at<=NOW() AND c.status IN ('scheduled','sending') ORDER BY d.due_at,d.id FOR UPDATE OF d SKIP LOCKED LIMIT 1").bind(campaign).fetch_optional(&mut *tx).await?;
    let Some(next) = next else {
        tx.commit().await?;
        return Ok(None);
    };
    let eligible:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries d JOIN crm_campaign_revisions r ON r.id=d.revision_id WHERE d.id=$1 AND EXISTS(SELECT 1 FROM newsletter_subscribers n WHERE n.id=(SELECT latest.id FROM newsletter_subscribers latest WHERE LOWER(latest.email)=d.email ORDER BY latest.updated_at DESC NULLS LAST,latest.id DESC LIMIT 1) AND n.status='confirmed' AND n.confirmed_at IS NOT NULL AND n.unsubscribed_at IS NULL) AND (COALESCE(r.definition->'subscriber_settings'->>'members','false')<>'true' OR EXISTS(SELECT 1 FROM users u WHERE to_jsonb(u.id)=d.contact_snapshot->'member_id' AND u.deleted_at IS NULL AND LOWER(u.email)=d.email)))").bind(next).fetch_one(&mut *tx).await?;
    if !eligible {
        sqlx::query("UPDATE crm_campaign_deliveries SET status='skipped',last_error='Recipient is no longer confirmed or the audience member is no longer available',updated_at=NOW() WHERE id=$1").bind(next).execute(&mut *tx).await?;
        refresh(&mut tx, campaign).await?;
        tx.commit().await?;
        return Ok(None);
    }
    let lease = Uuid::new_v4();
    let claimed:Claimed=sqlx::query_as("UPDATE crm_campaign_deliveries d SET status='processing',attempt_count=attempt_count+1,attempts_remaining=attempts_remaining-1,lease_token=$2,lease_expires_at=NOW()+INTERVAL '2 minutes',updated_at=NOW() FROM crm_campaign_revisions r WHERE d.id=$1 AND r.id=d.revision_id RETURNING d.id,d.campaign_id,d.email,d.unsubscribe_token,r.definition,d.lease_token AS lease").bind(next).bind(lease).fetch_one(&mut *tx).await?;
    sqlx::query("UPDATE crm_campaigns SET status='sending',next_delivery_at=CASE WHEN delivery_rate_limit IS NOT NULL THEN NOW()+INTERVAL '60 seconds'/delivery_rate_limit ELSE next_delivery_at END,updated_at=NOW() WHERE id=$1")
        .bind(campaign)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(Some(claimed))
}
fn identity(settings: &Value, prefix: &str) -> Option<Identity> {
    let email = settings.get(format!("{prefix}_email"))?.as_str()?;
    if email.is_empty() {
        return None;
    }
    Some(Identity {
        email: email.into(),
        name: settings
            .get(format!("{prefix}_name"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .into(),
    })
}
pub(crate) fn callback_url(app_url: &str) -> Result<reqwest::Url> {
    let parsed = reqwest::Url::parse(app_url)?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
    {
        return Err(anyhow!(
            "Configure a valid APP_URL for campaign unsubscribe links"
        ));
    }
    Ok(parsed)
}
pub async fn process_one(pool: &PgPool, mailer: &CampaignMailer, app_url: &str) -> Result<bool> {
    // Validate the callback origin before claiming.
    let mut unsubscribe = callback_url(app_url)?;
    let Some(job) = claim(pool).await? else {
        return Ok(false);
    };
    unsubscribe.set_path("/newsletter/unsubscribe");
    unsubscribe.set_query(Some(&format!("token={}", job.unsubscribe_token)));
    unsubscribe.set_fragment(None);
    let settings = &job.definition["settings"]["mailer_settings"];
    let from = if settings["is_custom"].as_bool().unwrap_or(false) {
        identity(settings, "from")
    } else {
        None
    };
    let metadata = BTreeMap::from([
        ("campaign_id".into(), job.campaign_id.to_string()),
        ("delivery_id".into(), job.id.to_string()),
    ]);
    let sender_identity = from.as_ref().unwrap_or(mailer.sender()).formatted().ok();
    let owned=sqlx::query("UPDATE crm_campaign_deliveries SET sender_identity=$3,provider_stream=$4,provider=$5 WHERE id=$1 AND lease_token=$2 AND status='processing' AND lease_expires_at>NOW()").bind(job.id).bind(job.lease).bind(sender_identity).bind(mailer.stream()).bind(mailer.provider()).execute(pool).await?;
    if owned.rows_affected() != 1 {
        return Err(anyhow!(
            "Campaign lease changed before provider submission; no request was sent"
        ));
    }
    let body = super::campaign_render::render(&job.definition, app_url, unsubscribe.as_str());
    let outcome = if let Ok(body) = body {
        let plain = job.definition["type"] == "plain_text";
        let text = job.definition["text_body"]
            .as_str()
            .filter(|text| !text.is_empty())
            .map(|text| format!("{text}\n\nUnsubscribe: {}", unsubscribe.as_str()));
        let (body, text_body) = if mailer.provider() == "smtp" {
            super::campaign_tracking::prepare(
                pool,
                job.id,
                &body,
                text.as_deref(),
                plain,
                app_url,
                unsubscribe.as_str(),
            )
            .await?
        } else {
            (body, text)
        };
        mailer
            .submit(Message {
                to: job.email,
                subject: job.definition["email_subject"]
                    .as_str()
                    .unwrap_or("")
                    .into(),
                body,
                plain_text: plain,
                text_body,
                from,
                reply_to: identity(settings, "reply_to"),
                unsubscribe_url: unsubscribe.to_string(),
                metadata,
            })
            .await
    } else {
        Submission::Rejected {
            reason: "Unable to render campaign content".into(),
            retryable: false,
        }
    };
    let mut tx = pool.begin().await?;
    sqlx::query("SELECT id FROM crm_campaigns WHERE id=$1 FOR UPDATE")
        .bind(job.campaign_id)
        .fetch_one(&mut *tx)
        .await?;
    let (status, message_id, error, retry) = match outcome {
        Submission::Accepted { message_id } => ("accepted", Some(message_id), None, false),
        Submission::Rejected { reason, retryable } => ("failed", None, Some(reason), retryable),
        Submission::Uncertain { reason } => ("unknown", None, Some(reason), false),
    };
    let saved=sqlx::query("UPDATE crm_campaign_deliveries SET status=CASE WHEN $6 AND attempts_remaining>0 THEN 'queued' ELSE $3 END,provider_message_id=$4,last_error=$5,accepted_at=CASE WHEN $3='accepted' THEN NOW() ELSE accepted_at END,due_at=CASE WHEN $6 THEN NOW()+INTERVAL '1 minute'*attempt_count ELSE due_at END,lease_token=NULL,lease_expires_at=NULL,updated_at=NOW() WHERE id=$1 AND lease_token=$2 AND status='processing'").bind(job.id).bind(job.lease).bind(status).bind(message_id).bind(error).bind(retry).execute(&mut *tx).await?;
    if saved.rows_affected() != 1 {
        return Err(anyhow!(
            "Campaign receipt lease no longer matches; provider outcome needs reconciliation"
        ));
    }
    super::crm_logs::record(&mut tx,if status=="accepted"{"info"}else if status=="unknown" || retry{"warning"}else{"error"},"email","Campaign provider submission recorded",serde_json::json!({"campaign_id":job.campaign_id,"delivery_id":job.id,"outcome":status,"retry_requested":retry}),None).await?;
    refresh(&mut tx, job.campaign_id).await?;
    tx.commit().await?;
    Ok(true)
}
pub async fn run(state: crate::AppState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        if let Err(error) = recover_expired(&state.db.pool).await {
            tracing::error!(%error,"Unable to recover expired campaign leases");
            continue;
        }
        let configured = match callback_url(&state.config.app_url) {
            Ok(_) => CampaignMailer::from_state(&state).await,
            Err(error) => Err(error),
        };
        match configured {
            Ok(_) => {}
            Err(error) => {
                if let Err(db_error)=sqlx::query("UPDATE crm_campaigns SET status='paused',pause_reason='provider_unavailable',last_error=$1,updated_at=NOW() WHERE status IN ('scheduled','sending') AND scheduled_at<=NOW()").bind(error.to_string()).execute(&state.db.pool).await{tracing::error!(%db_error,"Unable to record unavailable campaign provider");}
                continue;
            }
        }
        // Configuration recovery resumes only provider-paused work, never manual or uncertain work.
        if let Err(error)=sqlx::query("UPDATE crm_campaigns SET status=CASE WHEN scheduled_at>NOW() THEN 'scheduled' ELSE 'sending' END,pause_reason=NULL,last_error=NULL,updated_at=NOW() WHERE status='paused' AND pause_reason='provider_unavailable'").execute(&state.db.pool).await{tracing::error!(%error,"Unable to recover campaign provider state");continue;}
        for _ in 0..25 {
            // A batch can take minutes. Re-read saved configuration before each
            // claim so rotation/revocation takes effect for the next recipient.
            let mailer = match CampaignMailer::from_state(&state).await {
                Ok(mailer) => mailer,
                Err(error) => {
                    tracing::warn!(%error, "Campaign configuration changed; no next recipient was claimed");
                    break;
                }
            };
            match process_one(&state.db.pool, &mailer, &state.config.app_url).await {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => {
                    tracing::error!(%error,"Campaign worker failed; durable leases preserve uncertain outcomes");
                    break;
                }
            }
        }
    }
}

/// Lease recovery needs no provider configuration and never performs a send.
pub async fn recover_expired(pool: &PgPool) -> Result<u64> {
    let mut recovered = 0;
    for _ in 0..25 {
        let mut tx = pool.begin().await?;
        let id:Option<i64>=sqlx::query_scalar("SELECT c.id FROM crm_campaigns c WHERE EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE d.campaign_id=c.id AND d.status='processing' AND d.lease_expires_at<NOW()) ORDER BY c.id FOR UPDATE SKIP LOCKED LIMIT 1").fetch_optional(&mut *tx).await?;
        let Some(id) = id else {
            break;
        };
        recovered+=sqlx::query("UPDATE crm_campaign_deliveries SET status='unknown',last_error='Worker lease expired after a possible provider submission; reconcile before retrying',lease_token=NULL,lease_expires_at=NULL,updated_at=NOW() WHERE campaign_id=$1 AND status='processing' AND lease_expires_at<NOW()").bind(id).execute(&mut *tx).await?.rows_affected();
        refresh(&mut tx, id).await?;
        tx.commit().await?;
    }
    Ok(recovered)
}

/// A random per-delivery capability suppresses the email and all queued copies.
/// GET supports the established browser callback; POST supports RFC 8058 clients.
pub async fn unsubscribe(pool: &PgPool, token: Uuid) -> Result<bool> {
    let mut tx = pool.begin().await?;
    let email: Option<String> =
        sqlx::query_scalar("SELECT email FROM crm_campaign_deliveries WHERE unsubscribe_token=$1")
            .bind(token)
            .fetch_optional(&mut *tx)
            .await?;
    let Some(email) = email else { return Ok(false) };
    let campaigns:Vec<i64>=sqlx::query_scalar("SELECT DISTINCT campaign_id FROM crm_campaign_deliveries WHERE email=$1 ORDER BY campaign_id").bind(&email).fetch_all(&mut *tx).await?;
    for id in &campaigns {
        sqlx::query("SELECT id FROM crm_campaigns WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;
    }
    sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=COALESCE(unsubscribed_at,NOW()),updated_at=NOW() WHERE LOWER(email)=$1").bind(&email).execute(&mut *tx).await?;
    sqlx::query("UPDATE crm_campaign_deliveries SET unsubscribed_at=COALESCE(unsubscribed_at,NOW()),status=CASE WHEN status='queued' THEN 'skipped' ELSE status END,last_error=CASE WHEN status='queued' THEN 'Recipient unsubscribed' ELSE last_error END,updated_at=NOW() WHERE email=$1").bind(email).execute(&mut *tx).await?;
    for id in campaigns {
        refresh(&mut tx, id).await?;
    }
    tx.commit().await?;
    Ok(true)
}
