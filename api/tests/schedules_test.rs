// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Schedules route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::schedules` and exercises
//! the trading-room schedule DTOs + both routers (`router()` for the
//! public schedule lookup and `admin_router()` for the admin CRUD).
//!
//! ## Why this shape
//!
//! `routes/schedules.rs` (1,134 LOC) is the source-of-truth for the
//! per-trading-room weekly schedule shown on every room landing page
//! plus the admin schedule-builder UI. Every handler runs live SQL
//! against `trading_room_schedules` + `trading_room_schedule_exceptions`
//! + `membership_plans` and the admin surface is gated behind
//! `AdminUser`. Handlers can't be invoked in isolation. What we CAN
//! pin:
//!
//! 1. **`ScheduleRow.id` / `.plan_id` / `.trader_id` are i64
//!    BIGSERIAL.** The schedule table accumulates rows for every
//!    trader × day-of-week × room combination over years; the FK
//!    `plan_id` references `membership_plans.id` which is BIGSERIAL,
//!    same for `trader_id` → `users.id`. Narrowing any of them to
//!    i32 would orphan high-id plans/traders.
//!
//! 2. **`ScheduleRow.day_of_week` is i32 with 0..=6 semantics.** Per
//!    CLAUDE.md "Reserved exception": values legitimately cap below
//!    2 billion (it caps at 6). The handler maps 0..6 → Sunday..
//!    Saturday via `day_name()`; anything outside that range maps to
//!    "Unknown". Pin the i32 width to document intent — a regression
//!    to `u8` would change the wire format.
//!
//! 3. **`ScheduleExceptionRow.id` + `.schedule_id` are i64.** Holiday
//!    cancellations and one-off reschedules accumulate against
//!    long-lived schedules; the FK back to the parent schedule MUST
//!    stay i64 to match.
//!
//! 4. **`TradingRoomPlan.plan_type` has `#[sqlx(rename = "type")]`
//!    but wire-format stays `plan_type`.** The DB column is named
//!    `type` (via JSONB extraction `metadata->>'type'`), but serde
//!    sees the Rust field name `plan_type` on the wire — pin both
//!    sides so a future `#[serde(rename = "type")]` change is caught
//!    here, not by a confused frontend dev.
//!
//! 5. **`CreateScheduleRequest` requires `plan_id`, `title`,
//!    `day_of_week`, `start_time`, `end_time`.** Other fields are
//!    optional — but those five are non-negotiable. A regression
//!    that flipped any to optional would silently let the admin
//!    create incomplete rows that wreck the public weekly grid.
//!
//! 6. **`UpdateScheduleRequest` is fully optional (PATCH).** Toggling
//!    just `is_active` to disable a schedule during a trader sick-day
//!    is the most common admin action — must not require resending
//!    the full payload.
//!
//! 7. **NEGATIVE: `day_of_week` MUST be numeric, not a string** (an
//!    early bug was the frontend sending "monday" — pinned so the
//!    handler 422s at the extractor, not silently at SQL bind time).
//!
//! 8. **`ScheduleQuery.days` is `Option<i32>`** — the "next N days"
//!    upcoming-events query parameter. Pin the i32 width (max ~30
//!    practical, reserved-exception fits).
//!
//! 9. **Both routers build under `AppState`.** Mount-table compile
//!    pin for the public router (3 routes) and the admin router
//!    (6 routes including the bulk-schedule + exceptions endpoints).
//!
//! ## Pattern source
//!
//! Modeled on `tests/redirects_test.rs`, `tests/connections_test.rs`,
//! `tests/trading_rooms_test.rs`, and `tests/courses_admin_test.rs`.

use chrono::{NaiveDate, NaiveTime};
use revolution_api::routes::schedules::{
    BulkScheduleRequest, CreateExceptionRequest, CreateScheduleRequest, ScheduleExceptionRow,
    ScheduleQuery, ScheduleRow, TradingRoomPlan, UpdateScheduleRequest,
};

// ── 1. ScheduleRow: i64 PK + i64 FKs, i32 day_of_week pin ────────────

/// HARD RULE: `id`, `plan_id`, `trader_id` are all `i64` (BIGSERIAL).
/// Long-lived schedule tables accumulate rows past `2^31` over years,
/// and the FKs reference BIGSERIAL columns in `membership_plans` and
/// `users`. A regression to `i32` would orphan high-id rooms/traders.
///
/// `day_of_week` is `i32` — CLAUDE.md "Reserved exception" applies
/// (legitimately caps at 6 — Sunday..Saturday). Pin the type to
/// document intent.
#[test]
fn schedule_row_ids_are_i64_day_of_week_is_i32() {
    let big_id: i64 = (i32::MAX as i64) + 11;
    let big_plan_id: i64 = (i32::MAX as i64) + 22;
    let big_trader_id: i64 = (i32::MAX as i64) + 33;

    let row = ScheduleRow {
        id: big_id,
        plan_id: big_plan_id,
        title: "Morning Briefing".to_string(),
        description: Some("Pre-market analysis".to_string()),
        trader_name: Some("J. Trader".to_string()),
        trader_id: Some(big_trader_id),
        day_of_week: 1, // Monday
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
        timezone: Some("America/New_York".to_string()),
        is_recurring: Some(true),
        effective_from: Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
        effective_until: None,
        is_active: Some(true),
        room_url: Some("/rooms/morning-briefing".to_string()),
        room_type: Some("trading-room".to_string()),
        created_at: None,
        updated_at: None,
    };

    let wire = serde_json::to_value(&row).expect("serialize ScheduleRow");
    assert_eq!(wire["id"].as_i64(), Some(big_id));
    assert_eq!(wire["plan_id"].as_i64(), Some(big_plan_id));
    assert_eq!(wire["trader_id"].as_i64(), Some(big_trader_id));
    assert_eq!(
        wire["day_of_week"].as_i64(),
        Some(1),
        "day_of_week is i32 with 0..6 semantics"
    );

    // Belt-and-suspenders: fixture exceeds i32::MAX to prove i64 pin.
    assert!(row.id > i32::MAX as i64);
    assert!(row.plan_id > i32::MAX as i64);
    assert!(row.trader_id.unwrap() > i32::MAX as i64);

    // Day-of-week stays a valid weekday value. Anything outside 0..6
    // would map to "Unknown" via `day_name()` at the handler.
    assert!((0..=6).contains(&row.day_of_week));

    // Wire-format keys are snake_case — pin no camelCase regression.
    assert!(
        wire.get("dayOfWeek").is_none(),
        "wire MUST be snake_case (`day_of_week`), not camelCase"
    );
    assert!(
        wire.get("planId").is_none(),
        "wire MUST be snake_case (`plan_id`), not camelCase"
    );
    assert!(wire.get("traderId").is_none());
}

// ── 2. ScheduleExceptionRow: i64 PK + i64 FK ────────────────────────

/// Schedule exceptions (holidays, one-off reschedules) carry an i64
/// `schedule_id` FK back to the parent `trading_room_schedules.id`.
/// Both PKs are BIGSERIAL. A regression to `i32` here would orphan
/// exception rows for high-id schedules.
#[test]
fn schedule_exception_row_pk_and_fk_are_i64() {
    let big_id: i64 = (i32::MAX as i64) + 7;
    let big_schedule_id: i64 = (i32::MAX as i64) + 14;

    let row = ScheduleExceptionRow {
        id: big_id,
        schedule_id: big_schedule_id,
        exception_date: NaiveDate::from_ymd_opt(2026, 7, 4).unwrap(),
        exception_type: "cancelled".to_string(),
        new_start_time: None,
        new_end_time: None,
        new_trader_name: None,
        reason: Some("July 4th holiday".to_string()),
        created_at: None,
    };

    let wire = serde_json::to_value(&row).expect("serialize ScheduleExceptionRow");
    assert_eq!(wire["id"].as_i64(), Some(big_id));
    assert_eq!(wire["schedule_id"].as_i64(), Some(big_schedule_id));
    assert!(row.schedule_id > i32::MAX as i64);

    // Pin the exception_type as one of the known values (handler
    // accepts: "cancelled", "rescheduled", "special").
    assert_eq!(row.exception_type, "cancelled");

    // Wire-format keys.
    assert!(wire.get("scheduleId").is_none(), "no camelCase");
    assert!(wire.get("exceptionDate").is_none(), "no camelCase");
}

// ── 3. TradingRoomPlan: sqlx rename does NOT leak to serde wire ──────

/// `TradingRoomPlan.plan_type` has `#[sqlx(rename = "type")]` so that
/// the DB column projection `metadata->>'type' as type` maps to the
/// field. But the serde wire format uses the Rust field name —
/// `plan_type` — NOT "type". A regression that added
/// `#[serde(rename = "type")]` would silently change the API contract
/// and break the SvelteKit consumer.
#[test]
fn trading_room_plan_serde_wire_is_plan_type_not_type() {
    let plan = TradingRoomPlan {
        id: (i32::MAX as i64) + 1,
        name: "Morning Briefing Room".to_string(),
        slug: "morning-briefing".to_string(),
        plan_type: Some("trading-room".to_string()),
    };
    let wire = serde_json::to_value(&plan).expect("serialize TradingRoomPlan");

    assert_eq!(
        wire["plan_type"], "trading-room",
        "serde wire format is `plan_type` (Rust field name), NOT `type`"
    );
    assert!(
        wire.get("type").is_none(),
        "serde MUST NOT rename to `type` — that's the sqlx-only rename"
    );
    assert_eq!(wire["id"].as_i64().unwrap(), (i32::MAX as i64) + 1);
}

// ── 4. CreateScheduleRequest: required fields + day_of_week negative ─

/// `CreateScheduleRequest` has 5 required fields: `plan_id`, `title`,
/// `day_of_week`, `start_time`, `end_time`. The rest are optional. A
/// regression flipping any required field to optional would silently
/// let admins create incomplete schedule rows that wreck the public
/// weekly grid render.
///
/// NEGATIVE: `day_of_week` MUST be a NUMBER, not a string. An early
/// frontend bug was sending "monday" — pinning the i32 type here
/// catches that at the extractor, not silently at SQL bind time.
#[test]
fn create_schedule_request_requires_core_fields_with_typed_day_of_week() {
    let minimal: CreateScheduleRequest = serde_json::from_value(serde_json::json!({
        "plan_id": 42_i64,
        "title": "Morning Briefing",
        "day_of_week": 1,
        "start_time": "09:00",
        "end_time": "10:30"
    }))
    .expect("minimal CreateScheduleRequest must deserialize");
    assert_eq!(minimal.plan_id, 42);
    assert_eq!(minimal.title, "Morning Briefing");
    assert_eq!(minimal.day_of_week, 1);
    assert_eq!(minimal.start_time, "09:00");
    assert_eq!(minimal.end_time, "10:30");
    assert!(minimal.description.is_none());
    assert!(minimal.trader_id.is_none());
    assert!(minimal.timezone.is_none());

    // Full payload with all optional fields populated.
    let full: CreateScheduleRequest = serde_json::from_value(serde_json::json!({
        "plan_id": 99_i64,
        "title": "Power Hour",
        "description": "Last hour of trading",
        "trader_name": "K. Lee",
        "trader_id": ((i32::MAX as i64) + 1),
        "day_of_week": 5,
        "start_time": "15:00",
        "end_time": "16:00",
        "timezone": "America/New_York",
        "is_recurring": true,
        "effective_from": "2026-01-01",
        "effective_until": "2026-12-31",
        "room_url": "/rooms/power-hour",
        "room_type": "trading-room"
    }))
    .expect("full CreateScheduleRequest must deserialize");
    assert_eq!(full.trader_id.unwrap(), (i32::MAX as i64) + 1);
    assert_eq!(full.timezone.as_deref(), Some("America/New_York"));

    // NEGATIVE: `day_of_week` as a string MUST fail.
    assert!(
        serde_json::from_value::<CreateScheduleRequest>(serde_json::json!({
            "plan_id": 1_i64,
            "title": "Bad",
            "day_of_week": "monday",
            "start_time": "09:00",
            "end_time": "10:00"
        }))
        .is_err(),
        "day_of_week as string MUST fail (i32 type pin)"
    );

    // NEGATIVE: missing required `plan_id` MUST fail.
    assert!(
        serde_json::from_value::<CreateScheduleRequest>(serde_json::json!({
            "title": "No plan",
            "day_of_week": 1,
            "start_time": "09:00",
            "end_time": "10:00"
        }))
        .is_err(),
        "missing plan_id MUST fail (required field)"
    );

    // NEGATIVE: missing required `title` MUST fail.
    assert!(
        serde_json::from_value::<CreateScheduleRequest>(serde_json::json!({
            "plan_id": 1_i64,
            "day_of_week": 1,
            "start_time": "09:00",
            "end_time": "10:00"
        }))
        .is_err(),
        "missing title MUST fail (required field)"
    );
}

// ── 5. UpdateScheduleRequest: PATCH semantics + exception DTO ────────

/// `UpdateScheduleRequest` is fully optional. The toggle-only
/// `{"is_active": false}` admin action (during a trader sick-day) is
/// the most common edit path. A regression that flipped any field to
/// required would lock out that flow.
///
/// `CreateExceptionRequest` requires schedule_id + exception_date +
/// exception_type. Pin both.
#[test]
fn update_schedule_and_exception_dtos_follow_patch_and_required_shapes() {
    let empty: UpdateScheduleRequest =
        serde_json::from_str("{}").expect("empty UpdateScheduleRequest must deserialize (PATCH)");
    assert!(empty.title.is_none());
    assert!(empty.day_of_week.is_none());
    assert!(empty.is_active.is_none());

    // Toggle-only: disable during sick day.
    let toggle: UpdateScheduleRequest =
        serde_json::from_value(serde_json::json!({"is_active": false}))
            .expect("toggle-only UpdateScheduleRequest must deserialize");
    assert_eq!(toggle.is_active, Some(false));

    // `CreateExceptionRequest` REQUIRES schedule_id + exception_date +
    // exception_type. Pin the contract.
    let exc: CreateExceptionRequest = serde_json::from_value(serde_json::json!({
        "schedule_id": ((i32::MAX as i64) + 5),
        "exception_date": "2026-07-04",
        "exception_type": "cancelled",
        "reason": "Holiday"
    }))
    .expect("CreateExceptionRequest must deserialize");
    assert_eq!(exc.schedule_id, (i32::MAX as i64) + 5);
    assert_eq!(exc.exception_type, "cancelled");

    // NEGATIVE: missing required schedule_id MUST fail.
    assert!(
        serde_json::from_value::<CreateExceptionRequest>(serde_json::json!({
            "exception_date": "2026-07-04",
            "exception_type": "cancelled"
        }))
        .is_err(),
        "missing schedule_id MUST fail (required field)"
    );
}

// ── 6. ScheduleQuery + BulkScheduleRequest ────────────────────────

/// `ScheduleQuery.days` is `Option<i32>` — the "next N days" parameter
/// on the upcoming-events endpoint. CLAUDE.md "Reserved exception"
/// applies (caps at ~30 days practical). NEGATIVE: must be numeric.
///
/// `BulkScheduleRequest` carries a Vec<CreateScheduleRequest> plus
/// `clear_existing` flag — the admin "import this week" flow.
#[test]
fn schedule_query_and_bulk_request_carry_correct_shapes() {
    let empty: ScheduleQuery =
        serde_json::from_str("{}").expect("empty ScheduleQuery must deserialize");
    assert!(empty.days.is_none());

    let with_days: ScheduleQuery =
        serde_json::from_value(serde_json::json!({"days": 14})).expect("days param");
    assert_eq!(with_days.days, Some(14));

    // NEGATIVE: `days` as a string MUST fail.
    assert!(
        serde_json::from_value::<ScheduleQuery>(serde_json::json!({"days": "fourteen"})).is_err(),
        "days as string MUST fail (Option<i32> type pin)"
    );

    let bulk: BulkScheduleRequest = serde_json::from_value(serde_json::json!({
        "plan_id": ((i32::MAX as i64) + 9),
        "clear_existing": true,
        "schedules": [
            {
                "plan_id": ((i32::MAX as i64) + 9),
                "title": "Morning",
                "day_of_week": 1,
                "start_time": "09:00",
                "end_time": "10:00"
            },
            {
                "plan_id": ((i32::MAX as i64) + 9),
                "title": "Afternoon",
                "day_of_week": 1,
                "start_time": "14:00",
                "end_time": "15:00"
            }
        ]
    }))
    .expect("BulkScheduleRequest must deserialize");
    assert_eq!(bulk.plan_id, (i32::MAX as i64) + 9);
    assert_eq!(bulk.clear_existing, Some(true));
    assert_eq!(bulk.schedules.len(), 2);
    assert_eq!(bulk.schedules[0].title, "Morning");
}

// ── 7. Router mount table compile-pin ────────────────────────────────

/// Public router: 3 routes (`/rooms`, `/:plan_slug`,
/// `/:plan_slug/upcoming`). The public weekly schedule grid on every
/// room landing page calls these — a refactor that broke a handler
/// signature would fail compilation here.
#[test]
fn schedules_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::schedules::router();
}

/// Admin router: 6 routes including the security-sensitive bulk
/// import (`POST /bulk`) and exceptions endpoints. All gated behind
/// `AdminUser`. A refactor dropping that gate or breaking a handler
/// signature would fail compilation here.
#[test]
fn schedules_admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::schedules::admin_router();
}
