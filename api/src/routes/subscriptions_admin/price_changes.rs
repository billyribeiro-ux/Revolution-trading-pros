//! Subscriptions Admin — Stripe-syncing price-change endpoints
//!
//! Money path. Preserved byte-for-byte from the pre-split file:
//!   - `i64` cents end-to-end (`amount_cents`, `*_amount_cents`,
//!     `price_cents`); no narrowing casts introduced.
//!   - All audit/tracing events on the `stripe_price` / `stripe.price_change`
//!     targets kept identical (events: `plan_price_changed`, `sub_migrated`,
//!     `sub_migration_failed`).
//!   - Stripe API call ordering: ensure-product → create-price → DB tx →
//!     commit → optional per-subscription migration loop → history update.
//!   - BEGIN / COMMIT transaction wrapper preserved: DB pointer flip +
//!     history INSERT happen atomically; the per-sub Stripe migration loop
//!     is intentionally OUTSIDE the transaction (per the original comment:
//!     "DB writes use a transaction; Stripe calls do not — we record
//!     per-subscription failures rather than aborting the batch").
//!
//! Flow per `apply_to`:
//!   - new_only — create new Stripe Price → DB pointer flip → done.
//!   - next_renewal — same as above, then for each active sub on the old
//!     price, PATCH it to the new price with proration_behavior=none and
//!     billing_cycle_anchor pinned (no proration, renewal date intact).
//!   - immediate_proration — same as above, but
//!     proration_behavior=create_prorations (Stripe issues prorated
//!     credit/charge immediately).
//!
//! All three paths record a row in `membership_plan_price_history` so this is
//! auditable and rollback-able. DB writes use a transaction; Stripe calls do
//! not — we record per-subscription failures rather than aborting the batch.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::dto::{ChangePriceRequest, PriceHistoryRow};
use crate::{middleware::admin::AdminUser, AppState};

/// Change a membership plan's price (admin).
///
/// POST /admin/subscriptions/plans/:id/price
pub(super) async fn change_plan_price(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(plan_id): Path<i64>,
    Json(input): Json<ChangePriceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ── Validate input ──────────────────────────────────────────────────────
    if input.amount_cents <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "amount_cents must be > 0"})),
        ));
    }
    if !matches!(
        input.billing_interval.as_str(),
        "month" | "year" | "one_time"
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "billing_interval must be month|year|one_time"})),
        ));
    }
    if !matches!(
        input.apply_to.as_str(),
        "new_only" | "next_renewal" | "immediate_proration"
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "apply_to must be new_only|next_renewal|immediate_proration"
            })),
        ));
    }

    let currency = input.currency.unwrap_or_else(|| "usd".to_string());

    // ── Fetch the plan ──────────────────────────────────────────────────────
    #[derive(sqlx::FromRow)]
    struct PlanForPriceChange {
        id: i64,
        name: String,
        stripe_price_id: Option<String>,
        stripe_product_id: Option<String>,
        price_cents: i64,
    }

    let plan: PlanForPriceChange = sqlx::query_as(
        r"SELECT id, name, stripe_price_id, stripe_product_id,
                  (price * 100)::BIGINT AS price_cents
           FROM membership_plans WHERE id = $1",
    )
    .bind(plan_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, plan_id = %plan_id, "DB error fetching plan");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Plan not found"})),
        )
    })?;

    let old_price_id = plan.stripe_price_id.clone();
    let old_amount_cents: i64 = plan.price_cents;

    // ── Resolve a Stripe client from DB-stored creds (env fallback) ─────────
    // PE7 invariant 2A: prefer admin-pasted keys over env vars so price
    // changes work without a redeploy after Stripe rotation.
    let env_scope = state.config.environment.clone();
    let stripe = state
        .services
        .credentials
        .stripe_client(&state.db.pool, &env_scope)
        .await;

    // ── Ensure we have a Stripe Product to attach the new Price to ──────────
    let product_id = match plan.stripe_product_id.clone() {
        Some(pid) => pid,
        None => stripe
            .create_product(&plan.name)
            .await
            .map_err(|e| {
                tracing::error!(target: "stripe_price", error = %e, plan_id = %plan_id, "Failed to create Stripe product");
                (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"error": format!("Stripe product create failed: {}", e)})),
                )
            })?,
    };

    // ── Create the new Stripe Price ─────────────────────────────────────────
    let new_price = stripe
        .create_price(
            &product_id,
            input.amount_cents,
            &currency,
            &input.billing_interval,
        )
        .await
        .map_err(|e| {
            tracing::error!(target: "stripe_price", error = %e, plan_id = %plan_id, "Failed to create Stripe price");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Stripe price create failed: {}", e)})),
            )
        })?;

    let new_price_id = new_price.id.clone();

    // ── DB write: flip plan pointer + insert history (one transaction) ──────
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to start transaction"})),
        )
    })?;

    sqlx::query(
        r"UPDATE membership_plans
           SET stripe_price_id = $1, stripe_product_id = $2,
               price = $3::BIGINT / 100.0, updated_at = NOW()
           WHERE id = $4",
    )
    .bind(&new_price_id)
    .bind(&product_id)
    .bind(input.amount_cents)
    .bind(plan_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, plan_id = %plan_id, "Failed to update plan");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update plan"})),
        )
    })?;

    let history_id: i64 = sqlx::query_scalar(
        r"INSERT INTO membership_plan_price_history (
            plan_id, old_stripe_price_id, new_stripe_price_id,
            old_amount_cents, new_amount_cents, currency, billing_interval,
            apply_to, changed_by_user_id, changed_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
        RETURNING id",
    )
    .bind(plan_id)
    .bind(&old_price_id)
    .bind(&new_price_id)
    .bind(old_amount_cents)
    .bind(input.amount_cents)
    .bind(&currency)
    .bind(&input.billing_interval)
    .bind(&input.apply_to)
    .bind(admin.id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, plan_id = %plan_id, "Failed to insert price history");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to record price history"})),
        )
    })?;

    tx.commit().await.map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to commit transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit DB changes"})),
        )
    })?;

    tracing::info!(
        target: "stripe_price",
        event = "plan_price_changed",
        plan_id = %plan_id,
        old_price_id = ?old_price_id,
        new_price_id = %new_price_id,
        amount_cents = %input.amount_cents,
        apply_to = %input.apply_to,
        history_id = %history_id,
        admin_id = %admin.id,
        "Plan price updated; Stripe Price created and DB pointer flipped"
    );

    // ── Migration phase (only for next_renewal / immediate_proration) ───────
    let mut migrated = 0i32;
    let mut failed = 0i32;
    let mut failures: Vec<serde_json::Value> = Vec::new();

    if input.apply_to != "new_only" {
        // Fetch active subscriptions still on the old price.
        // We migrate every active subscription that's tied to this plan and
        // backed by Stripe — the per-row "stripe_price_id" lives only on the
        // Stripe side, so the plan_id + Stripe-provider check is sufficient.
        let active_subs: Vec<(String,)> = sqlx::query_as(
            r"SELECT stripe_subscription_id FROM user_memberships
               WHERE plan_id = $1
                 AND status IN ('active', 'trial', 'past_due')
                 AND stripe_subscription_id IS NOT NULL
                 AND payment_provider = 'stripe'",
        )
        .bind(plan_id)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "stripe_price", error = %e, "Failed to query active subs");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch subs to migrate"})),
            )
        })?;

        let proration_behavior = if input.apply_to == "next_renewal" {
            "none"
        } else {
            "create_prorations"
        };

        for (sub_id,) in active_subs {
            match stripe
                .migrate_subscription_to_price(&sub_id, &new_price_id, proration_behavior)
                .await
            {
                Ok(()) => {
                    migrated += 1;
                    tracing::info!(
                        target: "stripe.price_change",
                        event = "sub_migrated",
                        subscription_id = %sub_id,
                        old_price = ?old_price_id,
                        new_price = %new_price_id,
                        mode = %input.apply_to,
                        proration = %proration_behavior,
                        "Subscription migrated to new price"
                    );
                }
                Err(e) => {
                    failed += 1;
                    failures.push(json!({
                        "subscription_id": sub_id,
                        "error": e.to_string(),
                    }));
                    tracing::error!(
                        target: "stripe_price",
                        event = "sub_migration_failed",
                        subscription_id = %sub_id,
                        error = %e,
                        "Subscription migration failed"
                    );
                }
            }
        }

        // Record migration counts onto the history row.
        sqlx::query(
            r"UPDATE membership_plan_price_history
               SET subscriptions_migrated = $1,
                   subscriptions_failed = $2,
                   failure_details = $3
               WHERE id = $4",
        )
        .bind(migrated)
        .bind(failed)
        .bind(if failures.is_empty() {
            None
        } else {
            Some(json!(failures))
        })
        .bind(history_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "stripe_price", error = %e, "Failed to update history row");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to record migration outcome"})),
            )
        })?;
    }

    Ok(Json(json!({
        "success": true,
        "plan_id": plan_id,
        "history_id": history_id,
        "old_price_id": old_price_id,
        "new_price_id": new_price_id,
        "new_amount_cents": input.amount_cents,
        "currency": currency,
        "billing_interval": input.billing_interval,
        "apply_to": input.apply_to,
        "subscriptions_migrated": migrated,
        "subscriptions_failed": failed,
        "failures": failures,
    })))
}

/// List price-change history for a plan (admin).
/// GET /admin/subscriptions/plans/:id/price-history
pub(super) async fn list_plan_price_history(
    State(state): State<AppState>,
    AdminUser(_admin): AdminUser,
    Path(plan_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<PriceHistoryRow> = sqlx::query_as(
        r"SELECT id, plan_id, old_stripe_price_id, new_stripe_price_id,
                  old_amount_cents, new_amount_cents, currency, billing_interval,
                  apply_to, subscriptions_migrated, subscriptions_failed,
                  failure_details, changed_by_user_id, changed_at
           FROM membership_plan_price_history
           WHERE plan_id = $1
           ORDER BY changed_at DESC
           LIMIT 100",
    )
    .bind(plan_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to fetch price history");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to load price history"})),
        )
    })?;

    Ok(Json(json!({"data": rows, "total": rows.len()})))
}
