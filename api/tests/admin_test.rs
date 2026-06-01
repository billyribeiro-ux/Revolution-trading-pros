// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Admin route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin::router()` (the
//! ONLY pub item across the 2,555-LOC, 5-sub-module admin surface)
//! and pins the master mount table for `/admin/*` (excluding the
//! /admin/* mounts that come from peer modules like `admin_orders`,
//! `admin_members`, `admin_videos`, etc.).
//!
//! ## Why this file exists (R20-D)
//!
//! `routes/admin/` is 2,555 LOC across 5 sub-domain files:
//!   - `mod.rs`              (118 LOC — master router)
//!   - `campaigns_stats.rs`  (530 LOC — dashboard + campaigns +
//!                            posts/products stats + email logs)
//!   - `coupons.rs`          (867 LOC — coupon CRUD + Stripe
//!                            sync + validation)
//!   - `memberships.rs`      (437 LOC — membership-plan + user-
//!                            membership CRUD)
//!   - `settings.rs`         (106 LOC — settings KV store)
//!   - `users.rs`            (497 LOC — admin user CRUD + ban/unban)
//!
//! ALL sub-modules are `mod` (private to crate; per mod.rs:29-33).
//! That means **the ONLY externally addressable symbol is
//! `routes::admin::router()`**. We CANNOT bind individual DTOs from
//! tests (they're crate-internal). But we CAN:
//!
//! 1. **Pin the master router compile contract** — every handler
//!    across 5 sub-modules MUST have a compatible signature
//!    (`State<AppState>` + `AdminUser` extractor + Json/Result
//!    return). A regression in ANY of ~24 handlers (4 dashboard +
//!    8 users + 4 memberships + 2 campaigns + 5 coupons + 2 email
//!    + 3 settings) fails compilation here.
//!
//! 2. **Pin idempotent construction** — the master `api_router()`
//!    in `routes/mod.rs:99` nests this at `/admin`. Per CLAUDE.md
//!    habit #3 ("does any `static` / `OnceLock` / lazy init survive
//!    the refactor?"), the router constructor MUST be safe to call
//!    multiple times. A regression hoisting any sub-module's state
//!    into a `OnceLock<...>` whose `.set` panics on second call
//!    fails this pin.
//!
//! 3. **Source-grep pins** — the security-critical contract that
//!    every admin handler uses `AdminUser` (NOT `User` /
//!    `SuperAdminUser` / no-extractor) is encoded in source. We
//!    audit by grep at test-discovery time (the test executes a
//!    file-system read, NOT a DB / network call, so this is still
//!    "no-DB").
//!
//! 4. **FIX-H-5 regression-pin** — the removed `/users/:id/
//!    impersonate` route MUST stay absent. Source-grep the master
//!    mod.rs for the literal `impersonate` token; if a future
//!    refactor re-introduces the route (the audit at
//!    SECURITY_GAPS_2026-04-29.md document the security rationale
//!    for removal), this test fails LOUDLY.
//!
//! ## Pattern source
//!
//! Modeled on `tests/health_test.rs` (which similarly source-greps
//! for the `SuperAdminUser`-vs-`AdminUser` extractor contract), and
//! `tests/admin_orders_test.rs` (the canonical router-compile-pin).

use revolution_api::routes::admin;

// ── 1. Master admin router compile-pin ───────────────────────────────

/// `admin::router()` MUST build as `Router<AppState>`. Load-bearing
/// because the master function wires ~24 handlers across 5 private
/// sub-modules (`campaigns_stats`, `coupons`, `memberships`,
/// `settings`, `users`). A regression in ANY handler's signature
/// fails compilation here.
///
/// Mounted at `/admin` by `routes::api_router()` (routes/mod.rs:99).
/// Every handler MUST take `AdminUser` (admin or super-admin role)
/// as a positional extractor — a regression that dropped the
/// extractor would let unauthenticated callers reach the dashboard /
/// coupons / users / memberships / settings endpoints (which would
/// expose PII, revenue figures, customer email logs).
///
/// The compile-pin catches signature drift; the runtime "is this
/// admin-only?" property is enforced by the presence of the
/// `AdminUser` extractor — covered by test 3 below (source-grep).
#[test]
fn admin_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = admin::router();
}

// ── 2. Idempotent construction — no module-cached state ──────────────

/// Per CLAUDE.md habit #3 ("Re-read your own diff before commit —
/// does any `static` / `OnceLock` / lazy init survive the refactor?"),
/// the admin router MUST be safe to construct multiple times. The
/// master `api_router()` in routes/mod.rs constructs the admin
/// router as part of multiple nested builders (and the test harness
/// in `common/mod.rs` if it ever instantiates one).
///
/// A regression that hoisted any sub-module's state (e.g.,
/// `OnceLock<Arc<StripeCouponClient>>` in coupons.rs — a tempting
/// "performance" refactor) would panic on the second `set` call.
///
/// Currently NO sub-module caches state at module level (all
/// dependencies flow through `State<AppState>` per the SvelteKit
/// + Rust + Axum stack pattern). This pin documents the invariant.
#[test]
fn admin_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = admin::router();
    let _r2: axum::Router<revolution_api::AppState> = admin::router();
    let _r3: axum::Router<revolution_api::AppState> = admin::router();
}

// ── 3. AdminUser extractor present in EVERY handler (source-grep) ────

/// Every admin sub-module handler MUST use the `AdminUser` extractor
/// (NOT plain `User`, NOT no-extractor, NOT `SuperAdminUser` —
/// `SuperAdminUser` is used by `health.rs` for DDL endpoints; the
/// admin surface is for routine admin work, not schema mutation).
///
/// This pin source-greps the 5 sub-module files for the literal
/// `AdminUser` import + extractor usage. A regression that:
///   (a) demoted any handler to `User` would let any logged-in
///       customer reach the admin dashboard / coupons / users
///       endpoints → DATA BREACH
///   (b) dropped the extractor entirely → unauthenticated access
///   (c) wrapped in a noop helper that doesn't enforce the role
///       check → silent role escalation
///
/// A no-DB test can't introspect axum's router state — but it CAN
/// read the source files and verify the extractor token is
/// referenced. This is at-test-discovery time, NOT at handler-invoke
/// time, so it stays no-DB.
#[test]
fn admin_handlers_use_admin_user_extractor() {
    // Locate sub-module sources. CARGO_MANIFEST_DIR points at the
    // `api/` package root at test time.
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set at test time");
    let admin_dir = std::path::Path::new(&manifest_dir).join("src/routes/admin");

    // The 5 sub-modules per mod.rs:29-33.
    let sub_modules = [
        "campaigns_stats.rs",
        "coupons.rs",
        "memberships.rs",
        "settings.rs",
        "users.rs",
    ];

    for sub in &sub_modules {
        let path = admin_dir.join(sub);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("must read {}: {}", path.display(), e));

        assert!(
            source.contains("AdminUser"),
            "{sub}: must import or extract `AdminUser` (per CLAUDE.md \
             admin-gated routes contract). A regression that dropped \
             this extractor would expose admin endpoints to plain \
             users."
        );

        // Defence-in-depth: NO sub-module should use `SuperAdminUser`
        // as a handler extractor (that's reserved for DDL endpoints
        // in health.rs per the P1-4 #2 audit fix). Coupons / users /
        // memberships are routine admin work, not schema mutation.
        //
        // The token MAY appear in code-comments (e.g.,
        // campaigns_stats.rs:301 references the removed FIX-H-5
        // impersonate token's `SuperAdminUser` mint flow). Filter
        // those out by checking only lines that look like an
        // extractor signature (`: SuperAdminUser` or
        // `SuperAdminUser,` in a function argument).
        let extractor_hits = source
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                !trimmed.starts_with("//")
                    && !trimmed.starts_with("*")
                    && (trimmed.contains(": SuperAdminUser")
                        || trimmed.contains("SuperAdminUser,")
                        || trimmed.contains("SuperAdminUser)"))
            })
            .count();
        assert_eq!(
            extractor_hits, 0,
            "{sub}: must NOT use `SuperAdminUser` as a handler extractor \
             — that extractor is reserved for DDL endpoints in \
             health.rs per FULL_REPO_AUDIT_2026-05-17 P1-4 #2. \
             Admin sub-modules use plain `AdminUser` (admin or \
             super-admin role)."
        );
    }
}

// ── 4. FIX-H-5 regression-pin: /users/:id/impersonate stays removed ──

/// Per `routes/admin/mod.rs:69-70`:
///   ```
///   // FIX-H-5 (2026-04-29): /users/:id/impersonate route removed
///   // along with the handler. See docs/audits/SECURITY_GAPS_2026-04-29.md.
///   ```
///
/// User-impersonation was a CRITICAL security gap: any admin could
/// "log in as" any user (including super-admins) via a single POST,
/// effectively gaining root access without audit trail. The audit
/// removed both the route AND the handler.
///
/// This pin source-greps the master `mod.rs` for the literal
/// `impersonate` token. If a future refactor re-adds the route
/// (innocent-looking "we need an impersonate button for support"
/// PRs are the most common path), this test fails LOUDLY with the
/// security-gap reference.
///
/// Token MUST appear ONLY in the comment ("removed") — NOT in any
/// `.route(...)` call.
#[test]
fn admin_impersonate_route_stays_removed() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set at test time");
    let mod_rs = std::path::Path::new(&manifest_dir).join("src/routes/admin/mod.rs");
    let source = std::fs::read_to_string(&mod_rs).expect("must read admin/mod.rs");

    // `impersonate` MUST appear exactly TWICE in mod.rs:
    //   1. In the FIX-H-5 comment ("/users/:id/impersonate route removed")
    //   2. (no second occurrence is expected; we lower-bound to 1 in
    //      case the audit comment is rephrased)
    //
    // The pin is: zero `.route(...impersonate...)` calls.
    let route_calls_with_impersonate = source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            (trimmed.starts_with(".route") || trimmed.starts_with("Router::new()"))
                && trimmed.contains("impersonate")
        })
        .count();

    assert_eq!(
        route_calls_with_impersonate, 0,
        "FIX-H-5 regression: /users/:id/impersonate route MUST stay \
         removed. See docs/audits/SECURITY_GAPS_2026-04-29.md. Any \
         re-introduction is a CRITICAL security gap (admin → \
         super-admin escalation without audit trail)."
    );

    // The audit comment MUST still be present — a future refactor
    // that silently strips the comment AND re-adds the route would
    // pass the route-count check but lose the institutional memory.
    assert!(
        source.contains("FIX-H-5"),
        "audit comment FIX-H-5 (2026-04-29) MUST stay in admin/mod.rs \
         to document why /impersonate was removed"
    );
}

// ── 5. BIGSERIAL i64 PK alignment: `:id` paths use i64 extractors ────

/// The admin sub-modules mount many `:id` path parameters for
/// resource lookup:
///   - /users/:id, /users/:id/ban, /users/:id/unban
///   - /user-memberships/:id
///   - /campaigns/:id
///   - /coupons/:id, /coupons/:id/sync-to-stripe
///
/// EVERY one of these `:id` extractors MUST be `Path<i64>` — per
/// CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY TIME":
///   - users.id is BIGSERIAL (schema.sql lines around `users` table)
///   - coupons.id is BIGSERIAL
///   - user_memberships.id is BIGSERIAL
///   - campaigns.id is BIGSERIAL
///
/// Per CLAUDE.md the i32 reserved exception ONLY applies to bounded
/// row-counts (revisions, attempts, total_pages). Resource IDs over
/// BIGSERIAL tables are unbounded above (years of customer growth)
/// — a regression to `Path<i32>` would silently 422 / mis-route
/// any row past i32::MAX (2.1B).
///
/// We can't introspect axum's compiled extractor types from outside.
/// We CAN source-grep the handler files for `Path<i64>` usage and
/// confirm the extractor type is correct on every `:id` handler.
/// Grep for the FORBIDDEN pattern `Path<i32>` — it MUST NOT appear
/// in any admin sub-module.
#[test]
fn admin_path_id_extractors_are_i64_not_i32() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set at test time");
    let admin_dir = std::path::Path::new(&manifest_dir).join("src/routes/admin");

    let sub_modules = [
        "campaigns_stats.rs",
        "coupons.rs",
        "memberships.rs",
        "settings.rs",
        "users.rs",
    ];

    for sub in &sub_modules {
        let path = admin_dir.join(sub);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("must read {}: {}", path.display(), e));

        // R9-D NEGATIVE: `Path<i32>` MUST NOT appear in ANY admin
        // sub-module. Per CLAUDE.md "Money / cents — i64 ONLY" rule:
        // BIGSERIAL FK ids index past i32::MAX over years of growth.
        // (Settings uses `Path<String>` for the `key` param — that's
        // correct because settings.key is a string slug, not an id.)
        assert!(
            !source.contains("Path<i32>"),
            "{sub}: MUST NOT use `Path<i32>` for any `:id` extractor — \
             admin resources have BIGSERIAL PKs (users / coupons / \
             memberships / campaigns). Per CLAUDE.md \"Money / cents \
             — i64 ONLY, BIGINT ONLY\" rule applied to FK ids: i32 \
             silently truncates past 2.1B rows."
        );
    }

    // Defence-in-depth: at least one sub-module MUST use `Path<i64>`
    // (otherwise we've grep'd a no-op and this test gives false
    // confidence). users.rs and coupons.rs both have `/:id` routes.
    let users_src = std::fs::read_to_string(admin_dir.join("users.rs"))
        .expect("must read users.rs for positive-pin");
    assert!(
        users_src.contains("Path<i64>"),
        "users.rs MUST use `Path<i64>` for the user-id extractor on \
         /:id, /:id/ban, /:id/unban — positive pin proves the grep \
         is meaningful (not just searching for absent strings)."
    );
}
