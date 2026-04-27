# 09-system — Deferred Items

> Generated 2026-04-26 during audit remediation pass.
> Items below require either a project-wide architectural decision or a Rust API
> change and are therefore deferred from the current sprint.

---

## DEFERRED-1 — CSRF token strategy (project-wide decision)

**Audit ref:** §P1-9

SvelteKit's built-in CSRF protection covers form-encoded requests but not
`application/json` mutations. The `rtp_access_token` cookie is `sameSite: lax`,
which mitigates most top-level-navigation CSRF vectors, but explicit CSRF tokens
on POST/PUT/DELETE would provide defense-in-depth for the highest-impact
endpoints (role mutation, password reset, OAuth credential storage).

**Why deferred:** Adding CSRF tokens requires a project-wide token-generation
strategy (a server-side nonce, double-submit cookie, or Svelte form `enhance`
adapter). This decision affects every `+server.ts` proxy in the codebase, not
just the system cluster. It must be decided centrally before any cluster
implements it.

**Recommended approach when unblocked:**
- Use SvelteKit `csrf` hooks option (enable strict-mode JSON protection once
  SvelteKit ships it) or
- Issue a per-session CSRF token from `hooks.server.ts` and inject it into
  `event.locals`, then read it in each mutating proxy; or
- Adopt `@sveltejs/kit`'s `formEnhance` + `form actions` instead of JSON
  `+server.ts` for mutating operations — this gets CSRF for free.

---

## DEFERRED-2 — Rust API auth middleware changes

**Audit ref:** §P0-2 note, §P1-8

The audit recommended mirroring the `super-admin` role check in the Rust
handlers that serve role-mutation and password-reset requests (defense-in-depth
at the DB layer). This would require:

1. Modifying `api/src/routes/admin/users.rs` (or wherever the `PUT /admin/users/:id`
   handler lives) to extract the caller's role from the validated JWT and reject
   non-super-admin callers attempting role changes.
2. Same for `DELETE /admin/users/:id`.

**Why deferred:** The Rust middleware auth flow (JWT decode → role extraction) is
shared across all admin routes. Changing it without a full Rust audit risks
introducing regressions in unrelated handlers. This must be paired with
`cargo test` coverage of the modified handler.

**Current mitigation:** The SvelteKit proxy layer now enforces `requireSuperadmin`
on all role/password/delete mutations (P0-2 fixed in proxy). The Rust backend
remains the ultimate gatekeeper — this deferral only means the proxy check is
the *only* defense-in-depth layer, not that there is none.

---

## DEFERRED-3 — `users/create/+page.svelte` org-hierarchy endpoints (P0-5 partial)

**Audit ref:** §P0-5

The create-user page calls 5 `/api/admin/organization/*` endpoints (departments,
teams, locations, training-modules, onboarding-plans) that have no
`+server.ts` proxy. The page silently falls back to hard-coded defaults when
they 404, masking the missing API.

**Why deferred:** Building these 5 proxies requires knowing whether the Rust API
implements `/api/admin/organization/*`. A quick `curl` against the staging
backend is needed first. If the routes exist, proxy creation is trivial
(~10 min each). If not, the create form should be stripped of these fields.

**Short-term mitigation:** The fail-open validator bug (P1-3) was fixed — the
form no longer blocks submissions when the check endpoint is unreachable. The
org fields remain non-functional but don't break the happy path (create a user
without org data).

---

## DEFERRED-4 — `uploadProfilePhoto` stub returns `example.com` URL (P3-3)

**Audit ref:** §P3-3

`users/create/+page.svelte:901-907` returns a hardcoded
`'https://example.com/photo.jpg'` for every upload. Building a real upload
endpoint requires a storage bucket decision (S3, Cloudflare R2, or Bunny CDN —
the site already has Bunny wired for video). Deferred until the storage
strategy is confirmed.

---

## DEFERRED-5 — `checkBreachDatabase` is a 5-entry list, not HIBP (P3-4)

**Audit ref:** §P3-4

The breach-checking function in `users/create/+page.svelte` checks against 5
hard-coded common passwords rather than the HaveIBeenPwned k-anonymity API.
Integrating HIBP requires a server-side proxy (to avoid leaking the password
hash prefix to the browser) and a new `+server.ts` at
`/api/admin/users/check-password-breach` or similar. Deferred until the HIBP
integration is prioritized.

---

## Non-deferred items in this cluster

All P0 and P1 items not listed above were **fixed** in this pass. See
`09-system-RESULTS.md` for the full disposition table.
