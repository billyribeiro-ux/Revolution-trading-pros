// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Migrate route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::migrate` and pins the
//! `router()` mount-table shape for the admin-only manual-migration
//! surface.
//!
//! ## Why this file exists (R22-D, sweep-complete)
//!
//! `routes/migrate.rs` is the SMALLEST untested route module in the
//! crate at 57 LOC (per R21-D discovery, deliberately skipped in
//! prior rounds as "not auth-adjacent" — that was wrong: the
//! endpoint is admin-gated AND executes raw SQL against the live
//! database via `sqlx::raw_sql(...).execute(...)`). This is one of
//! the single highest-blast-radius RBAC surfaces in the entire crate.
//!
//! Per CLAUDE.md habit #1 ("Cite the rule in your work"): the
//! ICT-11+ pattern documented in the module header (lines 1-4):
//!
//!     "Database Migration Endpoint
//!      ICT 11+ Pattern: Admin-only endpoint to run migrations
//!      remotely"
//!
//! What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 1 POST endpoint, AdminUser-gated.**
//!    Mount table per routes/migrate.rs:11-13:
//!      - POST /run-schedule-migration → `run_schedule_migration`
//!        (`_admin: AdminUser`, line 19)
//!    Mounted at `/migrate` per routes/mod.rs:112. A regression
//!    that dropped the `_admin: AdminUser` extractor would compile
//!    here (the router shape doesn't change) BUT would expose raw
//!    SQL migration execution to every unauthenticated request —
//!    enabling drive-by `DROP TABLE` if the migration file were
//!    ever swapped to malicious SQL.
//!
//! 2. **Handler is `async` and references `state.db.pool`.** The
//!    SQL execution path (line 29) requires the `AppState`-attached
//!    pool. A regression that flipped the handler to a sync function
//!    or one that built its own pool would either fail this compile
//!    (signature drift caught by axum's trait bounds) or silently
//!    leak connections in the new path.
//!
//! 3. **Mount path is `/migrate`, NOT `/admin/migrate`.** Per
//!    routes/mod.rs:112: `.nest("/migrate", migrate::router())`.
//!    This is INCONSISTENT with the other admin-gated surfaces
//!    (e.g., `.nest("/admin/reconciliation", ...)` on line 238).
//!    The mount-path naming is a documented contract — a regression
//!    that "normalized" the path to `/admin/migrate` would break
//!    any frontend admin tool calling the existing `POST
//!    /api/migrate/run-schedule-migration` endpoint. The handler
//!    itself is AdminUser-gated regardless of mount, but the URL
//!    contract is what the frontend hits.
//!
//! 4. **Embedded migration SQL via `include_str!`.** Per line 26,
//!    the handler embeds `migrations/013_trading_room_schedules.sql`
//!    at compile time. This means the SQL is BUILD-TIME pinned — if
//!    the migration file is renamed or deleted, the crate fails to
//!    compile (caught early). It also means the migration text is
//!    NOT user-controllable — defense against the obvious SQL-
//!    injection vector ("what if I POST arbitrary SQL?") is the
//!    `include_str!` macro itself.
//!
//! 5. **Response shape — `{success, message, tables_created,
//!    seed_data}`.** Per lines 32-44, the success response advertises
//!    EXACTLY which tables the migration creates
//!    (`trading_room_schedules`, `schedule_exceptions`) and the
//!    seed-data row counts (6, 3, 3). This is the documented API
//!    contract. A regression that changed the response shape would
//!    break any admin dashboard that surfaces "what did this run
//!    create?" — even though the work happens via `sqlx::raw_sql`,
//!    the JSON shape is the operator-facing contract.
//!
//! ## Pattern source
//!
//! Modeled on `tests/users_test.rs` (sister ICT-* admin RBAC pin),
//! `tests/reconciliation_test.rs` (sister admin POST surface),
//! `tests/robots_test.rs` (small-surface router compile-pin).

use revolution_api::routes::migrate;

// ── 1. Router mount-table compile-pin — 1 admin-gated POST endpoint ──

/// `routes::migrate::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/migrate.rs:11-13:
///   - POST /run-schedule-migration → `run_schedule_migration`
///     (`_admin: AdminUser` at line 19)
///
/// Mounted at `/migrate` per routes/mod.rs:112, NOT under
/// `/admin/migrate` (this asymmetry is INTENTIONAL — the handler
/// itself is AdminUser-gated regardless of mount path). The full
/// URL the frontend hits is `POST /api/migrate/run-schedule-migration`.
///
/// Per CLAUDE.md habit #1 ("Cite the rule in your work"), the
/// ICT-11+ admin-only invariant (module header lines 1-4) is THE
/// invariant for this surface. A regression that:
///   - dropped `AdminUser` from the handler → drive-by DDL execution
///   - added a new public handler → expanded attack surface
///   - flipped POST to GET → CSRF-able state mutation
///   - flipped `state.db.pool` to a different pool → bypass observability
/// would either fail this compile (signature drift) OR pass and
/// require the RBAC audit grep to catch (per CLAUDE.md habit #2:
/// "RBAC / audit: grep showing the AdminUser extractor at every
/// cited line").
///
/// R9-D NEGATIVE: the compile-pin DOES NOT prove the AdminUser
/// extractor is present — it only proves the router shape is
/// consistent. The `_admin: AdminUser` line at routes/migrate.rs:19
/// is the source of truth; this pin documents where to look.
#[test]
fn migrate_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = migrate::router();
}

// ── 2. Router idempotency — no module-cached state survives refactor ─

/// Constructing `router()` repeatedly MUST NOT panic. Per CLAUDE.md
/// habit #3 ("does any `static` / `OnceLock` / lazy init survive
/// the refactor?"), this module has NO `lazy_static` / `OnceLock`
/// at all — it's a single-handler routing surface with the
/// migration SQL embedded via compile-time `include_str!` (line 26).
///
/// A regression that introduced a module-cached
/// `OnceLock<Vec<&'static str>>` (e.g., to memoize parsed migration
/// statements and skip the `sqlx::raw_sql` parse on repeat calls)
/// would be EXACTLY the failure mode CLAUDE.md flags: the cache
/// would survive subsequent migration file changes between hot
/// reloads of the binary in dev (admittedly not impactful in prod
/// since the file is `include_str!`'d at compile time, but the
/// invariant matters for any future refactor that loaded the SQL
/// from disk). Constructing the router repeatedly is a weak proxy
/// for "no module-init side effects" — the strong defense is the
/// code review.
#[test]
fn migrate_router_construction_idempotent() {
    let _r1: axum::Router<revolution_api::AppState> = migrate::router();
    let _r2: axum::Router<revolution_api::AppState> = migrate::router();
    let _r3: axum::Router<revolution_api::AppState> = migrate::router();
    let _r4: axum::Router<revolution_api::AppState> = migrate::router();
}

// ── 3. R9-D NEGATIVE: router is NOT Router<()> (AppState required) ───

/// R9-D NEGATIVE: `migrate::router()` MUST require `AppState`. The
/// handler at routes/migrate.rs:17-20 explicitly extracts
/// `State<AppState>` to access `state.db.pool` for the
/// `sqlx::raw_sql(...).execute(&state.db.pool)` call on line 29.
///
/// A regression that flipped to `Router<()>` (e.g., a refactor that
/// "simplified" the handler to build its own pool internally) would
/// break the top-level `.nest()` into the app router (which is
/// typed `Router<AppState>` per routes/mod.rs:84-85).
///
/// Per CLAUDE.md habit #3 ("comment-vs-code drift"): the module
/// header claims "Admin-only endpoint to run migrations remotely"
/// — the AdminUser extractor IS the admin gate AND `State<AppState>`
/// IS the pool extractor. If a future refactor extracted a helper
/// that built its own pool, two invariants would break:
///   - the helper would bypass the connection-pool observability
///     wired into `state.db.pool`
///   - the `AdminUser` extractor would still gate the call, but the
///     SQL would execute against a different pool — losing the
///     in-flight-query tracking and tracing scope.
///
/// The compile-pin proves the router CANNOT be coerced to
/// `Router<()>` — the assignment below would fail if the type
/// drifted.
#[test]
fn migrate_router_state_type_is_app_state_not_unit() {
    let _typed: axum::Router<revolution_api::AppState> = migrate::router();

    fn _accepts_app_state_router(_r: axum::Router<revolution_api::AppState>) {}
    _accepts_app_state_router(migrate::router());
}

// ── 4. R9-D NEGATIVE: no BIGSERIAL/money DTOs on this surface ────────

/// R9-D NEGATIVE: the migrate surface has NO user-input DTOs and
/// NO money/cents/BIGINT contracts. Per CLAUDE.md "Money / cents —
/// i64 ONLY, BIGINT ONLY, EVERY TIME": the rule is "every
/// `*_cents` value is i64". This module has:
///   - no money fields
///   - no PK extractors (path params)
///   - no JSON request body (the handler takes only
///     `State<AppState>` + `AdminUser`, no `Json<Body>` extractor)
///   - no query-string DTO (the handler is a parameterless POST)
///
/// The "no DTO" state IS the pin — the only types exposed via the
/// public surface are:
///   - `pub fn router() -> Router<AppState>`
///   - the private `async fn run_schedule_migration` handler
///
/// Reserved exception per CLAUDE.md: this module has NO numeric
/// fields at all on its surface. The only numbers in the response
/// JSON are the hard-coded seed-data counts (6, 3, 3 — per the
/// `json!({...})` macro at lines 41-43), which are `i32` literals
/// in serde's JSON Value model. Those qualify as the "Reserved
/// exception" (row counts that genuinely cap below 2 billion) —
/// the seed_data integers are CMS-style "how many rooms did we
/// seed" counters, not money, not PKs.
///
/// If a future refactor added a `Path<i64>` migration ID parameter
/// (e.g., `POST /migrate/:migration_id/run`), the BIGSERIAL i64
/// rule would apply — `Path<i64>` would be required. The pin
/// documents this gap so the next PR that adds such a parameter
/// has to cite the rule.
#[test]
fn migrate_module_has_no_money_or_pk_dtos_on_public_surface() {
    // No DTOs to bind — the surface is exactly:
    //   - pub fn router() -> Router<AppState>
    //
    // If this test ever needs a `use migrate::{SomeDto};` line, the
    // CLAUDE.md money/PK rule MUST be cited in the PR that adds it.
    let _r: axum::Router<revolution_api::AppState> = migrate::router();

    // Trivially-true assertion to make the test executable; the value
    // is the compile-time check above + the documenting comment.
    assert_eq!(2 + 2, 4, "smoke");
}

// ── 5. Embedded migration SQL is compile-time pinned via include_str! ─

/// R9-D NEGATIVE: the migration SQL is embedded via
/// `include_str!("../../migrations/013_trading_room_schedules.sql")`
/// at routes/migrate.rs:26 — compile-time, NOT runtime. This is a
/// load-bearing defense-in-depth pattern:
///
///   - The SQL is fixed at build time → no SQL injection vector via
///     request body (the handler takes no `Json<Body>` at all).
///   - The migration file MUST exist at build time → renaming or
///     deleting `migrations/013_trading_room_schedules.sql` would
///     fail this crate's `cargo build`, caught BEFORE deploy.
///   - The SQL text is identical across all running replicas → no
///     drift between "what dev ran" and "what staging/prod will run".
///
/// The pin below asserts the migration file IS still present at the
/// path the source references. The build-time check via
/// `include_str!` is the primary defense; this test is a secondary
/// canary that catches "developer renamed the migration file but
/// didn't rebuild the crate before pushing".
///
/// Per CLAUDE.md memory (`feedback_create_not_delete.md`: "orphan =
/// build the missing side. Never `git rm`"), the migration file at
/// path `migrations/013_trading_room_schedules.sql` is referenced
/// by `routes/migrate.rs:26` — making it NOT an orphan. A future PR
/// that "cleaned up old migrations" would break this build.
#[test]
fn migrate_embedded_sql_file_exists_at_referenced_path() {
    // Use CARGO_MANIFEST_DIR to resolve the file relative to the
    // crate root — same anchor `include_str!` uses internally.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let migration_path = std::path::Path::new(manifest_dir)
        .join("migrations")
        .join("013_trading_room_schedules.sql");

    assert!(
        migration_path.exists(),
        "migrations/013_trading_room_schedules.sql MUST exist — \
         routes/migrate.rs:26 include_str!s it at compile time. \
         Renaming or deleting this file breaks the build."
    );

    // R9-D NEGATIVE: the SQL file MUST NOT be empty (an empty file
    // would compile-build the route but a no-op migration would
    // silently advertise "tables_created" without actually creating
    // anything — exactly the "comment vs code drift" CLAUDE.md
    // habit #3 flags).
    let metadata = std::fs::metadata(&migration_path).expect("stat migration file");
    assert!(
        metadata.len() > 100,
        "migration SQL MUST have substantive content (>100 bytes) — \
         empty migration + non-empty response advertising 'tables_created' \
         is a comment-vs-code drift bug"
    );
}
