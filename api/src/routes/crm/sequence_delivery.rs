//! Advance frozen enrollment revisions and stage through the shared delivery outbox.
use super::{
    sequence_enrollment,
    sequence_model::{self as model, Mail, Settings, invalid},
    sequences,
};
use crate::utils::errors::{self, ApiError};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
#[derive(sqlx::FromRow)]
struct Tracker {
    id: i64,
    sequence_id: i64,
    revision_id: Uuid,
    contact_id: Option<i64>,
    contact_email: String,
    next_sequence_mail_id: Option<Uuid>,
    completed_steps: i32,
    total_steps: i32,
}
#[derive(sqlx::FromRow)]
struct Contact {
    email: String,
    first_name: String,
    last_name: String,
    company: Option<String>,
    tags: Value,
}
#[derive(sqlx::FromRow)]
struct Receipt {
    status: String,
    accepted_at: Option<DateTime<Utc>>,
    last_error: Option<String>,
}
#[derive(sqlx::FromRow)]
struct Engagement {
    accepted_at: Option<DateTime<Utc>>,
    opened_at: Option<DateTime<Utc>>,
    clicked_at: Option<DateTime<Utc>>,
}
async fn stop(
    tx: &mut Transaction<'_, Postgres>,
    tracker: &Tracker,
    status: &str,
    reason: &str,
) -> Result<(), ApiError> {
    sqlx::query("UPDATE crm_sequence_trackers SET status=$2,next_execution_at=NULL,last_error=$3,notes=notes||$4,updated_at=NOW() WHERE id=$1")
        .bind(tracker.id).bind(status).bind(reason).bind(json!([{"action":status,"at":Utc::now(),"mail_id":tracker.next_sequence_mail_id,"reason":reason}])).execute(&mut **tx).await.map_err(sequences::db)?;
    sqlx::query("UPDATE crm_campaign_deliveries SET status='skipped',last_error=$2,updated_at=NOW() WHERE sequence_tracker_id=$1 AND status='queued'")
        .bind(tracker.id).bind(reason).execute(&mut **tx).await.map_err(sequences::db)?;
    sequence_enrollment::refresh_engines(tx, tracker.sequence_id).await
}
async fn advance(
    tx: &mut Transaction<'_, Postgres>,
    tracker: &Tracker,
    active: &[&Mail],
    index: usize,
    at: DateTime<Utc>,
    reason: &str,
) -> Result<(), ApiError> {
    let next = active.get(index + 1);
    if tracker.completed_steps + 1 > tracker.total_steps {
        return Err(errors::service_unavailable(
            "Sequence progress does not match its frozen revision",
        ));
    }
    sqlx::query("UPDATE crm_sequence_trackers SET last_sequence_mail_id=$2,next_sequence_mail_id=$3,next_execution_at=CASE WHEN $3::uuid IS NULL THEN NULL ELSE $4::timestamptz+make_interval(secs=>$5::double precision) END,last_executed_at=$4,status=CASE WHEN $3::uuid IS NULL THEN 'completed' ELSE 'active' END,completed_at=CASE WHEN $3::uuid IS NULL THEN $4 ELSE NULL END,completed_steps=completed_steps+1,last_error=NULL,notes=notes||$6,updated_at=NOW() WHERE id=$1")
        .bind(tracker.id).bind(active[index].id).bind(next.map(|mail|mail.id)).bind(at).bind(next.map_or(0.0,|mail|mail.delay as f64)).bind(json!([{"action":reason,"mail_id":active[index].id,"at":at}])).execute(&mut **tx).await.map_err(sequences::db)?;
    Ok(())
}
async fn conditions(
    tx: &mut Transaction<'_, Postgres>,
    tracker: &Tracker,
    contact: &Contact,
    active: &[&Mail],
    index: usize,
) -> Result<bool, ApiError> {
    for condition in &active[index].conditions {
        let matched = match condition.kind.as_str() {
            "tag" => contact.tags.as_array().is_some_and(|tags| {
                condition
                    .value
                    .as_ref()
                    .is_some_and(|tag| tags.contains(tag))
            }),
            "custom" => {
                let predicate: super::segment_rules::Condition = serde_json::from_value(
                    condition
                        .value
                        .clone()
                        .ok_or_else(|| invalid("Missing custom condition"))?,
                )
                .map_err(|_| invalid("Invalid custom contact condition"))?;
                let mut query = super::segment_rules::query(
                    "SELECT c.id FROM contacts c",
                    &[predicate],
                    "all",
                )?;
                query.push(" AND c.id=").push_bind(tracker.contact_id);
                let found: Option<i64> = query
                    .build_query_scalar()
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(sequences::db)?;
                found.is_some()
            }
            "opened" | "clicked" | "not_opened" | "not_clicked" => {
                let previous = condition
                    .previous_email_id
                    .or_else(|| index.checked_sub(1).map(|previous| active[previous].id));
                let observed:Option<Engagement>=sqlx::query_as("SELECT accepted_at,opened_at,clicked_at FROM crm_campaign_deliveries WHERE sequence_tracker_id=$1 AND sequence_mail_id=$2")
                    .bind(tracker.id).bind(previous).fetch_optional(&mut **tx).await.map_err(sequences::db)?;
                observed.is_some_and(|observed| {
                    observed.accepted_at.is_some()
                        && match condition.kind.as_str() {
                            "opened" => observed.opened_at.is_some(),
                            "clicked" => observed.clicked_at.is_some(),
                            "not_opened" => observed.opened_at.is_none(),
                            _ => observed.clicked_at.is_none(),
                        }
                })
            }
            _ => return Err(invalid("Unsupported sequence condition")),
        };
        if !matched {
            return Ok(false);
        }
    }
    Ok(true)
}
async fn engine(
    tx: &mut Transaction<'_, Postgres>,
    tracker: &Tracker,
    definition: &Value,
    mail: &Mail,
) -> Result<(i64, Uuid), ApiError> {
    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT campaign_id FROM crm_sequence_engines WHERE revision_id=$1 AND mail_id=$2",
    )
    .bind(tracker.revision_id)
    .bind(mail.id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(sequences::db)?;
    let parent: Settings = serde_json::from_value(definition["settings"].clone())
        .map_err(|_| invalid("Stored sequence sender settings are invalid"))?;
    let sender = if mail.settings.is_custom {
        &mail.settings
    } else {
        &parent.mailer_settings
    };
    sender.validate()?;
    let kind = definition["design_template"].as_str().unwrap_or("raw_html");
    let campaign = if let Some(campaign) = existing {
        campaign
    } else {
        let name = format!("Sequence {} / {}", tracker.sequence_id, mail.title);
        let id:i64=sqlx::query_scalar("INSERT INTO crm_campaigns(name,title,subject,content,text_body,status,campaign_type,email_pre_header,settings,subscriber_settings,created_by,scheduled_at) VALUES(LEFT($1,255),LEFT($1,255),$2,$3,$4,'draft',$5,$6,$7,'{\"eligibility\":\"confirmed_only\"}'::jsonb,NULL,NOW()) RETURNING id")
            .bind(name).bind(&mail.email_subject).bind(&mail.email_body).bind(&mail.text_body).bind(kind).bind(&mail.email_pre_header).bind(json!({"mailer_settings":sender})).fetch_one(&mut **tx).await.map_err(sequences::db)?;
        sqlx::query("INSERT INTO crm_sequence_engines(campaign_id,sequence_id,revision_id,mail_id) VALUES($1,$2,$3,$4)").bind(id).bind(tracker.sequence_id).bind(tracker.revision_id).bind(mail.id).execute(&mut **tx).await.map_err(sequences::db)?;
        id
    };
    let snapshot = json!({"id":campaign.to_string(),"authoring_version":1,"sequence_id":tracker.sequence_id,"sequence_revision_id":tracker.revision_id,"sequence_mail_id":mail.id,"type":kind,"email_subject":mail.email_subject,"email_body":mail.email_body,"text_body":mail.text_body,"email_pre_header":mail.email_pre_header,"settings":{"mailer_settings":sender},"subscriber_settings":{"eligibility":"confirmed_only"},"sequence_audience":definition["subscriber_settings"]});
    sqlx::query("INSERT INTO crm_campaign_revisions(campaign_id,authoring_version,definition,created_by) VALUES($1,1,$2,NULL) ON CONFLICT(campaign_id,authoring_version) DO NOTHING").bind(campaign).bind(snapshot).execute(&mut **tx).await.map_err(sequences::db)?;
    let revision: Uuid = sqlx::query_scalar(
        "SELECT id FROM crm_campaign_revisions WHERE campaign_id=$1 AND authoring_version=1",
    )
    .bind(campaign)
    .fetch_one(&mut **tx)
    .await
    .map_err(sequences::db)?;
    Ok((campaign, revision))
}
async fn clear_schedule_failure(
    tx: &mut Transaction<'_, Postgres>,
    sequence: i64,
) -> Result<(), ApiError> {
    // Clear only the recovered scheduling failure, preserving uncertainty and
    // other operational errors. This shares the progress transaction.
    sqlx::query("UPDATE crm_sequences SET last_error=NULL WHERE id=$1 AND status='active' AND last_error='Sequence scheduling failed; no unconfirmed submission is retried'")
        .bind(sequence).execute(&mut **tx).await.map_err(sequences::db)?;
    Ok(())
}
async fn stage_on(pool: &PgPool, sequence: i64, selected: i64) -> Result<bool, ApiError> {
    let mut tx = pool.begin().await.map_err(sequences::db)?;
    let source = sequences::lock(&mut tx, sequence).await?;
    if source["status"] != "active" {
        return Ok(false);
    }
    let uncertain:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries d JOIN crm_sequence_engines e ON e.campaign_id=d.campaign_id WHERE e.sequence_id=$1 AND d.status='unknown')")
        .bind(sequence).fetch_one(&mut *tx).await.map_err(sequences::db)?;
    if uncertain {
        sequences::engine_status(&mut tx, sequence, "paused").await?;
        sqlx::query("UPDATE crm_sequences SET status='paused',is_active=false,last_error='A sequence submission has an unknown outcome; provider reconciliation is required',updated_at=NOW() WHERE id=$1")
            .bind(sequence).execute(&mut *tx).await.map_err(sequences::db)?;
        sqlx::query("UPDATE crm_sequence_trackers t SET status='paused',last_error='Provider reconciliation is required before this enrollment can continue',updated_at=NOW() WHERE t.sequence_id=$1 AND t.status='active' AND EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE d.sequence_tracker_id=t.id AND d.status='unknown')")
            .bind(sequence).execute(&mut *tx).await.map_err(sequences::db)?;
        clear_schedule_failure(&mut tx, sequence).await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    let tracker:Option<Tracker>=sqlx::query_as("SELECT id,sequence_id,revision_id,contact_id,contact_email,next_sequence_mail_id,completed_steps,total_steps FROM crm_sequence_trackers WHERE id=$1 AND status='active' AND next_execution_at<=NOW() FOR UPDATE")
        .bind(selected).fetch_optional(&mut *tx).await.map_err(sequences::db)?;
    let Some(tracker) = tracker else {
        return Ok(false);
    };
    let definition: Value = sqlx::query_scalar(
        "SELECT definition FROM crm_sequence_revisions WHERE id=$1 AND sequence_id=$2",
    )
    .bind(tracker.revision_id)
    .bind(sequence)
    .fetch_one(&mut *tx)
    .await
    .map_err(sequences::db)?;
    let mails = model::mails(definition["steps"].clone(), sequence)?;
    let active = mails
        .iter()
        .filter(|mail| mail.status == "active")
        .collect::<Vec<_>>();
    let index = active
        .iter()
        .position(|mail| Some(mail.id) == tracker.next_sequence_mail_id)
        .ok_or_else(|| {
            errors::service_unavailable(
                "Enrollment points to a missing email in its frozen revision",
            )
        })?;
    if active.len() != tracker.total_steps as usize {
        return Err(errors::service_unavailable(
            "Enrollment email count differs from its frozen revision",
        ));
    }
    let receipt:Option<Receipt>=sqlx::query_as("SELECT status,accepted_at,last_error FROM crm_campaign_deliveries WHERE sequence_tracker_id=$1 AND sequence_mail_id=$2")
        .bind(tracker.id).bind(active[index].id).fetch_optional(&mut *tx).await.map_err(sequences::db)?;
    if let Some(receipt) = receipt {
        match receipt.status.as_str() {
            "accepted" => {
                advance(
                    &mut tx,
                    &tracker,
                    &active,
                    index,
                    receipt.accepted_at.ok_or_else(|| {
                        errors::service_unavailable(
                            "Sequence acceptance receipt lacks its timestamp",
                        )
                    })?,
                    "provider_accepted",
                )
                .await?
            }
            "failed" => {
                stop(
                    &mut tx,
                    &tracker,
                    "failed",
                    receipt
                        .last_error
                        .as_deref()
                        .unwrap_or("Sequence provider submission was rejected"),
                )
                .await?
            }
            "skipped" => {
                stop(
                    &mut tx,
                    &tracker,
                    "cancelled",
                    receipt
                        .last_error
                        .as_deref()
                        .unwrap_or("Sequence recipient is no longer eligible"),
                )
                .await?
            }
            "unknown" => {
                sequences::engine_status(&mut tx, sequence, "paused").await?;
                sqlx::query("UPDATE crm_sequences SET status='paused',is_active=false,last_error='A sequence submission has an unknown outcome; provider reconciliation is required',updated_at=NOW() WHERE id=$1").bind(sequence).execute(&mut *tx).await.map_err(sequences::db)?;
                sqlx::query("UPDATE crm_sequence_trackers SET status='paused',last_error='Provider reconciliation is required before this enrollment can continue',updated_at=NOW() WHERE id=$1").bind(tracker.id).execute(&mut *tx).await.map_err(sequences::db)?;
            }
            _ => return Ok(false),
        }
        crate::services::crm_logs::record(&mut tx,"info","email","Sequence enrollment receipt evaluated",json!({"sequence_id":sequence,"tracker_id":tracker.id,"mail_id":active[index].id,"outcome":receipt.status}),None).await.map_err(sequences::db)?;
        clear_schedule_failure(&mut tx, sequence).await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    let contact:Option<Contact>=sqlx::query_as("SELECT LOWER(email) AS email,COALESCE(first_name,'') AS first_name,COALESCE(last_name,'') AS last_name,company,COALESCE(tags,'[]'::jsonb) AS tags FROM contacts WHERE id=$1 FOR SHARE")
        .bind(tracker.contact_id).fetch_optional(&mut *tx).await.map_err(sequences::db)?;
    let confirmed:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM newsletter_subscribers n WHERE n.id=(SELECT latest.id FROM newsletter_subscribers latest WHERE LOWER(email)=$1 ORDER BY updated_at DESC NULLS LAST,id DESC LIMIT 1) AND n.status='confirmed' AND n.confirmed_at IS NOT NULL AND n.unsubscribed_at IS NULL)")
        .bind(&tracker.contact_email).fetch_one(&mut *tx).await.map_err(sequences::db)?;
    let Some(contact) =
        contact.filter(|contact| contact.email == tracker.contact_email && confirmed)
    else {
        stop(
            &mut tx,
            &tracker,
            "cancelled",
            "Contact is unavailable, its address changed, or newsletter confirmation was withdrawn",
        )
        .await?;
        clear_schedule_failure(&mut tx, sequence).await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    };
    if !conditions(&mut tx, &tracker, &contact, &active, index).await? {
        advance(
            &mut tx,
            &tracker,
            &active,
            index,
            Utc::now(),
            "conditions_not_matched",
        )
        .await?;
        clear_schedule_failure(&mut tx, sequence).await?;
        tx.commit().await.map_err(sequences::db)?;
        return Ok(true);
    }
    let (campaign, revision) = engine(&mut tx, &tracker, &definition, active[index]).await?;
    let name = format!("{} {}", contact.first_name, contact.last_name)
        .trim()
        .to_string();
    let name = if name.is_empty() {
        contact.email.clone()
    } else {
        name
    };
    let snapshot = json!({"first_name":contact.first_name,"last_name":contact.last_name,"company":contact.company.as_deref().unwrap_or("")});
    sqlx::query("INSERT INTO crm_campaign_deliveries(campaign_id,revision_id,contact_id,email,display_name,contact_snapshot,due_at,subject_template,sequence_tracker_id,sequence_mail_id) VALUES($1,$2,$3,$4,$5,$6,NOW(),$7,$8,$9) ON CONFLICT(sequence_tracker_id,sequence_mail_id) WHERE sequence_tracker_id IS NOT NULL DO NOTHING")
        .bind(campaign).bind(revision).bind(tracker.contact_id).bind(&contact.email).bind(name).bind(snapshot).bind(&active[index].email_subject).bind(tracker.id).bind(active[index].id).execute(&mut *tx).await.map_err(sequences::db)?;
    let unknown:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='unknown')").bind(campaign).fetch_one(&mut *tx).await.map_err(sequences::db)?;
    if unknown {
        sequences::engine_status(&mut tx, sequence, "paused").await?;
        sqlx::query("UPDATE crm_sequences SET status='paused',is_active=false,last_error='A sequence delivery engine requires provider reconciliation',updated_at=NOW() WHERE id=$1").bind(sequence).execute(&mut *tx).await.map_err(sequences::db)?;
    } else {
        sqlx::query("UPDATE crm_campaigns SET status='sending',scheduled_at=COALESCE(scheduled_at,NOW()),updated_at=NOW() WHERE id=$1 AND (status IN ('draft','sent','failed') OR (status='paused' AND pause_reason IS NULL))").bind(campaign).execute(&mut *tx).await.map_err(sequences::db)?;
    }
    crate::services::campaign_delivery::refresh(&mut tx, campaign)
        .await
        .map_err(|error| {
            tracing::error!(%error,"Sequence outbox refresh failed");
            errors::service_unavailable("Sequence queue metrics are unavailable")
        })?;
    crate::services::crm_logs::record(&mut tx,"info","email","Sequence step staged",json!({"sequence_id":sequence,"tracker_id":tracker.id,"mail_id":active[index].id,"campaign_id":campaign}),None).await.map_err(sequences::db)?;
    clear_schedule_failure(&mut tx, sequence).await?;
    tx.commit().await.map_err(sequences::db)?;
    Ok(true)
}
pub async fn stage_one(pool: &PgPool) -> Result<bool, ApiError> {
    let next:Option<(i64,i64)>=sqlx::query_as("SELECT t.sequence_id,t.id FROM crm_sequence_trackers t JOIN crm_sequences s ON s.id=t.sequence_id WHERE s.status='active' AND s.deleted_at IS NULL AND t.status='active' AND t.next_execution_at<=NOW() AND NOT EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE d.sequence_tracker_id=t.id AND d.sequence_mail_id=t.next_sequence_mail_id AND d.status IN ('queued','processing')) ORDER BY t.next_execution_at,t.id LIMIT 1")
        .fetch_optional(pool).await.map_err(sequences::db)?;
    let Some((sequence, id)) = next else {
        return Ok(false);
    };
    let result = stage_on(pool, sequence, id).await;
    if let Err(error) = &result {
        tracing::error!(sequence,id,message=%error.message,"Sequence scheduler failed");
        if let Err(error)=sqlx::query("UPDATE crm_sequences SET last_error='Sequence scheduling failed; no unconfirmed submission is retried',updated_at=NOW() WHERE id=$1 AND deleted_at IS NULL").bind(sequence).execute(pool).await{tracing::error!(%error,"Sequence scheduler failure could not be recorded");}
    }
    result
}
pub async fn run(state: crate::AppState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    loop {
        interval.tick().await;
        for _ in 0..100 {
            match stage_one(&state.db.pool).await {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => {
                    tracing::error!(message=%error.message,"Sequence staging worker stopped this batch");
                    break;
                }
            }
        }
    }
}
