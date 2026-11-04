//! Durable CRM email templates and the library's category/prebuilt contracts.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize, Default)]
struct Filters {
    search: Option<String>,
    category: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}

#[derive(Deserialize)]
struct TemplateInput {
    title: String,
    subject: Option<String>,
    #[serde(alias = "body")]
    content: String,
    category: Option<String>,
    #[serde(default = "raw_design")]
    design_template: String,
    template_config: Option<Value>,
    #[serde(default)]
    is_default: bool,
}
fn raw_design() -> String {
    "raw".into()
}
fn db_error(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}

impl TemplateInput {
    fn validate(&mut self) -> Result<(), ApiError> {
        self.title = self.title.trim().to_owned();
        self.category = self
            .category
            .take()
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty());
        if self.title.is_empty() || self.title.chars().count() > 255 {
            return Err(ApiError::validation_error(
                "Template title must contain 1–255 characters",
            ));
        }
        if self
            .subject
            .as_ref()
            .is_some_and(|s| s.chars().count() > 500)
            || self
                .category
                .as_ref()
                .is_some_and(|s| s.chars().count() > 100)
            || self.content.len() > 1_000_000
            || self.content.trim().is_empty()
            || self
                .template_config
                .as_ref()
                .is_some_and(|v| !v.is_object() || v.to_string().len() > 256_000)
            || !matches!(self.design_template.as_str(), "raw" | "visual")
            || self.title.contains('\0')
            || self.content.contains('\0')
            || self.subject.as_ref().is_some_and(|s| s.contains('\0'))
            || self.category.as_ref().is_some_and(|s| s.contains('\0'))
        {
            return Err(ApiError::validation_error(
                "Provide nonempty HTML (up to 1 MB), a valid design, object configuration, and fields within their length limits",
            ));
        }
        Ok(())
    }
}

async fn list_templates(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search = filters.search.map(|s| format!("%{}%", s.replace('\0', "")));
    let rows: Vec<(Value,)> = sqlx::query_as("SELECT row_to_json(t) FROM (SELECT id::text AS id, COALESCE(title,name) AS title, subject, COALESCE(body,content) AS content, category, thumbnail, is_global, design_template, template_config, is_default, created_by::text AS created_by, created_at, updated_at FROM crm_templates WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1) AND ($2::text IS NULL OR category=$2) ORDER BY created_at DESC,id DESC LIMIT $3 OFFSET $4) t")
        .bind(&search).bind(&filters.category).bind(per_page).bind(page.saturating_sub(1).saturating_mul(per_page)).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_templates WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1) AND ($2::text IS NULL OR category=$2)")
        .bind(&search).bind(&filters.category).fetch_one(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(
        json!({"data":rows.into_iter().map(|r| r.0).collect::<Vec<_>>(),"meta":{"total":total,"current_page":page,"per_page":per_page,"total_pages":((total+per_page-1)/per_page).max(1)}}),
    ))
}

async fn categories(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Value>, ApiError> {
    let rows: Vec<(String,i64)> = sqlx::query_as("SELECT category,COUNT(*) FROM crm_templates WHERE category IS NOT NULL AND category<>'' GROUP BY category ORDER BY category")
        .fetch_all(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(json!(
        rows.into_iter()
            .map(|(name, count)| json!({"id":name,"slug":name,"name":name,"templates_count":count}))
            .collect::<Vec<_>>()
    )))
}

async fn prebuilt(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Value>, ApiError> {
    let rows: Vec<(Value,)> = sqlx::query_as("SELECT row_to_json(t) FROM (SELECT id::text AS id, COALESCE(title,name) AS title, subject, COALESCE(body,content) AS content, category, thumbnail, is_global, design_template, template_config, is_default, created_by::text AS created_by, created_at, updated_at FROM crm_templates WHERE is_global=true ORDER BY created_at DESC,id DESC) t")
        .fetch_all(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(json!(
        rows.into_iter().map(|r| r.0).collect::<Vec<_>>()
    )))
}

async fn get_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let row: Option<(Value,)> = sqlx::query_as("SELECT row_to_json(t) FROM (SELECT id::text AS id, COALESCE(title,name) AS title, subject, COALESCE(body,content) AS content, category, thumbnail, is_global, design_template, template_config, is_default, created_by::text AS created_by, created_at, updated_at FROM crm_templates WHERE id=$1) t")
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(db_error)?;
    Ok(Json(
        row.ok_or_else(|| ApiError::not_found("Template not found"))?
            .0,
    ))
}

async fn save(
    state: &AppState,
    id: Option<i64>,
    creator: i64,
    mut input: TemplateInput,
) -> Result<Json<Value>, ApiError> {
    input.validate()?;
    let row: Option<(Value,)> = if let Some(id) = id {
        sqlx::query_as("WITH saved AS (UPDATE crm_templates SET name=$2,title=$2,subject=$3,content=$4,body=$4,category=$5,design_template=$6,template_config=$7,is_default=$8,updated_at=NOW() WHERE id=$1 RETURNING *) SELECT row_to_json(t) FROM (SELECT id::text AS id, COALESCE(title,name) AS title, subject, COALESCE(body,content) AS content, category, thumbnail, is_global, design_template, template_config, is_default, created_by::text AS created_by, created_at, updated_at FROM saved) t")
            .bind(id).bind(&input.title).bind(&input.subject).bind(&input.content).bind(&input.category).bind(&input.design_template).bind(&input.template_config).bind(input.is_default).fetch_optional(&state.db.pool).await.map_err(db_error)?
    } else {
        sqlx::query_as("WITH saved AS (INSERT INTO crm_templates(name,title,subject,content,body,category,design_template,template_config,is_default,created_by) VALUES($1,$1,$2,$3,$3,$4,$5,$6,$7,$8) RETURNING *) SELECT row_to_json(t) FROM (SELECT id::text AS id, COALESCE(title,name) AS title, subject, COALESCE(body,content) AS content, category, thumbnail, is_global, design_template, template_config, is_default, created_by::text AS created_by, created_at, updated_at FROM saved) t")
            .bind(&input.title).bind(&input.subject).bind(&input.content).bind(&input.category).bind(&input.design_template).bind(&input.template_config).bind(input.is_default).bind(creator).fetch_optional(&state.db.pool).await.map_err(db_error)?
    };
    Ok(Json(
        row.ok_or_else(|| ApiError::not_found("Template not found"))?
            .0,
    ))
}
async fn create_template(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<TemplateInput>,
) -> Result<Json<Value>, ApiError> {
    save(&state, None, admin.0.id, input).await
}
async fn update_template(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<TemplateInput>,
) -> Result<Json<Value>, ApiError> {
    save(&state, Some(id), admin.0.id, input).await
}
async fn duplicate_template(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let row: Option<(Value,)> = sqlx::query_as("WITH saved AS (INSERT INTO crm_templates(name,title,subject,content,body,category,thumbnail,design_template,template_config,created_by) SELECT LEFT(COALESCE(title,name),248)||' (copy)',LEFT(COALESCE(title,name),248)||' (copy)',subject,content,body,category,thumbnail,design_template,template_config,$2 FROM crm_templates WHERE id=$1 RETURNING *) SELECT row_to_json(t) FROM (SELECT id::text AS id, COALESCE(title,name) AS title, subject, COALESCE(body,content) AS content, category, thumbnail, is_global, design_template, template_config, is_default, created_by::text AS created_by, created_at, updated_at FROM saved) t")
        .bind(id).bind(admin.0.id).fetch_optional(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(
        row.ok_or_else(|| ApiError::not_found("Template not found"))?
            .0,
    ))
}
async fn delete_template(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let result = sqlx::query("DELETE FROM crm_templates WHERE id=$1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(db_error)?;
    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Template not found"));
    }
    Ok(Json(json!({"message":"Template deleted"})))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/templates", get(list_templates).post(create_template))
        .route("/templates/categories", get(categories))
        .route("/templates/prebuilt", get(prebuilt))
        .route(
            "/templates/{id}",
            get(get_template)
                .put(update_template)
                .delete(delete_template),
        )
        .route(
            "/templates/{id}/duplicate",
            axum::routing::post(duplicate_template),
        )
}
