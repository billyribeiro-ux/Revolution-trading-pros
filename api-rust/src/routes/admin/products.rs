//! Admin products routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::product::{Product, CreateProductRequest, UpdateProductRequest, ProductListQuery};
use crate::models::post::{PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;

/// GET /api/admin/products - List all products
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: ProductListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM products WHERE 1=1");
    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(product_type) = &query.product_type {
        sql.push_str(&format!(" AND product_type = ${}", param_idx));
        params.push(serde_json::json!(format!("{:?}", product_type).to_lowercase()));
        param_idx += 1;
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(" AND (name ILIKE ${} OR description ILIKE ${})", param_idx, param_idx));
        params.push(serde_json::json!(format!("%{}%", search)));
        param_idx += 1;
    }

    // Count total
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*) as count");
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    // Add sorting and pagination
    sql.push_str(" ORDER BY sort_order ASC, created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let products: Vec<Product> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: products,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// POST /api/admin/products - Create a new product
pub async fn create(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let body: CreateProductRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let product_id = uuid::Uuid::new_v4();
    let now = crate::utils::now();
    let slug = body.slug.unwrap_or_else(|| slug::slugify(&body.name));

    // Check slug uniqueness
    let existing: Option<Product> = ctx.data.db.query_one(
        "SELECT * FROM products WHERE slug = $1",
        vec![serde_json::json!(&slug)]
    ).await?;

    if existing.is_some() {
        return Err(ApiError::Conflict("A product with this slug already exists".to_string()).into());
    }

    ctx.data.db.execute(
        r#"
        INSERT INTO products (
            id, name, slug, description, short_description, product_type, price, compare_price,
            currency, image_url, gallery, features, is_active, is_featured, stock, download_url,
            meta_title, meta_description, sort_order, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $20)
        "#,
        vec![
            serde_json::json!(product_id.to_string()),
            serde_json::json!(body.name),
            serde_json::json!(slug),
            serde_json::json!(body.description),
            serde_json::json!(body.short_description),
            serde_json::json!(format!("{:?}", body.product_type).to_lowercase()),
            serde_json::json!(body.price),
            serde_json::json!(body.compare_price),
            serde_json::json!(body.currency.unwrap_or_else(|| "usd".to_string())),
            serde_json::json!(body.image_url),
            serde_json::json!(body.gallery),
            serde_json::json!(body.features),
            serde_json::json!(body.is_active.unwrap_or(true)),
            serde_json::json!(body.is_featured.unwrap_or(false)),
            serde_json::json!(body.stock),
            serde_json::json!(body.download_url),
            serde_json::json!(body.meta_title),
            serde_json::json!(body.meta_description),
            serde_json::json!(0),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    let product: Product = ctx.data.db.query_one(
        "SELECT * FROM products WHERE id = $1",
        vec![serde_json::json!(product_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve created product".to_string()))?;

    Response::from_json(&product).map(|r| r.with_status(201))
}

/// PUT /api/admin/products/:id - Update a product
pub async fn update(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing product id".to_string()))?;

    let body: UpdateProductRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let now = crate::utils::now();

    // Build dynamic update query
    let mut updates = vec!["updated_at = $1".to_string()];
    let mut params: Vec<serde_json::Value> = vec![serde_json::json!(now.to_rfc3339())];
    let mut param_idx = 2;

    if let Some(name) = &body.name {
        updates.push(format!("name = ${}", param_idx));
        params.push(serde_json::json!(name));
        param_idx += 1;
    }

    if let Some(slug) = &body.slug {
        updates.push(format!("slug = ${}", param_idx));
        params.push(serde_json::json!(slug));
        param_idx += 1;
    }

    if let Some(description) = &body.description {
        updates.push(format!("description = ${}", param_idx));
        params.push(serde_json::json!(description));
        param_idx += 1;
    }

    if let Some(price) = body.price {
        updates.push(format!("price = ${}", param_idx));
        params.push(serde_json::json!(price));
        param_idx += 1;
    }

    if let Some(compare_price) = body.compare_price {
        updates.push(format!("compare_price = ${}", param_idx));
        params.push(serde_json::json!(compare_price));
        param_idx += 1;
    }

    if let Some(image_url) = &body.image_url {
        updates.push(format!("image_url = ${}", param_idx));
        params.push(serde_json::json!(image_url));
        param_idx += 1;
    }

    if let Some(is_active) = body.is_active {
        updates.push(format!("is_active = ${}", param_idx));
        params.push(serde_json::json!(is_active));
        param_idx += 1;
    }

    if let Some(is_featured) = body.is_featured {
        updates.push(format!("is_featured = ${}", param_idx));
        params.push(serde_json::json!(is_featured));
        param_idx += 1;
    }

    params.push(serde_json::json!(id));

    let sql = format!(
        "UPDATE products SET {} WHERE id = ${}",
        updates.join(", "),
        param_idx
    );

    let updated = ctx.data.db.execute(&sql, params).await?;

    if updated == 0 {
        return Err(ApiError::NotFound("Product not found".to_string()).into());
    }

    let product: Product = ctx.data.db.query_one(
        "SELECT * FROM products WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?
    .ok_or_else(|| ApiError::NotFound("Product not found".to_string()))?;

    Response::from_json(&product)
}

/// DELETE /api/admin/products/:id - Delete a product
pub async fn delete(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing product id".to_string()))?;

    // Check if product has active subscriptions
    let sub_count: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM user_subscriptions WHERE product_id = $1 AND status = 'active'",
        vec![serde_json::json!(id)]
    ).await?
    .map(|r| r.count)
    .unwrap_or(0);

    if sub_count > 0 {
        return Err(ApiError::Conflict(format!(
            "Cannot delete product with {} active subscriptions",
            sub_count
        )).into());
    }

    let deleted = ctx.data.db.execute(
        "DELETE FROM products WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    if deleted == 0 {
        return Err(ApiError::NotFound("Product not found".to_string()).into());
    }

    Response::from_json(&serde_json::json!({
        "message": "Product deleted successfully"
    }))
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}
