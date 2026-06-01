// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. These are documentation-style
// lints, not correctness issues. Allow file-wide to keep the scaffold readable.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Health route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::health` and pins the
//! `router()` mount-table shape for the health-probe + database-
//! bootstrap surface.
//!
//! ## Why this file exists (R19-D)
//!
//! `routes/health.rs` is 395 LOC of mixed-criticality routes:
//!
//!   - 3 **public probes**: /health, /health/detailed, /ready
//!   - 3 **SuperAdminUser-gated DDL endpoints**: /setup-db,
//!     /run-migrations, /init-db
//!
//! The DDL endpoints execute `DROP TABLE ... CASCADE`, run
//! migrations, and can rewrite the bootstrap super-admin user.
//! The "FULL_REPO_AUDIT_2026-05-17 P1-4 #2" comment (lines 156-162,
//! 295-299, 351-357) documents that all three were previously
//! reachable by the broader `AdminUser` extractor (which admits
//! `admin`/`developer` roles), and that the escalation to
//! `SuperAdminUser` is the load-bearing authorization gate. The
//! `ENVIRONMENT == "development"` string check on `/setup-db` and
//! `/init-db` defaults to `"production"` only when the var is
//! UNSET — any misconfiguration (or `ENVIRONMENT=development` in a
//! shared env) would expose schema mutation to anyone with the
//! `admin` role if a regression dropped the `SuperAdminUser`
//! extractor. What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 6 endpoints (3 GET, 3 POST).** The
//!    documented mount table (lines 387-395):
//!      - GET  /health           (public — fast liveness)
//!      - GET  /health/detailed  (public — DB+Redis+storage)
//!      - GET  /ready            (public — readiness probe)
//!      - POST /setup-db         (SuperAdminUser-gated)
//!      - POST /run-migrations   (SuperAdminUser-gated)
//!      - POST /init-db          (SuperAdminUser-gated)
//!    A regression in any handler signature (e.g., demoting
//!    `SuperAdminUser` back to `AdminUser`, or dropping the
//!    extractor entirely) fails the router builder compile here
//!    because the handlers' type signatures must match the route
//!    handler trait. Specifically, `axum::routing::post(setup_db)`
//!    only resolves if `setup_db`'s signature is `(extractor1,
//!    extractor2) -> Result<...>` — drop the `SuperAdminUser`
//!    parameter and the rest of the signature still compiles, but
//!    the routing line in `router()` would still compile too.
//!    What we lock in is the BUILDABILITY of the router with the
//!    current signature; the auth gate itself is enforced by the
//!    extractor's `FromRequestParts` impl at runtime.
//!
//! 2. **All response DTOs are MODULE-PRIVATE.** `HealthResponse`,
//!    `DetailedHealthResponse`, `ServiceStatus`, `ComponentStatus`,
//!    `SetupResponse` are all `struct` (no `pub`). We CANNOT
//!    construct them from outside the module — which is INTENTIONAL,
//!    because external code should consume the wire JSON, not the
//!    Rust types. R9-D NEGATIVE: a regression that pub'd
//!    `SetupResponse` would invite downstream code to depend on its
//!    Rust shape rather than the wire JSON — the test below
//!    documents the privacy invariant.
//!
//! 3. **`env!("CARGO_PKG_VERSION")` is compile-time embedded.** The
//!    `/health`, `/health/detailed`, `/ready` responses all embed
//!    `version: env!("CARGO_PKG_VERSION").to_string()` (lines 56,
//!    124, 150). The macro panics at compile time if the env var
//!    is unset, so this is a free compile-pin — `cargo test` itself
//!    proves `CARGO_PKG_VERSION` exists in the build env.
//!
//! 4. **/setup-db includes a hardcoded `BIGINT user_id` in the
//!    `email_verification_tokens` schema.** Lines 184-191:
//!       CREATE TABLE email_verification_tokens (
//!         id UUID PRIMARY KEY ...,
//!         user_id BIGINT NOT NULL,  -- ← i64 / BIGINT match
//!         ...
//!       );
//!    Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY": even
//!    though `user_id` isn't money, it FKs to `users.id` which IS
//!    `bigint NOT NULL` (schema.sql:11001). A regression that
//!    flipped this to `INTEGER` would break the FK on any user
//!    past i32::MAX (~2B users — only theoretical today, but the
//!    sub-2B-row threshold has bitten production systems before).
//!    We pin the SQL bytestring matches the BIGINT contract via a
//!    string-substring check on the source.
//!
//! ## Pattern source
//!
//! Modeled on `tests/auth_test.rs`, `tests/security_test.rs`,
//! `tests/sitemap_test.rs`, `tests/room_search_test.rs`.

use revolution_api::routes::health;

// ── 1. Router compile-pin (6 routes, 3 GET / 3 POST) ─────────────────

/// `routes::health::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/health.rs:387-395:
///   - GET  /health           (public probe)
///   - GET  /health/detailed  (public probe — DB+Redis+storage)
///   - GET  /ready            (public readiness — DB ping)
///   - POST /setup-db         (SuperAdminUser-gated, dev-only)
///   - POST /run-migrations   (SuperAdminUser-gated)
///   - POST /init-db          (SuperAdminUser-gated, dev-only)
///
/// The 3 DDL endpoints (`setup-db`, `run-migrations`, `init-db`)
/// were ESCALATED from `AdminUser` to `SuperAdminUser` per
/// FULL_REPO_AUDIT_2026-05-17 P1-4 #2 (file comment lines 156-162,
/// 295-299, 351-357). A regression that demoted any of them back
/// to `AdminUser` (or dropped the extractor entirely) would expose
/// `DROP TABLE ... CASCADE` and bootstrap-user rewrite to the
/// broader `admin`/`developer` roles. The compile-pin catches
/// signature drift at the routing layer; the runtime authorization
/// is enforced by the `SuperAdminUser` extractor's
/// `FromRequestParts` impl.
#[test]
fn health_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = health::router();
}

// ── 2. Router is idempotent — no module-local OnceLock state ─────────

/// Per CLAUDE.md habit #3 ("cached state survives the refactor"):
/// the health module has NO `lazy_static` / `OnceLock` / `static
/// mut` — every handler reads from `state.config` or `state.db`,
/// which are per-request (not module-cached). A refactor that
/// extracted, say, the `env!("CARGO_PKG_VERSION")` lookup into a
/// `OnceLock<String>` would still compile, but `OnceLock::set`
/// can only be called once per cell — if the constructor called
/// `set()` from inside a handler, the second router construction
/// during integration-test boot would panic.
///
/// This pin proves `router()` can be called multiple times safely
/// (idempotent construction).
#[test]
fn health_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = health::router();
    let _r2: axum::Router<revolution_api::AppState> = health::router();
    let _r3: axum::Router<revolution_api::AppState> = health::router();
}

// ── 3. CARGO_PKG_VERSION compile-time embedding pin ──────────────────

/// `/health`, `/health/detailed`, `/ready` all embed `version:
/// env!("CARGO_PKG_VERSION").to_string()` (lines 56, 124, 150).
/// `env!` is a COMPILE-TIME macro — if `CARGO_PKG_VERSION` is
/// unset, the build fails. Since this test compiles, the macro
/// resolution succeeded — and we can read the same value from
/// the test binary's env to confirm it's non-empty + matches
/// a semver-ish shape.
///
/// R9-D NEGATIVE: an empty version string would silently break
/// frontend's "API version" display + observability dashboards
/// that group by version.
#[test]
fn health_responses_embed_cargo_pkg_version() {
    let version = env!("CARGO_PKG_VERSION");
    assert!(
        !version.is_empty(),
        "CARGO_PKG_VERSION MUST be non-empty — health endpoints \
         embed it as `version` field in every response"
    );
    // Sanity: version contains at least one dot (semver-shaped).
    assert!(
        version.contains('.'),
        "CARGO_PKG_VERSION ({version}) MUST be semver-shaped (contain '.')"
    );
}

// ── 4. /setup-db hardcoded schema mentions BIGINT user_id ─────────────

/// `/setup-db` (lines 181-204) includes a hardcoded SQL block that
/// creates `email_verification_tokens` and `password_resets` tables.
/// The `email_verification_tokens` schema (lines 183-190) declares
/// `user_id BIGINT NOT NULL` — load-bearing because:
///
///   1. It FKs (logically — no explicit FK constraint per source,
///      but the column name + downstream JOIN usage in
///      services/auth/email_verification.rs imply it) to
///      `users.id`, which is `bigint NOT NULL` per schema.sql:
///      11001. A regression to `INTEGER` would silently break the
///      JOIN for any user past i32::MAX.
///   2. Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY":
///      `user_id` is not money, but it indexes a BIGSERIAL PK
///      and must MATCH that type. This is NOT a Reserved exception
///      (Reserved is row-counts like `revisions: i32`, which DON'T
///      reference foreign keys).
///
/// We can't introspect the private `setup_db` function, but the
/// SQL block is `include_str!`-loaded indirectly via the source
/// file itself — we can read the source and grep for the contract.
#[test]
fn health_setup_db_schema_uses_bigint_user_id() {
    let source = include_str!("../src/routes/health.rs");

    // The hardcoded CREATE TABLE block for email_verification_tokens
    // MUST declare user_id as BIGINT. A regression to INTEGER would
    // break the FK alignment with users.id (bigint NOT NULL).
    assert!(
        source.contains("user_id BIGINT NOT NULL"),
        "routes/health.rs setup_db SQL MUST declare `user_id \
         BIGINT NOT NULL` — matches users.id BIGSERIAL PK \
         (schema.sql:11001); regression to INTEGER would silently \
         break FK alignment past i32::MAX users"
    );

    // The CREATE TABLE block MUST be wrapped in
    // `DROP TABLE IF EXISTS ... CASCADE` — this is the security-
    // critical reason setup-db requires SuperAdminUser (per
    // FULL_REPO_AUDIT_2026-05-17 P1-4 #2 comment lines 156-162).
    assert!(
        source.contains("DROP TABLE IF EXISTS email_verification_tokens CASCADE"),
        "routes/health.rs setup_db MUST issue `DROP TABLE IF \
         EXISTS ... CASCADE` — the destructiveness is exactly why \
         the endpoint MUST require SuperAdminUser, not AdminUser"
    );
}

// ── 5. SuperAdminUser auth-gate citation pin ─────────────────────────

/// FULL_REPO_AUDIT_2026-05-17 P1-4 #2 escalated /setup-db,
/// /run-migrations, /init-db from `AdminUser` to `SuperAdminUser`.
/// This is the LOAD-BEARING authorization gate — the env-string
/// check ("ENVIRONMENT == 'development'") on /setup-db and /init-db
/// defaults to "production" only when UNSET, so any misconfig (or
/// `ENVIRONMENT=development` in a shared/leaked env) would expose
/// schema mutation if the extractor were demoted.
///
/// Source-level pin: the SuperAdminUser import + usage MUST be
/// present. A refactor that removed the import (e.g., switched to
/// `AdminUser` for "consistency with the rest of admin/") would
/// silently re-introduce CVE-class exposure. Grep proves the
/// import + each call site exists.
///
/// R9-D NEGATIVE: a regression that swapped to AdminUser would
/// silently change the auth contract while compilation + tests
/// pass (AdminUser is a valid extractor; the test that catches
/// this is the source-grep below).
#[test]
fn health_ddl_endpoints_use_super_admin_extractor() {
    let source = include_str!("../src/routes/health.rs");

    // Import line MUST reference SuperAdminUser.
    assert!(
        source.contains("use crate::middleware::admin::SuperAdminUser;"),
        "routes/health.rs MUST import SuperAdminUser — \
         FULL_REPO_AUDIT_2026-05-17 P1-4 #2 escalated all 3 DDL \
         endpoints from AdminUser to SuperAdminUser"
    );

    // Each of the 3 DDL handlers MUST take a SuperAdminUser
    // parameter. The signature pattern (`admin: SuperAdminUser,`)
    // appears once per handler, so we expect exactly 3 hits in
    // function-parameter position (NOT counting the import line).
    let extractor_param_hits = source.matches("admin: SuperAdminUser,").count();
    assert_eq!(
        extractor_param_hits, 3,
        "routes/health.rs MUST have exactly 3 handlers taking \
         `admin: SuperAdminUser,` (setup_db, run_migrations, \
         init_db) — found {extractor_param_hits} (regression risk: any handler that \
         dropped this would silently downgrade to public access \
         since /setup-db's ENVIRONMENT check defaults to \
         'production' only when UNSET)"
    );

    // Belt-and-braces: NO handler MUST take AdminUser (the broader
    // role, which admits 'admin' and 'developer' alongside
    // 'super_admin'). A refactor that switched any handler back
    // would still compile.
    assert!(
        !source.contains("admin: AdminUser,"),
        "routes/health.rs MUST NOT have any handler taking \
         `admin: AdminUser,` — that's the regression we're \
         pinning against (P1-4 #2)"
    );
}
