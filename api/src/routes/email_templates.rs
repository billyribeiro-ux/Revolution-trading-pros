//! Email Templates Admin Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! Full CRUD for email templates with admin protection.

use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AppState, middleware::admin::AdminUser};

// ═══════════════════════════════════════════════════════════════════════════
// DATA TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmailTemplateRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub subject: String,
    pub body: String,
    pub body_html: String,
    pub body_text: Option<String>,
    pub email_type: String,
    pub description: Option<String>,
    pub category: String,
    pub thumbnail: Option<String>,
    pub design_json: Option<serde_json::Value>,
    pub is_public: bool,
    pub tags: serde_json::Value,
    pub usage_count: i64,
    pub variables: serde_json::Value,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct TemplateListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub slug: Option<String>,
    pub subject: String,
    #[serde(alias = "body_html", alias = "html_content")]
    pub body: String,
    #[serde(alias = "text_content", alias = "plain_text_content")]
    pub body_text: Option<String>,
    #[serde(alias = "template_type", alias = "category")]
    pub email_type: Option<String>,
    #[serde(default, deserialize_with = "present_json")]
    pub description: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub thumbnail: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub design_json: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub is_public: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub tags: Option<serde_json::Value>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub subject: Option<String>,
    #[serde(alias = "body_html", alias = "html_content")]
    pub body: Option<String>,
    #[serde(alias = "text_content", alias = "plain_text_content")]
    pub body_text: Option<String>,
    #[serde(alias = "template_type", alias = "category")]
    pub email_type: Option<String>,
    #[serde(default, deserialize_with = "present_json")]
    pub description: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub thumbnail: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub design_json: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub is_public: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "present_json")]
    pub tags: Option<serde_json::Value>,
    pub variables: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PreviewRequest {
    #[serde(default = "empty_model")]
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct SendTestRequest {
    pub email: String,
    #[serde(default = "empty_model")]
    pub data: serde_json::Value,
}

// Both fragments and the projection are compile-time literals. User values stay bound.
macro_rules! template_sql {
    ($before:literal,$after:literal) => { concat!($before,"id,name,slug,subject,html_content AS body,html_content AS body_html,text_content AS body_text,COALESCE(template_type,'transactional') AS email_type,COALESCE(variables,'[]'::jsonb) AS variables,COALESCE(is_active,false) AS is_active,created_at,updated_at,catalog_metadata->>'description' AS description,COALESCE(template_type,'transactional') AS category,catalog_metadata->>'thumbnail' AS thumbnail,catalog_metadata->'design_json' AS design_json,COALESCE((catalog_metadata->>'is_public')::boolean,false) AS is_public,COALESCE(NULLIF(catalog_metadata->'tags','null'::jsonb),'[]'::jsonb) AS tags,((SELECT COUNT(*) FROM email_logs receipt WHERE receipt.email_template_id=email_templates.id AND receipt.status IN ('sent','delivered','opened','clicked','bounced'))+(SELECT COUNT(*) FROM crm_campaign_deliveries delivery WHERE delivery.email_template_id=email_templates.id AND delivery.status='accepted')) AS usage_count",$after) };
}
fn present_json<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<serde_json::Value>, D::Error> {
    serde_json::Value::deserialize(deserializer).map(Some)
}
fn catalog_metadata(
    description: Option<&serde_json::Value>,
    thumbnail: Option<&serde_json::Value>,
    design: Option<&serde_json::Value>,
    public: Option<&serde_json::Value>,
    tags: Option<&serde_json::Value>,
) -> Result<serde_json::Value, Failure> {
    let mut metadata = serde_json::Map::new();
    for (key, value, max) in [
        ("description", description, 2000),
        ("thumbnail", thumbnail, 2048),
    ] {
        if let Some(value) = value {
            if let Some(text) = value.as_str() {
                validate_text(text, max, false, key == "thumbnail")?;
                if key == "thumbnail" && !text.is_empty() {
                    let relative =
                        text.starts_with('/') && !text.starts_with("//") && !text.contains('\\');
                    let absolute = reqwest::Url::parse(text).is_ok_and(|url| {
                        matches!(url.scheme(), "http" | "https")
                            && url.host_str().is_some()
                            && url.username().is_empty()
                            && url.password().is_none()
                    });
                    if !relative && !absolute {
                        return Err(invalid("Thumbnail must be an HTTP(S) or root-relative URL"));
                    }
                }
            } else if !value.is_null() {
                return Err(invalid("Description and thumbnail must be text or null"));
            }
            metadata.insert(key.into(), value.clone());
        }
    }
    if let Some(value) = design {
        metadata.insert("design_json".into(), value.clone());
    }
    if let Some(value) = public {
        if !value.is_boolean() && !value.is_null() {
            return Err(invalid("Template visibility must be a boolean"));
        }
        metadata.insert(
            "is_public".into(),
            if value.is_null() {
                json!(false)
            } else {
                value.clone()
            },
        );
    }
    if let Some(value) = tags {
        if value.is_null() {
            metadata.insert("tags".into(), json!([]));
        } else {
            let entries = value
                .as_array()
                .filter(|v| v.len() <= 100)
                .ok_or_else(|| invalid("Tags must be an array of at most100 names"))?;
            for entry in entries {
                validate_text(
                    entry
                        .as_str()
                        .ok_or_else(|| invalid("Tag names must be text"))?,
                    128,
                    true,
                    true,
                )?;
            }
            metadata.insert("tags".into(), value.clone());
        }
    }
    let value = serde_json::Value::Object(metadata);
    if !valid_json(&value)
        || serde_json::to_vec(&value).map_or(true, |bytes| bytes.len() > 1_000_000)
    {
        return Err(invalid(
            "Template metadata exceeds1MB or contains invalid characters",
        ));
    }
    Ok(value)
}

fn empty_model() -> serde_json::Value {
    json!({})
}
type Failure = (StatusCode, Json<serde_json::Value>);
fn invalid(message: &str) -> Failure {
    (StatusCode::BAD_REQUEST, Json(json!({"error":message})))
}
fn storage(error: sqlx::Error) -> Failure {
    tracing::error!(%error,"Email template persistence failed");
    if let Some(db) = error.as_database_error() {
        if db.code().as_deref() == Some("23505") {
            return (
                StatusCode::CONFLICT,
                Json(json!({"error":"A template with this slug already exists"})),
            );
        }
        if db.code().as_deref() == Some("23503") {
            return (
                StatusCode::CONFLICT,
                Json(json!({"error":"This template is still referenced by a campaign"})),
            );
        }
    }
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error":"Email template storage is unavailable"})),
    )
}
fn validate_text(value: &str, max: usize, required: bool, header: bool) -> Result<(), Failure> {
    if value.len() > 4_000_000
        || value.chars().count() > max
        || value.contains('\0')
        || required && value.trim().is_empty()
        || header && value.contains(['\r', '\n'])
    {
        return Err(invalid(
            "Template fields are empty, contain invalid characters or exceed their limits",
        ));
    }
    Ok(())
}
fn validate_template(
    subject: Option<&str>,
    body: Option<&str>,
    text: Option<&str>,
    name: Option<&str>,
    kind: Option<&str>,
    variables: Option<&serde_json::Value>,
) -> Result<(), Failure> {
    if let Some(value) = name {
        validate_text(value, 255, true, true)?;
    }
    if let Some(value) = kind {
        validate_text(value, 50, true, true)?;
    }
    for (value, max, required, header) in [
        (subject, 255, true, true),
        (body, 4_000_000, true, false),
        (text, 4_000_000, false, false),
    ] {
        if let Some(value) = value {
            validate_text(value, max, required, header)?;
            handlebars::Handlebars::new()
                .register_template_string("validation", value)
                .map_err(|_| invalid("Template syntax is invalid"))?;
        }
    }
    if let Some(value) = variables {
        if !valid_json(value) || serde_json::to_vec(value).map_or(true, |v| v.len() > 65536) {
            return Err(invalid("Variables exceed64KiB or contain null characters"));
        }
        let array = value
            .as_array()
            .filter(|v| v.len() <= 1000)
            .ok_or_else(|| invalid("Variables must be an array of names"))?;
        for item in array {
            let name = item
                .as_str()
                .or_else(|| item.get("key").and_then(serde_json::Value::as_str))
                .ok_or_else(|| invalid("Variables must contain names or objects with a key"))?;
            validate_text(name, 128, true, true)?;
        }
    }
    Ok(())
}
fn template_slug(value: &str) -> Result<String, Failure> {
    let value = value.trim().to_lowercase();
    if value.is_empty()
        || value.len() > 255
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return Err(invalid(
            "Slug must contain 1–255 letters, digits, hyphens or underscores",
        ));
    }
    Ok(value)
}
async fn revoke(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    alias: &str,
    id: i64,
    actor: i64,
) -> Result<(), Failure> {
    sqlx::query("INSERT INTO email_template_revocations(alias,template_id,revoked_by) VALUES($1,$2,$3) ON CONFLICT(alias) DO UPDATE SET template_id=EXCLUDED.template_id,revoked_by=EXCLUDED.revoked_by,revoked_at=NOW()")
        .bind(alias).bind(id).bind(actor).execute(&mut **tx).await.map_err(storage)?;
    Ok(())
}
async fn audit(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    action: &str,
    actor: i64,
    template: i64,
) -> Result<(), Failure> {
    crate::services::crm_logs::record(
        tx,
        "info",
        "email",
        action,
        json!({"template_id":template}),
        Some(actor),
    )
    .await
    .map_err(storage)?;
    Ok(())
}
fn valid_json(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(value) => !value.contains('\0'),
        serde_json::Value::Array(value) => value.iter().all(valid_json),
        serde_json::Value::Object(value) => value
            .iter()
            .all(|(k, v)| !k.contains('\0') && valid_json(v)),
        _ => true,
    }
}
fn model(value: &serde_json::Value) -> Result<(), Failure> {
    if !value.is_object()
        || !valid_json(value)
        || serde_json::to_vec(value).map_or(true, |v| v.len() > 65536)
    {
        return Err(invalid("Template data must be a JSON object within64KiB"));
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

pub(crate) struct TemplateCatalogPage {
    pub(crate) rows: Vec<EmailTemplateRow>,
    pub(crate) page: i64,
    pub(crate) per_page: i64,
    pub(crate) total: i64,
}
/// Pure catalog read shared by the global API and narrowly scoped CRM picker.
/// No privileged extractor is constructed or accepted by this function.
pub(crate) async fn read_catalog(
    pool: &sqlx::PgPool,
    query: &TemplateListQuery,
    active_only: bool,
) -> Result<TemplateCatalogPage, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).clamp(1, 10_000);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1).saturating_mul(per_page);
    let category = query
        .category
        .as_deref()
        .filter(|value| !value.trim().is_empty());
    if let Some(value) = category {
        validate_text(value, 50, true, true)?;
    }
    let search = query
        .search
        .as_ref()
        .map(|value| format!("%{}%", value.replace('\0', "")));
    let mut tx = pool.begin().await.map_err(storage)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    let rows=sqlx::query_as(template_sql!("SELECT "," FROM email_templates WHERE ($1::text IS NULL OR name ILIKE $1 OR subject ILIKE $1 OR slug ILIKE $1) AND ($2::text IS NULL OR LOWER(COALESCE(template_type,'transactional'))=LOWER($2)) AND (NOT $5 OR is_active=true) ORDER BY created_at DESC,id DESC LIMIT $3 OFFSET $4"))
        .bind(search.as_deref()).bind(category).bind(per_page).bind(offset).bind(active_only).fetch_all(&mut *tx).await.map_err(storage)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM email_templates WHERE ($1::text IS NULL OR name ILIKE $1 OR subject ILIKE $1 OR slug ILIKE $1) AND ($2::text IS NULL OR LOWER(COALESCE(template_type,'transactional'))=LOWER($2)) AND (NOT $3 OR is_active=true)")
        .bind(search.as_deref()).bind(category).bind(active_only).fetch_one(&mut *tx).await.map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    Ok(TemplateCatalogPage {
        rows,
        page,
        per_page,
        total,
    })
}

/// Native catalog entry keeps its existing global AdminUser boundary and DTO.
async fn list_templates(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<TemplateListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = read_catalog(&state.db.pool, &query, false).await?;
    Ok(Json(
        json!({"data":result.rows,"meta":{"current_page":result.page,"per_page":result.per_page,"total":result.total,"total_pages":(result.total+result.per_page-1)/result.per_page}}),
    ))
}

/// Get single email template by ID (admin)
async fn get_template(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let template: EmailTemplateRow = sqlx::query_as(template_sql!(
        "SELECT ",
        " FROM email_templates WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(storage)?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Template not found"})),
        )
    })?;

    Ok(Json(json!({"data": template})))
}

/// Create new email template (admin)
async fn create_template(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateTemplateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "template_create",
        user_id = %user.id,
        template_name = %input.name,
        "Admin creating email template"
    );

    validate_template(
        Some(&input.subject),
        Some(&input.body),
        input.body_text.as_deref(),
        Some(&input.name),
        input.email_type.as_deref(),
        input.variables.as_ref(),
    )?;
    let generated = slug::slugify(&input.name);
    let slug = template_slug(
        input
            .slug
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or(&generated),
    )?;
    let metadata = catalog_metadata(
        input.description.as_ref(),
        input.thumbnail.as_ref(),
        input.design_json.as_ref(),
        input.is_public.as_ref(),
        input.tags.as_ref(),
    )?;
    let variables = input.variables.unwrap_or(json!([]));
    let mut tx = state.db.pool.begin().await.map_err(storage)?;

    let template: EmailTemplateRow = sqlx::query_as(template_sql!("\n        INSERT INTO email_templates (name, slug, subject, html_content, variables, is_active, text_content, template_type, local_content_authoritative, catalog_metadata, created_at, updated_at)\n        VALUES ($1, $2, $3, $4, $5, $6, NULLIF($7,''), COALESCE($8,'transactional'), true, $9, NOW(), NOW())\n        RETURNING ","\n        "))
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.subject)
    .bind(&input.body)
    .bind(&variables)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.body_text).bind(&input.email_type).bind(metadata)
    .fetch_one(&mut *tx).await.map_err(storage)?;
    sqlx::query("DELETE FROM email_template_revocations WHERE alias=$1")
        .bind(&template.slug)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    audit(&mut tx, "Email template created", user.id, template.id).await?;
    tx.commit().await.map_err(storage)?;

    Ok(Json(
        json!({"data": template, "message": "Template created successfully"}),
    ))
}

/// Update email template (admin)
async fn update_template(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTemplateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "template_update",
        user_id = %user.id,
        template_id = %id,
        "Admin updating email template"
    );

    validate_template(
        input.subject.as_deref(),
        input.body.as_deref(),
        input.body_text.as_deref(),
        input.name.as_deref(),
        input.email_type.as_deref(),
        input.variables.as_ref(),
    )?;
    let metadata = catalog_metadata(
        input.description.as_ref(),
        input.thumbnail.as_ref(),
        input.design_json.as_ref(),
        input.is_public.as_ref(),
        input.tags.as_ref(),
    )?;
    let slug = input
        .slug
        .as_deref()
        .filter(|v| !v.trim().is_empty())
        .map(template_slug)
        .transpose()?;
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let previous: String =
        sqlx::query_scalar("SELECT slug FROM email_templates WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error":"Template not found"})),
                )
            })?;
    let authoritative = input.subject.is_some()
        || input.body.is_some()
        || input.body_text.is_some()
        || slug.as_ref().is_some_and(|v| v != &previous);
    let template: EmailTemplateRow = sqlx::query_as(template_sql!("\n        UPDATE email_templates SET\n            name = COALESCE($2, name),\n            slug = COALESCE($7,slug),\n            text_content = CASE WHEN $8::text IS NULL THEN text_content ELSE NULLIF($8,'') END,\n            template_type = COALESCE($9,template_type),\n            local_content_authoritative = CASE WHEN $10 THEN true ELSE local_content_authoritative END,\n            catalog_metadata = catalog_metadata || $11::jsonb,\n            subject = COALESCE($3, subject),\n            html_content = COALESCE($4, html_content),\n            variables = COALESCE($5, variables),\n            is_active = COALESCE($6, is_active),\n            updated_at = NOW()\n        WHERE id = $1\n        RETURNING ","\n        "))
    .bind(id)
    .bind(&input.name)
    .bind(&input.subject)
    .bind(&input.body)
    .bind(&input.variables)
    .bind(input.is_active).bind(slug).bind(&input.body_text).bind(&input.email_type).bind(authoritative).bind(metadata)
    .fetch_optional(&mut *tx).await.map_err(storage)?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Template not found"})),
        )
    })?;

    if previous != template.slug {
        revoke(&mut tx, &previous, id, user.id).await?;
    }
    if authoritative {
        sqlx::query("DELETE FROM email_template_revocations WHERE alias=$1")
            .bind(&template.slug)
            .execute(&mut *tx)
            .await
            .map_err(storage)?;
    }
    audit(&mut tx, "Email template updated", user.id, template.id).await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(
        json!({"data": template, "message": "Template updated successfully"}),
    ))
}

/// Delete email template (admin)
async fn delete_template(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "template_delete",
        user_id = %user.id,
        template_id = %id,
        "Admin deleting email template"
    );

    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let alias: String =
        sqlx::query_scalar("DELETE FROM email_templates WHERE id=$1 RETURNING slug")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error":"Template not found"})),
                )
            })?;
    revoke(&mut tx, &alias, id, user.id).await?;

    audit(&mut tx, "Email template deleted", user.id, id).await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(
        json!({"message": "Template deleted successfully","deleted":true,"id":id}),
    ))
}

/// Preview the same stored content/strict variables used by local template delivery.
async fn preview_template(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<PreviewRequest>,
) -> Result<Json<serde_json::Value>, Failure> {
    model(&input.data)?;
    let template: EmailTemplateRow = sqlx::query_as(template_sql!(
        "SELECT ",
        " FROM email_templates WHERE id=$1"
    ))
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(storage)?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error":"Template not found"})),
        )
    })?;
    let selected = state.services.email.resolved().await.ok();
    let brand = selected
        .as_ref()
        .map_or(state.services.email.from_name(), |email| email.from_name());
    let rendered = crate::services::transactional_templates::render_content(
        &template.subject,
        &template.body,
        template.body_text.as_deref(),
        input.data,
        &state.config.app_url,
        brand,
    )
    .map_err(|e| invalid(&e.to_string()))?;
    Ok(Json(
        json!({"data":{"html":rendered.html,"body_html":rendered.html,"subject":rendered.subject,"text":rendered.text,"body_text":rendered.text,"xss_protected":true}}),
    ))
}

/// Validate email address format
fn is_valid_email(email: &str) -> bool {
    if email.len() > 254 || email.len() < 5 {
        return false;
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || local.len() > 64 || domain.is_empty() || !domain.contains('.') {
        return false;
    }
    let valid_local = |c: char| c.is_alphanumeric() || "!#$%&'*+/=?^_`{|}~.-".contains(c);
    let valid_domain = |c: char| c.is_alphanumeric() || c == '.' || c == '-';
    local.chars().all(valid_local) && domain.chars().all(valid_domain)
}

/// Send test email (admin)
async fn send_test_email(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<SendTestRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    model(&input.data)?;
    // Validate email format
    let email = input.email.trim().to_lowercase();
    if !is_valid_email(&email) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid email address format"})),
        ));
    }

    tracing::info!(
        target: "security",
        event = "template_test_send",
        user_id = %user.id,
        template_id = %id,
        recipient = %email,
        "Admin sending test email"
    );

    // Resolve the active provider and require a persisted submission acknowledgment.
    let template: Option<EmailTemplateRow> = sqlx::query_as(template_sql!(
        "SELECT ",
        " FROM email_templates WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(storage)?;

    let template = template.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error":"Template not found"})),
    ))?;
    if !template.is_active {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Template is inactive"})),
        ));
    }
    let alias = template.slug;

    let mut data = input.data;
    let object = data
        .as_object_mut()
        .ok_or_else(|| invalid("Template data must be an object"))?;
    for (key, value) in [
        ("recipient", json!(email)),
        ("name", json!(user.name)),
        ("admin_name", json!(user.name)),
        ("template_id", json!(id)),
        ("app_url", json!(state.config.app_url)),
        ("test", json!(true)),
    ] {
        object.entry(key).or_insert(value);
    }
    let service = state.services.email.resolved().await.map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error":"Email provider configuration is unavailable"})),
        )
    })?;
    let mut rendered = crate::services::transactional_templates::render_content(
        &template.subject,
        &template.body,
        template.body_text.as_deref(),
        data.clone(),
        &state.config.app_url,
        service.from_name(),
    )
    .map_err(|e| invalid(&e.to_string()))?;

    rendered.template_id = Some(id);

    match state
        .services
        .email
        .send_template_receipt(&email, &alias, &rendered, &data)
        .await
    {
        Ok(message_id) => Ok(Json(json!({
            "message": format!("Email provider accepted the test message for {}", email),
            "provider_message_id":message_id,
            "submission_status":"accepted",
            "template_id": id,
            "alias": alias,
            "email_service_enabled": true,
        }))),
        Err(e) => {
            let (status, submission) =
                if e.is::<crate::services::campaign_mailer::UncertainSubmission>() {
                    (StatusCode::BAD_GATEWAY, "unknown")
                } else if e.is::<crate::services::campaign_mailer::RejectedSubmission>() {
                    (StatusCode::BAD_GATEWAY, "rejected")
                } else {
                    (StatusCode::SERVICE_UNAVAILABLE, "unavailable")
                };
            Err((
                status,
                Json(
                    json!({"error":format!("Test submission could not be confirmed: {e}"),"template_id":id,"submission_status":submission}),
                ),
            ))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

async fn duplicate_template(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, Failure> {
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let original: EmailTemplateRow = sqlx::query_as(template_sql!(
        "SELECT ",
        " FROM email_templates WHERE id=$1 FOR SHARE"
    ))
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(storage)?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error":"Template not found"})),
        )
    })?;
    let name = format!(
        "{} (Copy)",
        original.name.chars().take(248).collect::<String>()
    );
    let slug = format!(
        "{}-copy-{}",
        original.slug.chars().take(210).collect::<String>(),
        uuid::Uuid::new_v4()
    );
    let copy:EmailTemplateRow=sqlx::query_as(template_sql!("INSERT INTO email_templates(name,slug,subject,html_content,text_content,template_type,variables,is_active,catalog_metadata,local_content_authoritative) SELECT $2,$3,subject,html_content,text_content,template_type,variables,is_active,catalog_metadata,true FROM email_templates WHERE id=$1 RETURNING ",""))
        .bind(id).bind(name).bind(&slug).fetch_one(&mut *tx).await.map_err(storage)?;
    sqlx::query("DELETE FROM email_template_revocations WHERE alias=$1")
        .bind(slug)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    audit(&mut tx, "Email template duplicated", user.id, copy.id).await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(json!({"data":copy,"message":"Template duplicated"})))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_templates).post(create_template))
        .route(
            "/{id}",
            get(get_template)
                .put(update_template)
                .delete(delete_template),
        )
        .route("/{id}/preview", post(preview_template))
        .route("/{id}/test", post(send_test_email))
        .route("/{id}/duplicate", post(duplicate_template))
        .merge(super::email_settings::router())
        .layer(DefaultBodyLimit::max(9_000_000))
}
