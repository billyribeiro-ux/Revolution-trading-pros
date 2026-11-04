# Private rendering and static preview policy

Scope: source build/authentication repair only. The source-wide admin completion gate remains open.

Private pages must reach runtime authentication. The auth hook skips runtime checks during Kit's build phase so the prerender crawler can inspect each route's `prerender=false` setting. It never writes a login redirect as private static HTML. `/my` now explicitly uses runtime rendering. Existing create/billing links have real authenticated307 aliases with query preservation.

Authentication uses validated `/api/auth/me` responses. Every identity and refresh request has a10s bound. Malformed responses, transport errors, transient refresh errors and timeouts produce503/no-store/noindex with Retry-After5, retaining cookies; they cannot authorize from decoded JWT claims. Explicit credential denials produce a private303. Kit3 data/action redirects preserve the redirect envelope, version and private headers, including a freshly renewed cookie.

Cloudflare's routing planner uses actual written assets and exact public HTML paths, leaving data/private/unknown requests in the worker. It fails if the combined routing budget is exceeded, rather than silently dropping exclusions. The current build uses88of100rules. No broad marketing wildcard or weakened prerender handler is used. See [Cloudflare routing](https://developers.cloudflare.com/pages/functions/routing/).

Static assets bypass hooks. The adapter therefore preserves generated/user cache headers and appends noindex rules from the shared SEO configuration for localhost, loopback, Pages preview hosts and existing noindex routes. These rules leave production marketing pages indexable. Both header count and line length are checked. See [Cloudflare static headers](https://developers.cloudflare.com/pages/configuration/headers/).

Actual evidence:

- Production private rendering: `tests/admin/private-rendering.spec.ts`,1passed3.8s on build8, `/tmp/admin-move-private-rendering-browser4.log`. Anonymous HTML/data/enhanced-action denial, ordinary-user denial, privileged rendering, aliases and real subscriber UI; strict console/API checks.
- Real identity outage plus controlled refresh/identity failures: `tests/admin/private-rendering-failure.spec.ts`,2passed10.4s, `/tmp/admin-move-private-rendering-failure2.log`. Includes malformed identity, missing user, rejected refresh, unavailable refresh, actual10s timeout, follow-up identity denial/error, retained renewed cookies and ordinary-role rejection despite privileged flags. The disposable API was restored in finally.
- Actual local Pages static serving: `tests/admin/cloudflare-static-headers.spec.ts`,1passed373ms, `/tmp/admin-move-cloudflare-headers-browser1.log`. All socket connections use127.0.0.1:5276; simulated Host headers verify localhost/IPv6/Pages-preview rules, production marketing indexability, internal demo noindex, runtime private redirect and genuine404. This is a local emulator proof, not a deployment.
- Build8 and root `vp build`: exit0,88rules/no truncation warning. The Pages emulator parsed31valid header rules. SEO suite114passed20.5s, `/tmp/admin-move-private-seo10.log`. Frontend87files/2850passed; root88files/2858passed;32existing skips in each. Svelte0errors/0warnings, fullESLint0, root check0warnings, `vp pack`0. These results precede the next Connections source changes.

Reproduce positive production tests against the isolated preview on5275. Run the failure spec only with the task's disposable API8281 offline and restore that API in finally. Run the static header spec only against the isolated Pages emulator5276 with fixture configuration and an empty environment file. Never stop an unidentified listener or point these tests at production.

The original positive attempts1/2 failed missing redirect headers; those failures led to the explicit Kit3 response helper. They are not passing evidence. Source editor boundaries remain55originals/85integrations/15approved warning exceptions/0unexpected changes.
