//! `checkout.session.completed` webhook handler.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Order reconciliation,
//! membership upsert, coupon usage bump, product/course/indicator
//! access grants, and best-effort confirmation/receipt emails all
//! preserved byte-for-byte. The single webhook transaction (`tx`)
//! is still owned by the caller in `webhook.rs`.

use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

// IDEMPOTENT-BY: orders.stripe_session_id UNIQUE WHERE NOT NULL +
// user_memberships.stripe_subscription_id partial UNIQUE (Batch 4 / 063) +
// per-row ON CONFLICT upserts below.
//
// P0-4 (audit FULL_REPO_AUDIT_2026-05-17): every multi-table write in this
// handler now runs through the caller-owned `tx`. The orders reconcile, the
// membership upsert, the coupon usage_count bump, and the user_products /
// user_course_enrollments / user_indicator_access grants are ALL in one
// transaction with the `webhook_events.processed_at` stamp. Either the
// customer is charged AND fully provisioned AND the event is marked
// processed, or NONE of it commits and Stripe's retry re-runs the whole
// handler (the dedup row is left with processed_at NULL, so the retry is
// reprocessed, not skipped). Previously these were independent statements
// on the pool: a mid-handler failure charged the customer with partial/no
// access and the old presence-based dedup then dropped every retry.
//
// Per-statement ON CONFLICT upserts keep the handler safe to fully re-run:
// a replay re-asserts the same rows rather than duplicating them.
//
// Best-effort SIDE EFFECTS (confirmation/receipt emails) deliberately stay
// on `state.db.pool`, NOT `tx`: a transient email failure must not roll back
// a successful charge+provision, and the email service takes a pool handle.
pub(super) async fn handle_checkout_completed(
    state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let session = event.as_checkout_session().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid checkout session data"})),
        )
    })?;

    let order_id = event.get_order_id();
    let user_id = event.get_user_id();

    if let Some(order_id) = order_id {
        // ─── Reconcile order totals with Stripe-applied discount (arch §10) ───
        // The webhook payload's session usually carries `amount_total`,
        // `amount_subtotal`, and `total_details`, but we re-fetch with
        // `expand[]=total_details.breakdown.discounts.discount.coupon` so we
        // also get the human-readable promotion code/coupon name to backfill
        // `orders.coupon_code`. This is the single source of truth for the
        // amounts the customer was actually charged.
        let session_full = state
            .services
            .stripe
            .retrieve_checkout_session(&session.id)
            .await
            .ok();

        let (amount_subtotal_cents, amount_discount_cents, amount_total_cents, applied_code) =
            if let Some(s) = session_full.as_ref() {
                let subtotal = s.amount_subtotal.or(s.amount_total).unwrap_or(0);
                let discount = s
                    .total_details
                    .as_ref()
                    .and_then(|td| td.amount_discount)
                    .unwrap_or(0);
                let total = s.amount_total.unwrap_or(subtotal.saturating_sub(discount));
                let code = s
                    .total_details
                    .as_ref()
                    .and_then(|td| td.breakdown.as_ref())
                    .and_then(|b| b.discounts.first())
                    .and_then(|d| d.discount.as_ref())
                    .and_then(|inner| {
                        inner.promotion_code.clone().or_else(|| {
                            inner
                                .coupon
                                .as_ref()
                                .and_then(|c| c.id.clone().or_else(|| c.name.clone()))
                        })
                    });
                (subtotal, discount, total, code)
            } else {
                // Stripe re-fetch failed; fall back to webhook payload's amount_total.
                let total = session.amount_total.unwrap_or(0);
                (total, 0_i64, total, None)
            };

        sqlx::query(
            r"UPDATE orders SET
                status = 'completed',
                payment_provider = 'stripe',
                payment_intent_id = $1,
                subtotal = $3::BIGINT / 100.0,
                discount = $4::BIGINT / 100.0,
                total    = $5::BIGINT / 100.0,
                coupon_code = COALESCE($6, coupon_code),
                completed_at = NOW(),
                updated_at = NOW()
            WHERE id = $2",
        )
        .bind(&session.payment_intent)
        .bind(order_id)
        .bind(amount_subtotal_cents)
        .bind(amount_discount_cents)
        .bind(amount_total_cents)
        .bind(&applied_code)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

        tracing::info!(
            target: "payments",
            event = "checkout_completed_reconciled",
            order_id = %order_id,
            session_id = %session.id,
            subtotal_cents = %amount_subtotal_cents,
            discount_cents = %amount_discount_cents,
            total_cents = %amount_total_cents,
            applied_promo = ?applied_code,
            "Order reconciled with Stripe-charged amounts"
        );

        // If subscription, create user membership
        if let Some(ref subscription_id) = session.subscription {
            if let Some(user_id) = user_id {
                // Get subscription details
                let stripe_sub = state
                    .services
                    .stripe
                    .get_subscription(subscription_id)
                    .await
                    .ok();

                if let Some(sub) = stripe_sub {
                    // FIX-2026-04-26: was `.ok().flatten()` which silently dropped DB errors
                    // on the plan-id lookup. Now logs the error and proceeds without
                    // creating the membership (rather than 500'ing the webhook).
                    // Old: .fetch_optional(&state.db.pool).await.ok().flatten()
                    let plan_id: Option<i64> = match sqlx::query_scalar::<_, i64>(
                        "SELECT plan_id FROM order_items WHERE order_id = $1 AND plan_id IS NOT NULL LIMIT 1"
                    )
                    .bind(order_id)
                    .fetch_optional(&mut **tx)
                    .await {
                        Ok(p) => p,
                        Err(e) => {
                            tracing::error!(target: "payments", order_id = %order_id, "plan_id lookup failed in webhook: {}", e);
                            None
                        }
                    };

                    if let Some(plan_id) = plan_id {
                        // Batch 4: re-subscribe history.
                        //
                        // Was ON CONFLICT (user_id, plan_id) DO UPDATE — that
                        // path is now unreachable because migration 063
                        // replaced the (user_id, plan_id) unique constraint
                        // with a *partial* unique index that only covers
                        // live statuses ('active', 'trialing', 'past_due').
                        // A user re-subscribing after full cancellation
                        // therefore gets a NEW row, preserving the prior
                        // cancelled row for history. Stripe is the source
                        // of truth via stripe_subscription_id.
                        //
                        // We deliberately do not upsert. If the partial
                        // unique index fires (an active row already exists
                        // for this user+plan), Postgres returns an error
                        // and we log it — the webhook will retry per
                        // Stripe's idempotency, and on retry the existing
                        // row's stripe_subscription_id will already match.
                        // To handle the "duplicate retry" case cleanly we
                        // make the INSERT idempotent on stripe_subscription_id.
                        // P0-4: the customer has been charged; the membership
                        // grant is part of the SAME transaction. A failure here
                        // must roll the whole thing back and 5xx so Stripe
                        // retries — NOT log-and-continue (which previously left
                        // a charged customer with no membership).
                        sqlx::query(
                            r"INSERT INTO user_memberships (
                                user_id, plan_id, starts_at, status,
                                payment_provider, stripe_subscription_id, stripe_customer_id,
                                current_period_start, current_period_end,
                                cancel_at_period_end, created_at, updated_at
                            ) VALUES ($1, $2, NOW(), 'active', 'stripe', $3, $4, $5, $6, false, NOW(), NOW())
                            ON CONFLICT (stripe_subscription_id) DO UPDATE SET
                                status = 'active',
                                current_period_start = EXCLUDED.current_period_start,
                                current_period_end = EXCLUDED.current_period_end,
                                updated_at = NOW()",
                        )
                        .bind(user_id)
                        .bind(plan_id)
                        .bind(&sub.id)
                        .bind(&sub.customer)
                        .bind({
                            let (start, _) = sub.get_current_period();
                            chrono::DateTime::from_timestamp(start, 0).map(|d| d.naive_utc())
                        })
                        .bind({
                            let (_, end) = sub.get_current_period();
                            chrono::DateTime::from_timestamp(end, 0).map(|d| d.naive_utc())
                        })
                        .execute(&mut **tx)
                        .await
                        .map_err(|e| {
                            tracing::error!(target: "payments", user_id = %user_id, plan_id = %plan_id, "Failed to create user_membership — rolling back webhook tx: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": "Failed to create membership"})),
                            )
                        })?;
                    }
                }
            }
        }

        // P0-4: coupon usage accounting is part of the atomic checkout commit.
        // Routed through `tx` and propagated — a failure rolls back the whole
        // handler so we never under-count redemptions against a completed
        // order (and Stripe retries cleanly because processed_at stays NULL).
        sqlx::query(
            "UPDATE coupons SET usage_count = usage_count + 1, updated_at = NOW() WHERE id = (SELECT coupon_id FROM orders WHERE id = $1)"
        )
        .bind(order_id)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            tracing::error!(target: "payments", order_id = %order_id, "Failed to increment coupon usage — rolling back webhook tx: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update coupon usage"})),
            )
        })?;

        // ═══════════════════════════════════════════════════════════════════════════
        // ACCESS GRANTING - Apple ICT 11+ Principal Engineer Grade
        // ═══════════════════════════════════════════════════════════════════════════
        // Grant access for all purchased items: courses, indicators, and products
        if let Some(user_id) = user_id {
            #[derive(sqlx::FromRow)]
            struct ProductOrderItem {
                product_id: Option<i64>,
                name: String,
            }

            let product_items: Vec<ProductOrderItem> = sqlx::query_as(
                "SELECT product_id, name FROM order_items WHERE order_id = $1 AND product_id IS NOT NULL",
            )
            .bind(order_id)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(
                    target: "payments",
                    event = "access_grant_query_failed",
                    order_id = %order_id,
                    "Failed to fetch order items for access grant: {}",
                    e
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to fetch order items for access grant"})),
                )
            })?;

            for item in product_items {
                if let Some(product_id) = item.product_id {
                    // Get product details to determine type
                    #[derive(sqlx::FromRow)]
                    struct ProductInfo {
                        product_type: Option<String>,
                        course_id: Option<uuid::Uuid>,
                        indicator_id: Option<i64>,
                    }

                    // P0-4: routed through `tx`; a real DB error must roll back
                    // and 5xx (Stripe retries) rather than silently skip
                    // provisioning a paid product. `None` (product row genuinely
                    // absent) still falls through harmlessly.
                    let product_info: Option<ProductInfo> = sqlx::query_as::<_, ProductInfo>(
                        "SELECT type as product_type, course_id, indicator_id FROM products WHERE id = $1",
                    )
                    .bind(product_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(|e| {
                        tracing::error!(target: "payments", product_id = %product_id, "products lookup failed — rolling back webhook tx: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": "Failed to look up product for access grant"})),
                        )
                    })?;

                    if let Some(info) = product_info {
                        // ─────────────────────────────────────────────────────────
                        // 1. Create user_products record for ownership tracking
                        // ─────────────────────────────────────────────────────────
                        // P0-4: ownership row is part of the atomic provision.
                        // Propagate on failure — a charged customer must end up
                        // with the ownership row or the whole tx rolls back and
                        // Stripe retries.
                        sqlx::query(
                            r"INSERT INTO user_products (user_id, product_id, purchased_at, order_id)
                               VALUES ($1, $2, NOW(), $3)
                               ON CONFLICT (user_id, product_id) DO UPDATE SET
                                   purchased_at = COALESCE(user_products.purchased_at, NOW()),
                                   order_id = COALESCE(user_products.order_id, $3)"
                        )
                        .bind(user_id)
                        .bind(product_id)
                        .bind(order_id)
                        .execute(&mut **tx)
                        .await
                        .map_err(|e| {
                            tracing::error!(target: "payments", user_id = %user_id, product_id = %product_id, order_id = %order_id, "user_products INSERT failed — rolling back webhook tx: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": "Failed to record product ownership"})),
                            )
                        })?;

                        tracing::info!(
                            target: "payments",
                            event = "user_product_created",
                            user_id = %user_id,
                            product_id = %product_id,
                            order_id = %order_id,
                            "Product ownership recorded"
                        );

                        // ─────────────────────────────────────────────────────────
                        // 2. Course enrollment
                        // ─────────────────────────────────────────────────────────
                        if let Some(course_id) = info.course_id {
                            // P0-4: course access is part of the atomic
                            // provision; propagate on failure.
                            sqlx::query(
                                r"INSERT INTO user_course_enrollments (user_id, course_id, is_active, enrolled_at)
                                   VALUES ($1, $2, true, NOW())
                                   ON CONFLICT (user_id, course_id) DO UPDATE SET
                                       is_active = true"
                            )
                            .bind(user_id)
                            .bind(course_id)
                            .execute(&mut **tx)
                            .await
                            .map_err(|e| {
                                tracing::error!(target: "payments", user_id = %user_id, course_id = %course_id, "user_course_enrollments INSERT failed — rolling back webhook tx: {}", e);
                                (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(json!({"error": "Failed to enroll user in course"})),
                                )
                            })?;

                            tracing::info!(
                                target: "payments",
                                event = "course_enrollment_created",
                                user_id = %user_id,
                                course_id = %course_id,
                                order_id = %order_id,
                                "User enrolled in course after purchase"
                            );
                        }

                        // ─────────────────────────────────────────────────────────
                        // 3. Indicator access
                        // ─────────────────────────────────────────────────────────
                        if let Some(indicator_id) = info.indicator_id {
                            // P0-4: indicator access is part of the atomic
                            // provision; propagate on failure.
                            sqlx::query(
                                r"INSERT INTO user_indicator_access (user_id, indicator_id, is_active, granted_at)
                                   VALUES ($1, $2, true, NOW())
                                   ON CONFLICT (user_id, indicator_id) DO UPDATE SET
                                       is_active = true,
                                       granted_at = COALESCE(user_indicator_access.granted_at, NOW())"
                            )
                            .bind(user_id)
                            .bind(indicator_id)
                            .execute(&mut **tx)
                            .await
                            .map_err(|e| {
                                tracing::error!(target: "payments", user_id = %user_id, indicator_id = %indicator_id, "user_indicator_access INSERT failed — rolling back webhook tx: {}", e);
                                (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(json!({"error": "Failed to grant indicator access"})),
                                )
                            })?;

                            tracing::info!(
                                target: "payments",
                                event = "indicator_access_granted",
                                user_id = %user_id,
                                indicator_id = %indicator_id,
                                order_id = %order_id,
                                "Indicator access granted after purchase"
                            );
                        }

                        // ─────────────────────────────────────────────────────────
                        // 4. Handle indicator type products without indicator_id FK
                        // ─────────────────────────────────────────────────────────
                        if info.product_type.as_deref() == Some("indicator")
                            && info.indicator_id.is_none()
                        {
                            // Try to find indicator by matching product name/slug
                            // P0-4: routed through `tx`; real DB error
                            // propagates (rollback + Stripe retry). A genuine
                            // no-match still yields None and is skipped.
                            let indicator_id: Option<i64> = sqlx::query_scalar(
                                r"SELECT i.id FROM indicators i
                                   JOIN products p ON LOWER(p.name) LIKE CONCAT('%', LOWER(i.name), '%')
                                   WHERE p.id = $1
                                   LIMIT 1"
                            )
                            .bind(product_id)
                            .fetch_optional(&mut **tx)
                            .await
                            .map_err(|e| {
                                tracing::error!(target: "payments", product_id = %product_id, "indicator-by-name lookup failed — rolling back webhook tx: {}", e);
                                (
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(json!({"error": "Failed to resolve indicator for access grant"})),
                                )
                            })?;

                            if let Some(ind_id) = indicator_id {
                                // P0-4: by-type indicator access is part of the
                                // atomic provision; propagate on failure.
                                sqlx::query(
                                    r"INSERT INTO user_indicator_access (user_id, indicator_id, is_active, granted_at)
                                       VALUES ($1, $2, true, NOW())
                                       ON CONFLICT (user_id, indicator_id) DO UPDATE SET
                                           is_active = true,
                                           granted_at = COALESCE(user_indicator_access.granted_at, NOW())"
                                )
                                .bind(user_id)
                                .bind(ind_id)
                                .execute(&mut **tx)
                                .await
                                .map_err(|e| {
                                    tracing::error!(target: "payments", user_id = %user_id, indicator_id = %ind_id, "user_indicator_access (by-type) INSERT failed — rolling back webhook tx: {}", e);
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        Json(json!({"error": "Failed to grant indicator access"})),
                                    )
                                })?;

                                tracing::info!(
                                    target: "payments",
                                    event = "indicator_access_granted_by_type",
                                    user_id = %user_id,
                                    indicator_id = %ind_id,
                                    product_id = %product_id,
                                    order_id = %order_id,
                                    "Indicator access granted via product type matching"
                                );
                            }
                        }
                    }
                }
            }
        }

        // Batch 6: send subscription-confirmation OR receipt depending
        // on whether this checkout session created a subscription.
        // Best-effort — webhook 200 is more important than email send.
        //
        // P0-4: these reads feed ONLY the best-effort email but they read
        // rows mutated in `tx` (reconciled orders.total, freshly-inserted
        // membership), so they go through `&mut **tx` to see the in-flight
        // state. `.ok()` is retained (not `?`) deliberately: a failed *email*
        // lookup must never roll back a successful charge+provision.
        if let Some(user_id) = user_id {
            #[derive(sqlx::FromRow)]
            struct UserRow {
                email: String,
                name: String,
            }
            let user_row: Option<UserRow> =
                sqlx::query_as("SELECT email, name FROM users WHERE id = $1")
                    .bind(user_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .ok()
                    .flatten();

            #[derive(sqlx::FromRow)]
            struct OrderRow {
                order_number: String,
                total: rust_decimal::Decimal,
            }
            let order_row: Option<OrderRow> =
                sqlx::query_as("SELECT order_number, total FROM orders WHERE id = $1")
                    .bind(order_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .ok()
                    .flatten();

            if let (Some(u), Some(o)) = (user_row, order_row) {
                let amount_dollars: f64 = o.total.to_string().parse::<f64>().unwrap_or(0.0);
                let is_subscription = session.subscription.is_some();
                if is_subscription {
                    // Look up the plan name + period_end for the model.
                    #[derive(sqlx::FromRow)]
                    struct PlanInfo {
                        plan_name: String,
                        current_period_end: Option<chrono::NaiveDateTime>,
                    }
                    let plan_info: Option<PlanInfo> = sqlx::query_as(
                        r"SELECT mp.name AS plan_name, um.current_period_end
                           FROM user_memberships um
                           JOIN membership_plans mp ON mp.id = um.plan_id
                           WHERE um.stripe_subscription_id = $1
                           ORDER BY um.id DESC LIMIT 1",
                    )
                    .bind(session.subscription.as_deref().unwrap_or(""))
                    .fetch_optional(&mut **tx)
                    .await
                    .ok()
                    .flatten();

                    let _ = state
                        .services
                        .email
                        .send_transactional(
                            &state.db.pool,
                            &u.email,
                            "subscription-confirmation",
                            json!({
                                "name": u.name,
                                "plan_name": plan_info.as_ref().map(|p| p.plan_name.clone())
                                    .unwrap_or_else(|| "Membership".to_string()),
                                "amount_dollars": amount_dollars,
                                "period_end": plan_info.as_ref()
                                    .and_then(|p| p.current_period_end)
                                    .map(|t| t.and_utc().to_rfc3339()),
                                "order_number": o.order_number,
                            }),
                        )
                        .await;
                } else {
                    let _ = state
                        .services
                        .email
                        .send_transactional(
                            &state.db.pool,
                            &u.email,
                            "receipt",
                            json!({
                                "name": u.name,
                                "product_name": format!("Order {}", o.order_number),
                                "amount_dollars": amount_dollars,
                                "order_id": order_id,
                                "order_number": o.order_number,
                            }),
                        )
                        .await;
                }
            }
        }

        tracing::info!(
            target: "payments",
            event = "order_completed",
            order_id = %order_id,
            subscription_id = ?session.subscription,
            "Order completed via Stripe"
        );
    }

    Ok(())
}
