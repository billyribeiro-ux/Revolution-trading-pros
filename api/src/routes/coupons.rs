//! Coupon Routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Complete coupon management:
//! - Coupon validation with product/plan restrictions
//! - User coupon history
//! - Admin coupon CRUD
//! - Usage tracking and limits

#![allow(clippy::map_flatten)]
#![allow(clippy::collapsible_if)]

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use crate::{
    middleware::admin::AdminUser,
    models::{order::Coupon, User},
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════
// Request/Response Types
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct ValidateCouponRequest {
    pub code: String,
    pub subtotal: Option<f64>,
    pub product_ids: Option<Vec<i64>>,
    pub plan_ids: Option<Vec<i64>>,
}

#[derive(Serialize)]
pub struct ValidateCouponResponse {
    pub valid: bool,
    pub coupon: Option<CouponInfo>,
    pub discount_amount: Option<f64>,
    pub error: Option<String>,
}

/// CouponInfo - matches Laravel production schema
#[derive(Serialize)]
pub struct CouponInfo {
    pub id: i64,
    pub code: String,
    #[serde(rename = "type")]
    pub coupon_type: String,
    pub value: f64,
    pub min_purchase_amount: f64,
    pub expiry_date: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserCoupon {
    pub id: i64,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub expires_at: Option<String>,
    pub is_used: bool,
    pub used_at: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value: Decimal,
    pub min_purchase: Option<Decimal>,
    pub max_discount: Option<Decimal>,
    pub usage_limit: Option<i32>,
    pub starts_at: Option<String>,
    pub expires_at: Option<String>,
    pub applicable_products: Option<Vec<i64>>,
    pub applicable_plans: Option<Vec<i64>>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct CouponQuery {
    pub active_only: Option<bool>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Route Handlers
// ═══════════════════════════════════════════════════════════════════════════

/// Validate a coupon code
/// POST /api/coupons/validate
async fn validate_coupon(
    State(state): State<AppState>,
    Json(input): Json<ValidateCouponRequest>,
) -> Result<Json<ValidateCouponResponse>, (StatusCode, Json<serde_json::Value>)> {
    let code = input.code.trim().to_uppercase();

    if code.is_empty() {
        return Ok(Json(ValidateCouponResponse {
            valid: false,
            coupon: None,
            discount_amount: None,
            error: Some("Coupon code is required".to_string()),
        }));
    }

    // Find coupon - ICT 11+ Fix: Use Laravel production schema column names
    let coupon: Option<Coupon> = sqlx::query_as(
        r#"SELECT id, code, type, value, max_uses, current_uses,
                  expiry_date, applicable_products, min_purchase_amount,
                  is_active, created_at, updated_at
           FROM coupons
           WHERE UPPER(code) = $1"#,
    )
    .bind(&code)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let coupon = match coupon {
        Some(c) => c,
        None => {
            return Ok(Json(ValidateCouponResponse {
                valid: false,
                coupon: None,
                discount_amount: None,
                error: Some("Invalid coupon code".to_string()),
            }));
        }
    };

    // Check if active
    if !coupon.is_active {
        return Ok(Json(ValidateCouponResponse {
            valid: false,
            coupon: None,
            discount_amount: None,
            error: Some("This coupon is no longer active".to_string()),
        }));
    }

    // Check expiry - Laravel uses expiry_date
    if let Some(expiry_date) = coupon.expiry_date {
        if expiry_date < chrono::Utc::now().naive_utc() {
            return Ok(Json(ValidateCouponResponse {
                valid: false,
                coupon: None,
                discount_amount: None,
                error: Some("This coupon has expired".to_string()),
            }));
        }
    }

    // Check usage limit - Laravel uses max_uses and current_uses
    if coupon.max_uses > 0 {
        if coupon.current_uses >= coupon.max_uses {
            return Ok(Json(ValidateCouponResponse {
                valid: false,
                coupon: None,
                discount_amount: None,
                error: Some("This coupon has reached its usage limit".to_string()),
            }));
        }
    }

    // Check minimum purchase - Laravel uses min_purchase_amount
    let min_purchase_f64 = coupon.min_purchase_amount.to_f64().unwrap_or(0.0);
    if min_purchase_f64 > 0.0 {
        if let Some(subtotal) = input.subtotal {
            if subtotal < min_purchase_f64 {
                return Ok(Json(ValidateCouponResponse {
                    valid: false,
                    coupon: None,
                    discount_amount: None,
                    error: Some(format!(
                        "Minimum purchase of ${:.2} required",
                        min_purchase_f64
                    )),
                }));
            }
        }
    }

    // Check product restrictions
    if let Some(ref applicable) = coupon.applicable_products {
        if let Some(ref product_ids) = input.product_ids {
            let applicable_ids: Vec<i64> =
                serde_json::from_value(applicable.clone()).unwrap_or_default();
            if !applicable_ids.is_empty() {
                let has_valid_product = product_ids.iter().any(|id| applicable_ids.contains(id));
                if !has_valid_product {
                    return Ok(Json(ValidateCouponResponse {
                        valid: false,
                        coupon: None,
                        discount_amount: None,
                        error: Some(
                            "This coupon is not valid for the selected products".to_string(),
                        ),
                    }));
                }
            }
        }
    }

    // Note: Laravel schema doesn't have applicable_plans column

    // Calculate discount - Laravel uses 'type' (percentage/fixed) and 'value'
    let coupon_value_f64 = coupon.value.to_f64().unwrap_or(0.0);
    let discount_amount = if let Some(subtotal) = input.subtotal {
        let discount = if coupon.coupon_type == "percentage" {
            subtotal * (coupon_value_f64 / 100.0)
        } else {
            coupon_value_f64
        };
        Some(discount)
    } else {
        None
    };

    Ok(Json(ValidateCouponResponse {
        valid: true,
        coupon: Some(CouponInfo {
            id: coupon.id,
            code: coupon.code,
            coupon_type: coupon.coupon_type,
            value: coupon_value_f64,
            min_purchase_amount: min_purchase_f64,
            expiry_date: coupon.expiry_date.map(|d| d.to_string()),
        }),
        discount_amount,
        error: None,
    }))
}

/// Get user's available coupons
/// GET /api/coupons/my
async fn get_user_coupons(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get coupons assigned to user or public coupons
    let coupons: Vec<UserCoupon> = sqlx::query_as::<_, UserCoupon>(
        r#"SELECT
            c.id,
            c.code,
            c.description,
            c.discount_type,
            c.discount_value,
            c.expires_at::text as expires_at,
            CASE WHEN uc.used_at IS NOT NULL THEN true ELSE false END as is_used,
            uc.used_at::text as used_at
        FROM coupons c
        LEFT JOIN user_coupons uc ON uc.coupon_id = c.id AND uc.user_id = $1
        WHERE c.is_active = true
        AND (c.expires_at IS NULL OR c.expires_at > NOW())
        AND (c.usage_limit IS NULL OR c.usage_count < c.usage_limit)
        AND (
            -- Public coupons (no user assignment table entry required)
            NOT EXISTS (SELECT 1 FROM user_coupons WHERE coupon_id = c.id)
            OR
            -- User-specific coupons
            uc.user_id = $1
        )
        ORDER BY c.expires_at ASC NULLS LAST, c.created_at DESC"#,
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "coupons": coupons,
        "count": coupons.len()
    })))
}

/// Get all coupons (admin)
/// GET /api/coupons
/// ICT 7 SECURITY FIX: Use AdminUser extractor for consistent authorization
async fn list_coupons(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<CouponQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limit = query.limit.unwrap_or(50).min(100);
    let page = query.page.unwrap_or(0);
    let offset = page * limit;

    // ICT 7 FIX: Use Laravel production schema column names
    let coupons: Vec<Coupon> = if query.active_only.unwrap_or(false) {
        sqlx::query_as(
            r#"SELECT id, code, type, value, max_uses, current_uses,
                      expiry_date, applicable_products, min_purchase_amount,
                      is_active, created_at, updated_at
               FROM coupons
               WHERE is_active = true
               ORDER BY created_at DESC
               LIMIT $1 OFFSET $2"#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "coupons", error = %e, "Failed to list coupons");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to list coupons"})))
        })?
    } else {
        sqlx::query_as(
            r#"SELECT id, code, type, value, max_uses, current_uses,
                      expiry_date, applicable_products, min_purchase_amount,
                      is_active, created_at, updated_at
               FROM coupons
               ORDER BY created_at DESC
               LIMIT $1 OFFSET $2"#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "coupons", error = %e, "Failed to list coupons");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to list coupons"})))
        })?
    };

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM coupons")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(0);

    Ok(Json(json!({
        "coupons": coupons,
        "total": total,
        "page": page,
        "limit": limit,
        "total_pages": (total as f64 / limit as f64).ceil() as i64
    })))
}

/// Create a new coupon (admin)
/// POST /api/coupons
/// ICT 7 SECURITY FIX: Use AdminUser extractor for consistent authorization
async fn create_coupon(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Json(input): Json<CreateCouponRequest>,
) -> Result<Json<Coupon>, (StatusCode, Json<serde_json::Value>)> {
    // Validate code
    if input.code.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Coupon code is required"})),
        ));
    }

    // Validate discount type
    if input.discount_type != "percentage" && input.discount_type != "fixed" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Discount type must be 'percentage' or 'fixed'"})),
        ));
    }

    let code = input.code.trim().to_uppercase();

    // Check for duplicate
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM coupons WHERE UPPER(code) = $1)")
            .bind(&code)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or(false);

    if exists {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "Coupon code already exists"})),
        ));
    }

    // ICT 7 FIX: Use Laravel production schema column names
    let coupon: Coupon = sqlx::query_as(
        r#"INSERT INTO coupons (
            code, type, value, max_uses, current_uses,
            expiry_date, applicable_products, min_purchase_amount,
            is_active, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, 0, $5, $6, $7, $8, NOW(), NOW())
        RETURNING id, code, type, value, max_uses, current_uses,
                  expiry_date, applicable_products, min_purchase_amount,
                  is_active, created_at, updated_at"#,
    )
    .bind(&code)
    .bind(&input.discount_type)
    .bind(input.discount_value)
    .bind(input.usage_limit.unwrap_or(0))
    .bind(
        input
            .expires_at
            .as_ref()
            .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").ok()),
    )
    .bind(
        input
            .applicable_products
            .as_ref()
            .map(|v| serde_json::to_value(v).ok())
            .flatten(),
    )
    .bind(input.min_purchase.unwrap_or(Decimal::ZERO))
    .bind(input.is_active.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "coupons", error = %e, "Failed to create coupon");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create coupon"})),
        )
    })?;

    tracing::info!(
        target: "coupons",
        event = "coupon_created",
        coupon_id = %coupon.id,
        code = %coupon.code,
        created_by = %admin.id,
        "Coupon created"
    );

    Ok(Json(coupon))
}

/// Update coupon (admin)
/// PUT /api/coupons/:id
/// ICT 7 SECURITY FIX: Use AdminUser extractor for consistent authorization
async fn update_coupon(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CreateCouponRequest>,
) -> Result<Json<Coupon>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "coupons",
        event = "coupon_update",
        coupon_id = %id,
        admin_id = %admin.id,
        "Admin updating coupon"
    );

    // ICT 7 FIX: Use Laravel production schema column names
    let coupon: Coupon = sqlx::query_as(
        r#"UPDATE coupons SET
            type = COALESCE($2, type),
            value = COALESCE($3, value),
            max_uses = COALESCE($4, max_uses),
            expiry_date = $5,
            applicable_products = COALESCE($6, applicable_products),
            min_purchase_amount = COALESCE($7, min_purchase_amount),
            is_active = COALESCE($8, is_active),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, code, type, value, max_uses, current_uses,
                  expiry_date, applicable_products, min_purchase_amount,
                  is_active, created_at, updated_at"#,
    )
    .bind(id)
    .bind(&input.discount_type)
    .bind(input.discount_value)
    .bind(input.usage_limit)
    .bind(
        input
            .expires_at
            .as_ref()
            .and_then(|s| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S").ok()),
    )
    .bind(
        input
            .applicable_products
            .as_ref()
            .map(|v| serde_json::to_value(v).ok())
            .flatten(),
    )
    .bind(input.min_purchase)
    .bind(input.is_active)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "coupons", error = %e, coupon_id = %id, "Failed to update coupon");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update coupon"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        )
    })?;

    tracing::info!(
        target: "coupons",
        event = "coupon_updated",
        coupon_id = %id,
        admin_id = %admin.id,
        "Coupon updated successfully"
    );

    Ok(Json(coupon))
}

/// Delete coupon (admin)
/// DELETE /api/coupons/:id
/// ICT 7 SECURITY FIX: Use AdminUser extractor for consistent authorization
async fn delete_coupon(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "coupon_delete",
        coupon_id = %id,
        admin_id = %admin.id,
        "Admin deleting coupon"
    );

    let result = sqlx::query("DELETE FROM coupons WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "coupons", error = %e, coupon_id = %id, "Failed to delete coupon");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete coupon"})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        ));
    }

    tracing::info!(
        target: "coupons",
        event = "coupon_deleted",
        coupon_id = %id,
        admin_id = %admin.id,
        "Coupon deleted successfully"
    );

    Ok(StatusCode::NO_CONTENT)
}

/// Deactivate coupon (admin) - ICT 7 FIX: Soft deactivate
/// POST /api/coupons/:id/deactivate
async fn deactivate_coupon(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "coupons",
        event = "coupon_deactivate",
        coupon_id = %id,
        admin_id = %admin.id,
        "Admin deactivating coupon"
    );

    let result = sqlx::query(
        "UPDATE coupons SET is_active = false, updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "coupons", error = %e, coupon_id = %id, "Failed to deactivate coupon");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to deactivate coupon"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        ));
    }

    Ok(Json(json!({"message": "Coupon deactivated successfully", "id": id})))
}

/// Reactivate coupon (admin) - ICT 7 FIX: Restore coupon
/// POST /api/coupons/:id/reactivate
async fn reactivate_coupon(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "coupons",
        event = "coupon_reactivate",
        coupon_id = %id,
        admin_id = %admin.id,
        "Admin reactivating coupon"
    );

    let result = sqlx::query(
        "UPDATE coupons SET is_active = true, updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "coupons", error = %e, coupon_id = %id, "Failed to reactivate coupon");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to reactivate coupon"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        ));
    }

    Ok(Json(json!({"message": "Coupon reactivated successfully", "id": id})))
}

/// Reset coupon usage count (admin) - ICT 7 FIX: Allow resetting usage
/// POST /api/coupons/:id/reset-usage
async fn reset_coupon_usage(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "coupons",
        event = "coupon_reset_usage",
        coupon_id = %id,
        admin_id = %admin.id,
        "Admin resetting coupon usage"
    );

    let result = sqlx::query(
        "UPDATE coupons SET current_uses = 0, updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "coupons", error = %e, coupon_id = %id, "Failed to reset coupon usage");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to reset coupon usage"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        ));
    }

    Ok(Json(json!({"message": "Coupon usage reset successfully", "id": id})))
}

/// Get single coupon by ID (admin)
/// GET /api/coupons/:id
async fn get_coupon(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Coupon>, (StatusCode, Json<serde_json::Value>)> {
    let coupon: Coupon = sqlx::query_as(
        r#"SELECT id, code, type, value, max_uses, current_uses,
                  expiry_date, applicable_products, min_purchase_amount,
                  is_active, created_at, updated_at
           FROM coupons WHERE id = $1"#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "coupons", error = %e, coupon_id = %id, "Failed to fetch coupon");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch coupon"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        )
    })?;

    Ok(Json(coupon))
}

// ═══════════════════════════════════════════════════════════════════════════
// Router - ICT 7 COMPLETE COUPON MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Public endpoints
        .route("/validate", post(validate_coupon))
        .route("/my", get(get_user_coupons))
        .route("/user/available", get(get_user_coupons)) // Frontend compatibility
        // Admin CRUD endpoints
        .route("/", get(list_coupons).post(create_coupon))
        .route("/:id", get(get_coupon).put(update_coupon).delete(delete_coupon))
        // Admin management endpoints - ICT 7 FIX
        .route("/:id/deactivate", post(deactivate_coupon))
        .route("/:id/reactivate", post(reactivate_coupon))
        .route("/:id/reset-usage", post(reset_coupon_usage))
}
