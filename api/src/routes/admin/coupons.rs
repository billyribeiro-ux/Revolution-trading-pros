//! Admin coupon management — extracted from the original
//! `routes/admin.rs` as part of the R6-B split (2026-05-20).
//!
//! Handler bodies are byte-identical to the pre-split source.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// COUPON MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

/// Coupon row returned to admin UI. All monetary fields are integer cents.
/// For percent coupons: `discount_value_cents = percent * 100` (e.g. 5000 = 50%).
///
/// `stripe_coupon_id` mirrors the row into a Stripe Coupon; populated on
/// admin create. `duration` is one of "once" / "forever" / "repeating";
/// `duration_in_months` is required when duration='repeating'.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct CouponRow {
    pub id: i64,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value_cents: i64,
    pub min_purchase_cents: Option<i64>,
    pub max_discount_cents: Option<i64>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub is_active: bool,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub applicable_products: Option<serde_json::Value>,
    pub applicable_plans: Option<serde_json::Value>,
    pub stripe_coupon_id: Option<String>,
    pub duration: String,
    pub duration_in_months: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCouponRequest {
    pub code: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub discount_value_cents: i64,
    pub min_purchase_cents: Option<i64>,
    pub max_discount_cents: Option<i64>,
    pub usage_limit: Option<i32>,
    pub is_active: Option<bool>,
    pub starts_at: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub applicable_products: Option<serde_json::Value>,
    pub applicable_plans: Option<serde_json::Value>,
    /// "once" | "forever" | "repeating". Defaults to "once" when omitted.
    pub duration: Option<String>,
    /// Required when `duration == "repeating"`.
    pub duration_in_months: Option<i32>,
}

/// List coupons (admin)
/// ICT 7 FIX: Use explicit column list with FLOAT8 casting for DECIMAL columns
pub(super) async fn list_coupons(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Vec<CouponRow>>, (StatusCode, Json<serde_json::Value>)> {
    // Money is integer cents at the Rust boundary (architecture standard §1.2).
    // DB columns remain NUMERIC(10,2) as a display cache; convert at the SQL edge.
    let coupons: Vec<CouponRow> = sqlx::query_as(
        r"SELECT
            id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons ORDER BY created_at DESC",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "list_coupons error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(coupons))
}

/// Create coupon (admin).
///
/// Mirrors the row into a Stripe Coupon (Batch 3.5 directive). Stripe is
/// the source of truth for discount math at checkout time; this row stores
/// our admin metadata (code, description, usage tracking, applicable
/// products/plans) plus a pointer (`stripe_coupon_id`) to the Stripe object
/// the checkout flow attaches via `discounts[]`.
///
/// On Stripe-side failure the DB row is NOT created. On DB-side failure
/// after Stripe success, the orphaned Stripe coupon is deleted so the two
/// stores stay in sync.
pub(super) async fn create_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateCouponRequest>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    // ── Validate inputs ──────────────────────────────────────────────────
    let duration = input.duration.clone().unwrap_or_else(|| "once".into());
    if !matches!(duration.as_str(), "once" | "forever" | "repeating") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "duration must be 'once', 'forever', or 'repeating'"})),
        ));
    }
    let duration_in_months = input.duration_in_months;
    if duration == "repeating" && duration_in_months.unwrap_or(0) <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "duration_in_months > 0 required when duration='repeating'"})),
        ));
    }
    if duration != "repeating" && duration_in_months.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "duration_in_months only valid when duration='repeating'"})),
        ));
    }
    if !matches!(
        input.discount_type.as_str(),
        "percent" | "percentage" | "fixed" | "amount"
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "discount_type must be 'percent' or 'fixed'"})),
        ));
    }

    // ── Create the Stripe Coupon (canonical source for the math) ────────
    let (percent_off, amount_off_cents) = match input.discount_type.as_str() {
        "percent" | "percentage" => {
            // discount_value_cents stores 'percent * 100' (e.g. 1000 = 10.00%).
            let percent = (input.discount_value_cents as f64) / 100.0;
            if percent <= 0.0 || percent > 100.0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "percent must be > 0 and <= 100"})),
                ));
            }
            (Some(percent), None)
        }
        _ => {
            if input.discount_value_cents <= 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "discount_value_cents must be > 0 for fixed coupons"})),
                ));
            }
            (None, Some(input.discount_value_cents))
        }
    };

    let stripe_coupon = state
        .services
        .stripe
        .create_coupon(crate::services::stripe::CreateStripeCouponRequest {
            percent_off,
            amount_off_cents,
            currency: "usd".into(),
            duration: duration.clone(),
            duration_in_months: duration_in_months.map(|n| n as i64),
            name: Some(input.code.to_uppercase()),
            max_redemptions: input.usage_limit.map(|n| n as i64),
            redeem_by_unix: input.expires_at.map(|t| t.and_utc().timestamp()),
        })
        .await
        .map_err(|e| {
            tracing::error!(target: "admin", "Stripe coupon create failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Stripe coupon create failed: {}", e)})),
            )
        })?;

    // ── INSERT the DB row (with Stripe pointer) ─────────────────────────
    let coupon_result: Result<CouponRow, sqlx::Error> = sqlx::query_as(
        r"
        INSERT INTO coupons (code, description, discount_type, discount_value, min_purchase, max_discount,
                             usage_limit, usage_count, is_active, starts_at, expires_at,
                             applicable_products, applicable_plans,
                             stripe_coupon_id, duration, duration_in_months,
                             created_at, updated_at)
        VALUES (UPPER($1), $2, $3, $4::BIGINT / 100.0, $5::BIGINT / 100.0, $6::BIGINT / 100.0,
                $7, 0, $8, $9, $10, $11, $12,
                $13, $14, $15,
                NOW(), NOW())
        RETURNING id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        ",
    )
    .bind(&input.code)
    .bind(&input.description)
    .bind(&input.discount_type)
    .bind(input.discount_value_cents)
    .bind(input.min_purchase_cents)
    .bind(input.max_discount_cents)
    .bind(input.usage_limit)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .bind(&input.applicable_products)
    .bind(&input.applicable_plans)
    .bind(&stripe_coupon.id)
    .bind(&duration)
    .bind(duration_in_months)
    .fetch_one(&state.db.pool)
    .await;

    match coupon_result {
        Ok(coupon) => Ok(Json(coupon)),
        Err(e) => {
            // Roll back the Stripe coupon so the two stores stay in sync.
            tracing::error!(
                target: "admin",
                "create_coupon DB insert failed after Stripe coupon {} created; rolling back Stripe: {}",
                stripe_coupon.id, e
            );
            if let Err(del_err) = state.services.stripe.delete_coupon(&stripe_coupon.id).await {
                tracing::error!(
                    target: "admin",
                    "Failed to roll back orphan Stripe coupon {}: {}",
                    stripe_coupon.id, del_err
                );
            }
            if e.to_string().contains("duplicate") {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Coupon code already exists"})),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ))
            }
        }
    }
}

/// Delete coupon (admin). Removes the DB row and the Stripe-side coupon.
/// Stripe coupon deletion stops future redemptions; subscriptions that
/// already have the discount keep it for the rest of its `duration`.
pub(super) async fn delete_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let stripe_coupon_id: Option<Option<String>> =
        sqlx::query_scalar("SELECT stripe_coupon_id FROM coupons WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(target: "admin", "delete_coupon read error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    sqlx::query("DELETE FROM coupons WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if let Some(Some(sid)) = stripe_coupon_id {
        if let Err(e) = state.services.stripe.delete_coupon(&sid).await {
            tracing::warn!(
                target: "admin",
                "DB coupon {} deleted but Stripe coupon {} delete failed: {}; manual cleanup required",
                id, sid, e
            );
        }
    }

    Ok(Json(json!({"message": "Coupon deleted successfully"})))
}

// FIX-2026-04-26 (P0-1, CC-1): Add the missing GET /admin/coupons/:id and
// PUT /admin/coupons/:id handlers so the admin coupon edit page can load and
// save coupons against the migration-correct schema.

/// Get a single coupon by ID (admin)
/// ICT 7 FIX: Use explicit column list with FLOAT8 casting for DECIMAL columns
pub(super) async fn get_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    let coupon: Option<CouponRow> = sqlx::query_as(
        r"SELECT
            id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "get_coupon error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    coupon.map(Json).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Coupon not found"})),
        )
    })
}

/// Update coupon (admin)
/// ICT 7 FIX: Schema-aligned UPDATE matching the migration columns.
/// Uses COALESCE so partial updates leave untouched columns alone, except for
/// nullable date fields where the caller may want to clear them by sending null.
/// Update coupon (admin).
///
/// Two paths:
///
///   1. **Metadata-only edits** (description, expires_at, applicable_*,
///      usage_limit, is_active, code rename): just update the DB row.
///      The Stripe coupon stays as-is. Cheap.
///
///   2. **Discount-math edits** (discount_type, discount_value_cents,
///      duration, duration_in_months): Stripe Coupons are immutable, so we
///      create a NEW Stripe Coupon, flip `stripe_coupon_id` on the DB row,
///      then best-effort delete the old Stripe coupon. Existing customers
///      who already redeemed the old code keep their discount per Stripe's
///      `duration` semantics — that is the correct, audited behavior.
///
/// On Stripe-side failure during recreate, the DB row is left unchanged
/// (no half-state). On DB-side failure after Stripe recreate, the new
/// orphan Stripe coupon is deleted.
pub(super) async fn update_coupon(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CreateCouponRequest>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    // ── Validate discount_type if provided ──────────────────────────────
    if !input.discount_type.is_empty()
        && input.discount_type != "percent"
        && input.discount_type != "percentage"
        && input.discount_type != "fixed"
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Discount type must be 'percent' or 'fixed'"})),
        ));
    }

    // ── Validate duration semantics if provided ─────────────────────────
    if let Some(ref d) = input.duration {
        if !matches!(d.as_str(), "once" | "forever" | "repeating") {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "duration must be 'once', 'forever', or 'repeating'"})),
            ));
        }
        if d == "repeating" && input.duration_in_months.unwrap_or(0) <= 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "duration_in_months > 0 required when duration='repeating'"})),
            ));
        }
        if d != "repeating" && input.duration_in_months.is_some() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "duration_in_months only valid when duration='repeating'"})),
            ));
        }
    }

    // ── Snapshot current row so we can detect discount-math edits ───────
    #[derive(sqlx::FromRow)]
    struct ExistingCoupon {
        discount_type: String,
        discount_value_cents: i64,
        duration: String,
        duration_in_months: Option<i32>,
        stripe_coupon_id: Option<String>,
    }
    let existing: ExistingCoupon = sqlx::query_as(
        r"SELECT discount_type,
                  (discount_value * 100)::BIGINT AS discount_value_cents,
                  duration, duration_in_months, stripe_coupon_id
           FROM coupons WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "update_coupon snapshot error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Coupon not found"})),
    ))?;

    // Resolve effective new discount-math fields. Empty/None means "keep".
    let new_discount_type: String = if !input.discount_type.is_empty() {
        // Normalize 'percentage' → 'percent' for Stripe semantics.
        let dt = if input.discount_type == "percentage" {
            "percent"
        } else {
            input.discount_type.as_str()
        };
        dt.to_string()
    } else {
        existing.discount_type.clone()
    };
    let new_discount_value_cents: i64 = if input.discount_value_cents != 0 {
        input.discount_value_cents
    } else {
        existing.discount_value_cents
    };
    let new_duration: String = input
        .duration
        .clone()
        .unwrap_or_else(|| existing.duration.clone());
    let new_duration_in_months: Option<i32> = if input.duration.is_some() {
        input.duration_in_months
    } else {
        existing.duration_in_months
    };

    let math_changed = new_discount_type != existing.discount_type
        || new_discount_value_cents != existing.discount_value_cents
        || new_duration != existing.duration
        || new_duration_in_months != existing.duration_in_months
        || existing.stripe_coupon_id.is_none();

    // ── If math changed, create a new Stripe Coupon BEFORE the DB update.
    let mut new_stripe_coupon_id: Option<String> = None;
    if math_changed {
        let (percent_off, amount_off_cents) = match new_discount_type.as_str() {
            "percent" | "percentage" => {
                let percent = (new_discount_value_cents as f64) / 100.0;
                if percent <= 0.0 || percent > 100.0 {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "percent must be > 0 and <= 100"})),
                    ));
                }
                (Some(percent), None)
            }
            _ => {
                if new_discount_value_cents <= 0 {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(
                            json!({"error": "discount_value_cents must be > 0 for fixed coupons"}),
                        ),
                    ));
                }
                (None, Some(new_discount_value_cents))
            }
        };
        let req = crate::services::stripe::CreateStripeCouponRequest {
            percent_off,
            amount_off_cents,
            currency: "usd".into(),
            duration: new_duration.clone(),
            duration_in_months: new_duration_in_months.map(|n| n as i64),
            name: Some(input.code.to_uppercase()),
            max_redemptions: input.usage_limit.map(|n| n as i64),
            redeem_by_unix: input.expires_at.map(|t| t.and_utc().timestamp()),
        };
        let created = state
            .services
            .stripe
            .create_coupon(req)
            .await
            .map_err(|e| {
                tracing::error!(target: "admin", "Stripe coupon recreate failed: {}", e);
                (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"error": format!("Stripe coupon recreate failed: {}", e)})),
                )
            })?;
        new_stripe_coupon_id = Some(created.id);
    }

    // ── DB UPDATE ───────────────────────────────────────────────────────
    let coupon_result: Result<Option<CouponRow>, sqlx::Error> = sqlx::query_as(
        r"
        UPDATE coupons SET
            code = COALESCE(NULLIF(UPPER($2), ''), code),
            description = COALESCE($3, description),
            discount_type = COALESCE(NULLIF($4, ''), discount_type),
            discount_value = COALESCE(NULLIF($5, 0)::BIGINT / 100.0, discount_value),
            min_purchase = COALESCE($6::BIGINT / 100.0, min_purchase),
            max_discount = COALESCE($7::BIGINT / 100.0, max_discount),
            usage_limit = COALESCE($8, usage_limit),
            is_active = COALESCE($9, is_active),
            starts_at = COALESCE($10, starts_at),
            expires_at = COALESCE($11, expires_at),
            applicable_products = COALESCE($12, applicable_products),
            applicable_plans = COALESCE($13, applicable_plans),
            duration = COALESCE($14, duration),
            duration_in_months = CASE WHEN $14 IS NOT NULL THEN $15 ELSE duration_in_months END,
            stripe_coupon_id = COALESCE($16, stripe_coupon_id),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        ",
    )
    .bind(id)
    .bind(&input.code)
    .bind(&input.description)
    .bind(&input.discount_type)
    .bind(input.discount_value_cents)
    .bind(input.min_purchase_cents)
    .bind(input.max_discount_cents)
    .bind(input.usage_limit)
    .bind(input.is_active)
    .bind(&input.starts_at)
    .bind(&input.expires_at)
    .bind(&input.applicable_products)
    .bind(&input.applicable_plans)
    .bind(input.duration.as_deref())
    .bind(input.duration_in_months)
    .bind(new_stripe_coupon_id.as_deref())
    .fetch_optional(&state.db.pool)
    .await;

    match coupon_result {
        Ok(Some(coupon)) => {
            // DB success. If math changed, best-effort delete old Stripe coupon.
            if math_changed {
                if let Some(ref old) = existing.stripe_coupon_id {
                    if Some(old) != new_stripe_coupon_id.as_ref() {
                        if let Err(e) = state.services.stripe.delete_coupon(old).await {
                            tracing::warn!(
                                target: "admin",
                                "DB coupon {} updated; new Stripe coupon attached but old Stripe coupon {} delete failed: {}; manual cleanup may be needed",
                                id, old, e
                            );
                        }
                    }
                }

                // Batch 4: write an audit row for the recreate-and-swap so
                // we have a permanent paper trail of who flipped the
                // pointer and when — useful for support, billing
                // disputes, and compliance review. Best-effort: failure
                // here does NOT roll back the Stripe + DB swap.
                let mut fields_changed: Vec<&'static str> = Vec::new();
                if new_discount_type != existing.discount_type {
                    fields_changed.push("discount_type");
                }
                if new_discount_value_cents != existing.discount_value_cents {
                    fields_changed.push("discount_value_cents");
                }
                if new_duration != existing.duration {
                    fields_changed.push("duration");
                }
                if new_duration_in_months != existing.duration_in_months {
                    fields_changed.push("duration_in_months");
                }
                if let Err(e) = sqlx::query(
                    r"INSERT INTO security_events (user_id, event_type, details, created_at)
                       VALUES ($1, 'coupon_recreated', $2::JSONB, NOW())",
                )
                .bind(user.id)
                .bind(json!({
                    "coupon_id": id,
                    "old_stripe_coupon_id": existing.stripe_coupon_id,
                    "new_stripe_coupon_id": new_stripe_coupon_id,
                    "fields_changed": fields_changed,
                }))
                .execute(&state.db.pool)
                .await
                {
                    tracing::warn!(
                        target: "admin",
                        "coupon recreate audit log write failed for coupon {}: {}",
                        id, e
                    );
                }
            }
            Ok(Json(coupon))
        }
        Ok(None) => {
            // DB row not found after we'd already created a new Stripe coupon → orphan cleanup.
            if let Some(ref new_id) = new_stripe_coupon_id {
                let _ = state.services.stripe.delete_coupon(new_id).await;
            }
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Coupon not found"})),
            ))
        }
        Err(e) => {
            tracing::error!(target: "admin", "update_coupon DB error: {}", e);
            // Roll back orphan Stripe coupon if math changed.
            if let Some(ref new_id) = new_stripe_coupon_id {
                let _ = state.services.stripe.delete_coupon(new_id).await;
            }
            if e.to_string().contains("duplicate") {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({"error": "Coupon code already exists"})),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ))
            }
        }
    }
}

/// Mirror a DB coupon row into Stripe (Batch 4 backfill tool).
///
/// POST /api/admin/coupons/:id/sync-to-stripe
///
/// Use case: a DB row exists but `stripe_coupon_id IS NULL` — usually
/// because a Stripe API outage during create left the row half-mirrored,
/// or the row was imported from outside the admin UI. Calling this
/// endpoint creates a fresh Stripe Coupon from the DB fields and stores
/// its id on the row.
///
/// If the row is already mirrored (`stripe_coupon_id IS NOT NULL`) we
/// return 400 with a directive to use the edit endpoint instead. This
/// avoids the foot-gun of accidentally orphaning a Stripe coupon and
/// keeps the recreate-and-swap policy in a single place
/// (`update_coupon`). The check itself is race-safe: the read+create+
/// update sequence re-checks NULL by virtue of the unconditional
/// `WHERE id = $2` UPDATE; concurrent calls would race to fail with a
/// duplicate-stripe-id error on the second writer.
pub(super) async fn sync_coupon_to_stripe(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CouponRow>, (StatusCode, Json<serde_json::Value>)> {
    // Read existing row.
    let row: Option<CouponRow> = sqlx::query_as(
        r"SELECT id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "sync_coupon_to_stripe read error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    let coupon = row.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Coupon not found"})),
    ))?;

    // Already mirrored — return 400 per Batch 4 spec. Operator should
    // use the edit endpoint to change discount math.
    if coupon.stripe_coupon_id.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Coupon already synced to Stripe; use the edit endpoint to change it"
            })),
        ));
    }

    // Map DB discount_type → Stripe percent_off / amount_off.
    let (percent_off, amount_off_cents) = match coupon.discount_type.as_str() {
        "percent" | "percentage" => {
            let percent = (coupon.discount_value_cents as f64) / 100.0;
            if percent <= 0.0 || percent > 100.0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": "Stored discount_value invalid for percent coupon (must be > 0 and <= 100)"}),
                    ),
                ));
            }
            (Some(percent), None)
        }
        _ => {
            if coupon.discount_value_cents <= 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": "Stored discount_value_cents must be > 0 for fixed coupons"}),
                    ),
                ));
            }
            (None, Some(coupon.discount_value_cents))
        }
    };

    let req = crate::services::stripe::CreateStripeCouponRequest {
        percent_off,
        amount_off_cents,
        currency: "usd".into(),
        duration: coupon.duration.clone(),
        duration_in_months: coupon.duration_in_months.map(|n| n as i64),
        name: Some(coupon.code.to_uppercase()),
        max_redemptions: coupon.usage_limit.map(|n| n as i64),
        redeem_by_unix: coupon.expires_at.map(|t| t.and_utc().timestamp()),
    };
    let created = state
        .services
        .stripe
        .create_coupon(req)
        .await
        .map_err(|e| {
            tracing::error!(target: "admin", "sync_coupon_to_stripe Stripe create failed: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Stripe coupon create failed: {}", e)})),
            )
        })?;

    // Persist stripe_coupon_id on the DB row.
    let updated: CouponRow = sqlx::query_as(
        r"UPDATE coupons SET stripe_coupon_id = $1, updated_at = NOW()
           WHERE id = $2
           RETURNING id, code, description, discount_type,
               (discount_value * 100)::BIGINT AS discount_value_cents,
               (min_purchase * 100)::BIGINT AS min_purchase_cents,
               (max_discount * 100)::BIGINT AS max_discount_cents,
               usage_limit, usage_count, is_active, starts_at, expires_at,
               applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at",
    )
    .bind(&created.id)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        // DB write failed AFTER we created a new Stripe coupon → orphan cleanup.
        tracing::error!(
            target: "admin",
            "sync_coupon_to_stripe DB persist failed for coupon {}; rolling back Stripe coupon {}: {}",
            id, created.id, e
        );
        let stripe = state.services.stripe.clone();
        let stripe_id = created.id.clone();
        tokio::spawn(async move {
            let _ = stripe.delete_coupon(&stripe_id).await;
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    tracing::info!(
        target: "admin",
        event = "coupon_synced_to_stripe",
        coupon_id = %id,
        stripe_coupon_id = %created.id,
        "Mirrored DB coupon into Stripe"
    );

    Ok(Json(updated))
}

/// Validate coupon (admin)
/// ICT 7 FIX: Use explicit column list with FLOAT8 casting for DECIMAL columns
/// SECURITY FIX (FULL_REPO_AUDIT_2026-05-17 P1-4 #1): This handler was the lone
/// unauthenticated route in the admin module. Its response embeds the full
/// `CouponRow` — internal id, `stripe_coupon_id`, `usage_count`/`usage_limit`,
/// and `applicable_products`/`applicable_plans` — which allowed anonymous
/// coupon enumeration and Stripe-linkage disclosure. Gate it with the same
/// `AdminUser` extractor every sibling admin handler uses so the route is
/// fail-closed at the type level.
pub(super) async fn validate_coupon(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(code): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let coupon: Option<CouponRow> = sqlx::query_as(
        r"SELECT
            id, code, description, discount_type,
            (discount_value * 100)::BIGINT AS discount_value_cents,
            (min_purchase * 100)::BIGINT AS min_purchase_cents,
            (max_discount * 100)::BIGINT AS max_discount_cents,
            usage_limit, usage_count, is_active, starts_at, expires_at,
            applicable_products, applicable_plans, stripe_coupon_id, duration, duration_in_months, created_at, updated_at
        FROM coupons WHERE UPPER(code) = UPPER($1) AND is_active = true",
    )
    .bind(&code)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "admin", "validate_coupon error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    match coupon {
        Some(c) => {
            // Check expiry
            if let Some(expires_at) = c.expires_at {
                if expires_at < chrono::Utc::now().naive_utc() {
                    return Ok(Json(json!({"valid": false, "error": "Coupon has expired"})));
                }
            }
            // Check usage limit
            if let Some(limit) = c.usage_limit {
                if c.usage_count >= limit {
                    return Ok(Json(
                        json!({"valid": false, "error": "Coupon usage limit reached"}),
                    ));
                }
            }
            Ok(Json(json!({
                "valid": true,
                "coupon": c
            })))
        }
        None => Ok(Json(
            json!({"valid": false, "error": "Invalid coupon code"}),
        )),
    }
}
