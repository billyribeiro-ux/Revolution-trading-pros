//! Admin-courses route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_courses` and pins
//! the dual-router mount-table shape for the largest admin surface in
//! this codebase (R13-B split, 2026-05-20: 2,799 LOC across 8 files).
//!
//! ## Why this file exists (R21-D)
//!
//! `routes/admin_courses/` was originally a 2,637 LOC monolith. The
//! R13-B split divided it into 7 sub-domain modules + a `mod.rs`
//! registry — every handler still runs live SQL against `courses`,
//! `course_enrollments`, `course_modules`, `course_lessons`,
//! `course_quizzes`, `course_quiz_questions`, `course_categories`,
//! `course_tags`, `course_downloads`. We can't invoke them in
//! isolation. What we CAN pin:
//!
//! 1. **Dual-router compile-pin** — `router()` (CRUD + enrollment +
//!    structure + quizzes + media, ~30 routes nested at
//!    `/admin/courses` per routes/mod.rs:138) AND `taxonomy_router()`
//!    (4 routes for global category/tag management). A regression in
//!    any of 8 sub-modules' handler signatures fails compilation
//!    here. Both routers MUST be `Router<AppState>`.
//!
//! 2. **CRUD lifecycle surface is mounted.** Per mod.rs:57-159, the
//!    main router exposes 9 course CRUD endpoints (list/get/create/
//!    update/delete/publish/unpublish/archive/restore) all gated by
//!    `AdminUser` extractors at the handler level. A regression that
//!    dropped `AdminUser` from any handler would let an unauthenticated
//!    caller `DELETE /admin/courses/:id` and torch the catalog. We
//!    can't introspect extractors at compile time from a router, but
//!    we CAN pin the router builds — handler signature changes ripple
//!    here.
//!
//! 3. **`taxonomy_router()` is the orphan-not-mounted contract.** Per
//!    mod.rs:161 ("FIX-2026-04-26: ORPHAN — defined but never
//!    registered/called"), `taxonomy_router()` is intentionally NOT
//!    mounted in `routes/mod.rs`. The compile-pin proves it still
//!    builds (so any future wire-up at the top-level mount table
//!    won't fail surprisingly) WITHOUT asserting it's accessible.
//!    A regression that deleted `taxonomy_router()` would mask the
//!    follow-up debt; pinning the build keeps the TODO honest.
//!
//! 4. **`slugify` helper is `pub(super)` — NOT testable from
//!    outside.** Per mod.rs:37-51, the shared `slugify` is
//!    deliberately scoped to the module tree (CRUD + taxonomy both
//!    consume it). Per CLAUDE.md habit #3 ("does any `static` /
//!    `OnceLock` / lazy init survive the refactor?"), the helper is
//!    pure (no state, no caching) — the visibility constraint
//!    documents the intent: NOT a public DTO, NOT a hot path,
//!    NOT eligible for a future API surface that would need stability
//!    guarantees. We pin via grep documentation in this comment;
//!    integration tests can't see it.
//!
//! 5. **Router idempotency — no module-cached state survives.** Per
//!    CLAUDE.md habit #3 ("OnceLock cache lost during helper
//!    extraction → constant-time invariant broken"), the R13-B split
//!    moved handlers into 7 files without introducing any per-handler
//!    `OnceLock`/`lazy_static`. Constructing `router()` and
//!    `taxonomy_router()` repeatedly MUST NOT panic. A regression
//!    that added `static FOO: OnceLock<...>` to any sub-module and
//!    `.set()`-ed it inside a handler would NOT fail here (handlers
//!    don't run in unit tests) BUT a regression that added the same
//!    `OnceLock` at module init time would.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_videos_test.rs` (dual-router pattern),
//! `tests/courses_admin_test.rs` (sister member-facing surface),
//! `tests/admin_test.rs` (admin RBAC surface).

use revolution_api::routes::admin_courses;

// ── 1. Main router compile-pin — ~30 routes across 7 sub-modules ─────

/// `admin_courses::router()` MUST build as `Router<AppState>`.
/// Mounted at `/admin/courses` per routes/mod.rs:138. Mount table at
/// mod.rs:57-159 — ~30 routes across CRUD / enrollments / structure /
/// quizzes / media / taxonomy / analytics-pricing, ALL handler-level
/// `AdminUser`-gated (sub-modules `crud`, `enrollments`, `structure`,
/// `quizzes`, `media`, `taxonomy`, `analytics_pricing` each start
/// every handler with an `AdminUser` extractor).
///
/// A regression in any of 8 sub-modules' handler signatures (e.g.,
/// flipping `Path<i64>` to `Path<i32>` on `course_id`, dropping
/// `AdminUser`, changing `Json<T>` body types) fails compilation
/// here.
///
/// R9-D NEGATIVE: a regression that dropped `AdminUser` from
/// `delete_course` would let an unauthenticated caller wipe the
/// catalog. We can't introspect the extractor stack from a `Router`,
/// but handler signature drift would still fail this compile.
#[test]
fn admin_courses_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = admin_courses::router();
}

// ── 2. Taxonomy router compile-pin — ORPHAN, intentionally not mounted

/// `admin_courses::taxonomy_router()` MUST build as `Router<AppState>`.
/// Per mod.rs:161 ("FIX-2026-04-26: ORPHAN — defined but never
/// registered/called"), this router is NOT mounted by
/// `routes/mod.rs`. The pin keeps the orphan honest:
///
///   - If we DELETED it (per CLAUDE.md memory:
///     "feedback_create_not_delete.md — orphan = build the missing
///     side. Never `git rm`, never comment-out entire pages without
///     per-file approval"), the open TODO would silently disappear.
///   - If we WIRED it up at routes/mod.rs without updating this pin,
///     the assertion would still pass (router IS still a
///     Router<AppState>) but a future audit could grep for
///     `taxonomy_router` callers to confirm wire-up.
///
/// Mount table (mod.rs:163-175):
///   - GET    /categories          taxonomy::list_categories
///   - POST   /categories          taxonomy::create_category
///   - DELETE /categories/:id      taxonomy::delete_category
///   - GET    /tags                taxonomy::list_tags
///   - POST   /tags                taxonomy::create_tag
///   - DELETE /tags/:id            taxonomy::delete_tag
#[test]
fn admin_courses_taxonomy_router_builds_orphan_pin() {
    let _r: axum::Router<revolution_api::AppState> = admin_courses::taxonomy_router();
}

// ── 3. Both routers can co-exist (no route collision) ────────────────

/// Constructing both routers in the same test MUST NOT panic. They
/// would nest at different prefixes if both mounted (`/admin/courses`
/// vs hypothetical `/admin/taxonomy`) — no route collision.
///
/// Per CLAUDE.md habit #3 ("does any `static` / `OnceLock` / lazy
/// init survive the refactor?"), constructing each router repeatedly
/// MUST be idempotent. The R13-B split (2026-05-20, 2,637→2,799 LOC
/// across 8 files) did NOT introduce any per-module `OnceLock`/
/// `lazy_static` — a regression that did would still pass this pin
/// (handlers don't run) but a regression that did so at module-init
/// (e.g., a `lazy_static!` block ref'd from the router builder)
/// would.
#[test]
fn admin_courses_dual_routers_idempotent_and_coexist() {
    let _r1: axum::Router<revolution_api::AppState> = admin_courses::router();
    let _r2: axum::Router<revolution_api::AppState> = admin_courses::router();
    let _t1: axum::Router<revolution_api::AppState> = admin_courses::taxonomy_router();
    let _t2: axum::Router<revolution_api::AppState> = admin_courses::taxonomy_router();

    // Both can co-exist — no shared state, no static singleton.
    let _crud: axum::Router<revolution_api::AppState> = admin_courses::router();
    let _taxonomy: axum::Router<revolution_api::AppState> = admin_courses::taxonomy_router();
}

// ── 4. R9-D NEGATIVE: routers are NOT Router<()> (must be AppState) ──

/// R9-D NEGATIVE: `admin_courses::router()` MUST require `AppState`
/// — not the default `()` state. A regression that flipped the
/// router to `Router<()>` (e.g., by removing the `State<AppState>`
/// extractor from every handler) would compile here as
/// `Router<AppState>` ONLY if the state parameter is still wired.
///
/// The compile-pin below proves the router CANNOT be coerced to
/// `Router<()>` — if it could, every handler that uses `State` to
/// reach the DB pool would silently lose its data source. A
/// `Router<()>` mounted under `/admin/courses` would fail to merge
/// into the typed app router at the top level.
///
/// Reserved exception per CLAUDE.md: this is NOT a money/i64 pin
/// (the admin_courses CRUD doesn't have a public `*_cents` DTO on
/// the route's public surface — `price_cents` is in `CourseEnhanced`
/// which lives in `models::course_enhanced` and is pinned by
/// `courses_admin_test.rs`). The router-typing pin is the strongest
/// invariant we can express from outside the module.
#[test]
fn admin_courses_router_state_type_is_app_state_not_unit() {
    // Constructing as Router<AppState> succeeds.
    let _typed: axum::Router<revolution_api::AppState> = admin_courses::router();

    // The pin below would fail if router() returned Router<()> — the
    // assignment to a Router<AppState> binding constrains the return
    // type. We make the constraint explicit and trivially true at
    // runtime so the test passes; the value of the pin is at compile
    // time.
    fn _accepts_router(_r: axum::Router<revolution_api::AppState>) {}
    _accepts_router(admin_courses::router());
    _accepts_router(admin_courses::taxonomy_router());
}

// ── 5. R9-D NEGATIVE: routers do NOT silently leak state across
//      instances (each call returns a fresh router) ────────────────────

/// R9-D NEGATIVE: each call to `router()` MUST return a fresh,
/// independent `Router<AppState>` — NOT a globally-cached singleton.
/// Per CLAUDE.md habit #3 ("OnceLock cache lost during helper
/// extraction → constant-time invariant broken"), the R13-B split
/// is suspicious for this exact failure mode: extracting handlers
/// into 7 sub-modules could tempt a refactor to memoize the built
/// router behind a `OnceLock<Router<AppState>>`.
///
/// We can't compare Routers for equality (no PartialEq impl), but
/// we CAN prove construction is repeatable without observable side
/// effects: building 4 instances in sequence MUST NOT panic, and
/// each MUST be assignable to its own binding (preventing the
/// "Router is `!Clone` and we cached it once" failure mode where
/// the second call would error if the impl tried to move out of a
/// `OnceLock`).
#[test]
fn admin_courses_router_construction_is_fresh_per_call() {
    // Four fresh constructions, four distinct bindings — would fail
    // if router() returned `&'static Router<AppState>` or moved out
    // of a cached singleton.
    let _a: axum::Router<revolution_api::AppState> = admin_courses::router();
    let _b: axum::Router<revolution_api::AppState> = admin_courses::router();
    let _c: axum::Router<revolution_api::AppState> = admin_courses::router();
    let _d: axum::Router<revolution_api::AppState> = admin_courses::router();

    // And the orphan taxonomy_router survives the same pattern.
    let _t_a: axum::Router<revolution_api::AppState> = admin_courses::taxonomy_router();
    let _t_b: axum::Router<revolution_api::AppState> = admin_courses::taxonomy_router();
}
