//! Course analytics + Stripe price-change handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).
//!
//! MONEY PATH (R13-B): `change_course_price` operates on `amount_cents` (i64),
//! creates a Stripe Price + updates `courses.price_cents` + writes a
//! `security_events` audit row. The split is structural ONLY — every SQL
//! statement, tracing field, transaction boundary and audit payload below
//! is byte-identical to the pre-split monolith.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE ANALYTICS — wires CourseDetailDrawer "Analytics" tab to real DB rows.
// FIX-2026-04-26: New handler — replaces the hard-coded `-` placeholders the drawer
// previously rendered for Total Views / Unique Visitors / Conversion / Avg Time /
// Drop-off / Revenue. Aggregates over `analytics_events`, `user_course_enrollments`,
// `user_lesson_progress`, and `orders` + `order_items`.
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) async fn get_course_analytics(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Total Views: count of analytics_events where event_type='course_view'
    // and properties->>'course_id' matches. We use ::text comparison so this works
    // whether the property is stored as a string or a uuid.
    // TODO: implement dedicated `course_views` table for higher-fidelity tracking.
    let total_views: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)::bigint
        FROM analytics_events
        WHERE event_type IN ('course_view', 'page_view')
          AND (properties->>'course_id') = $1::text
        ",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Unique Visitors: distinct sessions (or users) who viewed this course.
    let unique_visitors: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(DISTINCT COALESCE(session_id, user_id::text))::bigint
        FROM analytics_events
        WHERE event_type IN ('course_view', 'page_view')
          AND (properties->>'course_id') = $1::text
        ",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Total enrollments — denominator for conversion rate.
    let total_enrollments: (i64,) =
        sqlx::query_as("SELECT COUNT(*)::bigint FROM user_course_enrollments WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    // Conversion Rate = enrollments / unique_visitors * 100. Guarded against /0.
    let conversion_rate = if unique_visitors.0 > 0 {
        (total_enrollments.0 as f64 / unique_visitors.0 as f64) * 100.0
    } else {
        0.0
    };

    // Avg Time on Course (seconds) = mean of per-user total watch time.
    // Falls back to 0 when no progress rows exist.
    let avg_time: (Option<f64>,) = sqlx::query_as(
        r"
        SELECT AVG(per_user_total)::float8
        FROM (
            SELECT user_id, SUM(watch_time_total_seconds)::bigint AS per_user_total
            FROM user_lesson_progress
            WHERE course_id = $1
            GROUP BY user_id
        ) t
        ",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));
    let avg_time_seconds = avg_time.0.unwrap_or(0.0).round() as i64;

    // Drop-off Rate = (started_but_not_completed) / started * 100.
    // "Started" = enrollment row exists with progress_percent > 0.
    let started: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint FROM user_course_enrollments
         WHERE course_id = $1 AND progress_percent > 0",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));
    let dropped: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint FROM user_course_enrollments
         WHERE course_id = $1
           AND progress_percent > 0
           AND progress_percent < 100
           AND completed_at IS NULL",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));
    let drop_off_rate = if started.0 > 0 {
        (dropped.0 as f64 / started.0 as f64) * 100.0
    } else {
        0.0
    };

    // Revenue (cents): sum of completed order_items for this course.
    // Joins order_items -> orders, filters status='completed'. Stores totals as
    // DECIMAL — multiply by 100 and cast to bigint for cents.
    // TODO: when order_items.course_id column lands, swap the metadata-JSON path
    // for a direct FK match. For now we look at order_items.metadata->>'course_id'.
    let revenue: (Option<f64>,) = sqlx::query_as(
        r"
        SELECT COALESCE(SUM(oi.total), 0)::float8
        FROM order_items oi
        JOIN orders o ON o.id = oi.order_id
        WHERE o.status = 'completed'
          AND (oi.metadata->>'course_id') = $1::text
        ",
    )
    .bind(course_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((Some(0.0),));
    let revenue_cents = (revenue.0.unwrap_or(0.0) * 100.0).round() as i64;

    // Recent Enrollments (last 5) — wires the "Enrollment data coming soon" empty
    // state in the Enrollments tab. Returns user email + enrolled_at.
    #[allow(clippy::type_complexity)]
    let recent_rows: Vec<(i64, Option<String>, Option<chrono::DateTime<chrono::Utc>>)> =
        sqlx::query_as(
            r"
            SELECT e.user_id, u.email, e.enrolled_at
            FROM user_course_enrollments e
            LEFT JOIN users u ON u.id = e.user_id
            WHERE e.course_id = $1
            ORDER BY e.enrolled_at DESC NULLS LAST
            LIMIT 5
            ",
        )
        .bind(course_id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    let recent_enrollments: Vec<serde_json::Value> = recent_rows
        .into_iter()
        .map(|(uid, email, ts)| {
            json!({
                "user_id": uid,
                "email": email,
                "enrolled_at": ts,
            })
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "course_id": course_id,
            "total_views": total_views.0,
            "unique_visitors": unique_visitors.0,
            "conversion_rate": conversion_rate,
            "avg_time_seconds": avg_time_seconds,
            "drop_off_rate": drop_off_rate,
            "revenue_cents": revenue_cents,
            "recent_enrollments": recent_enrollments,
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PRICE CHANGE
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub(super) struct ChangeCoursePrice {
    amount_cents: i64,
    currency: Option<String>,
}

pub(super) async fn change_course_price(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<Uuid>,
    Json(input): Json<ChangeCoursePrice>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.amount_cents <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "amount_cents must be > 0"})),
        ));
    }
    let currency = input.currency.unwrap_or_else(|| "usd".to_string());

    #[derive(sqlx::FromRow)]
    struct CourseForPriceChange {
        id: Uuid,
        title: String,
        stripe_price_id: Option<String>,
        stripe_product_id: Option<String>,
        // CLAUDE.md money rule: every *_cents value is i64 / BIGINT end-to-end.
        // Widened from i32 (audit R17-D, 2026-05-20). The DB column
        // `courses.price_cents` is BIGINT since migration 068; the i32
        // type here had been a silent overflow exposure on the read side.
        price_cents: i64,
    }

    let course: CourseForPriceChange = sqlx::query_as(
        r"SELECT id, title, stripe_price_id, stripe_product_id, price_cents
           FROM courses WHERE id = $1",
    )
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, course_id = %course_id, "DB error fetching course");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database error"})))
    })?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Course not found"}))))?;

    let old_price_id = course.stripe_price_id.clone();

    let env_scope = state.config.environment.clone();
    let stripe = state
        .services
        .credentials
        .stripe_client(&state.db.pool, &env_scope)
        .await;

    let product_id = match course.stripe_product_id.clone() {
        Some(pid) => pid,
        None => stripe
            .create_product(&course.title)
            .await
            .map_err(|e| {
                tracing::error!(target: "stripe_price", error = %e, course_id = %course_id, "Failed to create Stripe product");
                (StatusCode::BAD_GATEWAY, Json(json!({"error": format!("Stripe product create failed: {}", e)})))
            })?,
    };

    let new_price = stripe
        .create_price(&product_id, input.amount_cents, &currency, "one_time")
        .await
        .map_err(|e| {
            tracing::error!(target: "stripe_price", error = %e, course_id = %course_id, "Failed to create Stripe price");
            (StatusCode::BAD_GATEWAY, Json(json!({"error": format!("Stripe price create failed: {}", e)})))
        })?;

    let new_price_id = new_price.id.clone();

    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to start transaction"})),
        )
    })?;

    sqlx::query(
        r"UPDATE courses
           SET stripe_price_id = $1, stripe_product_id = $2, price_cents = $3, updated_at = NOW()
           WHERE id = $4",
    )
    .bind(&new_price_id)
    .bind(&product_id)
    // CLAUDE.md money rule: bind i64 to BIGINT, never downcast to i32.
    // Pre-fix: `.bind(input.amount_cents as i32)` silently truncated values
    // above $21,474,836.47. `courses.price_cents` is BIGINT since 068.
    // (audit R17-D, 2026-05-20)
    .bind(input.amount_cents)
    .bind(course_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, course_id = %course_id, "Failed to update course");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update course"})))
    })?;

    let details = json!({
        "course_id": course_id,
        "old_stripe_price_id": old_price_id,
        "new_stripe_price_id": &new_price_id,
        "old_amount_cents": course.price_cents,
        "new_amount_cents": input.amount_cents,
        "currency": &currency,
        "changed_by_user_id": admin.id,
    });
    if let Err(e) = sqlx::query(
        r"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
           VALUES ($1, 'course_price_changed', 'billing', 'medium', $2)",
    )
    .bind(admin.id)
    .bind(&details)
    .execute(&mut *tx)
    .await
    {
        tracing::warn!(target: "stripe_price", error = %e, "Failed to insert security_event for course price change");
    }

    tx.commit().await.map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to commit transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit DB changes"})),
        )
    })?;

    tracing::info!(
        target: "stripe_price",
        event = "course_price_changed",
        course_id = %course_id,
        old_price_id = ?old_price_id,
        new_price_id = %new_price_id,
        amount_cents = %input.amount_cents,
        admin_id = %admin.id,
        "Course price updated; Stripe Price created and DB pointer flipped"
    );

    Ok(Json(json!({
        "success": true,
        "course_id": course_id,
        "old_stripe_price_id": old_price_id,
        "new_stripe_price_id": new_price_id,
        "amount_cents": input.amount_cents,
        "currency": currency,
    })))
}
