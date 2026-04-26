# 09 — System Audit (Users / Settings / Connections / Site Health)

> Read-only audit dated 2026-04-26. Highest blast-radius cluster in the admin
> surface — auth, role mutation, OAuth credential storage. Findings here block
> shipping until the P0/P1 items in §Critical and §High-severity are remediated.

## Files reviewed

### Frontend pages

- [`frontend/src/routes/admin/users/+page.svelte`](../../../frontend/src/routes/admin/users/+page.svelte)
- [`frontend/src/routes/admin/users/create/+page.svelte`](../../../frontend/src/routes/admin/users/create/+page.svelte)
- [`frontend/src/routes/admin/users/edit/[id]/+page.svelte`](../../../frontend/src/routes/admin/users/edit/[id]/+page.svelte)
- [`frontend/src/routes/admin/settings/+page.svelte`](../../../frontend/src/routes/admin/settings/+page.svelte)
- [`frontend/src/routes/admin/connections/+page.svelte`](../../../frontend/src/routes/admin/connections/+page.svelte)
- [`frontend/src/routes/admin/site-health/+page.svelte`](../../../frontend/src/routes/admin/site-health/+page.svelte)

### Frontend `+server.ts` proxies

- [`frontend/src/routes/api/admin/users/+server.ts`](../../../frontend/src/routes/api/admin/users/+server.ts)
- [`frontend/src/routes/api/admin/users/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/users/%5B...rest%5D/+server.ts)
- [`frontend/src/routes/api/admin/users/[id]/+server.ts`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts)
- [`frontend/src/routes/api/admin/connections/+server.ts`](../../../frontend/src/routes/api/admin/connections/+server.ts)
- [`frontend/src/routes/api/admin/connections/status/+server.ts`](../../../frontend/src/routes/api/admin/connections/status/+server.ts)
- [`frontend/src/routes/api/admin/connections/[key]/connect/+server.ts`](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/connect/+server.ts)
- [`frontend/src/routes/api/admin/connections/[key]/disconnect/+server.ts`](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/disconnect/+server.ts)

### Supporting

- [`frontend/src/lib/utils/createProxyShim.ts`](../../../frontend/src/lib/utils/createProxyShim.ts) — the canonical pattern; users `[...rest]` shim delegates here
- [`frontend/src/lib/utils/adminFetch.ts`](../../../frontend/src/lib/utils/adminFetch.ts) — client-side authed fetch
- [`frontend/src/hooks.server.ts`](../../../frontend/src/hooks.server.ts) — token validation/refresh; sets `event.locals.user`

---

## Critical bugs (P0)

### P0-1 — `[id]/+server.ts` returns FAKE user data when backend 404s, including a hard-coded super-admin (CRITICAL AUTH)

**File:** [`frontend/src/routes/api/admin/users/[id]/+server.ts:18-55, 100-131`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts)

The single-user proxy ships an in-memory `mockUsers` table that is consulted whenever the backend returns 404 (or any non-2xx) **and** is mutated by PUT to "update" the mock. Two concrete catastrophes:

1. **Phantom super-admin synthesized for any unknown ID.** Lines 105-125:

   ```ts
   if (!user) {
       // Return a generic mock user for any ID
       return json({
           data: {
               id: userId,
               name: 'User',
               …
               roles: [{ name: 'member' }],
               …
           },
           _mock: true,
           _message: 'Using mock data. Backend not available.'
       });
   }
   ```

   …but for ID `1` (line 19-31) the mock is hard-wired to:

   ```ts
   1: {
       …
       email: 'admin@revolutiontrading.com',
       roles: [{ name: 'super-admin' }, { name: 'admin' }],
       is_active: true,
       …
   }
   ```

   Any authenticated bearer token can `GET /api/admin/users/1` and receive `super-admin` role data when the backend is unreachable. Edit-page UI (`edit/[id]/+page.svelte:64`) reads `user.roles?.map((r: any) => r.name)` and pre-populates the role checkboxes from this. A subsequent PUT will then submit `roles: ['super-admin', 'admin']` — see P0-2.

2. **PUT goes through the same fallback (lines 145-178).** When the backend errors, `fetchFromBackend` returns `null` and the route happily mutates the in-memory map and returns success:

   ```ts
   mockUsers[userId] = updatedUser;
   return json({ data: updatedUser, message: 'User updated successfully', _mock: true });
   ```

   The admin UI cannot distinguish a real save from a fake one. Operators get a green toast while no DB write happened — including for password and role changes. This is a **trust boundary failure**: the proxy lies to the operator.

3. **DELETE has the same lie surface (lines 192-211).** Returns `success: true` even when no real delete happened.

**Action:** Delete the entire `mockUsers` table and the fallback branches. Bubble backend errors to the client (the same way `[...rest]` shim and the connections endpoints do). Mock data has no place in a privileged-write proxy.

---

### P0-2 — No `isSuperadmin` guard before role assignment / password reset / delete (PRIVILEGE ESCALATION)

**Files:**
- [`frontend/src/routes/api/admin/users/+server.ts:69-96`](../../../frontend/src/routes/api/admin/users/+server.ts) (POST create)
- [`frontend/src/routes/api/admin/users/[id]/+server.ts:134-179, 182-212`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts) (PUT/DELETE)
- [`frontend/src/routes/api/admin/users/[...rest]/+server.ts:7`](../../../frontend/src/routes/api/admin/users/%5B...rest%5D/+server.ts) (catch-all)

The frontend proxy layer enforces only "is the bearer token present?" — there is **no** check that the caller has `super-admin` (or even `admin`) role before forwarding role-mutating requests:

```ts
// users/+server.ts POST
const cookieToken = cookies.get('rtp_access_token');
…
if (!token) error(401, 'Unauthorized');
const authHeader = `Bearer ${token}`;
…
const { data, status } = await fetchFromBackend('/admin/users', {
    method: 'POST',
    headers: { Authorization: authHeader },
    body: JSON.stringify(body)   // ← caller controls roles[] here
});
```

Edit page UI lets any logged-in admin (including `member`-role tokens that happened to bypass the page-level guard) check the "Super Admin" box ([`edit/[id]/+page.svelte:284-296`](../../../frontend/src/routes/admin/users/edit/%5Bid%5D/+page.svelte)) and PUT the change. The proxy doesn't read `event.locals.user` to verify the caller's role — it just forwards the bearer token. The catch-all shim ([`createProxyShim.ts:52-79`](../../../frontend/src/lib/utils/createProxyShim.ts)) does the same.

**This relies entirely on the Rust backend to enforce role-mutation ACLs.** The audit cannot confirm that here, but the principle of defense-in-depth requires the proxy to also reject:

- `POST /api/admin/users` with `roles` containing `super-admin` unless `event.locals.user.role === 'super-admin'`
- `PUT /api/admin/users/:id` mutating `roles` or `password` similarly
- `DELETE /api/admin/users/:id` where `:id === event.locals.user.id` (no self-delete) and where the target is the last super-admin

Same gap exists for connections (next finding) — `disconnect`/`connect` of `stripe` or `aws_s3` should require super-admin.

**Action:** Add `requireRole('super-admin')` middleware to every mutating proxy in this cluster. Read `event.locals.user.role` (already populated by `hooks.server.ts:99-103`) and 403 before forwarding. Mirror the same check in the Rust handler.

---

### P0-3 — `connections/status/+server.ts` is a public, unauthenticated endpoint exposing connection health

**File:** [`frontend/src/routes/api/admin/connections/status/+server.ts:160-198`](../../../frontend/src/routes/api/admin/connections/status/+server.ts)

```ts
export const GET: RequestHandler = async () => {
    // Build connections list combining built-in and external services
    const connections: ConnectionStatus[] = [
        // Built-in services are always connected
        ...BUILTIN_SERVICES,
        …
    ];
    …
    return json({ success: true, connections, summary: { … } });
};
```

No `cookies.get('rtp_access_token')` check, no 401 guard, no role gate. Anyone on the internet can hit `/api/admin/connections/status` and enumerate which third-party services this site uses, plus the built-in CRM/SMTP/forms/booking suite (lines 26-82). While the secret values aren't leaked here, the **service-key inventory itself** is reconnaissance gold: an attacker now knows the site uses Stripe/SendGrid/Mailgun/AWS SES/HubSpot/Salesforce/Sentry/Datadog/Twilio/Tradier/Polygon and can target attacks accordingly.

Compounds with P1-1 (orphan endpoint — see below): nothing in the codebase even calls this endpoint, so locking it down doesn't break anything.

**Action:** Either delete the file entirely (no callers — see P1-1) or add the same `cookies.get('rtp_access_token')` + super-admin guard the other connection endpoints have.

---

### P0-4 — Settings/Connections pages call `/api/admin/connections/{key}/test` which has no `+server.ts` handler

**Files:**
- [`frontend/src/routes/admin/settings/+page.svelte:540`](../../../frontend/src/routes/admin/settings/+page.svelte) — `adminFetch('/api/admin/connections/${selectedService.key}/test', …)`
- [`frontend/src/routes/admin/connections/+page.svelte:178`](../../../frontend/src/routes/admin/connections/+page.svelte) — same
- Directory tree under `frontend/src/routes/api/admin/connections/[key]/` contains only `connect/` and `disconnect/`. There is no `test/+server.ts`.

This call goes through SvelteKit's catch-all routing; with no proxy file, the request will 404 (or be served by an unrelated catch-all if one is added later). Worse — the "Test Connection" button submits the user's credential payload (`{ credentials: credentialValues }` line 542) to a non-existent route. In dev, that means credentials can leak into 404 logs; in prod, the user gets a confusing failure but the API key has already left their browser to a phantom endpoint.

**Action:** Either build the missing `[key]/test/+server.ts` proxy (mirroring `connect`'s shape) or hide the "Test Connection" button until backend support exists. Per the user's `feedback_create_not_delete.md` rule, prefer the former.

---

### P0-5 — `users/create/+page.svelte` calls 6 organization endpoints that don't exist

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:388-396`](../../../frontend/src/routes/admin/users/create/+page.svelte)

```ts
const [deptRes, teamsRes, managersRes, locationsRes, trainingRes, onboardingRes] =
    await Promise.allSettled([
        fetch('/api/admin/organization/departments'),
        fetch('/api/admin/organization/teams'),
        fetch('/api/admin/users?role=manager&limit=50'),
        fetch('/api/admin/organization/locations'),
        fetch('/api/admin/organization/training-modules'),
        fetch('/api/admin/organization/onboarding-plans')
    ]);
```

There is **no** `frontend/src/routes/api/admin/organization/` directory. Five of these six fetches will 404 every render. The page silently falls back to hard-coded defaults (lines 404-448), masking the bug.

Plus lines 696-697 and 723-724 call:
```
/api/admin/users/check-username?username=…
/api/admin/users/check-email?email=…
```
Neither exists in `routes/api/admin/users/` (only `+server.ts`, `[...rest]`, and `[id]`). The catch-all `[...rest]` will catch `check-username` but only because of the SvelteKit POST=405 shim (which forwards to backend at `/api/admin/users/check-username`) — which in turn has to exist on the Rust side. Audit cannot confirm. Most likely all three return 404 silently, so `usernameAvailable`/`emailAvailable` stay `null` and validation lets duplicate usernames through (see P1-3).

**Action:** Either build the missing endpoints or strip the form down to fields the backend actually supports. Fingers-crossed-it-works is unacceptable for the user creation flow.

---

## High-severity issues (P1) — AUTH FOCUS

### P1-1 — Orphan: `/api/admin/connections/status/+server.ts` has no callers

**File:** [`frontend/src/routes/api/admin/connections/status/+server.ts`](../../../frontend/src/routes/api/admin/connections/status/+server.ts)

`grep -rn "connections/status"` across the entire frontend returns zero matches. This file is dead code — but as long as it's deployed it's reachable on the public internet (P0-3). The `BUILTIN_SERVICES` list (Fluent suite) and the `EXTERNAL_SERVICES` list (44 entries) duplicate the canonical truth that `/api/admin/connections` (the proxy) returns. Two sources of truth, one of them stale and unauthenticated.

**Action:** Delete this file. Per the user's CREATE-not-DELETE rule, alternative is to **convert it into a proxy** to the Rust backend (mirroring `+server.ts:16-25`) and add the auth guard.

---

### P1-2 — `users/[id]/+server.ts` falls back to in-memory writes on backend 5xx (DATA INTEGRITY)

**File:** [`frontend/src/routes/api/admin/users/[id]/+server.ts:79`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts)

```ts
} catch (_err) {
    return null;
}
```

Network error / 5xx response → `fetchFromBackend` returns `null` → PUT and DELETE handlers cheerfully manipulate the in-memory mock and report success. This silently desynchronizes admin UI state from the database. After a restart, the "saved" change vanishes. After a deploy, it vanishes. After failover, it vanishes.

This is the same root cause as P0-1 but framed for write paths.

---

### P1-3 — `users/create/+page.svelte` username/email checks fail-open

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:701-711, 727-738`](../../../frontend/src/routes/admin/users/create/+page.svelte)

```ts
} else {
    // On API error, assume available and let server validate on submit
    usernameAvailable = null;
}
} catch (error) {
    console.error('Failed to check username:', error);
    usernameAvailable = null; // Don't block user, server will validate on submit
}
```

When `usernameAvailable === null`, `validateForm()` line 570:
```ts
} else if (!usernameAvailable) {
    errors.push({ field: 'username', message: 'Username is already taken', severity: 'error' });
}
```

`!null === true` — so this branch fires and *blocks* legitimate submissions when the check endpoint is missing, while the comment claims the opposite ("Don't block user"). Both behaviors are wrong: when you don't know, you should let the server be the source of truth, but the validator's `!usernameAvailable` is buggy. Same defect for email at line 578.

**Action:** Change to `usernameAvailable === false` for both checks. Then test the create flow end-to-end.

---

### P1-4 — Connection credentials POST'd to `/connect` are stored opaque on the proxy hop

**File:** [`frontend/src/routes/api/admin/connections/[key]/connect/+server.ts:21-29`](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/connect/+server.ts)

```ts
const body = await request.json();
const res = await fetch(`${API_URL}/api/admin/connections/${key}/connect`, {
    method: 'POST',
    headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify(body)
});
```

The proxy reads the request body to JSON, re-stringifies it, and forwards. That's fine on the wire (HTTPS hop). Two latent issues:

1. **No payload size cap.** A malicious admin could submit a 100MB credential payload; SvelteKit will buffer it. Add `request.headers.get('content-length')` cap (e.g., 64KB).
2. **No per-key validation.** Any string for `params.key` is forwarded as-is to `${API_URL}/api/admin/connections/${key}/connect`. If the Rust router doesn't constrain `key` to `[a-z_]+`, a path-traversal-shaped key (`../auth/users`, percent-encoded `%2e%2e`) could in theory hit unintended routes. SvelteKit decodes path params so percent-encoding is partially mitigated, but this should still be defense-in-depth: validate `key` matches `/^[a-z][a-z0-9_]*$/` before forwarding.

Same notes apply to `/disconnect` ([disconnect/+server.ts:21](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/disconnect/+server.ts)).

---

### P1-5 — `site-health/+page.svelte` calls `/api/admin/site-health` and `/api/admin/site-health/run-tests` — neither exists as a proxy

**Files:**
- [`frontend/src/routes/admin/site-health/+page.svelte:154`](../../../frontend/src/routes/admin/site-health/+page.svelte) — `await adminFetch('/api/admin/site-health')`
- [`frontend/src/routes/admin/site-health/+page.svelte:261`](../../../frontend/src/routes/admin/site-health/+page.svelte) — `await adminFetch('/api/admin/site-health/run-tests', { method: 'POST' })`
- No directory `frontend/src/routes/api/admin/site-health/` exists.

`adminFetch` will hit the SvelteKit dev resolver and 404 (since there's no catch-all `[...path]` for `/api/admin/`). The page swallows the error in `try/catch` and silently shows zeroes ([line 158-162](../../../frontend/src/routes/admin/site-health/+page.svelte)):

```ts
} catch (error) {
    console.error('Failed to load health data:', error);
    healthData = getDefaultHealthData();
    scoreDisplay.set(0);
}
```

The page is non-functional. The "Run Tests" button does nothing useful (toast claims success only because `data` is `null`/undefined and the catch path runs).

Per CREATE-not-DELETE, build the proxy. Until then, this page should display "Coming Soon" rather than fake metrics.

---

### P1-6 — `+page.svelte` (admin/users): `$effect(() => { loadUsers(); })` is the wrong pattern for mount-only data

**File:** [`frontend/src/routes/admin/users/+page.svelte:14-16`](../../../frontend/src/routes/admin/users/+page.svelte)

```ts
$effect(() => {
    loadUsers();
});
```

Per recent commit `34a0bd070` ("replace $effect with onMount to resolve write-while-reading cascade issues"), this pattern was systematically migrated for CRM/campaigns. The same defect lives here: `loadUsers()` writes to `users`, `loading`, `error` state — all of which can become tracked dependencies of the parent component's reactive graph. `$effect` runs on every component re-instantiation including HMR; `onMount` runs once.

**Action:** Convert to `onMount`. Same fix to apply at:
- [`admin/users/edit/[id]/+page.svelte:24-26`](../../../frontend/src/routes/admin/users/edit/%5Bid%5D/+page.svelte)
- [`admin/users/create/+page.svelte:375-379`](../../../frontend/src/routes/admin/users/create/+page.svelte)
- [`admin/connections/+page.svelte:282-308`](../../../frontend/src/routes/admin/connections/+page.svelte)
- [`admin/settings/+page.svelte:707-718`](../../../frontend/src/routes/admin/settings/+page.svelte) (this one has cleanup so keep the cleanup but use onMount/onDestroy)
- [`admin/site-health/+page.svelte:131-146`](../../../frontend/src/routes/admin/site-health/+page.svelte)

The admin layout already corrected its own auth-guard $effect (see comments at lines 53-83) — those comments cite the exact pattern.

---

### P1-7 — `users/create/+page.svelte` has cascading `$effect` on form fields → infinite-loop risk

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:1057-1073`](../../../frontend/src/routes/admin/users/create/+page.svelte)

```ts
$effect(() => {
    if (formData.username) {
        checkUsernameAvailability();
    }
});

$effect(() => {
    if (formData.email) {
        checkEmailAvailability();
    }
});

$effect(() => {
    if (formData.password) {
        checkPasswordStrength(formData.password);
    }
});
```

`checkUsernameAvailability` writes to `usernameAvailable`, `checkingUsername`. `checkPasswordStrength` writes to `passwordStrength` *and* awaits `checkBreachDatabase`. Each effect fires on every keystroke and dispatches an unthrottled fetch. No debouncing — typing "billy" fires 5 username-availability checks. Combined with P1-3's fail-open broken validator, this is a flood-the-server pattern.

**Action:** Debounce input (300-500ms). Migrate to event-driven (`oninput` handler with throttle) rather than reactive `$effect`.

---

### P1-8 — Edit-user page sends raw passwords to the proxy as plaintext JSON without confirmation step

**File:** [`frontend/src/routes/admin/users/edit/[id]/+page.svelte:103-106`](../../../frontend/src/routes/admin/users/edit/%5Bid%5D/+page.svelte)

```ts
if (formData.password) {
    payload.password = formData.password;
    payload.password_confirmation = formData.password_confirmation;
}
```

No re-authentication step before changing another user's password. A super-admin's stolen session cookie → instant takeover of any account. Industry standard: require the operator to type their own password before submitting a password reset for a different user.

Additionally, the password input (`type="password"` line 235) has `autocomplete="current-password"` which is wrong (should be `new-password`); the confirmation field correctly has `autocomplete="new-password"` line 247 — inconsistent.

---

### P1-9 — No CSRF token on POST/PUT/DELETE proxies

**Files:** all `+server.ts` proxies in this audit

SvelteKit ships built-in CSRF protection that checks `Origin` against the host on `application/x-www-form-urlencoded`, `multipart/form-data`, and `text/plain` requests — but not on `application/json` (the bug-class CSRF protection here misses). Combined with the cookie-based auth (`rtp_access_token` is `httpOnly`, `sameSite: 'lax'` per [`hooks.server.ts:190-196`](../../../frontend/src/hooks.server.ts)), `lax` does protect top-level navigations but a `<form method="POST" enctype="application/json">` (or a `fetch` from a third-party origin) can theoretically still execute mutations.

`sameSite: 'lax'` is the practical mitigation here. But this audit recommends adding an explicit CSRF check for all `POST/PUT/DELETE` mutations in the system cluster, since these are the highest-impact: roles, password resets, payment-processor secrets.

---

## Medium issues (P2)

### P2-1 — Inconsistent auth-extraction pattern across proxies

The connections endpoints use the canonical pattern:
```ts
const token = cookies.get('rtp_access_token');
if (!token) error(401, 'Unauthorized');
```
([connections/+server.ts:17-18](../../../frontend/src/routes/api/admin/connections/+server.ts), [connect/+server.ts:18-19](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/connect/+server.ts), [disconnect/+server.ts:18-19](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/disconnect/+server.ts), [createProxyShim.ts:52-53](../../../frontend/src/lib/utils/createProxyShim.ts))

The users endpoints use the **fallback** pattern:
```ts
const cookieToken = cookies.get('rtp_access_token');
const headerToken = request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
const token = cookieToken || headerToken;
if (!token) error(401, 'Unauthorized');
```
([users/+server.ts:45-49](../../../frontend/src/routes/api/admin/users/+server.ts), [users/[id]/+server.ts:90-93](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts) — repeated 3×)

The fallback to header isn't bad per se (allows server-side internal callers), but the pattern divergence makes auditing harder. Either standardize on one (preferably cookie-only, since the SvelteKit hook is the only supported auth-token producer) or extract to a single helper.

---

### P2-2 — `users/+server.ts` POST returns generic error on 4xx, masking validation messages

**File:** [`frontend/src/routes/api/admin/users/+server.ts:87-89`](../../../frontend/src/routes/api/admin/users/+server.ts)

```ts
if (status >= 400) {
    error(status, 'Failed to create user');
}
```

Backend returns `{ errors: { email: ['already taken'] } }` and the proxy swallows it. The client sees only the status code with no field-level error mapping. Forward the body verbatim:

```ts
if (status >= 400) error(status as Parameters<typeof error>[0], JSON.stringify(data));
```

(The connection endpoints use `error(res.status as Parameters<typeof error>[0], ...)` which is closer to the right shape.)

---

### P2-3 — `Number(parseInt(params.id ?? '0'))` allows ID=0 to silently match nothing

**File:** [`frontend/src/routes/api/admin/users/[id]/+server.ts:86, 135, 183`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts)

`parseInt('foo')` → `NaN`; `parseInt('') ?? '0'` → `0`. Either of these passed to `fetchFromBackend('/api/admin/users/0')` is meaningless. Add validation:

```ts
const userId = parseInt(params.id ?? '');
if (!Number.isFinite(userId) || userId <= 0) error(400, 'Invalid user id');
```

---

### P2-4 — `connections/+server.ts:23, 30` cast `res.status` via `Parameters<typeof error>[0]` is unsafe

```ts
if (!res.ok) error(res.status as Parameters<typeof error>[0], `Backend returned ${res.status}`);
```

`error()` accepts only 4xx/5xx codes. Backend can return e.g. `502` and that's fine; but a malformed proxy that tries to forward `200` (`!res.ok` false branch is taken so this can't happen, but for safety on `connect/disconnect`) — the cast bypasses the type system. Wrap in:
```ts
const status: number = res.status >= 400 && res.status <= 599 ? res.status : 500;
error(status as 400 | 500, …);
```

---

### P2-5 — `connections/[key]/connect/+server.ts` does not pass through Backend's response status

**File:** [`frontend/src/routes/api/admin/connections/[key]/connect/+server.ts:31`](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/connect/+server.ts)

```ts
return json(await res.json(), { status: res.status });
```

This passes status through — but only when `res.ok`. When `!res.ok` (line 30) it raises `error()` which throws a SvelteKit `HttpError`. That's fine when the body is a SvelteKit error object, but the upstream JSON body (e.g. validation details) is discarded (only the status forwards). Match the shape of the user-listing endpoint instead — return the upstream JSON as-is.

---

### P2-6 — `admin/connections/+page.svelte:282-308` mixes `$effect` with async init that reads URL params before `connections` is populated

**File:** [`frontend/src/routes/admin/connections/+page.svelte:282-308`](../../../frontend/src/routes/admin/connections/+page.svelte)

```ts
$effect(() => {
    if (!browser) return;
    const init = async () => {
        await fetchConnections();
        const urlParams = page.url.searchParams;
        const connectServiceKey = urlParams.get('connect');
        if (connectServiceKey) {
            const serviceToConnect = connections.find((s) => s.key === connectServiceKey);
            …
        }
        …
    };
    init();
});
```

`page.url.searchParams` is reactive — reading it inside `$effect` makes the effect rerun on every URL change, re-fetching the entire connections list and possibly re-opening the connect modal. Combined with `selectedCategory` state mutations triggering URL updates elsewhere, this can loop.

**Action:** Move to `onMount` (P1-6) and use `untrack(() => page.url.searchParams)` if reactivity not needed.

---

### P2-7 — `admin/users/+page.svelte:129` iteration without `key` for each user row

**File:** [`frontend/src/routes/admin/users/+page.svelte:129`](../../../frontend/src/routes/admin/users/+page.svelte)

```svelte
{#each users as user}
```

After delete-and-reload, Svelte's keyed-each invariant breaks (rows shift indices). Should be `{#each users as user (user.id)}`. Same defect on roles `{#each user.roles as role}` line 151.

---

### P2-8 — `admin/users/create/+page.svelte` `prepareSubmitData()` injects `metadata.created_by: 'admin'` (string literal)

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:989-1000`](../../../frontend/src/routes/admin/users/create/+page.svelte)

Hard-coded `'admin'` instead of the actual creator's identity (`event.locals.user.id` server-side, or the auth store's user). Audit-trail rot: every user appears to have been created by "admin".

---

## Low / nits (P3)

### P3-1 — Duplicate `id="page-checkbox"` and `name="page-checkbox"` for both role checkboxes

**File:** [`frontend/src/routes/admin/users/edit/[id]/+page.svelte:262, 281`](../../../frontend/src/routes/admin/users/edit/%5Bid%5D/+page.svelte)

```svelte
<input id="page-checkbox" name="page-checkbox" type="checkbox" …/>
<input id="page-checkbox" name="page-checkbox" type="checkbox" …/>
```

Two checkboxes with the same `id` is invalid HTML. Screen readers and label clicks will misbehave. Use `id="role-admin"` / `id="role-super-admin"`.

### P3-2 — `usePhotoUpload` keeps a previously-uploaded file in memory (no revoke on retry)

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:894-898`](../../../frontend/src/routes/admin/users/create/+page.svelte)

`profilePhotoPreview` is set via `FileReader.readAsDataURL`. Re-uploading creates a new data URL string but the previous one stays in memory until GC. Minor memory leak. Use `URL.createObjectURL` and pair with `URL.revokeObjectURL` on replace.

### P3-3 — `uploadProfilePhoto` returns hard-coded `'https://example.com/photo.jpg'`

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:901-907`](../../../frontend/src/routes/admin/users/create/+page.svelte)

```ts
async function uploadProfilePhoto(file: File): Promise<string> {
    const formData = new FormData();
    formData.append('photo', file);
    // Mock upload - would be real API call
    return 'https://example.com/photo.jpg';
}
```

Every created user's `profile_photo` field is set to `https://example.com/photo.jpg`. Either build the upload endpoint or remove the field from `prepareSubmitData()`.

### P3-4 — `checkBreachDatabase` is a 5-entry list, not an actual breach DB

**File:** [`frontend/src/routes/admin/users/create/+page.svelte:798-801`](../../../frontend/src/routes/admin/users/create/+page.svelte)

```ts
async function checkBreachDatabase(password: string): Promise<boolean> {
    const commonPasswords = ['password', '123456', 'qwerty', 'admin', 'letmein'];
    return commonPasswords.includes(password.toLowerCase());
}
```

The component header (line 11) advertises "Breach database checking (HaveIBeenPwned)". The implementation is theatre. Either integrate the real HIBP k-anonymity API or rename the function and remove the marketing copy.

### P3-5 — `BUILTIN_SERVICES` in `connections/status/+server.ts` always reports `last_verified_at: new Date().toISOString()`

**File:** [`frontend/src/routes/api/admin/connections/status/+server.ts:34, 45, 56, 67, 78`](../../../frontend/src/routes/api/admin/connections/status/+server.ts)

`new Date().toISOString()` is evaluated at module-load time, **once**. For long-running serverless instances, "last verified" gets stuck at "when the worker first booted". Cosmetic but misleading.

### P3-6 — Hard-coded `created_at` / `updated_at` 2025 dates in `mockUsers`

**File:** [`frontend/src/routes/api/admin/users/[id]/+server.ts:28-53`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts)

If P0-1 isn't fully removed (which it should be), at least the dates should be relative.

### P3-7 — `PROD_BACKEND` const declared but assigned at module top-level — fine, but inconsistent capitalization

**File:** [`frontend/src/routes/api/admin/users/+server.ts:14, [id]/+server.ts:14`](../../../frontend/src/routes/api/admin/users/+server.ts)

`PROD_BACKEND` vs `API_URL` (in the connections proxies). Standardize on `API_URL`.

### P3-8 — `connections/+page.svelte:288-291` does linear search by key

```ts
const serviceToConnect = connections.find((s) => s.key === connectServiceKey);
```

Fine for ≤44 services, but if the catalog grows, build a `Map<key, Service>` derived state.

---

## Auth-pattern consistency check (re commit e2356fa46)

**Verdict: 65% consistent — acceptable but spot fixes needed.**

Commit message claimed: *"update authorization handling across admin API routes to prefer rtp_access_token cookie"*. Checking each proxy in this cluster:

| File | Pattern | Notes |
| --- | --- | --- |
| [`api/admin/users/+server.ts:42-49, 69-76`](../../../frontend/src/routes/api/admin/users/+server.ts) | cookie + header fallback | Documented FIX-2026-04-26 marker. Good. |
| [`api/admin/users/[id]/+server.ts:90-93, 140-143, 187-190`](../../../frontend/src/routes/api/admin/users/%5Bid%5D/+server.ts) | cookie + header fallback | Same pattern, repeated 3×. Should be a helper. |
| [`api/admin/users/[...rest]/+server.ts`](../../../frontend/src/routes/api/admin/users/%5B...rest%5D/+server.ts) → [`createProxyShim.ts:52`](../../../frontend/src/lib/utils/createProxyShim.ts) | cookie-only | No header fallback. |
| [`api/admin/connections/+server.ts:17, 29`](../../../frontend/src/routes/api/admin/connections/+server.ts) | cookie-only | Clean pattern. |
| [`api/admin/connections/[key]/connect/+server.ts:18`](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/connect/+server.ts) | cookie-only | Clean pattern. |
| [`api/admin/connections/[key]/disconnect/+server.ts:18`](../../../frontend/src/routes/api/admin/connections/%5Bkey%5D/disconnect/+server.ts) | cookie-only | Clean pattern. |
| [`api/admin/connections/status/+server.ts`](../../../frontend/src/routes/api/admin/connections/status/+server.ts) | **NONE** | P0-3: no auth at all. The commit missed this file. |

Two divergences from the pure cookie-only pattern:
1. The users endpoints add a header fallback. Defensible (allows server-to-server testing) but undocumented elsewhere — pick one and write it down.
2. `connections/status` has zero auth. The commit's title "across admin API routes" should have caught it; it didn't because nothing calls it (orphan, P1-1).

**Recommendation:** Extract a single helper:
```ts
// $lib/server/auth.ts
export function requireAdminToken(event: RequestEvent): string {
    const cookieToken = event.cookies.get('rtp_access_token');
    const headerToken = event.request.headers.get('Authorization')?.replace(/^Bearer\s+/i, '');
    const token = cookieToken || headerToken;
    if (!token) throw error(401, 'Unauthorized');
    return token;
}

export function requireSuperadmin(event: RequestEvent): { token: string; user: App.Locals['user'] } {
    const token = requireAdminToken(event);
    if (event.locals.user?.role !== 'super-admin') throw error(403, 'Forbidden');
    return { token, user: event.locals.user };
}
```
Use `requireSuperadmin` in users PUT/DELETE/POST and in connection mutations. Use `requireAdminToken` for read-only GETs.

---

## Summary

This cluster has the highest blast radius in the admin surface and is the **least defensible** of any audited cluster so far. Three categories of failure compound:

1. **Phantom data masquerades as real** — `users/[id]/+server.ts` ships a fake super-admin and silently accepts writes when the backend errors. Operators get green-toast feedback for changes that never happened. (P0-1, P0-2, P1-2)
2. **The proxy layer trusts the page layer** — there is no `super-admin` guard on role assignment, password reset, or connection-credential storage. The Rust backend has to enforce ACLs alone, with no defense-in-depth. (P0-2, P1-8)
3. **Several pages call non-existent endpoints** — `/api/admin/site-health`, `/api/admin/connections/{key}/test`, `/api/admin/users/check-username`, `/api/admin/organization/*`. The pages silently swallow errors and show fake/zero data. (P0-4, P0-5, P1-5)

A 4th systemic issue is `$effect`-driven mount logic that should be `onMount` (P1-6) — directly tracked in commit `34a0bd070` for sister surfaces but not yet applied here.

**Recommended sequence:**
1. Strip `mockUsers` from `users/[id]/+server.ts`. (P0-1, P1-2)
2. Add `requireSuperadmin` helper and apply it to all role/credential-mutating proxies. (P0-2, P1-4, P1-8)
3. Lock down or delete `connections/status/+server.ts`. (P0-3, P1-1)
4. Decide for each missing endpoint: build it or hide the UI. (P0-4, P0-5, P1-3, P1-5)
5. Sweep `$effect` → `onMount` migrations. (P1-6, P1-7)
6. Cleanup pass: P2/P3 nits.

Until items 1-3 land, the system cluster cannot ship to production.
