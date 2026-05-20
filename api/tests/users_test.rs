//! Users route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::users` and pins the
//! `router()` mount-table shape for the ICT-7 RBAC-hardened user
//! lookup surface.
//!
//! ## Why this file exists (R21-D)
//!
//! `routes/users.rs` is a 67 LOC critical-path admin surface:
//! `GET /users/` (list all users) and `GET /users/:id` (lookup by
//! BIGSERIAL i64 PK). Both handlers gate on `AdminUser` (lines 19,
//! 43) — the module header explicitly calls this out:
//!
//!     "ICT Level 7 Security Fix
//!      CRITICAL: All routes now require AdminUser authentication
//!      Previously these endpoints were PUBLIC - exposing all user
//!      data"
//!
//! Per CLAUDE.md habit #1 ("Cite the rule in your work"), this is
//! the EXACT class of regression CLAUDE.md is designed to catch:
//! a refactor that drops the `_admin: AdminUser` extractor for
//! "simplicity" would silently re-open the user database to the
//! internet. What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin — 2 GET endpoints, AdminUser-gated.**
//!    Mount table at routes/users.rs:63-67:
//!      - GET /     → `list_users`  (`_admin: AdminUser`, line 43)
//!      - GET /:id  → `get_user`    (`_admin: AdminUser`, line 19)
//!    Mounted at `/users` per routes/mod.rs:88. A regression that
//!    dropped `AdminUser` from either handler would compile here
//!    (the router still nests fine) but would re-open the PII leak.
//!    Per CLAUDE.md habit #4 ("Trust the operator's gut over CI"),
//!    this compile-pin is necessary but NOT sufficient — a manual
//!    RBAC audit grep is the real defense. The pin catches handler
//!    signature drift (e.g., `get_user` flipped to a non-async fn).
//!
//! 2. **`Path<i64>` BIGSERIAL PK pin.** Per routes/users.rs:20,
//!    `Path(id): Path<i64>` — `users.id` is BIGSERIAL (per
//!    schema.sql, line covered by every other test's BIGSERIAL
//!    citation). A regression to `Path<i32>` would silently 400
//!    every user with `id > 2_147_483_647`. With BIGSERIAL the
//!    catalog can legitimately hit those IDs over time (especially
//!    with bot-signup spam events that burn IDs without producing
//!    real rows). Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT
//!    ONLY, EVERY TIME": although `id` is NOT money, it indexes
//!    into a BIGSERIAL table — same i64 invariant applies.
//!
//! 3. **`UserResponse` is the public DTO** (from
//!    `models::user::UserResponse`, lines 122-141). The
//!    `is_admin: bool` field is the load-bearing RBAC signal that
//!    the frontend admin dashboard reads to decide whether to
//!    render the admin nav. A regression that dropped `is_admin`
//!    from the DTO would silently break the admin dashboard
//!    render (frontend defaults `is_admin` to false on undefined,
//!    so non-admins would see a working app and admins would lose
//!    the admin nav).
//!
//! 4. **`UserResponse.id: i64` matches the route's `Path<i64>`.**
//!    Per models/user.rs:124, the response carries the same i64
//!    PK the route extracts. A regression to i32 on either side
//!    would create a type-mismatch at the handler return
//!    (caught at compile time) BUT a regression on the JSON wire
//!    (e.g., serializing `id` as a string) would compile and only
//!    surface in the browser as undefined-shaped data.
//!
//! 5. **R9-D NEGATIVE: `UserResponse.password_hash` is NOT exposed.**
//!    Per models/user.rs:122-141, the response DTO has NO
//!    `password_hash` field — even though the underlying `User`
//!    struct (line 16-34) DOES carry `password_hash: String`
//!    (with `#[serde(skip_serializing)]` as defense-in-depth). The
//!    handler explicitly maps `User → UserResponse` via the
//!    `From<User>` impl, which acts as a typed projection — a
//!    regression that returned `Json<User>` directly would compile
//!    BUT (a) the `#[serde(skip_serializing)]` on `password_hash`
//!    would prevent the hash from going over the wire, AND (b) the
//!    typed projection through UserResponse is the documented
//!    contract. We pin the typed projection.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_members_test.rs` (sister admin surface),
//! `tests/auth_test.rs` (load-bearing auth surface),
//! `tests/admin_test.rs` (admin RBAC surface).

use revolution_api::routes::users;

// ── 1. Router mount-table compile-pin — 2 AdminUser-gated GETs ───────

/// `routes::users::router()` MUST build as `Router<AppState>`.
/// Mount table per routes/users.rs:63-67:
///   - GET /     → `list_users`  (admin-gated)
///   - GET /:id  → `get_user`    (admin-gated, Path<i64>)
///
/// Both handlers carry `_admin: AdminUser` extractors (lines 19, 43).
/// Per CLAUDE.md habit #1 ("Cite the rule in your work") — the
/// ICT-7 RBAC fix (module header lines 1-4) is THE invariant for
/// this surface. A regression that:
///   - dropped `AdminUser` from either handler → PII leak
///   - added a new public handler → expanded attack surface
///   - changed `Path<i64>` to `Path<i32>` → silently 400 large IDs
/// would either fail this compile (signature drift) OR pass and
/// require the RBAC audit grep to catch (per CLAUDE.md habit #2:
/// "RBAC / audit: grep showing `policy.require()` + audit at every
/// cited line").
///
/// R9-D NEGATIVE: the compile-pin DOES NOT prove the AdminUser
/// extractor is present — it only proves the router shape is
/// consistent. The header comment + the in-handler `_admin:
/// AdminUser` lines are the source of truth; the pin documents
/// where to look.
#[test]
fn users_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = users::router();
}

// ── 2. Router idempotency — no module-cached state survives refactor ─

/// Constructing `router()` repeatedly MUST NOT panic. Per CLAUDE.md
/// habit #3 ("does any `static` / `OnceLock` / lazy init survive
/// the refactor?"), this module has NO `lazy_static` / `OnceLock`
/// at all — it's purely a routing surface over SQL handlers.
///
/// A regression that introduced a module-cached `OnceLock<Vec<i64>>`
/// (e.g., to memoize admin user IDs and skip the DB roundtrip on
/// the AdminUser extractor) would be EXACTLY the failure mode
/// CLAUDE.md flags: the cache would survive role changes in the
/// `users.role` column and silently grant ex-admins continued
/// access. Constructing the router repeatedly is a weak proxy for
/// "no module-init side effects" — the strong defense is the code
/// review.
#[test]
fn users_router_construction_idempotent() {
    let _r1: axum::Router<revolution_api::AppState> = users::router();
    let _r2: axum::Router<revolution_api::AppState> = users::router();
    let _r3: axum::Router<revolution_api::AppState> = users::router();
    let _r4: axum::Router<revolution_api::AppState> = users::router();
}

// ── 3. UserResponse is the public DTO — i64 PK + RBAC signal ─────────

/// `UserResponse { id: i64, ..., is_admin: bool, ... }` MUST be the
/// public response shape. Per `models/user.rs:122-141`:
///   - id              i64                (BIGSERIAL PK)
///   - email           String
///   - name            String
///   - first_name      Option<String>     (split from name)
///   - last_name       Option<String>     (split from name)
///   - role            String             (raw role text)
///   - roles           Vec<String>        (multi-role expansion)
///   - permissions     Vec<String>        (RBAC permission set)
///   - email_verified  bool
///   - avatar_url      Option<String>
///   - mfa_enabled     bool
///   - is_admin        bool               (LOAD-BEARING: frontend
///                                         admin nav gate)
///   - created_at      NaiveDateTime
///   - updated_at      NaiveDateTime
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY TIME":
/// `id` is NOT money but indexes into a BIGSERIAL table — the i64
/// invariant applies. The pin builds a UserResponse with
/// `(i32::MAX as i64) + 1` and asserts the wire JSON carries it
/// intact.
///
/// R9-D NEGATIVE: a regression that dropped `is_admin` from
/// UserResponse would compile (the `From<User>` impl computes it
/// from `role`) BUT the frontend `is_admin || roles.includes(...)`
/// pattern would default to false on undefined — silently breaking
/// the admin nav for legitimate admins.
#[test]
fn user_response_bigserial_i64_pk_and_is_admin_signal() {
    use revolution_api::models::UserResponse;

    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let resp = UserResponse {
        id: above_i32_max,
        email: "admin@revolutiontradingpros.com".to_string(),
        name: "Billy Ribeiro".to_string(),
        first_name: Some("Billy".to_string()),
        last_name: Some("Ribeiro".to_string()),
        role: "admin".to_string(),
        roles: vec!["admin".to_string()],
        permissions: vec!["users:read".to_string(), "users:write".to_string()],
        email_verified: true,
        avatar_url: None,
        mfa_enabled: true,
        // LOAD-BEARING: frontend reads this to gate the admin nav.
        is_admin: true,
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
    };

    let wire = serde_json::to_value(&resp).expect("serialize UserResponse");

    // BIGSERIAL pin: id round-trips past i32::MAX.
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert!(
        wire["id"].as_i64().unwrap() > i32::MAX as i64,
        "UserResponse.id MUST round-trip past i32::MAX (BIGSERIAL PK)"
    );

    // RBAC signal pin: is_admin MUST be present and boolean.
    assert!(wire["is_admin"].is_boolean(), "is_admin MUST be bool");
    assert_eq!(wire["is_admin"].as_bool(), Some(true));

    // Reserved exception: there are NO i32 counter fields on
    // UserResponse — every numeric field is i64 (id) or bool. If
    // a future refactor added e.g. `login_count: i32`, the
    // Reserved exception rationale ("row counts genuinely cap below
    // 2 billion") would apply — but no such field exists today.
}

// ── 4. R9-D NEGATIVE: UserResponse does NOT leak password_hash ───────

/// R9-D NEGATIVE: `UserResponse` MUST NOT expose `password_hash`.
/// Per `models/user.rs:122-141`, the response DTO has NO
/// `password_hash` field — it's intentionally projected out of the
/// `User → UserResponse` conversion. Even the underlying `User`
/// struct (line 16-34) carries `password_hash` with
/// `#[serde(skip_serializing)]` as defense-in-depth.
///
/// Per CLAUDE.md habit #1 ("Cite the rule in your work"), this is
/// the ICT-7 RBAC fix's PII corollary: the handler returns
/// `Json<UserResponse>`, NOT `Json<User>`, precisely to make the
/// password-hash exclusion compile-time-checked.
///
/// R9-D NEGATIVE: a regression that returned `Json<User>` from
/// `get_user` would compile (User implements Serialize) BUT the
/// `#[serde(skip_serializing)]` attribute on password_hash would
/// still skip the field. The deeper risk is: a regression that
/// removed the `#[serde(skip_serializing)]` attribute AND flipped
/// the handler to return `Json<User>` would leak the hash.
///
/// The pin serializes a `User` (NOT UserResponse) and asserts
/// password_hash is absent from the JSON output — proving the
/// skip_serializing attribute is in force.
#[test]
fn user_struct_password_hash_skip_serializing_pin() {
    use revolution_api::models::User;

    let user = User {
        id: 1,
        email: "leak-canary@example.com".to_string(),
        password_hash: "$argon2id$v=19$m=65536,t=3,p=4$abc$def".to_string(),
        name: "Canary".to_string(),
        role: Some("user".to_string()),
        is_active: Some(true),
        email_verified_at: None,
        avatar_url: None,
        mfa_enabled: Some(false),
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
    };

    let wire = serde_json::to_value(&user).expect("serialize User");

    // R9-D NEGATIVE: password_hash MUST NOT be in the JSON output.
    assert!(
        wire.get("password_hash").is_none(),
        "User.password_hash MUST be skipped — #[serde(skip_serializing)] is load-bearing"
    );

    // Other fields ARE present (smoke check the User itself
    // serializes correctly; only the hash is skipped).
    assert_eq!(wire["id"].as_i64(), Some(1));
    assert_eq!(wire["email"].as_str(), Some("leak-canary@example.com"));
}

// ── 5. R9-D NEGATIVE: From<User> projection drops password_hash ──────

/// R9-D NEGATIVE: the `From<User> for UserResponse` impl in
/// `models/user.rs` (line 155+) MUST drop `password_hash` from the
/// projection. The handler (`routes/users.rs:36`) calls
/// `Ok(Json(user.into()))` — converting `User` → `UserResponse` via
/// this `From` impl. If a regression added `password_hash` to
/// UserResponse and copied it through the projection, the hash
/// would leak.
///
/// Per CLAUDE.md habit #3 ("re-read your own diff"): the projection
/// is THE defense. The pin builds a `User` with a recognizable
/// password_hash, runs it through `.into::<UserResponse>()`, and
/// serializes the result — asserting no field with that hash
/// value appears anywhere in the JSON.
///
/// Reserved exception per CLAUDE.md: this is a security pin, not a
/// money/PK pin. The user/admin/PII surface is exactly where
/// CLAUDE.md habit #4 ("Trust the operator's gut over CI") applies:
/// CI passes when the field-drop test passes; a real adversary
/// would test the live endpoint with a curl from outside the VPC.
#[test]
fn user_to_user_response_projection_drops_password_hash() {
    use revolution_api::models::{User, UserResponse};

    const CANARY_HASH: &str = "$argon2id$CANARY-CANARY-CANARY-CANARY";

    let user = User {
        id: 42,
        email: "projection-canary@example.com".to_string(),
        password_hash: CANARY_HASH.to_string(),
        name: "Projection Canary".to_string(),
        role: Some("admin".to_string()),
        is_active: Some(true),
        email_verified_at: Some(chrono::NaiveDateTime::default()),
        avatar_url: None,
        mfa_enabled: Some(false),
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
    };

    let response: UserResponse = user.into();
    let wire = serde_json::to_string(&response).expect("serialize UserResponse");

    // R9-D NEGATIVE: the canary hash MUST NOT appear anywhere in the
    // JSON output of UserResponse. If a future refactor added a
    // `password_hash` field to UserResponse (or any field that
    // forwarded user.password_hash), the canary would surface here.
    assert!(
        !wire.contains(CANARY_HASH),
        "UserResponse JSON MUST NOT contain password_hash — projection is the defense"
    );

    // And the load-bearing is_admin signal IS computed from role.
    assert!(
        response.is_admin,
        "is_admin MUST be true for role=admin (computed in From<User>)"
    );
}
