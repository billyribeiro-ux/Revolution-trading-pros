//! Admin categories routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::post::{Category, CreateCategoryRequest, PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;

/// GET /api/admin/categories - List all categories
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let categories: Vec<Category> = ctx.data.db.query(
        "SELECT * FROM categories ORDER BY sort_order ASC, name ASC",
        vec![]
    ).await?;

    Response::from_json(&categories)
}

/// POST /api/admin/categories - Create a new category
pub async fn create(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let body: CreateCategoryRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let category_id = uuid::Uuid::new_v4();
    let now = crate::utils::now();
    let slug = body.slug.unwrap_or_else(|| slug::slugify(&body.name));

    // Check slug uniqueness
    let existing: Option<Category> = ctx.data.db.query_one(
        "SELECT * FROM categories WHERE slug = $1",
        vec![serde_json::json!(&slug)]
    ).await?;

    if existing.is_some() {
        return Err(ApiError::Conflict("A category with this slug already exists".to_string()).into());
    }

    ctx.data.db.execute(
        r#"
        INSERT INTO categories (
            id, name, slug, description, parent_id, image_url, post_count,
            is_visible, sort_order, meta_title, meta_description, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, 0, $7, $8, $9, $10, $11, $11)
        "#,
        vec![
            serde_json::json!(category_id.to_string()),
            serde_json::json!(body.name),
            serde_json::json!(slug),
            serde_json::json!(body.description),
            serde_json::json!(body.parent_id.map(|id| id.to_string())),
            serde_json::json!(body.image_url),
            serde_json::json!(body.is_visible.unwrap_or(true)),
            serde_json::json!(body.sort_order.unwrap_or(0)),
            serde_json::json!(body.meta_title),
            serde_json::json!(body.meta_description),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    let category: Category = ctx.data.db.query_one(
        "SELECT * FROM categories WHERE id = $1",
        vec![serde_json::json!(category_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve created category".to_string()))?;

    Response::from_json(&category).map(|r| r.with_status(201))
}

/// PUT /api/admin/categories/:id - Update a category
pub async fn update(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing category id".to_string()))?;

    let body: CreateCategoryRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let now = crate::utils::now();

    ctx.data.db.execute(
        r#"
        UPDATE categories SET
            name = $1, slug = $2, description = $3, parent_id = $4, image_url = $5,
            is_visible = $6, sort_order = $7, meta_title = $8, meta_description = $9, updated_at = $10
        WHERE id = $11
        "#,
        vec![
            serde_json::json!(body.name),
            serde_json::json!(body.slug.unwrap_or_else(|| slug::slugify(&body.name))),
            serde_json::json!(body.description),
            serde_json::json!(body.parent_id.map(|id| id.to_string())),
            serde_json::json!(body.image_url),
            serde_json::json!(body.is_visible.unwrap_or(true)),
            serde_json::json!(body.sort_order.unwrap_or(0)),
            serde_json::json!(body.meta_title),
            serde_json::json!(body.meta_description),
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(id),
        ]
    ).await?;

    let category: Category = ctx.data.db.query_one(
        "SELECT * FROM categories WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?
    .ok_or_else(|| ApiError::NotFound("Category not found".to_string()))?;

    Response::from_json(&category)
}

/// DELETE /api/admin/categories/:id - Delete a category
pub async fn delete(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing category id".to_string()))?;

    // Check if category has posts
    let post_count: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM posts WHERE category_id = $1",
        vec![serde_json::json!(id)]
    ).await?
    .map(|r| r.count)
    .unwrap_or(0);

    if post_count > 0 {
        return Err(ApiError::Conflict(format!(
            "Cannot delete category with {} posts. Move or delete posts first.",
            post_count
        )).into());
    }

    let deleted = ctx.data.db.execute(
        "DELETE FROM categories WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    if deleted == 0 {
        return Err(ApiError::NotFound("Category not found".to_string()).into());
    }

    Response::from_json(&serde_json::json!({
        "message": "Category deleted successfully"
    }))
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}
