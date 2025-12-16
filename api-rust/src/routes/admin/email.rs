//! Admin email routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::email::{EmailTemplate, EmailCampaign, EmailSubscriber, CreateTemplateRequest, CreateCampaignRequest};
use crate::models::post::{PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;

/// GET /api/admin/email/templates - List email templates
pub async fn templates_list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let templates: Vec<EmailTemplate> = ctx.data.db.query(
        "SELECT * FROM email_templates ORDER BY created_at DESC",
        vec![]
    ).await?;

    Response::from_json(&templates)
}

/// POST /api/admin/email/templates - Create email template
pub async fn templates_create(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let body: CreateTemplateRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let template_id = uuid::Uuid::new_v4();
    let now = crate::utils::now();
    let slug = body.slug.unwrap_or_else(|| slug::slugify(&body.name));

    ctx.data.db.execute(
        r#"
        INSERT INTO email_templates (
            id, name, slug, subject, html_content, text_content, template_type,
            variables, is_active, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $10)
        "#,
        vec![
            serde_json::json!(template_id.to_string()),
            serde_json::json!(body.name),
            serde_json::json!(slug),
            serde_json::json!(body.subject),
            serde_json::json!(body.html_content),
            serde_json::json!(body.text_content),
            serde_json::json!(body.template_type.map(|t| format!("{:?}", t).to_lowercase()).unwrap_or_else(|| "custom".to_string())),
            serde_json::json!(body.variables),
            serde_json::json!(body.is_active.unwrap_or(true)),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    let template: EmailTemplate = ctx.data.db.query_one(
        "SELECT * FROM email_templates WHERE id = $1",
        vec![serde_json::json!(template_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve created template".to_string()))?;

    Response::from_json(&template).map(|r| r.with_status(201))
}

/// GET /api/admin/email/campaigns - List email campaigns
pub async fn campaigns_list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: CampaignListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM email_campaigns WHERE 1=1");
    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND status = ${}", param_idx));
        params.push(serde_json::json!(status));
    }

    // Count total
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*) as count");
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    sql.push_str(" ORDER BY created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let campaigns: Vec<EmailCampaign> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: campaigns,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// POST /api/admin/email/campaigns - Create email campaign
pub async fn campaigns_create(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let body: CreateCampaignRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let campaign_id = uuid::Uuid::new_v4();
    let now = crate::utils::now();

    ctx.data.db.execute(
        r#"
        INSERT INTO email_campaigns (
            id, name, subject, preview_text, from_name, from_email, reply_to,
            template_id, html_content, text_content, status, segment_type,
            segment_filters, scheduled_at, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'draft', $11, $12, $13, $14, $14)
        "#,
        vec![
            serde_json::json!(campaign_id.to_string()),
            serde_json::json!(body.name),
            serde_json::json!(body.subject),
            serde_json::json!(body.preview_text),
            serde_json::json!(body.from_name.unwrap_or_else(|| "Revolution Trading Pros".to_string())),
            serde_json::json!(body.from_email.unwrap_or_else(|| "noreply@revolutiontradingpros.com".to_string())),
            serde_json::json!(body.reply_to),
            serde_json::json!(body.template_id.map(|id| id.to_string())),
            serde_json::json!(body.html_content),
            serde_json::json!(body.text_content),
            serde_json::json!(body.segment_type.map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_else(|| "all".to_string())),
            serde_json::json!(body.segment_filters),
            serde_json::json!(body.scheduled_at.map(|dt| dt.to_rfc3339())),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    let campaign: EmailCampaign = ctx.data.db.query_one(
        "SELECT * FROM email_campaigns WHERE id = $1",
        vec![serde_json::json!(campaign_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve created campaign".to_string()))?;

    Response::from_json(&campaign).map(|r| r.with_status(201))
}

/// GET /api/admin/email/subscribers - List email subscribers
pub async fn subscribers_list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: SubscriberListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM email_subscribers WHERE 1=1");
    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND status = ${}", param_idx));
        params.push(serde_json::json!(status));
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(" AND (email ILIKE ${} OR name ILIKE ${})", param_idx, param_idx));
        params.push(serde_json::json!(format!("%{}%", search)));
    }

    // Count total
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*) as count");
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    sql.push_str(" ORDER BY created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let subscribers: Vec<EmailSubscriber> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: subscribers,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

#[derive(serde::Deserialize, Default)]
struct CampaignListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct SubscriberListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
    search: Option<String>,
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}
