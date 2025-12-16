//! Public products routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::product::{Product, Indicator, ProductListQuery};
use crate::models::post::{PaginatedResponse, PaginationMeta};

/// GET /api/products - List active products
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let url = req.url()?;
    let query: ProductListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM products WHERE is_active = true");
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

    if let Some(min_price) = query.min_price {
        sql.push_str(&format!(" AND price >= ${}", param_idx));
        params.push(serde_json::json!(min_price));
        param_idx += 1;
    }

    if let Some(max_price) = query.max_price {
        sql.push_str(&format!(" AND price <= ${}", param_idx));
        params.push(serde_json::json!(max_price));
    }

    // Count total
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*) as count");
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    // Add sorting and pagination
    let sort_by = query.sort_by.as_deref().unwrap_or("sort_order");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");
    sql.push_str(&format!(" ORDER BY {} {}", sort_by, sort_order));
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let products: Vec<Product> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: products,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// GET /api/products/:id - Get a single product
pub async fn show(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing product id".to_string()))?;

    let product: Option<Product> = ctx.data.db.query_one(
        "SELECT * FROM products WHERE id = $1 AND is_active = true",
        vec![serde_json::json!(id)]
    ).await?;

    match product {
        Some(p) => Response::from_json(&p),
        None => Err(ApiError::NotFound("Product not found".to_string()).into())
    }
}

/// GET /api/indicators - List trading indicators
pub async fn indicators(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let url = req.url()?;
    let query: ProductListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let sql = r#"
        SELECT i.*, p.price, p.compare_price 
        FROM indicators i
        JOIN products p ON p.id = i.product_id
        WHERE i.is_active = true AND p.is_active = true
        ORDER BY i.is_featured DESC, p.sort_order ASC
        LIMIT $1 OFFSET $2
    "#;

    let indicators: Vec<Indicator> = ctx.data.db.query(
        sql,
        vec![serde_json::json!(per_page), serde_json::json!(offset)]
    ).await?;

    let total: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM indicators WHERE is_active = true",
        vec![]
    ).await?
    .map(|r| r.count)
    .unwrap_or(0);

    let response = PaginatedResponse {
        data: indicators,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// GET /api/indicators/:slug - Get a single indicator by slug
pub async fn indicator_show(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let slug = ctx.param("slug")
        .ok_or_else(|| ApiError::BadRequest("Missing indicator slug".to_string()))?;

    let indicator: Option<Indicator> = ctx.data.db.query_one(
        r#"
        SELECT i.*, p.price, p.compare_price, p.features, p.is_featured
        FROM indicators i
        JOIN products p ON p.id = i.product_id
        WHERE i.slug = $1 AND i.is_active = true
        "#,
        vec![serde_json::json!(slug)]
    ).await?;

    match indicator {
        Some(i) => Response::from_json(&i),
        None => Err(ApiError::NotFound("Indicator not found".to_string()).into())
    }
}

/// Deserialize i64 from either number or string (Neon returns BIGINT as string)
fn deserialize_i64_local<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }
    
    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<i64>().map_err(Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

#[derive(serde::Deserialize)]
struct CountResult {
    #[serde(deserialize_with = "deserialize_i64_local")]
    count: i64,
}

impl Default for ProductListQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            product_type: None,
            is_active: None,
            is_featured: None,
            search: None,
            min_price: None,
            max_price: None,
            sort_by: None,
            sort_order: None,
        }
    }
}
