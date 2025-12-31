//! Product routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::{CreateProduct, UpdateProduct, User},
    middleware::admin::AdminUser,
    AppState,
};

/// Product list query parameters
#[derive(Debug, Deserialize)]
pub struct ProductListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub product_type: Option<String>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
}

/// Product response (simplified for API)
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct ProductRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[sqlx(rename = "type")]
    pub product_type: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: f64,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub thumbnail: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// List all products (public)
/// ICT 11+ Security: Fixed SQL injection vulnerability - using parameterized queries
async fn list_products(
    State(state): State<AppState>,
    Query(query): Query<ProductListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build query with parameterized conditions to prevent SQL injection
    let is_active = query.is_active.unwrap_or(true);
    
    let (products, total) = if let Some(ref product_type) = query.product_type {
        if let Some(ref search) = query.search {
            // Both type and search filters
            let search_pattern = format!("%{}%", search);
            let products: Vec<ProductRow> = sqlx::query_as(
                "SELECT * FROM products WHERE is_active = $1 AND type = $2 AND (name ILIKE $3 OR description ILIKE $3) ORDER BY created_at DESC LIMIT $4 OFFSET $5"
            )
            .bind(is_active)
            .bind(product_type)
            .bind(&search_pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM products WHERE is_active = $1 AND type = $2 AND (name ILIKE $3 OR description ILIKE $3)"
            )
            .bind(is_active)
            .bind(product_type)
            .bind(&search_pattern)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

            (products, total)
        } else {
            // Only type filter
            let products: Vec<ProductRow> = sqlx::query_as(
                "SELECT * FROM products WHERE is_active = $1 AND type = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(is_active)
            .bind(product_type)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

            let total: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM products WHERE is_active = $1 AND type = $2"
            )
            .bind(is_active)
            .bind(product_type)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

            (products, total)
        }
    } else if let Some(ref search) = query.search {
        // Only search filter
        let search_pattern = format!("%{}%", search);
        let products: Vec<ProductRow> = sqlx::query_as(
            "SELECT * FROM products WHERE is_active = $1 AND (name ILIKE $2 OR description ILIKE $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4"
        )
        .bind(is_active)
        .bind(&search_pattern)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM products WHERE is_active = $1 AND (name ILIKE $2 OR description ILIKE $2)"
        )
        .bind(is_active)
        .bind(&search_pattern)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        (products, total)
    } else {
        // No filters except is_active
        let products: Vec<ProductRow> = sqlx::query_as(
            "SELECT * FROM products WHERE is_active = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(is_active)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM products WHERE is_active = $1"
        )
        .bind(is_active)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

        (products, total)
    };

    Ok(Json(json!({
        "data": products,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get product by slug (public)
async fn get_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ProductRow>, (StatusCode, Json<serde_json::Value>)> {
    let product: ProductRow = sqlx::query_as("SELECT * FROM products WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Product not found"}))))?;

    Ok(Json(product))
}

/// Create product (admin only)
/// ICT 11+ Security: Admin authorization enforced via AdminUser extractor
async fn create_product(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateProduct>,
) -> Result<Json<ProductRow>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "product_create",
        user_id = %user.id,
        email = %user.email,
        "Admin creating product"
    );

    let slug = slug::slugify(&input.name);

    let product: ProductRow = sqlx::query_as(
        r#"
        INSERT INTO products (name, slug, type, description, long_description, price, is_active, metadata, thumbnail, meta_title, meta_description, indexable, canonical_url, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.product_type)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.metadata)
    .bind(&input.thumbnail)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .bind(input.indexable.unwrap_or(true))
    .bind(&input.canonical_url)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(product))
}

/// Update product (admin only)
/// ICT 11+ Security: Admin authorization enforced via AdminUser extractor
async fn update_product(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateProduct>,
) -> Result<Json<ProductRow>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "product_update",
        user_id = %user.id,
        product_id = %id,
        "Admin updating product"
    );

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if input.name.is_some() { updates.push(format!("name = ${}", { param_idx += 1; param_idx - 1 })); }
    if input.name.is_some() { updates.push(format!("slug = ${}", { param_idx += 1; param_idx - 1 })); }
    if input.product_type.is_some() { updates.push(format!("type = ${}", { param_idx += 1; param_idx - 1 })); }
    if input.description.is_some() { updates.push(format!("description = ${}", { param_idx += 1; param_idx - 1 })); }
    if input.long_description.is_some() { updates.push(format!("long_description = ${}", { param_idx += 1; param_idx - 1 })); }
    if input.price.is_some() { updates.push(format!("price = ${}", { param_idx += 1; param_idx - 1 })); }
    if input.is_active.is_some() { updates.push(format!("is_active = ${}", { param_idx += 1; param_idx - 1 })); }

    updates.push("updated_at = NOW()".to_string());

    if updates.len() <= 1 {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "No fields to update"}))));
    }

    // For simplicity, fetch and update the whole record
    let product: ProductRow = sqlx::query_as(
        &format!("UPDATE products SET {} WHERE id = $1 RETURNING *", updates.join(", "))
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(product))
}

/// Delete product (admin only)
/// ICT 11+ Security: Admin authorization enforced via AdminUser extractor
async fn delete_product(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "product_delete",
        user_id = %user.id,
        product_id = %id,
        "Admin deleting product"
    );

    sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"message": "Product deleted successfully"})))
}

/// Get user's purchased products
async fn get_user_products(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Vec<ProductRow>>, (StatusCode, Json<serde_json::Value>)> {
    let products: Vec<ProductRow> = sqlx::query_as(
        r#"
        SELECT p.* FROM products p
        INNER JOIN user_products up ON p.id = up.product_id
        WHERE up.user_id = $1
        ORDER BY up.purchased_at DESC
        "#
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(products))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_products).post(create_product))
        .route("/my", get(get_user_products))
        .route("/:slug", get(get_product).put(update_product).delete(delete_product))
}
