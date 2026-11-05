//! Durable sequence authoring and ordered emails; delivery revisions remain immutable.
use super::sequence_model::{self as model, Mail, MailInput, Settings, invalid, text};
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
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use uuid::Uuid;
pub(super) fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Sequence persistence failed");
    if error
        .as_database_error()
        .is_some_and(|error| matches!(error.code().as_deref(), Some("23505" | "23503")))
    {
        return errors::conflict(
            "A sequence slug or related record changed; reload before retrying",
        );
    }
    errors::service_unavailable("Sequence storage is unavailable")
}
macro_rules! sequence_sql {($tail:literal)=>{concat!(
    "SELECT to_jsonb(q) FROM(SELECT s.*,COALESCE(s.title,s.name) AS display_title,COALESCE(s.status,'draft') AS display_status,",
    "(SELECT COUNT(*) FROM crm_sequence_trackers t WHERE t.sequence_id=s.id) AS recorded_subscribers,",
    "(SELECT COUNT(*) FROM crm_sequence_trackers t WHERE t.sequence_id=s.id AND t.status='active') AS recorded_active,",
    "(SELECT COUNT(*) FROM crm_sequence_trackers t WHERE t.sequence_id=s.id AND t.status='completed') AS recorded_completed,",
    "COALESCE((SELECT jsonb_object_agg(a.mail_id,a.counts) FROM(SELECT d.sequence_mail_id::text AS mail_id,jsonb_build_object('sent_count',COUNT(*) FILTER(WHERE d.accepted_at IS NOT NULL),'open_count',COUNT(*) FILTER(WHERE d.opened_at IS NOT NULL AND d.accepted_at IS NOT NULL),'click_count',COUNT(*) FILTER(WHERE d.clicked_at IS NOT NULL AND d.accepted_at IS NOT NULL),'unsubscribe_count',COUNT(*) FILTER(WHERE d.unsubscribed_at IS NOT NULL AND d.accepted_at IS NOT NULL),'delivered_count',COUNT(*) FILTER(WHERE d.delivered_at IS NOT NULL AND d.accepted_at IS NOT NULL),'bounced_count',COUNT(*) FILTER(WHERE d.bounced_at IS NOT NULL AND d.accepted_at IS NOT NULL),'complained_count',COUNT(*) FILTER(WHERE d.complained_at IS NOT NULL AND d.accepted_at IS NOT NULL),'failed_count',COUNT(*) FILTER(WHERE d.status='failed'),'unknown_count',COUNT(*) FILTER(WHERE d.status='unknown'),'queued_count',COUNT(*) FILTER(WHERE d.status='queued'),'processing_count',COUNT(*) FILTER(WHERE d.status='processing')) AS counts FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=s.id GROUP BY d.sequence_mail_id)a),'{}'::jsonb) AS recorded_mails ",
    "FROM crm_sequences s ",$tail,")q")};}
fn normalized(mut row: Value) -> Result<Value, ApiError> {
    let id = row["id"]
        .as_i64()
        .ok_or_else(|| errors::service_unavailable("Stored sequence identity is invalid"))?;
    let original = row["steps"].clone();
    let (mut emails, definition_error) = match model::mails(original.clone(), id) {
        Ok(mails) => (
            serde_json::to_value(mails)
                .map_err(|_| errors::internal_error("Cannot encode sequence emails"))?,
            Value::Null,
        ),
        Err(error) => (json!([]), json!(error.message)),
    };
    let mut sent = 0_i64;
    let mut opened = 0_i64;
    let mut clicked = 0_i64;
    // Aggregate actual receipts across all revisions, including removed emails.
    if let Some(mails) = row["recorded_mails"].as_object() {
        for counts in mails.values() {
            sent += counts["sent_count"].as_i64().unwrap_or(0);
            opened += counts["open_count"].as_i64().unwrap_or(0);
            clicked += counts["click_count"].as_i64().unwrap_or(0);
        }
    }
    let counts = row["recorded_mails"].clone();
    if let Some(emails) = emails.as_array_mut() {
        for mail in emails.iter_mut() {
            let counts = &counts[mail["id"].as_str().unwrap_or("")];
            for key in [
                "sent_count",
                "open_count",
                "click_count",
                "unsubscribe_count",
            ] {
                mail[key] = counts.get(key).cloned().unwrap_or(json!(0));
            }
            let denominator = mail["sent_count"].as_i64().unwrap_or(0);
            mail["open_rate"] = if denominator > 0 {
                json!(100.0 * mail["open_count"].as_f64().unwrap_or(0.0) / denominator as f64)
            } else {
                Value::Null
            };
            mail["click_rate"] = if denominator > 0 {
                json!(100.0 * mail["click_count"].as_f64().unwrap_or(0.0) / denominator as f64)
            } else {
                Value::Null
            };
            mail["revenue"] = Value::Null;
        }
    }
    row["historical_counts"] = json!({"subscribers":row["total_subscribers"],"sent":row["emails_sent"],"open_rate":row["open_rate"],"click_rate":row["click_rate"],"email_count":row["email_count"]});
    row["id"] = json!(id.to_string());
    row["title"] = row["display_title"].clone();
    row["status"] = row["display_status"].clone();
    row["created_by"] = row["created_by"]
        .as_i64()
        .map(|value| json!(value.to_string()))
        .unwrap_or(Value::Null);
    row["emails_count"] = if definition_error.is_null() {
        json!(emails.as_array().map_or(0, Vec::len))
    } else {
        Value::Null
    };
    row["emails"] = emails;
    row["definition_error"] = definition_error;
    row["historical_steps"] = original;
    row["subscribers_count"] = row["recorded_subscribers"].clone();
    row["active_subscribers"] = row["recorded_active"].clone();
    row["completed_subscribers"] = row["recorded_completed"].clone();
    row["total_sent"] = json!(sent);
    row["total_opened"] = json!(opened);
    row["total_clicked"] = json!(clicked);
    for (source, target) in [
        ("delivered_count", "total_delivered"),
        ("bounced_count", "total_bounced"),
        ("complained_count", "total_complained"),
        ("unsubscribe_count", "total_unsubscribed"),
        ("failed_count", "total_failed"),
        ("unknown_count", "total_unknown"),
        ("queued_count", "total_queued"),
        ("processing_count", "total_processing"),
    ] {
        let count: i64 = row["recorded_mails"]
            .as_object()
            .map(|mails| {
                mails
                    .values()
                    .map(|counts| counts[source].as_i64().unwrap_or(0))
                    .sum()
            })
            .unwrap_or(0);
        row[target] = json!(count);
    }
    row["total_revenue"] = Value::Null;
    row["currency"] = Value::Null;
    row["open_rate"] = if sent > 0 {
        json!(100.0 * opened as f64 / sent as f64)
    } else {
        Value::Null
    };
    row["click_rate"] = if sent > 0 {
        json!(100.0 * clicked as f64 / sent as f64)
    } else {
        Value::Null
    };
    row["metrics_scope"] = json!("sequence_outbox");
    if row["trigger_type"].is_null() {
        row["trigger_type"] = json!("manual");
    }
    Ok(row)
}
pub(super) async fn row(pool: &PgPool, id: i64) -> Result<Value, ApiError> {
    let row: Value = sqlx::query_scalar(sequence_sql!("WHERE s.id=$1 AND s.deleted_at IS NULL"))
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Sequence"))?;
    normalized(row)
}
fn stats(row: &Value) -> Value {
    json!({"sequence_id":row["id"],"emails":row["emails_count"],"subscribers":row["subscribers_count"],"active_subscribers":row["active_subscribers"],"completed":row["completed_subscribers"],"total_sent":row["total_sent"],"total_opened":row["total_opened"],"total_clicked":row["total_clicked"],"total_delivered":row["total_delivered"],"total_bounced":row["total_bounced"],"total_complained":row["total_complained"],"total_unsubscribed":row["total_unsubscribed"],"failed":row["total_failed"],"unknown":row["total_unknown"],"queued":row["total_queued"],"processing":row["total_processing"],"open_rate":row["open_rate"],"click_rate":row["click_rate"],"revenue":{"amount":null,"currency":null},"metrics_scope":"sequence_outbox"})
}
pub(super) async fn record(
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
        json!({"sequence_id":id}),
        Some(actor),
    )
    .await
    .map_err(db)
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    status: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}
async fn list(
    State(state): State<AppState>,
    _: AdminUser,
    Query(query): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let page = query.page.unwrap_or(1);
    let size = query.per_page.unwrap_or(20);
    if page < 1 || !(1..=100).contains(&size) {
        return Err(invalid("Use a positive page and page size1–100"));
    }
    if let Some(value) = &query.search {
        text(value, 255, false, false)?;
    }
    if query
        .status
        .as_deref()
        .is_some_and(|value| !matches!(value, "draft" | "active" | "paused" | "completed"))
    {
        return Err(invalid("Invalid sequence status"));
    }
    let sort = query.sort_by.as_deref().unwrap_or("created_at");
    let direction = query.sort_order.as_deref().unwrap_or("desc");
    if !matches!(sort, "created_at" | "updated_at" | "title")
        || !matches!(direction, "asc" | "desc")
    {
        return Err(invalid(
            "Choose a supported sequence sort field and direction",
        ));
    }
    let search = query.search.map(|value| format!("%{value}%"));
    let rows:Vec<Value>=sqlx::query_scalar(sequence_sql!("WHERE s.deleted_at IS NULL AND ($1::text IS NULL OR COALESCE(s.title,s.name) ILIKE $1 OR s.description ILIKE $1) AND ($2::text IS NULL OR COALESCE(s.status,'draft')=$2) ORDER BY CASE WHEN $3='title' AND $4='asc' THEN LOWER(COALESCE(s.title,s.name)) END ASC,CASE WHEN $3='title' AND $4='desc' THEN LOWER(COALESCE(s.title,s.name)) END DESC,CASE WHEN $3='created_at' AND $4='asc' THEN s.created_at END ASC,CASE WHEN $3='created_at' AND $4='desc' THEN s.created_at END DESC,CASE WHEN $3='updated_at' AND $4='asc' THEN s.updated_at END ASC,CASE WHEN $3='updated_at' AND $4='desc' THEN s.updated_at END DESC,s.id DESC LIMIT $5 OFFSET $6"))
        .bind(&search).bind(&query.status).bind(sort).bind(direction).bind(size).bind(page.saturating_sub(1).saturating_mul(size)).fetch_all(&state.db.pool).await.map_err(db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequences WHERE deleted_at IS NULL AND ($1::text IS NULL OR COALESCE(title,name) ILIKE $1 OR description ILIKE $1) AND ($2::text IS NULL OR COALESCE(status,'draft')=$2)").bind(search).bind(query.status).fetch_one(&state.db.pool).await.map_err(db)?;
    let rows = rows
        .into_iter()
        .map(normalized)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(
        json!({"data":rows,"meta":{"total":total,"current_page":page,"per_page":size,"last_page":((total+size-1)/size).max(1)}}),
    ))
}
async fn get_one(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let row = row(&state.db.pool, id).await?;
    Ok(Json(json!({"stats":stats(&row),"sequence":row})))
}
async fn get_stats(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(stats(&row(&state.db.pool, id).await?)))
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Input {
    #[serde(alias = "name")]
    title: Option<String>,
    description: Option<String>,
    slug: Option<String>,
    status: Option<String>,
    trigger_type: Option<String>,
    design_template: Option<String>,
    settings: Option<Settings>,
    subscriber_settings: Option<Value>,
    trigger_config: Option<Value>,
    expected_version: Option<i64>,
    emails: Option<Vec<MailInput>>,
}
impl Input {
    fn validate(&self, create: bool) -> Result<(), ApiError> {
        if create || self.title.is_some() {
            text(self.title.as_deref().unwrap_or(""), 255, true, false)?;
        }
        if let Some(value) = &self.description {
            text(value, 4000, false, false)?;
        }
        if let Some(value) = &self.slug {
            text(value, 255, true, false)?;
            if !value
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
            {
                return Err(invalid(
                    "Sequence slug must contain lowercase letters, digits or hyphens",
                ));
            }
        }
        if self
            .status
            .as_deref()
            .is_some_and(|value| !matches!(value, "draft" | "active" | "paused" | "completed"))
        {
            return Err(invalid("Invalid sequence status"));
        }
        if self.trigger_type.as_deref().is_some_and(|value| {
            !matches!(
                value,
                "manual" | "signup" | "purchase" | "tag_added" | "custom_event"
            )
        }) {
            return Err(invalid("Invalid sequence trigger"));
        }
        if self
            .design_template
            .as_deref()
            .is_some_and(|value| !matches!(value, "raw_html" | "plain_text"))
        {
            return Err(invalid("Choose raw_html or plain_text sequence content"));
        }
        if let Some(settings) = &self.settings {
            settings.mailer_settings.validate()?;
        }
        for value in [&self.subscriber_settings, &self.trigger_config]
            .into_iter()
            .flatten()
        {
            model::object(value)?;
        }
        if self.expected_version.is_some_and(|value| value < 1) {
            return Err(invalid("Sequence version must be positive"));
        }
        Ok(())
    }
    fn authoring(&self) -> bool {
        self.emails.is_some()
            || self.title.is_some()
            || self.description.is_some()
            || self.slug.is_some()
            || self.trigger_type.is_some()
            || self.design_template.is_some()
            || self.settings.is_some()
            || self.subscriber_settings.is_some()
            || self.trigger_config.is_some()
    }
}
pub(super) async fn lock(connection: &mut PgConnection, id: i64) -> Result<Value, ApiError> {
    // Sequence writes and the scheduler serialize before engine creation. The
    // provider worker takes engine locks only and never waits on this advisory lock.
    sqlx::query(
        "SELECT pg_advisory_xact_lock(hashtextextended('crm-sequence:'||$1::bigint::text,0))",
    )
    .bind(id)
    .execute(&mut *connection)
    .await
    .map_err(db)?;
    let _:Vec<i64>=sqlx::query_scalar("SELECT c.id FROM crm_campaigns c JOIN crm_sequence_engines e ON e.campaign_id=c.id WHERE e.sequence_id=$1 ORDER BY c.id FOR UPDATE OF c")
        .bind(id).fetch_all(&mut *connection).await.map_err(db)?;
    let mut row: Value = sqlx::query_scalar(
        "SELECT to_jsonb(s) FROM crm_sequences s WHERE id=$1 AND deleted_at IS NULL FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *connection)
    .await
    .map_err(db)?
    .ok_or_else(|| ApiError::not_found("Sequence"))?;
    let revision = revision(connection, id, &row).await?;
    row["revision_id"] = json!(revision);
    Ok(row)
}
pub(super) async fn revision(
    connection: &mut PgConnection,
    id: i64,
    source: &Value,
) -> Result<Uuid, ApiError> {
    let version = source["authoring_version"]
        .as_i64()
        .ok_or_else(|| invalid("Stored sequence version is invalid"))?;
    let definition = json!({"sequence_id":id,"authoring_version":version,"title":source["title"].as_str().or_else(||source["name"].as_str()),"steps":source["steps"],"design_template":source["design_template"],"settings":source["settings"],"subscriber_settings":source["subscriber_settings"],"trigger_type":source["trigger_type"],"trigger_config":source["trigger_config"]});
    sqlx::query("INSERT INTO crm_sequence_revisions(sequence_id,authoring_version,definition) VALUES($1,$2,$3) ON CONFLICT(sequence_id,authoring_version) DO NOTHING")
        .bind(id).bind(version).bind(definition).execute(&mut *connection).await.map_err(db)?;
    sqlx::query_scalar(
        "SELECT id FROM crm_sequence_revisions WHERE sequence_id=$1 AND authoring_version=$2",
    )
    .bind(id)
    .bind(version)
    .fetch_one(connection)
    .await
    .map_err(db)
}
fn replace_mails(
    id: i64,
    actor: i64,
    inputs: Vec<MailInput>,
    previous: Option<Vec<Mail>>,
) -> Result<Value, ApiError> {
    if inputs.len() > 50 {
        return Err(invalid("A sequence supports at most50emails"));
    }
    let mut result = Vec::with_capacity(inputs.len());
    for (position, input) in inputs.into_iter().enumerate() {
        let old = input
            .id
            .and_then(|id| {
                previous
                    .as_ref()
                    .and_then(|mails| mails.iter().find(|mail| mail.id == id))
            })
            .cloned();
        let mut mail = if let Some(mut old) = old {
            old.position = position;
            old.apply(input)?
        } else {
            Mail::new(id, actor, input, position)?
        };
        mail.position = position;
        result.push(mail);
    }
    let result = json!(result);
    model::mails(result.clone(), id)?;
    Ok(result)
}
fn editable(row: &Value) -> Result<(), ApiError> {
    if row["status"] == "active" {
        return Err(errors::conflict(
            "Pause this sequence before changing its definition",
        ));
    }
    Ok(())
}
async fn create(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(mut input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    input.validate(true)?;
    if input
        .status
        .as_deref()
        .is_some_and(|value| value != "draft")
    {
        return Err(invalid(
            "Create a draft, add its emails, then activate explicitly",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let initial_emails = input.emails.take();
    let id:i64=sqlx::query_scalar("INSERT INTO crm_sequences(name,title,description,slug,status,is_active,steps,trigger_type,design_template,settings,subscriber_settings,trigger_config,created_by) VALUES($1,$1,$2,COALESCE($3,'sequence-'||gen_random_uuid()::text),'draft',false,'[]'::jsonb,$4,$5,$6,$7,$8,$9) RETURNING id")
        .bind(input.title.as_deref().map(str::trim)).bind(input.description).bind(input.slug).bind(input.trigger_type.unwrap_or_else(||"manual".into())).bind(input.design_template.unwrap_or_else(||"raw_html".into())).bind(json!(input.settings.unwrap_or_default())).bind(input.subscriber_settings.unwrap_or(json!({}))).bind(input.trigger_config.unwrap_or(json!({}))).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(db)?;
    if let Some(emails) = initial_emails {
        let mails = replace_mails(id, admin.0.id, emails, None)?;
        sqlx::query("UPDATE crm_sequences SET steps=$2 WHERE id=$1")
            .bind(id)
            .bind(mails)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
    }
    record(&mut tx, admin.0.id, id, "Sequence draft created").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, id).await?))
}
pub(super) fn activation(row: &Value, app_url: &str) -> Result<(), ApiError> {
    let id = row["id"]
        .as_i64()
        .ok_or_else(|| invalid("Invalid stored sequence identity"))?;
    let mails = model::mails(row["steps"].clone(), id)?;
    let active = mails
        .iter()
        .filter(|mail| mail.status == "active")
        .collect::<Vec<_>>();
    if active.is_empty() {
        return Err(invalid(
            "Add at least one active sequence email before activation",
        ));
    }
    crate::services::campaign_delivery::callback_url(app_url).map_err(|_| {
        errors::service_unavailable("Configure a valid APP_URL before activating sequence delivery")
    })?;
    let snapshot =
        json!({"first_name":"Validation","last_name":"Recipient","company":"Validation"});
    let unsubscribe = format!(
        "{}/newsletter/unsubscribe?preview=validation",
        app_url.trim_end_matches('/')
    );
    for mail in active {
        mail.validate(true)?;
        let definition = json!({"sequence_id":id,"type":row["design_template"],"email_body":mail.email_body,"text_body":mail.text_body});
        crate::services::campaign_personalization::render(&definition,crate::services::campaign_personalization::Recipient{email:"validation@example.invalid",name:"Validation Recipient",snapshot:&snapshot,app_url,unsubscribe_url:&unsubscribe,brand:"Validation"},&mail.email_subject)
            .map_err(|_|invalid("Sequence email has unsupported recipient variables, invalid template syntax or excessive output"))?;
    }
    let settings: Settings = serde_json::from_value(row["settings"].clone())
        .map_err(|_| invalid("Review the stored sequence sender settings"))?;
    settings.mailer_settings.validate()?;
    Ok(())
}
async fn update(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(mut input): Json<Input>,
) -> Result<Json<Value>, ApiError> {
    input.validate(false)?;
    if !input.authoring() && input.status.is_none() {
        return Err(invalid("Specify sequence changes"));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    if input
        .expected_version
        .is_some_and(|version| saved["authoring_version"].as_i64() != Some(version))
    {
        return Err(errors::conflict("Sequence changed; reload before saving"));
    }
    if input.authoring() {
        editable(&saved)?;
    }
    if input.status.as_deref() == Some("active") {
        if input.authoring() {
            return Err(invalid(
                "Save the definition, then activate in a separate action",
            ));
        }
        activation(&saved, &state.config.app_url)?;
    }
    if let Some(status) = input.status.as_deref() {
        engine_status(&mut tx, id, status).await?;
    }
    let authoring = input.authoring();
    // An explicit complete replacement can repair legacy definitions; lock()
    // already preserved the original revision before any overwrite.
    let steps = input
        .emails
        .take()
        .map(|inputs| {
            replace_mails(
                id,
                admin.0.id,
                inputs,
                model::mails(saved["steps"].clone(), id).ok(),
            )
        })
        .transpose()?;
    sqlx::query("UPDATE crm_sequences SET name=COALESCE($2,name),title=COALESCE($2,title),description=COALESCE($3,description),slug=COALESCE($4,slug),status=COALESCE($5,status),is_active=CASE WHEN $5::text IS NULL THEN is_active ELSE $5='active' END,trigger_type=COALESCE($6,trigger_type),design_template=COALESCE($7,design_template),settings=COALESCE($8,settings),subscriber_settings=COALESCE($9,subscriber_settings),trigger_config=COALESCE($10,trigger_config),authoring_version=authoring_version+CASE WHEN $11 THEN 1 ELSE 0 END,steps=COALESCE($12,steps),updated_at=NOW() WHERE id=$1")
        .bind(id).bind(input.title.as_deref().map(str::trim)).bind(input.description).bind(input.slug).bind(input.status).bind(input.trigger_type).bind(input.design_template).bind(input.settings.map(|value|json!(value))).bind(input.subscriber_settings).bind(input.trigger_config).bind(authoring).bind(steps).execute(&mut *tx).await.map_err(db)?;
    record(&mut tx, admin.0.id, id, "Sequence updated").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, id).await?))
}
async fn delete(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    editable(&saved)?;
    let active:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=$1 AND d.status='processing')").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    if active {
        return Err(errors::conflict(
            "A sequence provider submission is still in flight; refresh after it finishes",
        ));
    }
    engine_status(&mut tx, id, "paused").await?;
    sqlx::query("UPDATE crm_campaign_deliveries d SET status='skipped',last_error='Sequence archived before submission',updated_at=NOW() FROM crm_sequence_trackers t WHERE t.id=d.sequence_tracker_id AND t.sequence_id=$1 AND d.status='queued'")
        .bind(id).execute(&mut *tx).await.map_err(db)?;
    sqlx::query("UPDATE crm_sequences SET deleted_at=NOW(),status='paused',is_active=false,updated_at=NOW() WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
    sqlx::query("UPDATE crm_sequence_trackers SET status='cancelled',next_execution_at=NULL,last_error='Sequence archived',updated_at=NOW() WHERE sequence_id=$1 AND status IN ('active','paused')").bind(id).execute(&mut *tx).await.map_err(db)?;
    super::sequence_enrollment::refresh_engines(&mut tx, id).await?;
    record(&mut tx, admin.0.id, id, "Sequence archived").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"deleted":true,"id":id.to_string()})))
}
async fn emails(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let saved = row(&state.db.pool, id).await?;
    if !saved["definition_error"].is_null() {
        return Err(invalid(
            saved["definition_error"]
                .as_str()
                .unwrap_or("Sequence definition needs review"),
        ));
    }
    Ok(Json(saved["emails"].clone()))
}
async fn save_mails(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    mails: Vec<Mail>,
    actor: i64,
    action: &str,
) -> Result<(), ApiError> {
    let encoded = json!(mails);
    model::mails(encoded.clone(), id)?;
    sqlx::query("UPDATE crm_sequences SET steps=$2,authoring_version=authoring_version+1,updated_at=NOW() WHERE id=$1").bind(id).bind(encoded).execute(&mut **tx).await.map_err(db)?;
    record(tx, actor, id, action).await
}
async fn add_mail(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<MailInput>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    editable(&saved)?;
    let mut mails = model::mails(saved["steps"].clone(), id)?;
    let mail = Mail::new(id, admin.0.id, input, mails.len())?;
    let key = mail.id;
    mails.push(mail);
    save_mails(&mut tx, id, mails, admin.0.id, "Sequence email added").await?;
    tx.commit().await.map_err(db)?;
    let saved = row(&state.db.pool, id).await?;
    let mail = saved["emails"]
        .as_array()
        .and_then(|mails| mails.iter().find(|mail| mail["id"] == key.to_string()))
        .cloned()
        .ok_or_else(|| errors::service_unavailable("Saved sequence email is unavailable"))?;
    Ok(Json(mail))
}
async fn update_mail(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((id, key)): Path<(i64, Uuid)>,
    Json(input): Json<MailInput>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    editable(&saved)?;
    let mut mails = model::mails(saved["steps"].clone(), id)?;
    let index = mails
        .iter()
        .position(|mail| mail.id == key)
        .ok_or_else(|| ApiError::not_found("Sequence email"))?;
    mails[index] = mails[index].clone().apply(input)?;
    save_mails(&mut tx, id, mails, admin.0.id, "Sequence email updated").await?;
    tx.commit().await.map_err(db)?;
    let saved = row(&state.db.pool, id).await?;
    let mail = saved["emails"]
        .as_array()
        .and_then(|mails| mails.iter().find(|mail| mail["id"] == key.to_string()))
        .cloned()
        .ok_or_else(|| errors::service_unavailable("Saved sequence email is unavailable"))?;
    Ok(Json(mail))
}
async fn delete_mail(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((id, key)): Path<(i64, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    editable(&saved)?;
    let mut mails = model::mails(saved["steps"].clone(), id)?;
    let index = mails
        .iter()
        .position(|mail| mail.id == key)
        .ok_or_else(|| ApiError::not_found("Sequence email"))?;
    mails.remove(index);
    for (index, mail) in mails.iter_mut().enumerate() {
        mail.position = index;
    }
    save_mails(&mut tx, id, mails, admin.0.id, "Sequence email removed").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"deleted":true,"id":key,"sequence_id":id.to_string()}),
    ))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Order {
    email_ids: Vec<Uuid>,
}
async fn reorder(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Order>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    editable(&saved)?;
    let mut mails = model::mails(saved["steps"].clone(), id)?;
    if input.email_ids.len() != mails.len() {
        return Err(invalid(
            "Provide every sequence email identity exactly once",
        ));
    }
    let mut ordered = Vec::with_capacity(mails.len());
    for key in input.email_ids {
        let index = mails
            .iter()
            .position(|mail| mail.id == key)
            .ok_or_else(|| invalid("Email order contains an unknown or repeated identity"))?;
        let mut mail = mails.remove(index);
        mail.position = ordered.len();
        ordered.push(mail);
    }
    save_mails(
        &mut tx,
        id,
        ordered,
        admin.0.id,
        "Sequence emails reordered",
    )
    .await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, id).await?["emails"].clone()))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/sequences", get(list).post(create))
        .route("/sequences/{id}", get(get_one).put(update).delete(delete))
        .route("/sequences/{id}/stats", get(get_stats))
        .route("/sequences/{id}/duplicate", post(duplicate))
        .route("/sequences/{id}/activate", post(activate))
        .route("/sequences/{id}/pause", post(pause))
        .route("/sequences/{id}/emails", get(emails).post(add_mail))
        .route("/sequences/{id}/emails/reorder", post(reorder))
        .route(
            "/sequences/{id}/emails/{key}",
            axum::routing::put(update_mail).delete(delete_mail),
        )
}

pub(super) async fn engine_status(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    status: &str,
) -> Result<(), ApiError> {
    // lock() already acquired all mapped engine locks before the sequence row.
    if status == "active" {
        let unsettled:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries d JOIN crm_sequence_engines e ON e.campaign_id=d.campaign_id WHERE e.sequence_id=$1 AND d.status IN ('unknown','processing'))")
            .bind(id).fetch_one(&mut **tx).await.map_err(db)?;
        if unsettled {
            return Err(errors::conflict(
                "A sequence submission is in flight or has an unknown outcome; reconcile it before resuming",
            ));
        }
        sqlx::query("UPDATE crm_campaigns c SET status=CASE WHEN c.scheduled_at>NOW() THEN 'scheduled' ELSE 'sending' END,pause_reason=NULL,last_error=NULL,updated_at=NOW() FROM crm_sequence_engines e WHERE e.campaign_id=c.id AND e.sequence_id=$1 AND c.status='paused' AND c.pause_reason IN ('sequence_paused','uncertain_submission')")
            .bind(id).execute(&mut **tx).await.map_err(db)?;
        sqlx::query("UPDATE crm_sequence_trackers t SET status='active',next_execution_at=COALESCE(next_execution_at,NOW()),last_error=NULL,updated_at=NOW() WHERE t.sequence_id=$1 AND t.status='paused' AND EXISTS(SELECT 1 FROM crm_campaign_deliveries d WHERE d.sequence_tracker_id=t.id AND d.sequence_mail_id=t.next_sequence_mail_id AND d.status IN ('accepted','failed','skipped'))")
            .bind(id).execute(&mut **tx).await.map_err(db)?;
    } else {
        sqlx::query("UPDATE crm_campaigns c SET status='paused',pause_reason='sequence_paused',updated_at=NOW() FROM crm_sequence_engines e WHERE e.campaign_id=c.id AND e.sequence_id=$1 AND (c.status IN ('scheduled','sending') OR (c.status='paused' AND c.pause_reason='provider_unavailable'))")
            .bind(id).execute(&mut **tx).await.map_err(db)?;
    }
    Ok(())
}
async fn change_status(
    state: AppState,
    admin: AdminUser,
    id: i64,
    status: &str,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = lock(&mut tx, id).await?;
    if status == "active" {
        activation(&saved, &state.config.app_url)?;
    }
    engine_status(&mut tx, id, status).await?;
    sqlx::query(
        "UPDATE crm_sequences SET status=$2,is_active=($2='active'),updated_at=NOW() WHERE id=$1",
    )
    .bind(id)
    .bind(status)
    .execute(&mut *tx)
    .await
    .map_err(db)?;
    record(
        &mut tx,
        admin.0.id,
        id,
        if status == "active" {
            "Sequence activated"
        } else {
            "Sequence paused"
        },
    )
    .await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, id).await?))
}
async fn activate(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    change_status(state, admin, id, "active").await
}
async fn pause(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    change_status(state, admin, id, "paused").await
}
async fn duplicate(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let source = lock(&mut tx, id).await?;
    let mut mails = model::mails(source["steps"].clone(), id)?;
    let new:i64=sqlx::query_scalar("INSERT INTO crm_sequences(name,title,description,slug,status,is_active,steps,trigger_type,design_template,settings,subscriber_settings,trigger_config,created_by) SELECT LEFT(COALESCE(title,name),248)||' (Copy)',LEFT(COALESCE(title,name),248)||' (Copy)',description,'sequence-'||gen_random_uuid()::text,'draft',false,'[]'::jsonb,trigger_type,design_template,settings,subscriber_settings,trigger_config,$2 FROM crm_sequences WHERE id=$1 RETURNING id")
        .bind(id).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(db)?;
    let keys = mails
        .iter()
        .map(|mail| (mail.id, Uuid::new_v4()))
        .collect::<std::collections::BTreeMap<_, _>>();
    let now = chrono::Utc::now();
    for mail in &mut mails {
        mail.id = *keys
            .get(&mail.id)
            .ok_or_else(|| invalid("Sequence email identity cannot be copied"))?;
        mail.sequence_id = new.to_string();
        mail.created_at = now;
        mail.updated_at = now;
        mail.created_by = Some(admin.0.id.to_string());
        for condition in &mut mail.conditions {
            condition.previous_email_id = condition
                .previous_email_id
                .map(|key| {
                    keys.get(&key)
                        .copied()
                        .ok_or_else(|| invalid("Sequence condition reference cannot be copied"))
                })
                .transpose()?;
        }
    }
    save_mails(&mut tx, new, mails, admin.0.id, "Sequence duplicated").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(row(&state.db.pool, new).await?))
}
