# Rust Backend Forensic Audit — 2026-04-25

**Scope:** `api/` (Rust + Axum + sqlx + Postgres)
**Toolchain:** stable-aarch64-apple-darwin (rustc default)
**Baseline (this commit):** build OK, clippy 0 warnings, fmt clean, tests 32/32 pass
(`utils_test` 17, `stripe_test` 15).

---

## Top 5 systemic issues (by severity)

### 1. Pervasive `Result.unwrap_or_default()` on database calls — silent error masking (HIGH)

**Pattern:** `>200` call sites where `sqlx::query_*().await.unwrap_or_default()` swallows
DB errors and returns empty results. Worst offenders:
- `api/src/routes/crm.rs` — 15+ occurrences across listing/aggregation handlers.
- `api/src/routes/courses_admin.rs:154,162,170,179,1062,1419,…` — analytics + dashboards.
- `api/src/routes/orders.rs:211,386` — order item counts and product metadata join.
- `api/src/routes/room_resources.rs:1812,1563,2106,…` — list_stock_lists ignores fetch errors.

**Why it's bad:** A failing query (timeout, deadlock, malformed data) renders as a
successful empty list. Users see “no orders,” engineers see no telemetry. Project rule
in `CLAUDE.md` explicitly bans this pattern. An incident here is undetectable.

**Action:** Replace with `.map_err(|e| { tracing::error!(…); … })?` or, for non-fatal
fallbacks, log a warn before falling back. Triage in waves: auth/payments first, then
admin, then analytics.

---

### 2. Format-string SQL building without parameter placeholders (MEDIUM)

**Files (verified safe today, but fragile):**
- `api/src/routes/media.rs:198-204` — `ORDER BY {sort_column} {direction} LIMIT {n} OFFSET {n}`.
- `api/src/routes/cms_assets.rs:593-603` — same pattern.
- `api/src/routes/admin_popups.rs:96` — count query string-built.
- `api/src/routes/room_resources.rs:1787-1791,1463-1465` — same.
- `api/src/routes/member_courses.rs:1145,1163` — order by RANDOM()/sort_order.

**Why fragile:** Today these are guarded by allow-listed columns + integer clamping
+ hard-coded values, so they’re not exploitable. But the pattern means a future
contributor who adds a new branch without an allow-list opens an injection vector.
The class of bug is one careless edit away.

**Action:** Move all of these to either `query_as!` macros (compile-time checked) or
explicit `bind()` for limit/offset. Sort columns can stay string-formatted **iff**
they’re always selected from a const allow-list (document it inline).

---

### 3. Multi-table mutations without transactions (MEDIUM)

**Evidence:** Only two `pool.begin()` call sites in the entire codebase
(`api/src/routes/schedules.rs:899`, `api/src/routes/settings.rs:439`), but dozens of
handlers do sequential `INSERT` / `UPDATE` / `DELETE` on multiple tables. Examples:
- `api/src/routes/connections.rs::delete_connection` — deletes webhooks then connection.
- `api/src/services/mfa.rs::verify_backup_code` — updates `user_mfa_secrets` and
  later inserts an audit row in `record_mfa_attempt` without a tx.
- Order/subscription flows write to `orders`, `order_items`, `subscriptions`.

**Why bad:** Crash mid-flow leaves partial state — orphaned webhooks,
half-applied subscriptions, used backup codes that didn’t register the audit log.
Project rule in `CLAUDE.md`: “New mutations that touch >1 table need a `Pool::begin()`
→ `tx.commit()` transaction wrapper.”

**Action:** Audit every mutation handler. Wrap the multi-write set in a tx; pass
`&mut *tx` instead of `&pool`. Convert `record_mfa_attempt` to take a tx parameter.

---

### 4. Regex compiled per request in hot paths (LOW-MEDIUM perf)

**Files:**
- `api/src/routes/cms_seo.rs:351,354,365,969,1167,1272,1273,1309,1310,1311` — all
  compile constant regexes inside per-call helpers.

**Why bad:** `Regex::new("…").unwrap()` compiles a DFA on every invocation. Under load,
this is wasteful CPU — particularly in the SEO analysis handlers which run regex passes
back-to-back over post bodies.

**Action:** Move to `std::sync::OnceLock<Regex>` (or `lazy_static!` since the crate is
already a dep). 10× speedup on these paths is realistic.

---

### 5. `.body(...).unwrap()` on user-controlled HTTP header values — DoS vector (FIXED)

**Was:** `api/src/routes/member_indicators.rs:562` —
`Response::builder().header(CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", file.file_name)).body(…).unwrap()`.

**Why bad:** Filename came from DB, but DB content is admin-uploaded; CRLF or non-ASCII
in `file_name` could fail header parsing → panic in handler → request worker dies.
A malicious admin or compromised upload pipeline could DoS the download path.

**Fix in this commit:** Sanitizer (`sanitize_filename_for_disposition`) drops control
chars, escapes `"` and `\`, falls back to `download` if empty. Body builder error is
mapped to 500 instead of panicking.

---

## 30-day action plan

| Week | Track | Deliverable |
|------|-------|-------------|
| 1 | DB error handling | Sweep `routes/auth.rs`, `routes/orders.rs`, `routes/checkout.rs`, `routes/payments.rs`, `routes/subscriptions.rs` for `Result.unwrap_or_default()` and replace with `?` propagation. |
| 2 | Transactions | Add `pool.begin()` to every multi-table mutation in `connections`, `forms`, `organization`, `user`, `services/mfa`. |
| 3 | SQL safety | Convert format-string ORDER BY/LIMIT in `media.rs`, `cms_assets.rs`, `admin_popups.rs`, `room_resources.rs` to bind-parameter form. |
| 4 | Perf + observability | Hoist `cms_seo.rs` regexes to `OnceLock`; add Prometheus counters for the now-propagated DB errors so we can see them. |

---

## Confirmed clean

- **No SQL injection vectors found.** All `format!`-built SQL uses parameter
  placeholders (`${idx}`) or allow-listed identifiers.
- **No CORS wildcard with credentials.** Origins parsed from config explicitly
  (`api/src/main.rs:116-129`).
- **No password / token data in logs.** Library errors only (`{}: {e}`).
- **JWT secret redacted in `Debug` impl** (`api/src/config/mod.rs:84`).
- **Auth path uses constant-time comparison** via `bcrypt`/`argon2` libraries +
  `subtle` crate; verified at `api/src/utils/mod.rs:175-208`.
- **No `block_on` / `block_in_place` inside async runtime.**
- **No shared `std::sync::Mutex` held across `.await`** (none found in `src/`).

---

## Tooling state at audit

- `cargo build --locked` → 0 errors, 0 warnings.
- `cargo clippy --locked` → 0 warnings.
- `cargo fmt --check` → clean.
- `cargo machete` → not installed (skipped, see backlog).
- `cargo deny` → no `deny.toml` configured (skipped, see backlog).

**Backlog:** Install `cargo-machete` + `cargo-deny`, add `deny.toml` policy
(advisories + license allow-list), wire both into CI.
