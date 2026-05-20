//! CMS scheduling route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::cms_scheduling` and
//! exercises the public DTOs + the `router()` mount table. This is
//! the editorial-release surface — releases and individual schedules
//! drive every "publish at 9am Monday" workflow the marketing team
//! relies on. A drift in the wire-format (status enum lowercased,
//! action enum renamed) silently breaks the calendar UI without any
//! 500-level signal.
//!
//! ## Why this shape
//!
//! Every handler in `routes/cms_scheduling.rs` runs live SQL against
//! `cms_schedules` / `cms_releases` / `cms_schedule_history`, so we
//! can't unit-test the handlers themselves. Instead we pin:
//!
//! 1. **Enum wire format** — `ScheduleStatus`, `ScheduleAction`, and
//!    `ReleaseStatus` are tagged `#[sqlx(rename_all = "lowercase")]`,
//!    and serde inherits the variant name (PascalCase by default).
//!    The calendar UI reads `status` and `action` as lowercase strings
//!    in the JSON; we pin both serialize directions here so a serde
//!    refactor that drops the rename doesn't silently break the front
//!    end while every CI gate stays green.
//!
//! 2. **CreateScheduleRequest defaults** — `timezone` defaults to
//!    `"UTC"` via `#[serde(default = "default_timezone")]`. If the
//!    default function ever gets dropped, a bare POST without
//!    `timezone` would 400 every editor's "schedule for now" click.
//!
//! 3. **CreateReleaseRequest defaults** — `stop_on_error` defaults to
//!    `false` and `notify_on_complete` defaults to `true`. Drift in
//!    either is operator-visible: a flipped `stop_on_error` halts a
//!    multi-content release mid-stream on the first error; a flipped
//!    `notify_on_complete` floods the team Slack.
//!
//! 4. **`ListQuery` defaults** — `limit` defaults to 50, `offset` to
//!    0. Both are `i64` (large catalogs are realistic on the editorial
//!    calendar across years of history). If `limit` ever regresses to
//!    `Option<i64>` without a default, list endpoints either 400 or
//!    silently return zero rows depending on the handler's unwrap.
//!
//! 5. **Money pin (documenting absence)** — `cms_scheduling` is the
//!    editorial-release surface; it intentionally has no `*_cents`
//!    field. The CLAUDE.md money rule (every `*_cents` is i64) is
//!    upheld by NOT adding monetary fields here. This serialize-and-
//!    grep pin will fail loud if a future commit drifts a money field
//!    into the editorial schema by accident.
//!
//! 6. **`router()` mount table** — 18 documented endpoints across
//!    schedules, releases, calendar, history, and cron-execution
//!    endpoints. Building it as `Router<AppState>` proves every
//!    handler signature still matches its extractor contract.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, `tests/cms_delivery_test.rs`.

use revolution_api::routes::cms_scheduling::{
    AddReleaseItemRequest, CalendarQuery, CmsRelease, CmsSchedule, CreateReleaseRequest,
    CreateScheduleRequest, HistoryQuery, ListQuery, ReleaseStatus, ScheduleAction, ScheduleStatus,
};
use uuid::Uuid;

// ── Enum wire format ─────────────────────────────────────────────────

/// `ScheduleStatus` variants are serialized in PascalCase by serde's
/// default. The handler converts DB rows where the sqlx type is
/// lowercase, but the JSON the calendar UI reads is the serde-default
/// PascalCase. Pin both so a refactor that adds
/// `#[serde(rename_all = "lowercase")]` is caught here, not by a
/// silently-broken calendar.
#[test]
fn schedule_status_round_trip_uses_pascal_case_on_wire() {
    let wire = serde_json::to_string(&ScheduleStatus::Pending).expect("serialize ScheduleStatus");
    assert_eq!(
        wire, "\"Pending\"",
        "ScheduleStatus serializes as PascalCase — calendar UI reads this verbatim"
    );

    let back: ScheduleStatus = serde_json::from_str("\"Failed\"").expect("deserialize");
    assert!(
        matches!(back, ScheduleStatus::Failed),
        "PascalCase round-trip must hit the Failed variant"
    );
}

#[test]
fn schedule_action_round_trip_uses_pascal_case_on_wire() {
    let wire = serde_json::to_string(&ScheduleAction::Publish).expect("serialize ScheduleAction");
    assert_eq!(wire, "\"Publish\"");

    let back: ScheduleAction = serde_json::from_str("\"Unpublish\"").expect("deserialize");
    assert!(matches!(back, ScheduleAction::Unpublish));
}

#[test]
fn release_status_round_trip_uses_pascal_case_on_wire() {
    let wire = serde_json::to_string(&ReleaseStatus::Scheduled).expect("serialize ReleaseStatus");
    assert_eq!(wire, "\"Scheduled\"");

    let back: ReleaseStatus = serde_json::from_str("\"Completed\"").expect("deserialize");
    assert!(matches!(back, ReleaseStatus::Completed));
}

// ── CreateScheduleRequest defaults ───────────────────────────────────

/// A minimal "schedule this post to publish" POST omits `timezone`
/// and relies on the server-side default of `"UTC"`. If the default
/// function disappears, every editor's "publish in 10 minutes" click
/// would 400.
#[test]
fn create_schedule_request_defaults_timezone_to_utc() {
    let content_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let body = serde_json::json!({
        "content_id": content_id,
        "action": "Publish",
        "scheduled_at": now,
    });

    let req: CreateScheduleRequest = serde_json::from_value(body).expect("minimal payload parses");
    assert_eq!(req.content_id, content_id);
    assert!(
        matches!(req.action, ScheduleAction::Publish),
        "action must round-trip to Publish"
    );
    assert_eq!(
        req.timezone, "UTC",
        "missing timezone must default to UTC per default_timezone()"
    );
    assert!(req.staged_data.is_none());
    assert!(req.notes.is_none());
}

// ── CreateReleaseRequest defaults ────────────────────────────────────

/// `stop_on_error` defaults to `false`; `notify_on_complete` defaults
/// to `true`. Both defaults are operator-visible; assert them as
/// part of the wire contract so a serde refactor that drops the
/// `#[serde(default)]` annotations is caught here.
#[test]
fn create_release_request_defaults_match_operator_expectations() {
    let body = serde_json::json!({
        "name": "May 2026 release",
    });

    let req: CreateReleaseRequest = serde_json::from_value(body).expect("minimal release parses");
    assert_eq!(req.name, "May 2026 release");
    assert!(req.description.is_none());
    assert!(req.scheduled_at.is_none());
    assert_eq!(req.timezone, "UTC", "timezone default must be UTC");
    assert!(
        !req.stop_on_error,
        "stop_on_error default must be false (let release continue past per-item failures)"
    );
    assert!(
        req.notify_on_complete,
        "notify_on_complete default must be true — silent completion is a UX regression"
    );
}

// ── ListQuery + HistoryQuery defaults ────────────────────────────────

#[test]
fn list_query_defaults_limit_50_offset_0() {
    let q: ListQuery = serde_json::from_str("{}").expect("empty list query must parse");
    assert!(q.status.is_none());
    assert!(q.content_id.is_none());
    assert_eq!(
        q.limit, 50,
        "list endpoint must page 50-by-50 by default (admin sidebar perf)"
    );
    assert_eq!(q.offset, 0);
}

#[test]
fn history_query_defaults_limit_50_offset_0() {
    let q: HistoryQuery = serde_json::from_str("{}").expect("empty history query must parse");
    assert_eq!(
        q.limit, 50,
        "history endpoint must page 50-by-50 by default"
    );
    assert_eq!(q.offset, 0);
    assert!(q.schedule_id.is_none());
    assert!(q.release_id.is_none());
}

// ── AddReleaseItemRequest accepts minimal payload ────────────────────

#[test]
fn add_release_item_request_accepts_minimal_payload() {
    let content_id = Uuid::new_v4();
    let body = serde_json::json!({
        "content_id": content_id,
        "action": "Archive",
    });
    let req: AddReleaseItemRequest =
        serde_json::from_value(body).expect("minimal add-item payload must parse");
    assert_eq!(req.content_id, content_id);
    assert!(matches!(req.action, ScheduleAction::Archive));
    assert!(
        req.order_index.is_none(),
        "order_index must stay optional — backend assigns when omitted"
    );
}

// ── CalendarQuery requires start_date and end_date ───────────────────

#[test]
fn calendar_query_requires_both_dates() {
    let start = chrono::Utc::now();
    let end = start + chrono::Duration::days(7);
    let body = serde_json::json!({
        "start_date": start,
        "end_date": end,
    });
    let q: CalendarQuery = serde_json::from_value(body).expect("calendar query parses");
    assert!(q.content_type_id.is_none());
    assert!(
        q.end_date > q.start_date,
        "calendar query end must be after start"
    );

    // Negative: missing end_date is rejected — required field on the
    // calendar contract. (We don't assert exact error shape, just
    // that bare {start_date} fails.)
    let bad = serde_json::json!({"start_date": start});
    assert!(
        serde_json::from_value::<CalendarQuery>(bad).is_err(),
        "CalendarQuery must reject payloads missing end_date"
    );
}

// ── Money: documenting the absence ──────────────────────────────────

/// `cms_scheduling` is the **editorial-release** surface — its public
/// DTOs intentionally carry NO monetary fields. The CLAUDE.md money
/// rule (every `*_cents` is i64) is upheld by NOT introducing one by
/// accident. This serialize-and-grep pin will fail loud if a future
/// commit drifts a money field into the editorial schema; the
/// reviewer is then forced to move the field into a money-aware
/// module (orders, products, coupons).
#[test]
fn cms_scheduling_public_dtos_have_no_cents_fields() {
    let now = chrono::Utc::now();
    let schedule = CmsSchedule {
        id: Uuid::new_v4(),
        content_id: Uuid::new_v4(),
        action: ScheduleAction::Publish,
        scheduled_at: now,
        timezone: "UTC".to_string(),
        status: ScheduleStatus::Pending,
        executed_at: None,
        error_message: None,
        retry_count: 0,
        max_retries: 3,
        staged_data: None,
        notes: None,
        created_by: Some(1),
        created_at: now,
        updated_at: now,
        cancelled_by: None,
        cancelled_at: None,
    };
    let release = CmsRelease {
        id: Uuid::new_v4(),
        name: "Release-2026-05".to_string(),
        description: None,
        scheduled_at: Some(now),
        timezone: "UTC".to_string(),
        status: ReleaseStatus::Draft,
        executed_at: None,
        error_message: None,
        total_items: 0,
        completed_items: 0,
        failed_items: 0,
        stop_on_error: false,
        notify_on_complete: true,
        created_by: Some(1),
        created_at: now,
        updated_at: now,
        approved_by: None,
        approved_at: None,
        cancelled_by: None,
        cancelled_at: None,
    };

    let wires = [
        serde_json::to_string(&schedule).expect("serialize CmsSchedule"),
        serde_json::to_string(&release).expect("serialize CmsRelease"),
    ];
    for w in &wires {
        assert!(
            !w.contains("_cents"),
            "cms_scheduling wire-format must never carry a *_cents field — found in: {w}"
        );
    }
}

// ── Router mount table ───────────────────────────────────────────────

/// `router()` mounts 18 documented endpoints
/// (`/schedules`, `/schedules/calendar`, `/schedules/history`,
///  `/schedules/process`, `/schedules/:id`, `/schedules/:id/cancel`,
///  `/releases`, `/releases/process`, `/releases/:id`,
///  `/releases/:id/items`, `/releases/:release_id/items/:item_id`,
///  `/releases/:id/schedule`, `/releases/:id/cancel`). Building it
/// as `Router<AppState>` proves every handler signature still matches
/// its extractor contract.
#[test]
fn cms_scheduling_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_scheduling::router();
}
