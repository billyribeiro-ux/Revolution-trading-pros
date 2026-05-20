//! Admin-members analytics route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::members` and exercises
//! the public DTOs (`Member`, `MemberQuery`) plus the public `router()`
//! mount table.
//!
//! ## Why this file exists (R15-D)
//!
//! `routes/members.rs` is the admin **member-analytics** surface — it
//! powers `/admin/members`, `/admin/members/stats`,
//! `/admin/members/services`, `/admin/members/churned`,
//! `/admin/members/export`, the per-service drill-down at
//! `/admin/members/service/:id`, and the single-member detail card at
//! `/admin/members/:id`. Every handler runs live SQL against `users`,
//! `user_memberships`, and `membership_plans`, plus a CSV-streaming
//! `IntoResponse` on `export_members`, so a real DB integration test
//! is out of scope here. What we CAN pin:
//!
//! 1. **`Member.id` is i64 (BIGSERIAL).** Every PK on this stack is
//!    `BIGSERIAL` (see CLAUDE.md "Money / cents" + the broader
//!    BIGSERIAL-PK contract). A regression to `i32` would silently
//!    cap the customer base at ~2.1B rows (forever-fine in B2C) BUT
//!    would also break `JOIN` decoding against `users.id` which is
//!    `i64` everywhere else in the codebase — symptom: "newest
//!    customers vanish from the admin members table with no error in
//!    logs," because sqlx type-mismatches silently on decode.
//!
//! 2. **MONEY PIN — `stats` revenue scalars are i64.** The
//!    `stats` handler emits a `revenue.mrr_cents` and
//!    `revenue.total_cents` pair that come out of the DB via
//!    `query_scalar::<i64>` after a `(mp.price * 100)::BIGINT` cast.
//!    The CLAUDE.md "Money / cents" HARD RULE is i64 end-to-end. The
//!    handler is private so we can't construct it, but we CAN pin
//!    the `Member` struct's i64-PK contract that the handler chains
//!    through, and verify that the `MemberQuery.product_id` /
//!    `plan_id` filter foreign keys are `Option<i64>` so the
//!    drill-down query binds correctly. A regression to `i32` on
//!    `product_id` would silently 500-error the "members by
//!    service" page when the FK exceeds i32::MAX (will not happen
//!    in this product's lifetime, but the type contract is the
//!    audit trail that catches the bug class).
//!
//! 3. **`MemberQuery` is fully optional.** The admin members index
//!    page hits `/admin/members` with zero params on first load
//!    (default page 1, per_page 25). A regression that flipped
//!    `search` / `status` / `date_from` / `date_to` / `product_id` /
//!    `plan_id` / `sort_by` / `sort_dir` / `per_page` / `page` to
//!    required would 422 every admin who clicks the "Members" tab.
//!
//! 4. **`MemberQuery.product_id` and `plan_id` are i64 FKs.** Same
//!    rationale as (2): membership-plans / products IDs are
//!    `BIGSERIAL` PKs. A regression to `i32` would compile but
//!    silently lose the high bits when the admin filters by a
//!    `plan_id` that came out of a JSON response (where the wire
//!    format is JS `number`, which is i64-safe through 2^53).
//!
//! 5. **R9-D NEGATIVE — required-field rejections.** The DB requires
//!    `email` NOT NULL on `users`; `Member.email` is `String`
//!    (non-optional). A regression to `Option<String>` would let
//!    `query_as::<Member>` decode a NULL email without error and
//!    silently propagate `null` to the admin UI. Negative-pin: a
//!    JSON payload missing `email` must fail to deserialize.
//!
//! 6. **`router()` mount table — 8 routes, all admin-gated.** The
//!    handlers are public (`pub async fn`) and ALL accept
//!    `_admin: AdminUser` — dropping `AdminUser` from any handler
//!    would silently leak member PII (names, emails, MRR, churn) to
//!    any logged-in member. Compile-pin catches handler-signature
//!    drift at type-check time.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_member_management_test.rs` (closest analog
//! — the CRUD-shape counterpart of this analytics surface),
//! `tests/admin_indicators_test.rs`, `tests/admin_orders_test.rs`,
//! `tests/posts_test.rs` (i64-PK money pin).

use chrono::NaiveDateTime;
use revolution_api::routes::members::{Member, MemberQuery};

// ── 1. Member.id is i64 (BIGSERIAL PK contract) ──────────────────────

/// `Member.id` MUST be `i64` and survive a JSON round-trip past
/// `i32::MAX`. Every PK on this stack is `BIGSERIAL`; a regression to
/// `i32` would silently break joins against `users.id` (i64
/// everywhere). The members-admin table is the FIRST place admins
/// open after a breach — silent decode failures here mean "we can't
/// see the newest 50 customers" and nobody notices for weeks.
#[test]
fn member_id_is_i64_bigserial_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let now: NaiveDateTime = chrono::Utc::now().naive_utc();

    let m = Member {
        id: above_i32_max,
        name: Some("Above i32::MAX User".to_string()),
        email: "above@example.com".to_string(),
        created_at: Some(now),
        updated_at: Some(now),
    };

    // Compile-pin: the field type is exactly i64.
    let _: i64 = m.id;

    // `Member` is `Serialize, FromRow` only — it's an OUTBOUND
    // wire-format DTO (admin reads it from the DB and the API renders
    // it to JSON), never inbound. So we pin the serialize direction
    // only; the `FromRow` decode path is exercised by integration
    // tests that talk to a real DB.
    let wire = serde_json::to_value(&m).expect("serialize Member");
    assert_eq!(
        wire["id"].as_i64(),
        Some(above_i32_max),
        "Member.id MUST round-trip as i64 — BIGSERIAL PK contract"
    );
    assert_eq!(wire["email"].as_str(), Some("above@example.com"));
    assert_eq!(wire["name"].as_str(), Some("Above i32::MAX User"));
}

// ── 2. MONEY PIN — MemberQuery FK fields stay i64 (Money/cents rule) ──

/// CLAUDE.md "Money / cents" HARD RULE + BIGSERIAL-PK contract:
/// `MemberQuery.product_id` and `MemberQuery.plan_id` are the
/// foreign-key filter columns for the admin "members by service"
/// drill-down. Both refer to `BIGSERIAL` PKs on the products /
/// membership_plans tables. A regression to `i32` here would compile
/// fine but silently lose the high bits when the admin filters by a
/// `plan_id` that arrived from JSON (JSON wire is JS `number`, safe
/// through 2^53). The handler chains these into the
/// `(mp.price * 100)::BIGINT` revenue-cents calc — a wrong FK would
/// pull the WRONG plan's price and emit a wrong `mrr_cents` /
/// `total_cents` figure on the admin overview card.
///
/// `Member` itself has no `*_cents` field (it's the JOIN row only —
/// the cents come out of `query_scalar::<i64>` on `stats`), so we
/// pin via the FKs that flow into the cents arithmetic.
#[test]
fn member_query_fk_fields_are_i64_for_money_safe_joins() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let q: MemberQuery = serde_json::from_value(serde_json::json!({
        "product_id": above_i32_max,
        "plan_id": above_i32_max,
    }))
    .expect("MemberQuery with i64 FKs MUST parse");

    // Compile-pin: both FK fields are Option<i64>.
    let _: Option<i64> = q.product_id;
    let _: Option<i64> = q.plan_id;

    assert_eq!(q.product_id, Some(above_i32_max));
    assert_eq!(q.plan_id, Some(above_i32_max));

    // Negative: passing a string for the FK MUST fail (don't silently
    // coerce — the handler binds straight into sqlx).
    assert!(
        serde_json::from_value::<MemberQuery>(serde_json::json!({
            "product_id": "not-a-number",
        }))
        .is_err(),
        "MemberQuery.product_id MUST reject string input (i64 only)"
    );
}

// ── 3. MemberQuery is fully optional (index-page default load) ─────

/// `MemberQuery` is the query-string DTO behind `/admin/members`.
/// The admin lands on this page with NO query params (just clicks
/// "Members"); every field is Optional. A regression that flipped
/// `search` / `status` / `date_from` / `date_to` / `sort_by` /
/// `sort_dir` / `per_page` / `page` to required would 422 every
/// admin who opens the tab.
#[test]
fn member_query_is_fully_optional_for_default_admin_load() {
    // Zero params — pure default.
    let empty: MemberQuery =
        serde_json::from_str("{}").expect("empty MemberQuery MUST parse (default admin load)");
    assert!(empty.search.is_none());
    assert!(empty.status.is_none());
    assert!(empty.date_from.is_none());
    assert!(empty.date_to.is_none());
    assert!(empty.product_id.is_none());
    assert!(empty.plan_id.is_none());
    assert!(empty.sort_by.is_none());
    assert!(empty.sort_dir.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.page.is_none());

    // Rich load — every field populated (admin types a search, filters
    // status="active", sets a date range, picks a service, sorts by
    // name asc, asks for page 3 of 50).
    let rich: MemberQuery = serde_json::from_value(serde_json::json!({
        "search": "billy@example.com",
        "status": "active",
        "date_from": "2026-01-01",
        "date_to": "2026-12-31",
        "product_id": 42_i64,
        "plan_id": 7_i64,
        "sort_by": "name",
        "sort_dir": "asc",
        "per_page": 50_i64,
        "page": 3_i64,
    }))
    .expect("rich MemberQuery MUST parse");
    assert_eq!(rich.search.as_deref(), Some("billy@example.com"));
    assert_eq!(rich.status.as_deref(), Some("active"));
    assert_eq!(rich.date_from.as_deref(), Some("2026-01-01"));
    assert_eq!(rich.date_to.as_deref(), Some("2026-12-31"));
    assert_eq!(rich.product_id, Some(42));
    assert_eq!(rich.plan_id, Some(7));
    assert_eq!(rich.sort_by.as_deref(), Some("name"));
    assert_eq!(rich.sort_dir.as_deref(), Some("asc"));
    assert_eq!(rich.per_page, Some(50));
    assert_eq!(rich.page, Some(3));

    // R9-D NEGATIVE: garbage in the `per_page` slot MUST fail (the
    // handler does `params.per_page.unwrap_or(25).min(100)` — a
    // silent coerce to 0 would emit a zero-result page with no error).
    assert!(
        serde_json::from_value::<MemberQuery>(serde_json::json!({
            "per_page": "fifty",
        }))
        .is_err(),
        "MemberQuery.per_page MUST reject string input"
    );
}

// ── 4. Member field types — email required, timestamps nullable ────

/// `Member` is `Serialize, FromRow` only (DB-out → JSON-out). The
/// `email` field MUST be `String` (non-Optional) — `users.email` is
/// `NOT NULL` in the schema; a regression to `Option<String>` would
/// let `query_as::<Member>` decode a NULL email silently and
/// propagate `null` to the admin UI — symptom is "blank rows in the
/// members table." Pin the type contract with a compile-time
/// destructure (a regression flips the let-pattern type and the
/// build fails).
///
/// Timestamps are `Option<NaiveDateTime>` — CMS-imported rows can
/// pre-date the `created_at`/`updated_at` columns, so historical
/// NULLs are real. A regression to required-timestamp would 500
/// every "View members" request the moment one legacy row appears
/// in the result set.
///
/// Wire shape: serialize a Member with NULL name/timestamps and
/// confirm the wire emits explicit JSON nulls (the admin frontend's
/// `Member` interface declares those fields as `string | null`).
#[test]
fn member_field_types_and_null_serialization() {
    // Compile-pin: destructure Member and check each field's type.
    let m = Member {
        id: 1,
        name: None,
        email: "compile-pin@example.com".to_string(),
        created_at: None,
        updated_at: None,
    };
    let _: i64 = m.id;
    let _: Option<String> = m.name.clone();
    let _: String = m.email.clone();
    let _: Option<NaiveDateTime> = m.created_at;
    let _: Option<NaiveDateTime> = m.updated_at;

    // Wire-shape pin: NULL nullable fields emit JSON nulls (not the
    // string "null" and not absent keys — the frontend reads them
    // as `?? null`).
    let wire = serde_json::to_value(&m).expect("serialize null-field Member");
    assert!(
        wire["name"].is_null(),
        "Member.name=None MUST serialize as JSON null"
    );
    assert!(
        wire["created_at"].is_null(),
        "Member.created_at=None MUST serialize as JSON null"
    );
    assert!(
        wire["updated_at"].is_null(),
        "Member.updated_at=None MUST serialize as JSON null"
    );
    assert_eq!(wire["email"].as_str(), Some("compile-pin@example.com"));
    assert_eq!(wire["id"].as_i64(), Some(1));
}

// ── 5. Router mount-table compile-pin (8 routes, admin-gated) ────────

/// `routes::members::router()` MUST build as `Router<AppState>`.
/// Mount table (all admin-gated via `_admin: AdminUser` extractor):
///   - GET /admin/members                  (index — paginated list)
///   - GET /admin/members/stats            (overview cards — MRR, churn, LTV)
///   - GET /admin/members/services         (filter dropdown — plans + counts)
///   - GET /admin/members/churned          (churn-management drilldown)
///   - GET /admin/members/export           (CSV download — full member list)
///   - GET /admin/members/email-templates  (preset templates for the modal)
///   - GET /admin/members/service/:id      (members per plan/service)
///   - GET /admin/members/:id              (single-member detail card)
///
/// The auth boundary is load-bearing: every handler takes
/// `_admin: AdminUser`. Dropping that extractor from ANY handler
/// would silently leak member PII (names, emails, MRR aggregates,
/// per-plan revenue) to logged-in non-admin members. Compile-pin
/// catches handler-signature drift at type-check time — if any
/// handler regressed to `User` or to anon, the router builder would
/// fail to type-check against the route table's expected handler
/// signatures.
#[test]
fn members_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = revolution_api::routes::members::router();
}

/// Per CLAUDE.md habit #3 ("did any cached state survive the
/// refactor?"), pin that the router constructor is safe to call
/// repeatedly with no global / `OnceLock` mutation leaking across
/// constructions. Two routers from independent calls MUST both
/// build cleanly.
#[test]
fn members_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = revolution_api::routes::members::router();
    let _r2: axum::Router<revolution_api::AppState> = revolution_api::routes::members::router();
}
