//! Organization route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::organization` and
//! exercises the public DTOs (`OrganizationProfile`,
//! `UpdateOrganizationRequest`, `Team`, `CreateTeamRequest`,
//! `UpdateTeamRequest`, `Department`, `CreateDepartmentRequest`,
//! `UpdateDepartmentRequest`, `ListQuery`) + the four router
//! constructors (`teams_router`, `departments_router`, `profile_router`,
//! `admin_router`).
//!
//! ## Why this shape
//!
//! `routes/organization.rs` is the admin-only Organization Settings
//! surface — every handler requires `AdminUser` and writes to
//! `admin_audit_logs`. Every handler runs live SQL, so we can't run
//! them in isolation. What we CAN pin:
//!
//! 1. **`OrganizationProfile.id` MUST stay `i64`.** Every primary
//!    key on this stack is `BIGSERIAL`. A regression to `i32` would
//!    silently cap at 2.1B rows — irrelevant for a single-org table,
//!    but the same type contract is shared with the admin-audit /
//!    member-management surfaces, so the type pin is load-bearing for
//!    cross-table joins (audit log references org id).
//!
//! 2. **Update DTOs are all-Optional<>** (partial-update semantics).
//!    The frontend PATCH-style update sends only the changed fields;
//!    a regression that flipped any field to required would 422-error
//!    every form submit that didn't include every column.
//!
//! 3. **`CreateTeamRequest.name` is required**, `CreateDepartmentRequest.
//!    name` is required — these are the only required fields on create.
//!    Pin them: a regression that made `name` optional would let the
//!    admin create a nameless team (DB-side NOT NULL would 500, not
//!    422 — bad UX).
//!
//! 4. **`Department.parent_id: Option<i64>`** — departments form a
//!    parent-child hierarchy. A regression that flipped to `i64`
//!    (NOT NULL) would break the "root department" case (no parent).
//!
//! 5. **`ListQuery` is fully optional** — the admin grid's filter bar
//!    POSTs an empty query for "show everything". Pin the no-filter
//!    case.
//!
//! 6. **Four routers compile** — `admin_router()` nests `teams_router()`
//!    + `departments_router()` at `/teams` + `/departments` and mounts
//!    `/profile` directly. A regression that broke any handler
//!    signature would fail compilation here.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_orders_test.rs`, `tests/products_test.rs`,
//! `tests/payments_test.rs`, `tests/admin_member_management_test.rs`.

use revolution_api::routes::organization::{
    CreateDepartmentRequest, CreateTeamRequest, Department, ListQuery, OrganizationProfile, Team,
    UpdateDepartmentRequest, UpdateOrganizationRequest, UpdateTeamRequest,
};

// ── 1. ID pin: every primary key on this stack is i64 ────────────────

/// HARD RULE (matches the rest of this stack): every primary key on
/// every domain table is `BIGSERIAL` / `i64`. `OrganizationProfile.id`,
/// `Team.id`, `Team.member_count`, `Department.id`, `Department.parent_id`
/// all live on tables joined into admin-audit logs. A regression to
/// `i32` would silently truncate during cross-table joins and corrupt
/// the audit trail.
///
/// `member_count` is `Option<i32>` (legitimate exception per CLAUDE.md
/// — row counts cap below 2B), so pin THAT explicitly as i32, but
/// every other id is i64.
#[test]
fn organization_id_types_match_bigserial_contract() {
    let now = chrono::Utc::now();

    // OrganizationProfile.id is i64 — fixture that exceeds i32::MAX
    // would compile-fail under a narrowing regression.
    let above_i32_max: i64 = (i32::MAX as i64) + 7;
    let profile = OrganizationProfile {
        id: above_i32_max,
        name: "Revolution Trading Pros".to_string(),
        slug: "revolution-trading-pros".to_string(),
        description: None,
        logo_url: None,
        favicon_url: None,
        primary_color: None,
        secondary_color: None,
        contact_email: None,
        contact_phone: None,
        address: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        website_url: None,
        social_links: None,
        business_hours: None,
        timezone: None,
        currency: None,
        tax_id: None,
        created_at: Some(now),
        updated_at: Some(now),
    };
    let wire = serde_json::to_value(&profile).expect("serialize profile");
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert!(
        profile.id > i32::MAX as i64,
        "OrganizationProfile.id fixture must exceed i32::MAX or it doesn't prove the i64 pin"
    );

    // Department.id + Department.parent_id both i64 (parent_id Optional
    // so root departments can have None — pin that too).
    let dept = Department {
        id: above_i32_max,
        name: "Engineering".to_string(),
        slug: "engineering".to_string(),
        description: None,
        color: None,
        icon: None,
        parent_id: Some(above_i32_max - 1),
        is_active: true,
        member_count: Some(42_i32), // i32 here is the legitimate row-count exception
        created_at: None,
        updated_at: None,
    };
    let dw = serde_json::to_value(&dept).expect("serialize dept");
    assert_eq!(dw["id"].as_i64(), Some(above_i32_max));
    assert_eq!(dw["parent_id"].as_i64(), Some(above_i32_max - 1));
    assert_eq!(dw["member_count"].as_i64(), Some(42));

    // Root department: parent_id = None must serialize as null
    // (frontend uses `.parent_id ?? null` to decide whether to render
    // the breadcrumb).
    let root = Department {
        id: 1,
        name: "Root".to_string(),
        slug: "root".to_string(),
        description: None,
        color: None,
        icon: None,
        parent_id: None,
        is_active: true,
        member_count: None,
        created_at: None,
        updated_at: None,
    };
    let rw = serde_json::to_value(&root).expect("serialize root");
    assert!(
        rw["parent_id"].is_null(),
        "root department must serialize parent_id as null"
    );
}

// ── 2. Update DTOs: partial-update semantics, all fields Optional ────

/// The frontend's settings form does a single PATCH-style PUT with only
/// the fields the admin actually changed. Every field on every Update*
/// DTO MUST be `Option<>` for this to work. A regression that flipped
/// even ONE field to required would 422-error every partial save.
///
/// Empty-body PUT is the load-bearing case: it means "no-op, but
/// touch updated_at and write an audit log" — used when the admin
/// clicks Save without changing anything (to bump the audit trail).
#[test]
fn update_dtos_accept_empty_body_for_partial_updates() {
    // UpdateOrganizationRequest — 19 fields, all Optional
    let empty_org: UpdateOrganizationRequest =
        serde_json::from_str("{}").expect("empty org update must deserialize");
    assert!(empty_org.name.is_none());
    assert!(empty_org.description.is_none());
    assert!(empty_org.contact_email.is_none());
    assert!(empty_org.primary_color.is_none());
    assert!(empty_org.tax_id.is_none());

    // Partial update — only one field. The "rename the org" flow.
    let rename: UpdateOrganizationRequest =
        serde_json::from_value(serde_json::json!({ "name": "RTP Holdings" }))
            .expect("partial update must deserialize");
    assert_eq!(rename.name.as_deref(), Some("RTP Holdings"));
    assert!(
        rename.description.is_none(),
        "unchanged fields must stay None — partial update is the WHOLE point"
    );

    // UpdateTeamRequest — partial-update semantics
    let empty_team: UpdateTeamRequest =
        serde_json::from_str("{}").expect("empty team update must deserialize");
    assert!(empty_team.name.is_none());
    assert!(empty_team.is_active.is_none());

    // UpdateDepartmentRequest — partial-update + nullable parent_id
    let empty_dept: UpdateDepartmentRequest =
        serde_json::from_str("{}").expect("empty dept update must deserialize");
    assert!(empty_dept.name.is_none());
    assert!(empty_dept.parent_id.is_none());

    // Reparent flow — only change parent_id (root → under another dept)
    let reparent: UpdateDepartmentRequest =
        serde_json::from_value(serde_json::json!({ "parent_id": 42 }))
            .expect("reparent must deserialize");
    assert_eq!(reparent.parent_id, Some(42));
    assert!(reparent.name.is_none());
}

// ── 3. Create DTOs: name is required, everything else Optional ───────

/// `CreateTeamRequest.name` and `CreateDepartmentRequest.name` are the
/// only required fields. DB has a NOT NULL on the column, so a
/// regression that made `name` `Option<>` would let the admin form
/// POST without a name, then 500 at the DB layer — worse UX than the
/// current 422-from-serde flow. Pin both the required `name` and the
/// optional everything-else.
#[test]
fn create_dtos_require_name_only() {
    // CreateTeamRequest — name required, rest optional
    let minimal: CreateTeamRequest = serde_json::from_value(serde_json::json!({
        "name": "Backend Team",
    }))
    .expect("name-only team create must deserialize");
    assert_eq!(minimal.name, "Backend Team");
    assert!(minimal.description.is_none());
    assert!(minimal.color.is_none());
    assert!(minimal.icon.is_none());

    // Missing name must FAIL — pin the required-ness.
    let no_name = serde_json::from_value::<CreateTeamRequest>(serde_json::json!({
        "description": "no name here",
    }));
    assert!(
        no_name.is_err(),
        "CreateTeamRequest without name must fail deserialization — DB has NOT NULL"
    );

    // Full payload
    let full: CreateTeamRequest = serde_json::from_value(serde_json::json!({
        "name": "Trading Floor",
        "description": "Day-trading mentors",
        "color": "#3b82f6",
        "icon": "chart-line",
    }))
    .expect("full team create must deserialize");
    assert_eq!(full.name, "Trading Floor");
    assert_eq!(full.color.as_deref(), Some("#3b82f6"));

    // CreateDepartmentRequest — same shape, plus optional parent_id
    let minimal_dept: CreateDepartmentRequest = serde_json::from_value(serde_json::json!({
        "name": "Operations",
    }))
    .expect("name-only dept create must deserialize");
    assert_eq!(minimal_dept.name, "Operations");
    assert!(minimal_dept.parent_id.is_none());

    let with_parent: CreateDepartmentRequest = serde_json::from_value(serde_json::json!({
        "name": "QA",
        "parent_id": 5,
    }))
    .expect("dept with parent must deserialize");
    assert_eq!(with_parent.parent_id, Some(5));

    // Missing name must FAIL for departments too.
    let no_name_dept = serde_json::from_value::<CreateDepartmentRequest>(serde_json::json!({
        "parent_id": 1,
    }));
    assert!(
        no_name_dept.is_err(),
        "CreateDepartmentRequest without name must fail deserialization"
    );
}

// ── 4. ListQuery: fully optional, supports the "show all" default ────

/// The admin grid's filter bar GETs `/teams` with an empty query string
/// when no filters are applied. `ListQuery` MUST deserialize from `{}`
/// to mean "show all"; a regression that flipped either field to
/// required would break the default "no filter" flow.
#[test]
fn list_query_accepts_empty_and_full_filters() {
    let empty: ListQuery = serde_json::from_str("{}").expect("empty filter must deserialize");
    assert!(empty.search.is_none());
    assert!(empty.active_only.is_none());

    let with_search: ListQuery = serde_json::from_value(serde_json::json!({ "search": "trading" }))
        .expect("search filter must deserialize");
    assert_eq!(with_search.search.as_deref(), Some("trading"));
    assert!(with_search.active_only.is_none());

    let active_only: ListQuery = serde_json::from_value(serde_json::json!({ "active_only": true }))
        .expect("active_only filter must deserialize");
    assert_eq!(active_only.active_only, Some(true));

    let combined: ListQuery = serde_json::from_value(serde_json::json!({
        "search": "QA",
        "active_only": false,
    }))
    .expect("combined filter must deserialize");
    assert_eq!(combined.search.as_deref(), Some("QA"));
    assert_eq!(combined.active_only, Some(false));
}

// ── 5. Team wire format: member_count is i32 (legitimate exception) ──

/// `Team.member_count: Option<i32>` is the only legitimate `i32` field
/// in this module — it's a row count, not money, and per CLAUDE.md
/// row counts cap below 2B safely. Pin the i32 (NOT i64) shape so a
/// well-intentioned "make everything i64" refactor doesn't widen this
/// to `Option<i64>` (which would break the frontend's `number` type
/// alignment).
#[test]
fn team_member_count_is_i32_per_row_count_exception() {
    let team = Team {
        id: 1,
        name: "Day Traders".to_string(),
        slug: "day-traders".to_string(),
        description: None,
        color: None,
        icon: None,
        is_active: true,
        member_count: Some(127_i32),
        created_at: None,
        updated_at: None,
    };
    let wire = serde_json::to_value(&team).expect("serialize team");
    assert_eq!(wire["member_count"], 127);
    assert_eq!(wire["id"].as_i64(), Some(1));
    assert_eq!(wire["is_active"], true);
    assert!(wire.get("description").is_none() || wire["description"].is_null());

    // Round-trip: serialize then deserialize back. Catches any
    // accidental rename that the frontend would see as a different key.
    let restored: Team = serde_json::from_value(wire).expect("Team must round-trip");
    assert_eq!(restored.member_count, Some(127));
    assert_eq!(restored.slug, "day-traders");
}

// ── 6. Router mount table compile-pin (4 routers) ────────────────────

/// All four router constructors must build as `Router<AppState>`.
/// `admin_router()` is the canonical mount-point that nests the other
/// three; a regression that broke ANY handler signature (drop
/// `AdminUser`, wrong extractor) would fail compilation here.
///
/// `profile_router()` is explicitly an ORPHAN per the source comment
/// ("defined but never registered") — we still compile-pin it so that
/// when someone wires it up (or deletes it), the change is intentional,
/// not an accidental break.
#[test]
fn organization_all_routers_build_with_app_state() {
    let _teams: axum::Router<revolution_api::AppState> =
        revolution_api::routes::organization::teams_router();
    let _depts: axum::Router<revolution_api::AppState> =
        revolution_api::routes::organization::departments_router();
    let _profile: axum::Router<revolution_api::AppState> =
        revolution_api::routes::organization::profile_router();
    let _admin: axum::Router<revolution_api::AppState> =
        revolution_api::routes::organization::admin_router();
}

/// Idempotent construction — `admin_router()` must be safe to build
/// multiple times. A regression that introduced a global `static`
/// mutable inside the constructor would fail here. Particularly
/// relevant for `admin_router` since it `.nest()`s two child routers;
/// a stateful nest implementation would fail on second invocation.
#[test]
fn organization_admin_router_is_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> =
        revolution_api::routes::organization::admin_router();
    let _r2: axum::Router<revolution_api::AppState> =
        revolution_api::routes::organization::admin_router();
}
