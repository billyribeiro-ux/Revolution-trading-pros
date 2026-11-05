//! CRM campaign authoring. Delivery state is owned by the delivery worker.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

macro_rules! campaign_sql {
    ($before:literal,$after:literal)=>{concat!($before,"id::text AS id,COALESCE(title,name) AS title,subject,subject AS email_subject,content AS email_body,text_body,status,campaign_type AS type,email_pre_header,design_template,template_id::text AS template_id,template_config,settings,subscriber_settings,utm_settings,email_template_id::text AS email_template_id,ab_test_config,(SELECT e.id FROM email_campaigns e WHERE e.delivery_campaign_id=crm_campaigns.id) AS native_email_campaign_id,created_by::text AS created_by,scheduled_at,sent_at,recipients_count,emails_sent,emails_sent AS sent_count,opens,opens AS open_count,clicks,clicks AS click_count,open_rate,click_rate,failed_count,last_error,pause_reason,delivery_stats,authoring_version,created_at,updated_at",$after)};
}

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"CRM campaign persistence failed");
    ApiError::database_error("Unable to access campaigns")
}
fn validation(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn text(value: &str, field: &str, max: usize, required: bool) -> Result<(), ApiError> {
    if value.contains('\0') || value.chars().count() > max || required && value.trim().is_empty() {
        return Err(validation(&format!(
            "{field} must contain {}–{max} characters without null characters",
            if required { 1 } else { 0 }
        )));
    }
    Ok(())
}
fn valid_json(value: &Value, depth: usize) -> bool {
    if depth > 32 {
        return false;
    }
    match value {
        Value::String(v) => !v.contains('\0'),
        Value::Array(v) => v.iter().all(|v| valid_json(v, depth + 1)),
        Value::Object(v) => v
            .iter()
            .all(|(k, v)| !k.contains('\0') && valid_json(v, depth + 1)),
        _ => true,
    }
}
fn patch<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Input {
    title: Option<String>,
    #[serde(alias = "subject")]
    email_subject: Option<String>,
    #[serde(alias = "content")]
    email_body: Option<String>,
    #[serde(default, deserialize_with = "patch")]
    text_body: Option<Option<String>>,
    #[serde(rename = "type")]
    campaign_type: Option<String>,
    email_pre_header: Option<String>,
    design_template: Option<String>,
    #[serde(default, deserialize_with = "patch")]
    template_id: Option<Option<i64>>,
    #[serde(default, deserialize_with = "patch")]
    template_config: Option<Option<Value>>,
    #[serde(default, deserialize_with = "patch")]
    settings: Option<Option<Value>>,
    #[serde(default, deserialize_with = "patch")]
    subscriber_settings: Option<Option<Value>>,
    #[serde(default, deserialize_with = "patch")]
    utm_settings: Option<Option<Value>>,
}
impl Input {
    fn validate(&self, create: bool) -> Result<(), ApiError> {
        if create || self.title.is_some() {
            text(self.title.as_deref().unwrap_or(""), "Title", 255, true)?;
        }
        if let Some(v) = &self.email_subject {
            text(v, "Subject", 500, false)?;
        }
        if let Some(v) = &self.email_body {
            text(v, "Email body", 1_000_000, false)?;
        }
        if let Some(Some(value)) = &self.text_body {
            text(value, "Text alternative", 1_000_000, false)?;
        }
        if let Some(v) = &self.email_pre_header {
            text(v, "Preheader", 500, false)?;
        }
        if let Some(v) = &self.design_template {
            text(v, "Design template", 100, true)?;
        }
        if self
            .campaign_type
            .as_ref()
            .is_some_and(|v| !["regular", "plain_text", "raw_html"].contains(&v.as_str()))
        {
            return Err(validation("Invalid campaign type"));
        }
        for v in [
            &self.template_config,
            &self.settings,
            &self.subscriber_settings,
            &self.utm_settings,
        ]
        .into_iter()
        .flatten()
        .flatten()
        {
            if !v.is_object() || v.to_string().len() > 65536 || !valid_json(v, 0) {
                return Err(validation(
                    "Campaign configuration must be an object of at most 64 KiB without null characters or excessive nesting",
                ));
            }
        }
        Ok(())
    }
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Mailer {
    from_name: String,
    from_email: String,
    reply_to_name: String,
    reply_to_email: String,
    is_custom: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SenderSettings {
    mailer_settings: Mailer,
}
fn validate_sender(input: &Input) -> Result<(), ApiError> {
    if let Some(Some(settings)) = &input.settings {
        let sender: SenderSettings = serde_json::from_value(settings.clone())
            .map_err(|_| validation("Provide valid sender settings"))?;
        let sender = sender.mailer_settings;
        for value in [
            &sender.from_name,
            &sender.from_email,
            &sender.reply_to_name,
            &sender.reply_to_email,
        ] {
            text(value, "Sender field", 255, false)?;
            if value.contains(['\r', '\n']) {
                return Err(validation("Sender fields cannot contain line breaks"));
            }
        }
        if sender.is_custom || !sender.from_email.is_empty() {
            crate::utils::errors::validate_email(&sender.from_email)
                .map_err(|_| validation("Provide a valid sender email"))?;
        }
        if !sender.reply_to_email.is_empty() {
            crate::utils::errors::validate_email(&sender.reply_to_email)
                .map_err(|_| validation("Provide a valid reply-to email"))?;
        }
    }
    if let Some(Some(settings)) = &input.utm_settings {
        let object = settings
            .as_object()
            .ok_or_else(|| validation("Invalid campaign tracking settings"))?;
        for (key, value) in object {
            if ![
                "utm_source",
                "utm_medium",
                "utm_campaign",
                "utm_term",
                "utm_content",
            ]
            .contains(&key.as_str())
            {
                return Err(validation("Unsupported campaign tracking parameter"));
            }
            text(
                value
                    .as_str()
                    .ok_or_else(|| validation("Campaign tracking values must be text"))?,
                "Campaign tracking value",
                255,
                false,
            )?;
        }
    }
    Ok(())
}
async fn validate_audience(pool: &sqlx::PgPool, input: &Input) -> Result<(), ApiError> {
    if let Some(Some(value)) = &input.subscriber_settings {
        super::campaign_audience::resolve(pool, Some(value.clone())).await?;
    }
    Ok(())
}

#[derive(Deserialize, Default)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    status: Option<String>,
}
impl Filters {
    fn bounds(&self) -> Result<(i64, i64, i64), ApiError> {
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(25);
        if page < 1 || !(1..=100).contains(&size) {
            return Err(validation(
                "Page must be positive and page size must be 1–100",
            ));
        }
        if self.status.as_ref().is_some_and(|v| {
            ![
                "draft",
                "scheduled",
                "sending",
                "sent",
                "paused",
                "failed",
                "archived",
            ]
            .contains(&v.as_str())
        }) {
            return Err(validation("Invalid campaign status"));
        }
        if let Some(v) = &self.search {
            text(v, "Search", 255, false)?;
        }
        Ok((page, size, page.saturating_sub(1).saturating_mul(size)))
    }
}
pub(super) async fn row(pool: &sqlx::PgPool, id: i64) -> Result<Value, ApiError> {
    let mut connection = pool.acquire().await.map_err(db)?;
    row_on(&mut connection, id).await
}
pub(crate) async fn row_on(
    connection: &mut sqlx::PgConnection,
    id: i64,
) -> Result<Value, ApiError> {
    sqlx::query_scalar(campaign_sql!(
        "SELECT to_jsonb(c) FROM (SELECT ",
        " FROM crm_campaigns WHERE id=$1) c"
    ))
    .bind(id)
    .fetch_optional(connection)
    .await
    .map_err(db)?
    .ok_or_else(|| ApiError::not_found("Campaign not found"))
}

async fn stats_value(pool: &sqlx::PgPool) -> Result<Value, ApiError> {
    sqlx::query_scalar("SELECT jsonb_build_object('total',COUNT(*),'sent',COUNT(*) FILTER(WHERE status='sent'),'scheduled',COUNT(*) FILTER(WHERE status='scheduled'),'draft',COUNT(*) FILTER(WHERE status='draft'),'total_emails_sent',COALESCE(SUM(emails_sent),0),'total_opens',COALESCE(SUM(opens),0),'total_clicks',COALESCE(SUM(clicks),0),'avg_open_rate',CASE WHEN COALESCE(SUM(emails_sent),0)=0 THEN 0 ELSE ROUND(SUM(opens)*100.0/SUM(emails_sent),2) END,'avg_click_rate',CASE WHEN COALESCE(SUM(emails_sent),0)=0 THEN 0 ELSE ROUND(SUM(clicks)*100.0/SUM(emails_sent),2) END) FROM crm_campaigns").fetch_one(pool).await.map_err(db)
}
async fn list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = filters.bounds()?;
    let search = filters.search.map(|v| format!("%{v}%"));
    let rows: Vec<Value> = sqlx::query_scalar(campaign_sql!("SELECT to_jsonb(c) FROM (SELECT "," FROM crm_campaigns WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1 OR subject ILIKE $1) AND ($2::text IS NULL OR status=$2) ORDER BY created_at DESC,id DESC LIMIT $3 OFFSET $4) c"))
        .bind(&search)
        .bind(&filters.status)
        .bind(size)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaigns WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1 OR subject ILIKE $1) AND ($2::text IS NULL OR status=$2)").bind(search).bind(filters.status).fetch_one(&state.db.pool).await.map_err(db)?;
    Ok(Json(
        json!({"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":((total+size-1)/size).max(1)},"stats":stats_value(&state.db.pool).await?}),
    ))
}
async fn get_one(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(row(&state.db.pool, id).await?))
}
async fn check_template(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    id: Option<i64>,
) -> Result<(), ApiError> {
    if let Some(id) = id {
        let found: Option<i64> =
            sqlx::query_scalar("SELECT id FROM crm_templates WHERE id=$1 FOR KEY SHARE")
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .map_err(db)?;
        if found.is_none() {
            return Err(validation("Template does not exist"));
        }
    }
    Ok(())
}
async fn create(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    input.validate(true)?;
    validate_sender(&input)?;
    validate_audience(&state.db.pool, &input).await?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    check_template(&mut tx, input.template_id.flatten()).await?;
    let id:i64=sqlx::query_scalar("INSERT INTO crm_campaigns(name,title,subject,content,status,campaign_type,email_pre_header,design_template,template_id,template_config,settings,subscriber_settings,utm_settings,created_by,failed_count,text_body) VALUES($1,$1,$2,$3,'draft',$4,$5,$6,$7,$8,$9,$10,$11,$12,0,$13) RETURNING id")
  .bind(input.title.as_deref().unwrap_or("").trim()).bind(input.email_subject.unwrap_or_default()).bind(input.email_body.unwrap_or_default()).bind(input.campaign_type.as_deref().unwrap_or("regular")).bind(input.email_pre_header).bind(input.design_template.as_deref().unwrap_or("raw")).bind(input.template_id.flatten()).bind(input.template_config.flatten()).bind(input.settings.flatten()).bind(input.subscriber_settings.flatten()).bind(input.utm_settings.flatten()).bind(admin.0.id).bind(input.text_body.flatten()).fetch_one(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, id).await?))
}
async fn update(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    input.validate(false)?;
    validate_sender(&input)?;
    validate_audience(&state.db.pool, &input).await?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let status: Option<String> =
        sqlx::query_scalar("SELECT status FROM crm_campaigns WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?;
    match status.as_deref() {
        None => return Err(ApiError::not_found("Campaign not found")),
        Some("draft" | "paused" | "failed") => {}
        _ => {
            return Err(validation(
                "Pause the campaign before editing; sent campaigns must be duplicated",
            ));
        }
    }
    let processing:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='processing')").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    if processing {
        return Err(crate::utils::errors::conflict(
            "A provider submission is in flight; refresh after it finishes before editing",
        ));
    }
    check_template(&mut tx, input.template_id.flatten()).await?;
    sqlx::query("UPDATE crm_campaigns SET name=COALESCE($2,name),title=COALESCE($2,title),subject=COALESCE($3,subject),content=COALESCE($4,content),campaign_type=COALESCE($5,campaign_type),email_pre_header=COALESCE($6,email_pre_header),design_template=COALESCE($7,design_template),template_id=CASE WHEN $8 THEN $9 ELSE template_id END,template_config=CASE WHEN $10 THEN $11 ELSE template_config END,settings=CASE WHEN $12 THEN $13 ELSE settings END,subscriber_settings=CASE WHEN $14 THEN $15 ELSE subscriber_settings END,utm_settings=CASE WHEN $16 THEN $17 ELSE utm_settings END,text_body=CASE WHEN $18 THEN $19 ELSE text_body END,authoring_version=authoring_version+1,updated_at=NOW() WHERE id=$1")
 .bind(id).bind(input.title.as_deref().map(str::trim)).bind(input.email_subject).bind(input.email_body).bind(input.campaign_type).bind(input.email_pre_header).bind(input.design_template).bind(input.template_id.is_some()).bind(input.template_id.flatten()).bind(input.template_config.is_some()).bind(input.template_config.flatten()).bind(input.settings.is_some()).bind(input.settings.flatten()).bind(input.subscriber_settings.is_some()).bind(input.subscriber_settings.flatten()).bind(input.utm_settings.is_some()).bind(input.utm_settings.flatten()).bind(input.text_body.is_some()).bind(input.text_body.flatten()).execute(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, id).await?))
}
async fn duplicate(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let new_id:Option<i64>=sqlx::query_scalar("INSERT INTO crm_campaigns(name,title,subject,content,status,campaign_type,email_pre_header,design_template,template_id,template_config,settings,subscriber_settings,utm_settings,created_by,failed_count,text_body,delivery_rate_limit) SELECT LEFT(COALESCE(title,name),248)||' (Copy)',LEFT(COALESCE(title,name),248)||' (Copy)',subject,content,'draft',campaign_type,email_pre_header,design_template,template_id,template_config,settings,subscriber_settings,utm_settings,$2,0,text_body,delivery_rate_limit FROM crm_campaigns WHERE id=$1 RETURNING id").bind(id).bind(admin.0.id).fetch_optional(&state.db.pool).await.map_err(db)?;
    Ok(Json(
        row(
            &state.db.pool,
            new_id.ok_or_else(|| ApiError::not_found("Campaign not found"))?,
        )
        .await?,
    ))
}
async fn delete(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let status: Option<String> =
        sqlx::query_scalar("SELECT status FROM crm_campaigns WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?;
    match status.as_deref() {
        None => return Err(ApiError::not_found("Campaign not found")),
        Some("sending") => return Err(validation("Pause the campaign before deleting it")),
        _ => {}
    }
    let processing:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='processing')").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    if processing {
        return Err(crate::utils::errors::conflict(
            "A provider submission is in flight; refresh after it finishes before deleting",
        ));
    }
    sqlx::query("DELETE FROM crm_campaigns WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"deleted":true})))
}
async fn stats(State(state): State<AppState>, _admin: AdminUser) -> Result<Json<Value>, ApiError> {
    Ok(Json(stats_value(&state.db.pool).await?))
}
#[derive(Deserialize)]
struct RecipientPage {
    page: Option<i64>,
    per_page: Option<i64>,
}
async fn recipients(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(page): Query<RecipientPage>,
) -> Result<Json<Value>, ApiError> {
    let campaign = row(&state.db.pool, id).await?;
    Ok(Json(
        super::campaign_audience::preview(
            &state.db.pool,
            campaign.get("subscriber_settings").cloned(),
            page.page.unwrap_or(1),
            page.per_page.unwrap_or(25),
        )
        .await?,
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/campaigns", get(list).post(create))
        .route("/campaigns/stats", get(stats))
        .route("/campaigns/{id}", get(get_one).put(update).delete(delete))
        .route("/campaigns/{id}/duplicate", post(duplicate))
        .route("/campaigns/{id}/recipients", get(recipients))
        .merge(super::campaign_delivery::router())
}
