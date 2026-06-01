# TODO Resolution Log — 2026-06-01

Audit performed via `cargo check`, `cargo clippy -D warnings`, `cargo fmt --check`, and
manual grep across all `.rs` files.  Every TODO/FIXME found in the Rust backend is
addressed below.  All checks pass after these changes.

---

## 1. WebSocket JWT Validation (RESOLVED — stale comments removed)

**File:** `api/src/routes/websocket.rs`  
**Original TODOs:**
```
// TODO: This token is currently accepted but never validated against JWT.
// TODO: Validate JWT token — validation now happens here.
```
**Resolution:** JWT validation was already fully implemented in `ws_handler` (calls
`verify_jwt(token, &state.config.jwt_secret, "access")` and rejects with HTTP 401
before the WebSocket upgrade).  Stale TODO comments were replaced with accurate doc
comments explaining the actual behaviour.

---

## 2. Newsletter Confirmation Email — New Subscription (IMPLEMENTED)

**File:** `api/src/routes/newsletter.rs` — `subscribe()` handler  
**Original TODO:**
```rust
// TODO: Send confirmation email with token
// email_service.send_newsletter_confirmation(&email, &subscriber.name.unwrap_or_default(), &confirm_token).await?;
```
**Resolution:** Wired `state.services.email.send_transactional()` with the
`newsletter-confirmation` Postmark template alias.  Builds a `confirm_url` from
`state.config.app_url` and passes `{name, confirm_url, app_url}` as the template
model.  Failure is non-fatal (warns to tracing but does not abort the subscription
insert) so the user record is always created.

---

## 3. Newsletter Confirmation Email — Resend Path (IMPLEMENTED)

**File:** `api/src/routes/newsletter.rs` — `subscribe()` resend branch  
**Original TODO:**
```rust
// TODO: Actually send confirmation email via email service
```
**Resolution:** Same `send_transactional` call as #2 above, added to the
already-subscribed-but-pending resend branch.  Non-fatal on failure with a `WARN`
log entry including `subscriber_id` and `error`.

---

## 4. Admin Videos — Trader Fetch (IMPLEMENTED)

**File:** `api/src/routes/admin_videos/crud.rs` — `list_videos()` and `get_video()`  
**Original TODOs:**
```rust
let trader = None; // TODO: Fetch trader if trader_id exists
let trader = None; // TODO: Fetch trader
```
**Resolution:** Replaced with a conditional `sqlx::query_as` against `traders` table
when `video.trader_id` is `Some`.  Uses `fetch_optional` so missing rows return
`None` safely.

---

## 5. Admin Videos — Room Assignments Fetch (IMPLEMENTED)

**File:** `api/src/routes/admin_videos/crud.rs` — `list_videos()` and `get_video()`  
**Original TODOs:**
```rust
let rooms: Vec<RoomInfo> = vec![]; // TODO: Fetch room assignments
let rooms: Vec<RoomInfo> = vec![]; // TODO: Fetch rooms
```
**Resolution:** Replaced with a `sqlx::query_as` joining `trading_rooms` through
`video_room_assignments` on `video_id`.  Uses `fetch_all(...).unwrap_or_default()`
so a missing join table degrades gracefully to an empty vec.

---

## 6. Admin Videos — `this_week` / `this_month` Stats (IMPLEMENTED)

**File:** `api/src/routes/admin_videos/crud.rs` — `get_stats()`  
**Original TODOs:**
```rust
this_week: 0,  // TODO: Calculate
this_month: 0, // TODO: Calculate
```
**Resolution:** Computed with `date_trunc('week', NOW())` and `date_trunc('month', NOW())`
respectively via inline `sqlx::query_as` blocks inside the struct literal.

---

## 7. Admin Videos — `trading_rooms` / `traders` in Options (IMPLEMENTED)

**File:** `api/src/routes/admin_videos/crud.rs` — `get_options()`  
**Original TODOs:**
```rust
"trading_rooms": [], // TODO: Fetch from database
"traders": []        // TODO: Fetch from database
```
**Resolution:** Both fields now execute `sqlx::query_as::<_, RoomInfo>` and
`sqlx::query_as::<_, TraderInfo>` against `trading_rooms` (filtered by `is_active`)
and `traders` respectively.  `unwrap_or_default()` keeps the endpoint alive even if
the tables do not yet exist in the dev schema.

---

## 8. Email Templates — Test Send Integration (IMPLEMENTED)

**File:** `api/src/routes/email_templates.rs` — `send_test_email()`  
**Original TODO:**
```rust
// TODO: Integrate with actual email service (Postmark, SendGrid, etc.)
// For now, return success as placeholder
```
**Resolution:** Handler now fetches the template row by `id` to obtain its `slug`
(used as the Postmark template alias), constructs a test model with
`{recipient, admin_name, template_id, app_url, test: true}`, and calls
`state.services.email.send_transactional()`.  Returns the Postmark result including
`email_service_enabled` flag so the admin UI can show a meaningful status.

---

## 9. Analytics — Bounce Rate Schema Limitation (DOCUMENTED)

**File:** `api/src/routes/analytics.rs` — `get_realtime()`  
**Original TODOs:**
```
// TODO: integrate with full analytics service for bounce rate.
// TODO: integrate with full analytics service to compute true bounce rate
```
**Resolution:** `bounce_rate: 0.0` is intentional and schema-bound — accurate
computation requires per-session pageview counts and time-on-page, which are not
stored in the current `analytics_events` table.  The inline TODO comments were
removed; the limitation is now documented in the function's doc comment instead.
A note about requiring an `analytics_sessions` table is left for future reference.

---

## 10. Webhook Subscription — Trial Email TODO (RESOLVED — already implemented)

**File:** `api/src/routes/payments/webhook_subscription.rs`  
**Original TODO:**
```rust
// TODO Task 4: send Postmark "trial ending soon" email to user
```
**Resolution:** The trial-ending email was fully implemented in Batch 6 (the code
block immediately below the `security_events` insert sends the Postmark email via
`send_transactional`).  The stale TODO was removed.

---

## 11. CMS Redirect — Regex Matching (IMPLEMENTED)

**File:** `api/src/services/cms_content.rs` — `match_redirect()`  
**Original TODO:**
```rust
// TODO: Implement regex matching if needed
Ok(None)
```
**Resolution:** After the exact-match path, the function now fetches all active
`is_regex = true` redirects ordered by `length(source_path) DESC` (most-specific
first), compiles each `source_path` with `regex::Regex::new()`, skips malformed
patterns with a WARN log, and returns the first match.  Hit count is updated on
successful regex match.  `regex` was already a declared dependency in `Cargo.toml`.

---

## Remaining Known Gaps (not TODO comments — tracked separately)

| Location | Description |
|---|---|
| `routes/media/scanning.rs` | Malware scanning service not integrated (ClamAV / VirusTotal) |
| `routes/admin_courses/analytics_pricing.rs` | `course_views` table not yet created (uses metadata JSON path) |
| `routes/admin_courses/analytics_pricing.rs` | `order_items.course_id` FK column not yet added |
| `routes/mod.rs` | `indicators_admin` module disabled (SQLx tuple decoding issue) |
| `routes/health.rs` | Coupon seed migration commented out pending migration file |

---

## Validation

All checks passed after implementation:

```
SQLX_OFFLINE=true cargo check --manifest-path api/Cargo.toml --all-targets  ✅
SQLX_OFFLINE=true cargo clippy ... -- -D warnings                            ✅
cargo fmt --manifest-path api/Cargo.toml --all -- --check                    ✅
```
