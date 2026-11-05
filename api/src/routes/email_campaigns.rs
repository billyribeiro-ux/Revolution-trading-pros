//! Legacy email campaign identities and authoring. Delivery uses the shared outbox.
mod delivery;
mod reports;
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
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, Transaction};

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Email campaign persistence failed");
    if error
        .as_database_error()
        .is_some_and(|e| e.code().as_deref() == Some("23503"))
    {
        return errors::conflict(
            "Campaign has related records, or a referenced record changed; reload before trying again",
        );
    }
    errors::service_unavailable("Email campaign storage is unavailable")
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn text(value: &str, max: usize, required: bool, header: bool) -> Result<(), ApiError> {
    if value.contains('\0')
        || value.chars().count() > max
        || required && value.trim().is_empty()
        || header && value.contains(['\r', '\n'])
    {
        return Err(invalid(
            "Provide bounded text without null characters or multiline headers",
        ));
    }
    Ok(())
}
fn patch<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct AbTest {
    enabled: bool,
    subject_b: Option<String>,
    split_percentage: Option<u8>,
}
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct Config {
    from_name: String,
    from_email: String,
    reply_to: Option<String>,
    preview_text: String,
    #[serde(rename = "type")]
    kind: String,
    tags: Vec<String>,
    ab_test_config: Option<AbTest>,
    planned_at: Option<DateTime<Utc>>,
}
impl Config {
    fn validate(&self) -> Result<(), ApiError> {
        for value in [&self.from_name, &self.from_email] {
            text(value, 255, false, true)?;
        }
        if !self.from_email.is_empty() {
            errors::validate_email(&self.from_email)
                .map_err(|_| invalid("Provide a valid sender email"))?;
        }
        if let Some(reply) = &self.reply_to {
            text(reply, 255, true, true)?;
            errors::validate_email(reply).map_err(|_| invalid("Provide a valid reply-to email"))?;
        }
        text(&self.preview_text, 500, false, false)?;
        if !self.kind.is_empty()
            && !matches!(
                self.kind.as_str(),
                "broadcast" | "automation" | "sequence" | "transactional"
            )
        {
            return Err(invalid("Invalid email campaign type"));
        }
        if self.tags.len() > 100 {
            return Err(invalid("At most100 campaign tags are allowed"));
        }
        for tag in &self.tags {
            text(tag, 128, true, true)?;
        }
        if let Some(ab) = &self.ab_test_config {
            if let Some(subject) = &ab.subject_b {
                text(subject, 255, ab.enabled, true)?;
            }
            if (ab.enabled && ab.subject_b.as_ref().is_none_or(|v| v.trim().is_empty()))
                || ab.split_percentage.is_some_and(|v| !(1..=99).contains(&v))
            {
                return Err(invalid(
                    "A/B testing requires Subject B and a split from1to99percent",
                ));
            }
        }
        Ok(())
    }
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    name: Option<String>,
    subject: Option<String>,
    #[serde(default, deserialize_with = "patch")]
    template_id: Option<Option<i64>>,
    #[serde(default, deserialize_with = "patch")]
    segment_id: Option<Option<i64>>,
    html_content: Option<String>,
    #[serde(alias = "text_content", default, deserialize_with = "patch")]
    plain_text_content: Option<Option<String>>,
    from_name: Option<String>,
    from_email: Option<String>,
    #[serde(default, deserialize_with = "patch")]
    reply_to: Option<Option<String>>,
    preview_text: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    tags: Option<Vec<String>>,
    #[serde(alias = "ab_test", default, deserialize_with = "patch")]
    ab_test_config: Option<Option<AbTest>>,
    #[serde(alias = "send_at", default, deserialize_with = "patch")]
    scheduled_at: Option<Option<DateTime<Utc>>>,
}
impl Input {
    fn validate(&self, create: bool) -> Result<(), ApiError> {
        if create || self.name.is_some() {
            text(self.name.as_deref().unwrap_or(""), 255, true, true)?;
        }
        if let Some(v) = &self.subject {
            text(v, 255, false, true)?;
        }
        if let Some(v) = &self.html_content {
            text(v, 4_000_000, false, false)?;
        }
        if let Some(Some(v)) = &self.plain_text_content {
            text(v, 4_000_000, false, false)?;
        }
        if [self.template_id.flatten(), self.segment_id.flatten()]
            .into_iter()
            .flatten()
            .any(|v| v <= 0)
        {
            return Err(invalid("Campaign references must be positive record IDs"));
        }
        Ok(())
    }
    fn merge(self, mut config: Config) -> Result<Config, ApiError> {
        if let Some(v) = self.from_name {
            config.from_name = v;
        }
        if let Some(v) = self.from_email {
            config.from_email = v;
        }
        if let Some(v) = self.reply_to {
            config.reply_to = v;
        }
        if let Some(v) = self.preview_text {
            config.preview_text = v;
        }
        if let Some(v) = self.kind {
            config.kind = v;
        }
        if let Some(v) = self.tags {
            config.tags = v;
        }
        if let Some(v) = self.ab_test_config {
            config.ab_test_config = v;
        }
        if let Some(v) = self.scheduled_at {
            config.planned_at = v;
        }
        config.validate()?;
        Ok(config)
    }
}
macro_rules! campaign_sql {
 ($before:literal,$after:literal)=>{concat!($before,
 "c.id,c.name,c.subject,c.template_id,c.segment_id,COALESCE(c.html_content,'') AS html_content,c.text_content AS plain_text_content,COALESCE(c.authoring_config->>'from_name','') AS from_name,COALESCE(c.authoring_config->>'from_email','') AS from_email,c.authoring_config->>'reply_to' AS reply_to,COALESCE(c.authoring_config->>'preview_text','') AS preview_text,COALESCE(NULLIF(c.authoring_config->>'type',''),'broadcast') AS type,COALESCE(c.authoring_config->'tags','[]'::jsonb) AS tags,c.authoring_config->'ab_test_config' AS ab_test_config,COALESCE(d.status,c.status,'draft') AS status,COALESCE(d.scheduled_at,c.scheduled_at AT TIME ZONE 'UTC') AS scheduled_at,c.authoring_config->>'planned_at' AS planned_at,COALESCE(d.sent_at,c.sent_at AT TIME ZONE 'UTC') AS sent_at,c.created_by,c.authoring_version,c.created_at AT TIME ZONE 'UTC' AS created_at,GREATEST(d.updated_at,c.updated_at AT TIME ZONE 'UTC') AS updated_at,COALESCE(d.recipients_count,0)::bigint AS total_recipients,COALESCE(d.emails_sent,0)::bigint AS sent_count,COALESCE(d.opens,0)::bigint AS opened_count,COALESCE(d.clicks,0)::bigint AS clicked_count,COALESCE((d.delivery_stats->>'bounced')::bigint,0) AS bounced_count,COALESCE((d.delivery_stats->>'unsubscribed')::bigint,0) AS unsubscribed_count,CASE WHEN d.emails_sent>0 THEN d.open_rate ELSE NULL END AS open_rate,CASE WHEN d.emails_sent>0 THEN d.click_rate ELSE NULL END AS click_rate,'recorded_outbox'::text AS metrics_scope,jsonb_build_object('recipients',c.recipient_count,'sent',c.sent_count,'opened',c.open_count,'clicked',c.click_count,'bounced',c.bounce_count,'unsubscribed',c.unsubscribe_count) AS historical_counts,CASE WHEN t.id IS NULL THEN NULL ELSE jsonb_build_object('id',t.id,'name',t.name) END AS template,CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id,'name',u.name) END AS creator FROM email_campaigns c LEFT JOIN crm_campaigns d ON d.id=c.delivery_campaign_id LEFT JOIN email_templates t ON t.id=c.template_id LEFT JOIN users u ON u.id=c.created_by ",$after)};
}
async fn row(pool: &PgPool, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar(campaign_sql!(
        "SELECT to_jsonb(campaign) FROM (SELECT ",
        "WHERE c.id=$1) campaign"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(db)?
    .ok_or_else(|| ApiError::not_found("Campaign"))
}
async fn record(
    tx: &mut Transaction<'_, Postgres>,
    actor: i64,
    id: i64,
    action: &str,
) -> Result<(), ApiError> {
    crate::services::crm_logs::record(
        tx,
        "info",
        "email",
        action,
        json!({"email_campaign_id":id}),
        Some(actor),
    )
    .await
    .map_err(db)
}
async fn references(
    tx: &mut Transaction<'_, Postgres>,
    template: Option<i64>,
    segment: Option<i64>,
) -> Result<Option<(String, Option<String>)>, ApiError> {
    let content = if let Some(id) = template {
        Some(sqlx::query_as::<_,(String,Option<String>)>("SELECT html_content,text_content FROM email_templates WHERE id=$1 AND is_active=true FOR KEY SHARE").bind(id).fetch_optional(&mut **tx).await.map_err(db)?.ok_or_else(||invalid("Select an active email template"))?)
    } else {
        None
    };
    if let Some(id) = segment {
        let found: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM member_segments WHERE id=$1 AND is_active=true)",
        )
        .bind(id)
        .fetch_one(&mut **tx)
        .await
        .map_err(db)?;
        if !found {
            return Err(invalid("Select an active member segment"));
        }
    }
    Ok(content)
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    status: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    from_date: Option<DateTime<Utc>>,
    to_date: Option<DateTime<Utc>>,
}
async fn list(
    State(state): State<AppState>,
    _: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(20);
    if page < 1 || !(1..=100).contains(&size) {
        return Err(invalid("Page must be positive and page size from1to100"));
    }
    if let Some(search) = &filters.search {
        text(search, 255, false, false)?;
    }
    if filters.status.as_ref().is_some_and(|v| {
        !matches!(
            v.as_str(),
            "draft"
                | "scheduled"
                | "sending"
                | "sent"
                | "paused"
                | "failed"
                | "cancelled"
                | "archived"
        )
    }) {
        return Err(invalid("Invalid campaign status"));
    }
    if filters.kind.as_ref().is_some_and(|v| {
        !matches!(
            v.as_str(),
            "broadcast" | "automation" | "sequence" | "transactional"
        )
    }) {
        return Err(invalid("Invalid campaign type"));
    }
    if filters
        .from_date
        .zip(filters.to_date)
        .is_some_and(|(from, to)| from > to)
    {
        return Err(invalid("Campaign date range is reversed"));
    }
    let search = filters.search.map(|v| format!("%{v}%"));
    let rows:Vec<Value>=sqlx::query_scalar(campaign_sql!("SELECT to_jsonb(campaign) FROM (SELECT ","WHERE ($1::text IS NULL OR c.name ILIKE $1 OR c.subject ILIKE $1) AND ($2::text IS NULL OR COALESCE(d.status,c.status,'draft')=$2) AND ($3::text IS NULL OR COALESCE(NULLIF(c.authoring_config->>'type',''),'broadcast')=$3) AND ($4::timestamptz IS NULL OR c.created_at AT TIME ZONE 'UTC'>=$4) AND ($5::timestamptz IS NULL OR c.created_at AT TIME ZONE 'UTC'<=$5) ORDER BY c.created_at DESC,c.id DESC LIMIT $6 OFFSET $7) campaign"))
 .bind(&search).bind(&filters.status).bind(&filters.kind).bind(filters.from_date).bind(filters.to_date).bind(size).bind(page.saturating_sub(1).saturating_mul(size)).fetch_all(&state.db.pool).await.map_err(db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM email_campaigns c LEFT JOIN crm_campaigns d ON d.id=c.delivery_campaign_id WHERE ($1::text IS NULL OR c.name ILIKE $1 OR c.subject ILIKE $1) AND ($2::text IS NULL OR COALESCE(d.status,c.status,'draft')=$2) AND ($3::text IS NULL OR COALESCE(NULLIF(c.authoring_config->>'type',''),'broadcast')=$3) AND ($4::timestamptz IS NULL OR c.created_at AT TIME ZONE 'UTC'>=$4) AND ($5::timestamptz IS NULL OR c.created_at AT TIME ZONE 'UTC'<=$5)").bind(search).bind(filters.status).bind(filters.kind).bind(filters.from_date).bind(filters.to_date).fetch_one(&state.db.pool).await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":rows,"meta":{"total":total,"current_page":page,"per_page":size,"last_page":((total+size-1)/size).max(1)}}),
    ))
}
async fn get_one(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        json!({"success":true,"data":row(&state.db.pool,id).await?}),
    ))
}
async fn create(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    input.validate(true)?;
    let template = input.template_id.flatten();
    let segment = input.segment_id.flatten();
    let name = input.name.clone().unwrap_or_default();
    let subject = input.subject.clone().unwrap_or_default();
    let html = input.html_content.clone();
    let plain = input.plain_text_content.clone();
    let config = input.merge(Config::default())?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let content = references(&mut tx, template, segment).await?;
    let html = html
        .or_else(|| content.as_ref().map(|v| v.0.clone()))
        .unwrap_or_default();
    let plain = plain.unwrap_or_else(|| content.and_then(|v| v.1));
    let id:i64=sqlx::query_scalar("INSERT INTO email_campaigns(name,subject,template_id,segment_id,html_content,text_content,status,authoring_config,created_by,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,'draft',$7,$8,NOW() AT TIME ZONE 'UTC',NOW() AT TIME ZONE 'UTC') RETURNING id").bind(name.trim()).bind(subject).bind(template).bind(segment).bind(html).bind(plain).bind(json!(config)).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(db)?;
    record(&mut tx, admin.0.id, id, "Email campaign created").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":row(&state.db.pool,id).await?}),
    ))
}
async fn lock_row(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
) -> Result<(Value, Option<i64>, String), ApiError> {
    // All shared-worker paths lock engine before native identity. A concurrent
    // first submission is detected after the native lock, without reversing it.
    let engine: Option<i64> =
        sqlx::query_scalar("SELECT delivery_campaign_id FROM email_campaigns WHERE id=$1")
            .bind(id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(db)?
            .ok_or_else(|| ApiError::not_found("Campaign"))?;
    let engine_status = if let Some(engine) = engine {
        Some(
            sqlx::query_scalar::<_, String>(
                "SELECT status FROM crm_campaigns WHERE id=$1 FOR UPDATE",
            )
            .bind(engine)
            .fetch_optional(&mut **tx)
            .await
            .map_err(db)?
            .ok_or_else(|| ApiError::not_found("Campaign"))?,
        )
    } else {
        None
    };
    let stored:Option<(Value,Option<i64>,String)>=sqlx::query_as("SELECT authoring_config,delivery_campaign_id,COALESCE(status,'draft') FROM email_campaigns WHERE id=$1 FOR UPDATE").bind(id).fetch_optional(&mut **tx).await.map_err(db)?;
    let (config, current, status) = stored.ok_or_else(|| ApiError::not_found("Campaign"))?;
    if current != engine {
        return Err(errors::conflict(
            "Campaign submission changed; reload before trying again",
        ));
    }
    Ok((config, engine, engine_status.unwrap_or(status)))
}
async fn quiescent(
    tx: &mut Transaction<'_, Postgres>,
    engine: Option<i64>,
) -> Result<(), ApiError> {
    if let Some(engine) = engine {
        let processing:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='processing')").bind(engine).fetch_one(&mut **tx).await.map_err(db)?;
        if processing {
            return Err(errors::conflict(
                "A provider submission is in flight; pause future sends and refresh after it finishes",
            ));
        }
    }
    Ok(())
}
async fn locked(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
) -> Result<(Value, Option<i64>), ApiError> {
    let (config, engine, status) = lock_row(tx, id).await?;
    quiescent(tx, engine).await?;
    if !matches!(status.as_str(), "draft" | "paused" | "failed") {
        return Err(errors::conflict(
            "Pause before editing; duplicate a sent campaign",
        ));
    }
    Ok((config, engine))
}

async fn update(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    input.validate(false)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (stored, engine) = locked(&mut tx, id).await?;
    let config: Config = serde_json::from_value(stored)
        .map_err(|_| ApiError::database_error("Stored campaign configuration is invalid"))?;
    let old_ab = config.ab_test_config.as_ref().is_some_and(|ab| ab.enabled);
    let delivery_change = input.subject.is_some()
        || input.template_id.is_some()
        || input.segment_id.is_some()
        || input.html_content.is_some()
        || input.plain_text_content.is_some()
        || input.from_name.is_some()
        || input.from_email.is_some()
        || input.reply_to.is_some()
        || input.preview_text.is_some()
        || input.kind.is_some()
        || input.tags.is_some()
        || input.ab_test_config.is_some()
        || input.scheduled_at.is_some();
    let name = input.name.clone();
    let subject = input.subject.clone();
    let template = input.template_id;
    let segment = input.segment_id;
    let html = input.html_content.clone();
    let plain = input.plain_text_content.clone();
    let config = input.merge(config)?;
    if delivery_change
        && (old_ab || config.ab_test_config.as_ref().is_some_and(|ab| ab.enabled))
        && let Some(engine) = engine
    {
        let accepted:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND accepted_at IS NOT NULL)").bind(engine).fetch_one(&mut *tx).await.map_err(db)?;
        if accepted {
            return Err(errors::conflict(
                "Duplicate a started A/B campaign before changing its creative or audience",
            ));
        }
    }

    let previous_template: Option<i64> =
        sqlx::query_scalar("SELECT template_id FROM email_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
    let content = references(&mut tx, template.flatten(), segment.flatten()).await?;
    // A form may include the unchanged reference. Its saved creative remains
    // authoritative until the author selects a different physical template.
    let content = content.filter(|_| template.flatten() != previous_template);
    let html = html.or_else(|| content.as_ref().map(|v| v.0.clone()));
    let plain = plain.or_else(|| content.map(|v| v.1));
    sqlx::query("UPDATE email_campaigns SET name=COALESCE($2,name),subject=COALESCE($3,subject),template_id=CASE WHEN $4 THEN $5 ELSE template_id END,segment_id=CASE WHEN $6 THEN $7 ELSE segment_id END,html_content=COALESCE($8,html_content),text_content=CASE WHEN $9 THEN $10 ELSE text_content END,authoring_config=$11,authoring_version=authoring_version+1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).bind(name.as_deref().map(str::trim)).bind(subject).bind(template.is_some()).bind(template.flatten()).bind(segment.is_some()).bind(segment.flatten()).bind(html).bind(plain.is_some()).bind(plain.flatten()).bind(json!(config)).execute(&mut *tx).await.map_err(db)?;
    if let Some(engine) = engine {
        delivery::synchronize(&mut tx, id, engine).await?;
    }
    record(&mut tx, admin.0.id, id, "Email campaign updated").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":row(&state.db.pool,id).await?}),
    ))
}
async fn duplicate(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let new:Option<i64>=sqlx::query_scalar("INSERT INTO email_campaigns(name,subject,template_id,segment_id,html_content,text_content,status,authoring_config,created_by,created_at,updated_at) SELECT LEFT(name,248)||' (Copy)',subject,template_id,segment_id,html_content,text_content,'draft',authoring_config-'planned_at',$2,NOW() AT TIME ZONE 'UTC',NOW() AT TIME ZONE 'UTC' FROM email_campaigns WHERE id=$1 RETURNING id").bind(id).bind(admin.0.id).fetch_optional(&mut *tx).await.map_err(db)?;
    let new = new.ok_or_else(|| ApiError::not_found("Campaign"))?;
    record(&mut tx, admin.0.id, new, "Email campaign duplicated").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":row(&state.db.pool,new).await?}),
    ))
}
async fn delete(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (_, engine) = locked(&mut tx, id).await?;
    sqlx::query("DELETE FROM email_campaigns WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    if let Some(engine) = engine {
        sqlx::query("DELETE FROM crm_campaigns WHERE id=$1")
            .bind(engine)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
    }
    record(&mut tx, admin.0.id, id, "Email campaign deleted").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"success":true,"deleted_id":id})))
}
async fn stats(State(state): State<AppState>, _: AdminUser) -> Result<Json<Value>, ApiError> {
    let data:Value=sqlx::query_scalar("WITH campaigns AS (SELECT c.id,c.created_at,COALESCE(d.status,c.status,'draft') AS status,COALESCE(d.emails_sent,0) AS sent,COALESCE(d.opens,0) AS opened,COALESCE(d.clicks,0) AS clicked,COALESCE((d.delivery_stats->>'bounced')::bigint,0) AS bounced,COALESCE((d.delivery_stats->>'unsubscribed')::bigint,0) AS unsubscribed FROM email_campaigns c LEFT JOIN crm_campaigns d ON d.id=c.delivery_campaign_id), recent AS (SELECT COUNT(*) FILTER(WHERE receipt.accepted_at>=NOW()-INTERVAL '30 days') AS sent,COUNT(*) FILTER(WHERE receipt.opened_at>=NOW()-INTERVAL '30 days') AS opened,COUNT(*) FILTER(WHERE receipt.clicked_at>=NOW()-INTERVAL '30 days') AS clicked FROM crm_campaign_deliveries receipt JOIN email_campaigns c ON c.delivery_campaign_id=receipt.campaign_id) SELECT jsonb_build_object('total_campaigns',COUNT(*),'total_sent',COALESCE(SUM(sent),0),'total_opened',COALESCE(SUM(opened),0),'total_clicked',COALESCE(SUM(clicked),0),'total_bounced',COALESCE(SUM(bounced),0),'total_unsubscribed',COALESCE(SUM(unsubscribed),0),'drafts',COUNT(*) FILTER(WHERE status='draft'),'scheduled',COUNT(*) FILTER(WHERE status='scheduled'),'sending',COUNT(*) FILTER(WHERE status='sending'),'sent',COUNT(*) FILTER(WHERE status='sent'),'avg_open_rate',ROUND(100.0*SUM(opened)/NULLIF(SUM(sent),0),2),'avg_click_rate',ROUND(100.0*SUM(clicked)/NULLIF(SUM(sent),0),2),'avg_bounce_rate',ROUND(100.0*SUM(bounced)/NULLIF(SUM(sent),0),2),'recent',jsonb_build_object('campaigns',COUNT(*) FILTER(WHERE created_at AT TIME ZONE 'UTC'>=NOW()-INTERVAL '30 days'),'sent',(SELECT sent FROM recent),'opened',(SELECT opened FROM recent),'clicked',(SELECT clicked FROM recent)),'metrics_scope','recorded_outbox') FROM campaigns").fetch_one(&state.db.pool).await.map_err(db)?;
    Ok(Json(json!({"success":true,"data":data})))
}
async fn preview(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let campaign = row(&state.db.pool, id).await?;
    Ok(Json(
        json!({"success":true,"data":{"subject":campaign["subject"],"from_name":campaign["from_name"],"from_email":campaign["from_email"],"html":campaign["html_content"],"plain_text":campaign["plain_text_content"],"preview_kind":"saved_source"}}),
    ))
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/stats", get(stats))
        .route("/{id}/preview", get(preview))
        .route("/{id}", get(get_one).put(update).delete(delete))
        .route("/{id}/duplicate", post(duplicate))
        .merge(delivery::router())
        .merge(reports::router())
}
