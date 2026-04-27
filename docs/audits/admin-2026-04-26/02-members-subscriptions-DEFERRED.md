# 02 — Members / Subscriptions / Memberships — Deferred Items

Date: 2026-04-26
Scope: Items from `02-members-subscriptions.md` that intentionally were NOT
implemented in the same pass because the blast radius is too high (Stripe
state machines, multi-table billing mutations) or because the work is a
cross-cutting refactor that needs its own review window.

These need explicit human approval before any code change.

---

## D1 — `delete_member` Stripe-cancel + transaction wrapper (audit §P0-2)

**Status:** DEFERRED — needs design approval before implementation.

### Problem

`api/src/routes/admin_member_management.rs::delete_member` (lines 660-749)
performs three independent writes with no `Pool::begin()` wrapper:

1. `UPDATE users SET email = NULL, deleted_at = NOW() ...` (anonymize)
2. `UPDATE user_memberships SET status = 'cancelled' ...` — result swallowed
   via `let _ = ... .await`
3. `INSERT INTO user_activity_log ...` — result swallowed via `let _`

Failure modes:

- Step 1 commits + step 2 fails → user anonymized, sub still `active`,
  Stripe will keep billing the (now-orphan) subscription forever.
- No call to Stripe to cancel the subscription on Stripe's side. Even on
  the happy path Stripe will retry charges against the dangling
  `stripe_customer_id` until the next webhook delivery is rejected.
- The `let _ = ...` swallow violates the CLAUDE.md "no swallowed `Result`"
  rule.

### Why deferred

Stripe state-machine bugs are the highest-blast-radius bugs in this
codebase. Half-fixing this (e.g. wrapping in a transaction but skipping
the Stripe cancel call) would be **worse** than the current state because
the local DB would be authoritative-looking while Stripe stayed live.
Half-fixing the other direction (calling Stripe cancel without
transactional reconciliation) double-cancels on retry.

### Proposed design (needs approval)

```text
delete_member(user_id):
  1. SELECT stripe_customer_id, list of stripe_subscription_id WHERE user_id = $1
     (read OUTSIDE the tx — we need to know the IDs before locking rows)

  2. tx = pool.begin()
  3. UPDATE users SET email = NULL, deleted_at = NOW(), ... WHERE id = $1
     (inside tx)
  4. UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW()
     WHERE user_id = $1 AND status IN ('active', 'trial')
     (inside tx, propagate errors via ?)
  5. INSERT INTO user_activity_log (...) VALUES (...)
     (inside tx, propagate errors via ?)

  6. For each stripe_subscription_id:
       try stripe.cancel_subscription(sub_id, at_period_end=false)
       on failure → tx.rollback(); return 502 with partial-state message
     (these are network calls — must have a timeout, must NOT hold the tx
      indefinitely; consider a 10s per-call cap)

  7. tx.commit()

  8. Enqueue a webhook-reconciliation job (idempotent) that:
       - re-checks Stripe sub status for the canonical IDs from step 1
       - if any are still 'active' on Stripe, retries the cancel
       - alerts on N retries
```

### Open design questions

- **Stripe error taxonomy**: should "subscription already cancelled"
  (Stripe returns 400 on a no-op) be treated as success? (Yes per the
  Stripe docs, but worth being explicit in code.)
- **Customer-level cancel vs sub-level**: do we also call
  `stripe.delete_customer(stripe_customer_id)`? Pros: stops all future
  charges including off-platform ones. Cons: irrevocable, deletes
  payment-method history admins may need for refund disputes.
  Recommendation: NO — leave the customer object live; only cancel subs.
- **Idempotency**: `delete_member` may be retried. The transaction makes
  the local writes idempotent (re-running on an already-deleted row is a
  no-op SELECT), but the Stripe call may double-fire. Use the Stripe
  `Idempotency-Key` header keyed on `user_id + sub_id`.
- **Webhook reconciliation**: the codebase needs a generic "retry until
  consistent" job runner. If one doesn't exist, this should be a separate
  PR (see also CC-4 — multi-table mutations across the whole module need
  the same treatment).

### Required before implementation

1. Confirm Stripe SDK / HTTP client choice (currently `stripe-rust` v?
   pinned in `api/Cargo.toml` — verify it's wired into `state`).
2. Confirm `stripe_customer_id` and `stripe_subscription_id` columns are
   actually populated for legacy users (audit suggests they are nullable —
   handler must tolerate `Option<String>`).
3. Decide on background-job runner for webhook reconciliation (or accept
   that the failure mode is "alert + manual rerun" for now).
4. Get explicit approval from a human reviewer.

---

## D2 — Invoice Settings backend (audit §P1-5)

**Status:** Frontend page exists. Frontend proxy now wired
(`api/admin/invoice-settings/+server.ts`). Backend handler MISSING.

### What's needed

- `GET    /api/admin/invoice-settings`            → return the singleton row
- `PUT    /api/admin/invoice-settings`            → update settings JSON
- `POST   /api/admin/invoice-settings?action=preview` → render PDF/HTML preview
- `POST   /api/admin/invoice-settings?action=reset`   → restore defaults
- (optional) `POST /api/admin/invoice-settings/logo` → multipart upload

The proxy in `frontend/src/routes/api/admin/invoice-settings/+server.ts`
forwards verbatim, so the backend just needs to honor the same route shape.

### Why deferred

Backend implementation requires a database migration for the
`invoice_settings` table (does not exist) plus the PDF rendering library
choice (printpdf? wkhtmltopdf? Stripe-hosted invoice templates?). Out of
scope for the per-page audit fix.

---

## D3 — Real CSV import for members (audit §P1-8)

**Status:** Frontend "Import" button now shows a toast warning that the
backend isn't wired (replaced the previous `Math.random()` fake-success).
Backend handler MISSING.

### What's needed

- Multipart endpoint that accepts CSV, validates header schema
  (`email,first_name,last_name,plan_slug,...`), streams insert into
  `users` + `user_memberships`, and returns row counts +
  per-row error reports.
- Wrapped in a transaction (or chunked with explicit per-batch tx and
  a job ID so the admin can poll for partial-success status).
- Frontend re-wires `handleImport` to do real upload + progress UI.

### Why deferred

Out of scope for a UI audit pass. Belongs in its own backend PR.

---

## D4 — Past-Members Dashboard backend (audit §P1-3)

**Status:** Frontend page (1994 lines) and `lib/api/past-members-dashboard.ts`
exist. Catch-all proxy now in place
(`api/admin/past-members-dashboard/[...path]/+server.ts`) so it
forwards every call to the backend with the canonical auth pattern.
Backend MISSING entirely.

### What's needed

- A new `api/src/routes/past_members.rs` (or similar) with handlers for:
  `/overview`, `/period/:period`, `/services`, `/churn-reasons`,
  `/campaigns`, `/realtime`, `/bulk-winback`, `/bulk-survey`,
  `/invalidate-cache`.
- Each `/bulk-*` handler is a mass-mutation (potentially across thousands
  of `users` + `user_memberships` rows) — must be a `Pool::begin()` tx and
  per CC-5 should be `require_superadmin` not `require_admin`.

### Why deferred

Same as D2 — needs its own backend PR with schema work.

---

## D5 — Mixed `apiClient` vs SvelteKit-proxy patterns (audit §P2-1 / CC-2)

**Status:** Cross-cutting refactor. DEFERRED.

### Problem

`frontend/src/lib/api/members.ts` makes some calls via `apiClient.get('/admin/...')`
(cross-origin, header auth) and others via `fetch('/api/admin/...')`
(same-origin proxy, cookie auth). The recent `e2356fa46` cookie-pref
commit only fixed the proxy side, so the `apiClient` path is now the
only one that *doesn't* benefit from the canonical cookie-auth flow.

### Why deferred

Pulling out every `apiClient.*` admin call and routing it through the
proxy layer is a multi-file refactor that touches:
- `lib/api/members.ts`
- `lib/api/subscriptions.ts`
- (per spot-check) probably `lib/api/admin.ts` and a few others
- every page that imports from those modules

It needs to land as one cohesive PR with full e2e regression testing,
not a drive-by per-page tweak. Track in an audit follow-up.

---

## D6 — Member soft-delete cascade across other domains (audit §CC-4)

**Status:** Same risk class as D1; DEFERRED until D1 is approved.

`update_member`, `cancel_subscription`, `revoke_membership` all touch
multiple tables and (per the audit spot-check) are non-transactional. They
should ride the same transaction-wrapper pattern that lands with D1 — not
fixed individually because the failure modes are correlated (cancel a sub
without telling Stripe → same billing-desync class as D1).

---

## Summary

| # | Item | Why deferred | Blocker for human |
|---|---|---|---|
| D1 | `delete_member` tx + Stripe cancel | Stripe state machine | Yes — design approval |
| D2 | Invoice Settings backend | Schema + PDF lib choice | Yes — scope decision |
| D3 | Member CSV import backend | Multipart + job runner | Yes — scope decision |
| D4 | Past-Members backend | Schema + analytics scope | Yes — scope decision |
| D5 | apiClient → proxy refactor | Cross-cutting | Yes — own PR |
| D6 | Tx-wrappers for other multi-table mutations | Same class as D1 | Yes — bundle with D1 |
