//! Subscriber administration uses the same identities and consent records as campaign delivery.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{NaiveDateTime, Utc};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

// Canonical identity selection matches the campaign audience resolver. Old case variants
// must never reappear as subscribed after a newer identity is suppressed or deleted.
const BASE: &str = r#"WITH identities AS (
 SELECT DISTINCT ON (LOWER(email)) * FROM newsletter_subscribers
 ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC
), subscribers AS (
 SELECT n.*, CASE WHEN n.status='pending' THEN 'pending'
 WHEN n.status='confirmed' AND n.confirmed_at IS NOT NULL AND n.unsubscribed_at IS NULL THEN 'subscribed'
 WHEN suppression.record_type='SpamComplaint' THEN 'complained'
 WHEN suppression.record_type='Bounce' THEN 'bounced' ELSE 'unsubscribed' END AS display_status,
 activity.last_opened_at,activity.last_clicked_at
 FROM identities n
 LEFT JOIN LATERAL (SELECT record_type FROM crm_campaign_events e
 WHERE LOWER(e.recipient_email)=LOWER(n.email)
 AND e.occurred_at>=COALESCE(n.confirmed_at AT TIME ZONE 'UTC','-infinity'::timestamptz)
 AND (e.record_type='SpamComplaint' OR (e.record_type='Bounce' AND e.details->>'Inactive'='true')
 OR (e.record_type='SubscriptionChange' AND e.details->>'SuppressSending'='true'))
 ORDER BY occurred_at DESC,received_at DESC,fingerprint DESC LIMIT 1) suppression ON TRUE
 LEFT JOIN LATERAL (SELECT MAX(occurred_at) FILTER(WHERE record_type='Open') AS last_opened_at,
 MAX(occurred_at) FILTER(WHERE record_type='Click') AS last_clicked_at FROM crm_campaign_events e
 WHERE LOWER(e.recipient_email)=LOWER(n.email)) activity ON TRUE
) "#;
const STATUSES: &[&str] = &[
    "pending",
    "subscribed",
    "unsubscribed",
    "bounced",
    "complained",
];
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Subscriber persistence failed");
    ApiError::database_error("Unable to access subscribers")
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn bounded(value: &str, limit: usize) -> bool {
    value.chars().count() <= limit && !value.chars().any(char::is_control)
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Filters {
    status: Option<String>,
    search: Option<String>,
    tags: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}
impl Filters {
    fn bounds(&self) -> Result<(i64, i64, i64), ApiError> {
        if self
            .status
            .as_deref()
            .is_some_and(|s| !STATUSES.contains(&s))
        {
            return Err(invalid("Invalid subscriber status"));
        }
        if self.search.as_ref().is_some_and(|v| !bounded(v, 500)) {
            return Err(invalid("Search must be at most 500 characters"));
        }
        if self.tags.as_ref().is_some_and(|v| !bounded(v, 10000)) {
            return Err(invalid("Invalid tag filter"));
        }
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(25);
        if page < 1 || !(1..=200).contains(&size) {
            return Err(invalid("Page must be positive and page size 1–200"));
        }
        Ok((page, size, page.saturating_sub(1).saturating_mul(size)))
    }
    fn query(&self, select: &str) -> Result<QueryBuilder<Postgres>, ApiError> {
        self.bounds()?;
        let mut q = QueryBuilder::new(BASE);
        q.push(select).push(" FROM subscribers WHERE TRUE");
        if let Some(s) = &self.status {
            q.push(" AND display_status=").push_bind(s.clone());
        }
        if let Some(s) = &self.search {
            q.push(" AND (email ILIKE ")
                .push_bind(format!("%{}%", crate::utils::escape_like(s)))
                .push(" OR name ILIKE ")
                .push_bind(format!("%{}%", crate::utils::escape_like(s)))
                .push(")");
        }
        if let Some(s) = &self.tags {
            for tag in tags(s.split(',').map(str::to_owned).collect())? {
                q.push(" AND COALESCE(tags,'[]'::jsonb) @> ")
                    .push_bind(json!([tag]));
            }
        }
        Ok(q)
    }
}
#[derive(sqlx::FromRow)]
struct Row {
    id: i64,
    email: String,
    name: Option<String>,
    display_status: String,
    tags: Option<Value>,
    metadata: Option<Value>,
    confirmed_at: Option<NaiveDateTime>,
    unsubscribed_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    last_opened_at: Option<chrono::DateTime<Utc>>,
    last_clicked_at: Option<chrono::DateTime<Utc>>,
}
#[derive(Serialize)]
struct Subscriber {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    display_name: Option<String>,
    status: String,
    tags: Vec<String>,
    custom_fields: Value,
    email_score: Option<f64>,
    subscribed_at: Option<String>,
    unsubscribed_at: Option<String>,
    created_at: String,
    updated_at: String,
    last_opened_at: Option<String>,
    last_clicked_at: Option<String>,
    confirmation: Option<Value>,
}
fn time(value: NaiveDateTime) -> String {
    value.and_utc().to_rfc3339()
}
impl From<Row> for Subscriber {
    fn from(r: Row) -> Self {
        let m = r.metadata.unwrap_or_else(|| json!({}));
        let confirmation = m.get("_confirmation").cloned().map(|mut c| {
            if c["status"] == "processing"
                && c["lease_until"]
                    .as_str()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .is_none_or(|t| t.with_timezone(&Utc) <= Utc::now())
            {
                c["status"] = json!("unknown");
            }
            // Internal request IDs are not needed by the UI.
            if let Some(o) = c.as_object_mut() {
                o.remove("request_id");
                o.remove("lease_until");
            }
            c
        });
        Self {
            id: r.id.to_string(),
            email: r.email,
            first_name: m["first_name"].as_str().map(str::to_owned),
            last_name: m["last_name"].as_str().map(str::to_owned),
            display_name: r.name,
            status: r.display_status,
            tags: r
                .tags
                .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
                .unwrap_or_default(),
            custom_fields: m
                .get("custom_fields")
                .filter(|v| v.is_object())
                .cloned()
                .unwrap_or_else(|| json!({})),
            email_score: m["email_score"]
                .as_f64()
                .filter(|v| v.is_finite() && (0.0..=100.0).contains(v)),
            subscribed_at: r.confirmed_at.map(time),
            unsubscribed_at: r.unsubscribed_at.map(time),
            created_at: time(r.created_at),
            updated_at: time(r.updated_at),
            last_opened_at: r.last_opened_at.map(|v| v.to_rfc3339()),
            last_clicked_at: r.last_clicked_at.map(|v| v.to_rfc3339()),
            confirmation,
        }
    }
}
async fn read(state: &AppState, id: i64) -> Result<Subscriber, ApiError> {
    let mut q = QueryBuilder::new(BASE);
    q.push("SELECT * FROM subscribers WHERE id=").push_bind(id);
    q.build_query_as::<Row>()
        .fetch_optional(&state.db.pool)
        .await
        .map_err(db)?
        .map(Into::into)
        .ok_or_else(|| {
            ApiError::not_found("Subscriber not found; refresh to retrieve the current identity")
        })
}
async fn list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(f): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = f.bounds()?;
    let mut count = f.query("SELECT COUNT(*)")?;
    let total: i64 = count
        .build_query_scalar()
        .fetch_one(&state.db.pool)
        .await
        .map_err(db)?;
    let mut q = f.query("SELECT *")?;
    q.push(" ORDER BY created_at DESC,id DESC LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind(offset);
    let rows = q
        .build_query_as::<Row>()
        .fetch_all(&state.db.pool)
        .await
        .map_err(db)?;
    Ok(Json(
        json!({"subscribers":rows.into_iter().map(Subscriber::from).collect::<Vec<_>>(),"total":total,"page":page,"per_page":size}),
    ))
}
async fn get_one(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({"subscriber":read(&state,id).await?})))
}
async fn stats(_admin: AdminUser, State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let mut q = QueryBuilder::new(BASE);
    q.push("SELECT COUNT(*),COUNT(*) FILTER(WHERE display_status='subscribed'),COUNT(*) FILTER(WHERE display_status='unsubscribed'),COUNT(*) FILTER(WHERE display_status='pending'),COUNT(*) FILTER(WHERE display_status='bounced'),COUNT(*) FILTER(WHERE display_status='complained') FROM subscribers");
    let (t, s, u, p, b, c): (i64, i64, i64, i64, i64, i64) = q
        .build_query_as()
        .fetch_one(&state.db.pool)
        .await
        .map_err(db)?;
    Ok(Json(
        json!({"total":t,"subscribed":s,"unsubscribed":u,"pending":p,"bounced":b,"complained":c}),
    ))
}
fn tags(values: Vec<String>) -> Result<Vec<String>, ApiError> {
    if values.len() > 100 {
        return Err(invalid("Use at most 100 tags"));
    }
    let mut out = Vec::new();
    for t in values {
        let t = t.trim().to_owned();
        if t.is_empty() || !bounded(&t, 100) {
            return Err(invalid("Tags must be 1–100 characters"));
        }
        if !out.contains(&t) {
            out.push(t);
        }
    }
    Ok(out)
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Write {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    tags: Option<Vec<String>>,
    custom_fields: Option<Value>,
}
impl Write {
    fn validate(&mut self) -> Result<(), ApiError> {
        if let Some(email) = &mut self.email {
            *email = email.trim().to_lowercase();
            if !bounded(email, 255) || !crate::routes::newsletter::is_valid_email(email) {
                return Err(invalid("Enter a valid email address"));
            }
        }
        for name in [&mut self.first_name, &mut self.last_name]
            .into_iter()
            .flatten()
        {
            *name = name.trim().to_owned();
            if !bounded(name, 120) {
                return Err(invalid("Names must be at most 120 characters"));
            }
        }
        if let Some(t) = self.tags.take() {
            self.tags = Some(tags(t)?);
        }
        if let Some(v) = &self.custom_fields
            && (!v.is_object() || v.to_string().len() > 16384 || v.to_string().contains("\\u0000"))
        {
            return Err(invalid(
                "Custom fields must be a JSON object of at most 16 KiB without NUL characters",
            ));
        }

        Ok(())
    }
    fn metadata(&self, mut value: Value) -> Value {
        if !value.is_object() {
            value = json!({});
        }
        if let Some(v) = &self.first_name {
            value["first_name"] = json!(v);
        }
        if let Some(v) = &self.last_name {
            value["last_name"] = json!(v);
        }
        if let Some(v) = &self.custom_fields {
            value["custom_fields"] = v.clone();
        }
        value
    }
}
async fn lock_email(tx: &mut Transaction<'_, Postgres>, email: &str) -> Result<(), ApiError> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended(LOWER($1),89201))")
        .bind(email)
        .execute(&mut **tx)
        .await
        .map_err(db)?;
    Ok(())
}
async fn current(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
) -> Result<(String, Value, Value, Option<String>), ApiError> {
    let email: String = sqlx::query_scalar("SELECT email FROM newsletter_subscribers WHERE id=$1")
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Subscriber not found"))?;
    lock_email(tx, &email).await?;
    type IdentityRow = (i64, String, Option<Value>, Option<Value>, Option<String>);
    let latest:Option<IdentityRow>=sqlx::query_as("SELECT id,email,metadata,tags,name FROM newsletter_subscribers WHERE LOWER(email)=LOWER($1) ORDER BY updated_at DESC NULLS LAST,id DESC LIMIT 1 FOR UPDATE").bind(&email).fetch_optional(&mut **tx).await.map_err(db)?;
    match latest {
        Some((actual, email, m, t, name)) if actual == id => Ok((
            email,
            m.unwrap_or_else(|| json!({})),
            t.unwrap_or_else(|| json!([])),
            name,
        )),
        _ => Err(ApiError::new(
            StatusCode::CONFLICT,
            "This subscriber identity changed; refresh before editing",
        )),
    }
}
fn display_name(m: &Value) -> Option<String> {
    let name = [
        m["first_name"].as_str().unwrap_or(""),
        m["last_name"].as_str().unwrap_or(""),
    ]
    .join(" ")
    .trim()
    .to_owned();
    if name.is_empty() { None } else { Some(name) }
}
async fn create(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(mut w): Json<Write>,
) -> Result<impl IntoResponse, ApiError> {
    w.validate()?;
    let email = w
        .email
        .as_deref()
        .ok_or_else(|| invalid("Email is required"))?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    lock_email(&mut tx, email).await?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM newsletter_subscribers WHERE LOWER(email)=LOWER($1))",
    )
    .bind(email)
    .fetch_one(&mut *tx)
    .await
    .map_err(db)?;
    if exists {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "This email already has a subscriber record",
        ));
    }
    let m = w.metadata(json!({}));
    let name = display_name(&m);
    let id:i64=sqlx::query_scalar("INSERT INTO newsletter_subscribers(email,name,status,source,tags,metadata,gdpr_consent,created_at,updated_at) VALUES($1,$2,'pending','admin',$3,$4,false,NOW(),NOW()) RETURNING id").bind(email).bind(name).bind(json!(w.tags.as_deref().unwrap_or(&[]))).bind(m).fetch_one(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    let c = request_confirmation(&state, id).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({"subscriber":read(&state,id).await?,"confirmation":c})),
    ))
}
async fn update(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut w): Json<Write>,
) -> Result<Json<Value>, ApiError> {
    w.validate()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (email, m, t, name) = current(&mut tx, id).await?;
    // An address belongs to an identity. Replacing it cannot transfer its owner's opt-in.
    // Require separate creation/opt-in and explicit unsubscribe for an address replacement.
    if w.email
        .as_ref()
        .is_some_and(|e| !e.eq_ignore_ascii_case(&email))
    {
        return Err(invalid(
            "Create the new email address as a pending subscriber, then unsubscribe the old address; consent cannot transfer between addresses",
        ));
    }
    let m = w.metadata(m);
    let name = if w.first_name.is_some() || w.last_name.is_some() {
        display_name(&m)
    } else {
        name
    };
    sqlx::query("UPDATE newsletter_subscribers SET name=$2,tags=$3,metadata=$4,updated_at=NOW() WHERE id=$1").bind(id).bind(name).bind(w.tags.map_or(t,|t|json!(t))).bind(m).execute(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"subscriber":read(&state,id).await?})))
}
pub(crate) async fn request_confirmation(state: &AppState, id: i64) -> Result<Value, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (email, mut m, _, name) = current(&mut tx, id).await?;
    let is_active:bool=sqlx::query_scalar("SELECT status='confirmed' AND confirmed_at IS NOT NULL AND unsubscribed_at IS NULL FROM newsletter_subscribers WHERE id=$1").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    if is_active {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "Subscriber is already confirmed",
        ));
    }
    let c = &m["_confirmation"];
    let now = Utc::now();
    if c["lease_until"]
        .as_str()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .is_some_and(|t| t.with_timezone(&Utc) > now)
        || c["requested_at"]
            .as_str()
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .is_some_and(|t| t.with_timezone(&Utc) > now - chrono::Duration::seconds(60))
    {
        return Err(ApiError::new(
            StatusCode::TOO_MANY_REQUESTS,
            "Wait before requesting another confirmation message",
        ));
    }
    let request_id = Uuid::new_v4().to_string();
    let mut receipt = json!({"request_id":request_id,"status":"processing","requested_at":now.to_rfc3339(),"lease_until":(now+chrono::Duration::seconds(120)).to_rfc3339()});
    if !m.is_object() {
        m = json!({});
    }
    m["_confirmation"] = receipt.clone();
    sqlx::query("UPDATE newsletter_subscribers SET status='pending',metadata=$2,updated_at=NOW() WHERE id=$1").bind(id).bind(m).execute(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    let result =
        crate::routes::newsletter::send_confirmation(state, id, &email, name.as_deref()).await;
    let (status, message) = match result {
        Ok(()) => (
            "accepted",
            "Confirmation submitted to the email provider; waiting for recipient confirmation",
        ),
        Err(ref e)
            if e.downcast_ref::<crate::services::campaign_mailer::UncertainSubmission>()
                .is_some() =>
        {
            (
                "unknown",
                "Confirmation submission is unconfirmed; check the inbox before requesting another message",
            )
        }
        Err(ref e)
            if e.downcast_ref::<crate::services::campaign_mailer::RejectedSubmission>()
                .is_none()
                && !state
                    .services
                    .email
                    .resolved()
                    .await
                    .is_ok_and(|email| email.is_enabled()) =>
        {
            tracing::warn!(subscriber_id = id, "Confirmation provider is unavailable");
            let _ = e;
            (
                "unavailable",
                "Subscriber saved as pending; configure email delivery to send confirmation",
            )
        }
        Err(e) => {
            tracing::warn!(subscriber_id=id,error=%e,"Confirmation submission failed");
            (
                "failed",
                "Subscriber saved as pending; confirmation submission failed",
            )
        }
    };
    receipt["status"] = json!(status);
    receipt["message"] = json!(message);
    receipt["finished_at"] = json!(Utc::now().to_rfc3339());
    receipt
        .as_object_mut()
        .expect("receipt is an object")
        .remove("lease_until");
    // Preserve concurrent profile changes and never overwrite a newer confirmation attempt.
    let saved=sqlx::query("UPDATE newsletter_subscribers SET metadata=jsonb_set(COALESCE(metadata,'{}'::jsonb),'{_confirmation}',$2) WHERE id=$1 AND metadata#>>'{_confirmation,request_id}'=$3").bind(id).bind(&receipt).bind(request_id).execute(&state.db.pool).await;
    if let Err(error) = saved {
        tracing::error!(%error,subscriber_id=id,"Unable to persist confirmation submission outcome");
        return Ok(
            json!({"status":"unknown","message":"Subscriber saved; confirmation outcome could not be recorded. Check the inbox before retrying."}),
        );
    }
    if let Some(o) = receipt.as_object_mut() {
        o.remove("request_id");
    }
    Ok(receipt)
}
async fn resubscribe(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let c = request_confirmation(&state, id).await?;
    Ok(Json(
        json!({"success":true,"subscriber":read(&state,id).await?,"confirmation":c}),
    ))
}
async fn unsubscribe(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (email, _, _, _) = current(&mut tx, id).await?;
    sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=COALESCE(unsubscribed_at,NOW()),updated_at=NOW() WHERE LOWER(email)=LOWER($1)").bind(email).execute(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"success":true})))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Tags {
    tags: Vec<String>,
}
async fn mutate_tags(
    state: &AppState,
    id: i64,
    input: Tags,
    remove: bool,
) -> Result<Json<Value>, ApiError> {
    let input = tags(input.tags)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (_, _, t, _) = current(&mut tx, id).await?;
    let mut existing: Vec<String> = serde_json::from_value(t)
        .map_err(|_| ApiError::database_error("Stored subscriber tags are invalid"))?;
    if remove {
        existing.retain(|v| !input.contains(v));
    } else {
        for tag in input {
            if !existing.contains(&tag) {
                existing.push(tag);
            }
        }
    }
    let existing = tags(existing)?;
    sqlx::query("UPDATE newsletter_subscribers SET tags=$2,updated_at=NOW() WHERE id=$1")
        .bind(id)
        .bind(json!(existing))
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"success":true})))
}
async fn add_tags(
    _a: AdminUser,
    State(s): State<AppState>,
    Path(id): Path<i64>,
    Json(t): Json<Tags>,
) -> Result<Json<Value>, ApiError> {
    mutate_tags(&s, id, t, false).await
}
async fn remove_tags(
    _a: AdminUser,
    State(s): State<AppState>,
    Path(id): Path<i64>,
    Json(t): Json<Tags>,
) -> Result<Json<Value>, ApiError> {
    mutate_tags(&s, id, t, true).await
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Bulk {
    ids: Vec<String>,
}
async fn delete_ids(state: &AppState, input: Bulk) -> Result<Json<Value>, ApiError> {
    if input.ids.is_empty() || input.ids.len() > 500 {
        return Err(invalid("Select 1–500 subscribers"));
    }
    let mut ids: Vec<i64> = input
        .ids
        .iter()
        .map(|v| {
            v.parse::<i64>()
                .map_err(|_| invalid("Invalid subscriber ID"))
        })
        .collect::<Result<_, _>>()?;
    ids.sort_unstable();
    ids.dedup();
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let rows:Vec<(i64,String)>=sqlx::query_as("SELECT id,LOWER(email) FROM newsletter_subscribers WHERE id=ANY($1) ORDER BY LOWER(email),id").bind(&ids).fetch_all(&mut *tx).await.map_err(db)?;
    if rows.len() != ids.len() {
        return Err(ApiError::not_found(
            "One or more subscribers no longer exist; nothing deleted",
        ));
    }
    let mut emails = Vec::new();
    for (id, email) in &rows {
        current(&mut tx, *id).await?;
        emails.push(email.clone());
    }
    let result = sqlx::query("DELETE FROM newsletter_subscribers WHERE LOWER(email)=ANY($1)")
        .bind(emails)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"deleted":ids.len(),"deleted_records":result.rows_affected()}),
    ))
}
pub(crate) async fn delete_identity(state: &AppState, id: i64) -> Result<Json<Value>, ApiError> {
    delete_ids(
        state,
        Bulk {
            ids: vec![id.to_string()],
        },
    )
    .await
}
async fn delete_one(
    _a: AdminUser,
    State(s): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    delete_identity(&s, id).await
}
async fn bulk_delete(
    _a: AdminUser,
    State(s): State<AppState>,
    Json(b): Json<Bulk>,
) -> Result<Json<Value>, ApiError> {
    delete_ids(&s, b).await
}
fn csv_cell(value: &str) -> String {
    let prefix = if value
        .trim_start()
        .starts_with(['=', '+', '-', '@', '\t', '\r'])
    {
        "'"
    } else {
        ""
    };
    format!("\"{}{}\"", prefix, value.replace('"', "\"\""))
}
async fn export(
    _a: AdminUser,
    State(state): State<AppState>,
    Query(f): Query<Filters>,
) -> Result<impl IntoResponse, ApiError> {
    f.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    // Fail before sending CSV headers if the database/filters cannot be queried.
    let mut count = f.query("SELECT COUNT(*)")?;
    let _: i64 = count
        .build_query_scalar()
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
    let mut q = f.query("SELECT *")?;
    q.push(" ORDER BY email,id");
    // Stream a consistent snapshot with bounded application memory. Client disconnect
    // drops the cursor/transaction; a mid-stream database failure aborts the response.
    let stream = async_stream::try_stream! {
        yield axum::body::Bytes::from_static(b"email,first_name,last_name,status,tags,subscribed_at,unsubscribed_at,created_at\r\n");
        {
            let mut rows=q.build_query_as::<Row>().fetch(&mut *tx);
            while let Some(row)=rows.try_next().await.map_err(std::io::Error::other)? {
                let s=Subscriber::from(row);
                let row=[s.email,s.first_name.unwrap_or_default(),s.last_name.unwrap_or_default(),s.status,s.tags.join(","),s.subscribed_at.unwrap_or_default(),s.unsubscribed_at.unwrap_or_default(),s.created_at];
                let mut line=row.iter().map(|s|csv_cell(s)).collect::<Vec<_>>().join(",");line.push_str("\r\n");
                yield axum::body::Bytes::from(line);
            }
        }
        tx.commit().await.map_err(std::io::Error::other)?;
    };
    type CsvStream = std::pin::Pin<
        Box<dyn futures::Stream<Item = Result<axum::body::Bytes, std::io::Error>> + Send>,
    >;
    let stream: CsvStream = Box::pin(stream);
    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"subscribers.csv\"",
            ),
            (header::CACHE_CONTROL, "no-store"),
        ],
        axum::body::Body::from_stream(stream),
    ))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/stats", get(stats))
        .route("/export", get(export))
        .route("/bulk-delete", post(bulk_delete))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
        .route("/{id}/tags", post(add_tags).delete(remove_tags))
        .route("/{id}/unsubscribe", post(unsubscribe))
        .route("/{id}/resubscribe", post(resubscribe))
}
