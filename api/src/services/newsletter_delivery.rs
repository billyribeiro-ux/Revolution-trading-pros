//! Queue legacy newsletter broadcasts in the common durable campaign outbox.
use crate::utils::errors::ApiError;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkEmailRequest {
    pub template_id: Option<i64>,
    pub subject: String,
    pub html_content: String,
    pub text_content: Option<String>,
    pub segment: Option<String>,
    pub rate_limit: Option<i32>,
}
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Unable to persist newsletter broadcast");
    ApiError::database_error("Unable to queue newsletter broadcast")
}
fn validate_text(value: &str, field: &str, max: usize, required: bool) -> Result<(), ApiError> {
    if value.contains('\0') || value.chars().count() > max || required && value.trim().is_empty() {
        return Err(ApiError::validation_error(&format!(
            "Provide valid {field} within {max} characters"
        )));
    }
    Ok(())
}
pub async fn queue(pool: &PgPool, actor: i64, input: BulkEmailRequest) -> Result<Value, ApiError> {
    validate_text(&input.subject, "subject", 500, true)?;
    if input.subject.contains(['\r', '\n']) {
        return Err(ApiError::validation_error("Subject must be a single line"));
    }
    validate_text(&input.html_content, "HTML content", 1_000_000, true)?;
    if let Some(text) = &input.text_content {
        validate_text(text, "text content", 1_000_000, false)?;
    }
    let segment = input.segment.as_deref().unwrap_or("confirmed");
    validate_text(segment, "segment", 255, true)?;
    let rate = input.rate_limit.unwrap_or(100);
    if !(1..=500).contains(&rate) {
        return Err(ApiError::validation_error(
            "Rate limit must be 1–500 emails per minute",
        ));
    }
    let mut tx = pool.begin().await.map_err(db)?;
    let template_config: Option<Value> = if let Some(id) = input.template_id {
        let template: (Option<Value>,) =
            sqlx::query_as("SELECT template_config FROM crm_templates WHERE id=$1 FOR KEY SHARE")
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(db)?
                .ok_or_else(|| ApiError::validation_error("Template does not exist"))?;
        template.0
    } else {
        None
    };
    let title: String = input.subject.chars().take(255).collect();
    let audience = json!({"newsletter_segment":segment,"eligibility":"confirmed_only"});
    let id: i64 = sqlx::query_scalar("INSERT INTO crm_campaigns(name,title,subject,content,status,campaign_type,design_template,template_id,template_config,created_by,scheduled_at,subscriber_settings,audience_snapshot,delivery_rate_limit,next_delivery_at,text_body) VALUES($1,$1,$2,$3,'sending','raw_html','raw',$4,$5,$6,NOW(),$7,$7,$8,NOW(),$9) RETURNING id")
        .bind(&title).bind(&input.subject).bind(&input.html_content).bind(input.template_id).bind(&template_config).bind(actor).bind(&audience).bind(rate).bind(&input.text_content)
        .fetch_one(&mut *tx).await.map_err(db)?;
    let definition = json!({"id":id.to_string(),"title":title,"email_subject":input.subject,"email_body":input.html_content,"text_body":input.text_content,"type":"raw_html","template_id":input.template_id,"template_config":template_config,"subscriber_settings":audience,"settings":{}});
    let revision: uuid::Uuid = sqlx::query_scalar("INSERT INTO crm_campaign_revisions(campaign_id,authoring_version,definition,created_by) VALUES($1,1,$2,$3) RETURNING id")
        .bind(id).bind(&definition).bind(actor).fetch_one(&mut *tx).await.map_err(db)?;
    // Choose the latest case-insensitive consent record before filtering. Even
    // `all` never bypasses confirmation or a later unsubscribe. No CRM contact is
    // invented just to send to an existing newsletter subscriber.
    let tag = if matches!(segment, "all" | "confirmed") {
        None
    } else {
        Some(segment)
    };
    let inserted = sqlx::query("WITH latest AS (SELECT DISTINCT ON(LOWER(email)) * FROM newsletter_subscribers ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC), eligible AS (SELECT * FROM latest WHERE status='confirmed' AND confirmed_at IS NOT NULL AND unsubscribed_at IS NULL AND ($3::text IS NULL OR tags ? $3) ORDER BY LOWER(email) LIMIT 100001) INSERT INTO crm_campaign_deliveries(campaign_id,revision_id,email,display_name,contact_snapshot,due_at) SELECT $1,$2,LOWER(email),COALESCE(NULLIF(name,''),email),jsonb_build_object('newsletter_subscriber_id',id),NOW() FROM eligible")
        .bind(id).bind(revision).bind(tag).execute(&mut *tx).await.map_err(db)?.rows_affected();
    if inserted == 0 || inserted > 100_000 {
        return Err(ApiError::validation_error(
            "Bulk audience must contain 1–100,000 confirmed recipients",
        ));
    }
    super::campaign_delivery::refresh(&mut tx, id)
        .await
        .map_err(|error| {
            tracing::error!(%error,"Unable to initialize newsletter delivery summary");
            ApiError::database_error("Unable to queue newsletter broadcast")
        })?;
    super::crm_logs::record(
        &mut tx,
        "info",
        "email",
        "Newsletter broadcast queued",
        json!({"campaign_id":id,"recipient_count":inserted,"rate_limit_per_minute":rate}),
        Some(actor),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    tracing::info!(target:"security",event="admin_bulk_email_queued",admin_id=actor,campaign_id=id,recipient_count=inserted,"Newsletter broadcast durably queued");
    Ok(
        json!({"message":"Newsletter broadcast durably queued","job":{"id":id.to_string(),"campaign_id":id.to_string(),"subject":definition["email_subject"],"segment":segment,"eligibility":"confirmed_only","recipient_count":inserted,"rate_limit_per_minute":rate,"estimated_duration_minutes":inserted.div_ceil(rate as u64),"status":"queued","initiated_by":actor},"note":"Provider acceptance and delivery are recorded separately; inspect the campaign delivery report"}),
    )
}
