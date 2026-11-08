# About and Explosive Swings audit — 2026-11-05

## Follow-up

The user subsequently expanded the audit to every marketing page and authorized Git delivery. The [expanded audit](../2026-11-05-all-marketing/REPORT.md) supersedes this report's pending SEO and delivery status. A real isolated Rust API resolves the earlier environment mismatch; no sitemap fallback was added.

## Result and scope

Five confirmed defect groups were repaired: About decoration covering shared UI, enlarged-text clipping, an inaccessible/clipped billing-plan selector, motion preference violations, and blocked integration requests. Six browser regression cases failed before implementation and pass against the repaired Cloudflare production build. Full SEO validation is **not green**: 112 cases pass, two sitemap cases fail because the configured publication API returns 404. No deployment, push, merge, purchase, message, trade, or account mutation was performed.

## Environment and revision

- Initial worktree: clean, HEAD `5fbdb06c6e15501280df4b95e2c3e855f743048f`.
- Deployed `/_app/version.json`: `0e3678098bd0f531b63882aec315817faa3e4e66`. This is one ancestor commit behind HEAD. `git diff <deployed> HEAD` changes only credential-test retention, its progress notes, and changelog; **no frontend drift** exists between these two revisions.
- Installed: Node 26.13.0, pnpm 12.16.1, Svelte 5.57.3, SvelteKit 3.1.0, Playwright/Chromium 1.64.1. The monorepo uses Vite Plus 1.4.0, a SvelteKit frontend, Rust/Axum API, and Cloudflare Pages adapter. Kit 3 configuration is in `frontend/vite.config.ts`; no `svelte.config.js` exists.
- Both requested pages use prerendered SSR and local content. Their shared root layout owns navigation, breadcrumbs, footer, marketing tokens, auth initialization, and SEO rendering. Pricing selection highlights one of three existing cards; all membership links intentionally continue to the existing `/pricing` page. No product-specific checkout or new prices were invented.
- Deployed anonymous Chromium contexts; local **actual production build** through `pnpm --filter revolution-svelte preview --host 127.0.0.1 --port 4174 --strictPort`. Port 4173 was occupied by an unrelated Python server; its 404 was an environment mistake, not an application finding. The audit used 4174 after identifying the listener, without stopping the unrelated server.
- macOS sandbox denies Chromium's Mach-port startup and loopback binding. Approved tool escalations were used for browser/test servers. No credentials or environment secrets were copied into artifacts.
- Live `X-Robots-Tag: noindex, nofollow` is expected for the preview hostname. Live `/api/auth/me?optional=1` temporarily redirects to maintenance; this is recorded as an operational observation, not classified as a page defect or bypassed. Localhost intentionally bypasses maintenance in the existing hook.

## Finding register

All repaired causes below are confirmed by source and reproducible browser behavior. Severity P2 means a user-visible or accessibility defect without a demonstrated loss of account/data integrity.

### F1 — P2 — About background covers breadcrumb; ticker is hidden behind navigation

**Reproduce:** Direct-load `/about` at 390×560 (also visible at desktop widths). Compare with `/alerts/explosive-swings`: the About breadcrumb is present in DOM but painted over. Its ticker is at viewport y=0..40, underneath the shared navbar.

**Expected:** The visible breadcrumb required by `AGENTS.md`/SEO contract remains readable, and route-local decoration stays within its page. A market ticker must be visible to be useful.

**Confirmed cause:** `.about__atmosphere { position: fixed; inset: 0 }` paints outside the route because `.about` creates no stacking context. `.ticker { position: fixed; top: 0; z-index: 50 }` conflicts with the navbar's sticky, z-index 1000 contract. At 390×560, the atmosphere covered y=0..560 while the breadcrumb occupied y=74..112.1875. The ticker's hit test returned the navbar logo.

**Repair:** In `frontend/src/routes/about/+page.svelte`, isolate the route's stacking context, make atmosphere absolute within the positioned route, and put the ticker into normal document flow. No arbitrary navbar offsets or global z-index escalation. Label existing hardcoded ticker values as illustrative; hide only the repeated copy from assistive technology. Provide a keyboard-operable Pause/Resume toggle for the continuous ticker.

**Acceptance:** The decoration's bounds stay within `.about`; the ticker starts after both navbar and breadcrumb. The regression case also toggles pause with Space and checks the animation's actual paused state.

**Artifacts:** [before header](artifacts/before-about-header.png), [after at 320px](artifacts/after-about-320.png), [baseline geometry](artifacts/controlled-comparisons.json), [before full page](artifacts/before-about-1280.png), [after full page](artifacts/after-about-1280.png).

### F2 — P2 — Heading text disappears at 200% text size on both pages

**Reproduce:** Set viewport to 390×560, load either route, then set the root font size to 32px (200% of its 16px baseline). Inspect actual text rectangles, not only document scroll width. On About, heading words extend past its clipped route boundary; on Swings, intrinsic grid sizing makes the hero wider than the available space.

**Expected:** Text remains available and readable when enlarged, consistent with WCAG text-resizing/reflow expectations and the existing responsive design.

**Confirmed cause:** Large headings contain words wider than the available content width without emergency wrapping. The Swings grid also has automatic intrinsic sizing. About's h1 scroll width was 456px for a 279px box; Swings' h1 was 527px wide on a 390px viewport. Ancestor clipping hides the symptom from a document-overflow-only check.

**Repair:** Allow emergency wrapping within both route roots. Give the Swings hero grid an explicit available width and `minmax(0, 1fr)` columns at its existing desktop breakpoint. Existing typography sizes, content-width tokens, and breakpoints are retained.

**Acceptance:** Two originally failing regressions measure rendered h1 text rectangles at 200%. After: About h1 scroll width 279px for a 279px box; Swings 311px for a 311px box. An additional scan of actual text nodes in main headings, paragraphs, links and buttons reports **zero offscreen copy** on both routes. Decorative shimmer element boxes were excluded after a controlled measurement showed they were false positives.

**Artifacts:** [before About enlarged](artifacts/before-about-large-text.png), [after About enlarged](artifacts/after-about-large-text.png), [before Swings enlarged](artifacts/before-swings-large-text.png), [after Swings enlarged](artifacts/after-swings-large-text.png), [final text-node measurements](artifacts/after-interactions.json).

### F3 — P2 — Billing-plan selector clips at 320px and has no accessible state

**Reproduce:** On `/alerts/explosive-swings` at 320px, scroll to Simple Pricing; select Monthly, Annual, Quarterly by keyboard. The original row's first button starts at x=-3.98; its last button ends at x=324. A screen reader receives no selected state after activation.

**Expected:** Each control remains within the viewport and programmatically exposes which billing plan is highlighted. This is a group of highlight toggles, not a tab interface with hidden panels.

**Confirmed cause:** Fixed horizontal padding and an intrinsic `inline-flex` selector exceed the narrow container. A separately positioned percentage-based indicator assumes equal widths despite unequal intrinsic button widths. Buttons do not expose `aria-pressed`.

**Repair:** In `frontend/src/routes/alerts/explosive-swings/_sections/PricingSection.svelte`, use a bounded equal-column grid with responsive padding, apply the existing green selected treatment to the button itself, and add a named group and mutually exclusive `aria-pressed` values. Remove the inaccurate floating-indicator positioning calculations.

**Acceptance:** Original failing regression now activates all three controls with Space, verifies their bounds, and asserts exactly one pressed button. Card content and membership destinations remain intact. The final viewport matrix has no offscreen main headings, links, paragraphs, or controls at its sampled widths.

**Artifacts:** [before pricing](artifacts/before-pricing-390.png), [after pricing](artifacts/after-pricing-390.png), [before 320px measurements](artifacts/before-browser.json), [after measurements](artifacts/after-browser.json).

### F4 — P2 — Both pages ignore reduced motion in several animation layers

**Reproduce:** Enable `prefers-reduced-motion: reduce` before direct entry, then inspect/scroll each page and operate a FAQ. Also change the preference while the page is open. Before: About runs ticker, ping, scanner and pulsing icon animations; Swings runs ping and channel pulses. About parallax and Svelte transitions are not preference-aware. Swings checks the preference only once for GSAP.

**Expected:** Nonessential motion is suppressed for the requested preference, including CSS, Svelte Web Animations, and GSAP. The ticker has a keyboard pause control when motion is permitted.

**Confirmed cause:** The global reset only changes scrolling. Page CSS animations and Svelte transitions are independent of that reset; the one-time Swings GSAP check cannot respond to preference changes. One SVG icon's more-specific descendant rule also beat the first reduced-motion patch; its failing regression was retained and fixed.

**Repair:** Route-scoped reduced-motion CSS covers descendant components and the SVG selector. Svelte transitions use reactive `prefersReducedMotion.current`. GSAP setup uses `matchMedia` with automatic reversion when preference changes and on route unmount. Swings batch targets are scoped to the captured page and callback-created tweens are registered with the owning context for cleanup.

**Acceptance:** Two originally failing regressions now see no running infinite animations on direct entry or after preference changes, and no running FAQ transition when reduced motion is active. Final browser capture also records zero running animations for both reduced-motion pages. Window/document listener counts remain identical across six complete About → Alerts hub → Swings cycles; this is evidence of stability for this sequence, not a heap-leak proof.

**Artifacts:** [before animation inventory](artifacts/before-browser.json), [after animation inventory](artifacts/after-browser.json), [listener counts](artifacts/after-interactions.json), [regression before](artifacts/regression-before.log), [regression after](artifacts/regression-after.log).

### F5 — P2 — CSP blocks stylesheet preloading and Cloudflare beacon requests

**Reproduce:** Navigate among the pages in an anonymous deployed browser and inspect console/network. Google Fonts stylesheet preload connections are blocked by `connect-src`, followed by a preload warning. The deployed Cloudflare beacon's request to `cloudflareinsights.com/cdn-cgi/rum` is also blocked.

**Expected:** Configured integrations can use their specific required connection endpoints. A stylesheet's permitted `style-src` does not automatically permit script-initiated fetch/XHR connections.

**Confirmed cause:** `frontend/vite.config.ts` allows Google Fonts under `style-src`, and the Cloudflare script host under `script-src`/`connect-src`, but omits the actual Google Fonts connection host and Cloudflare collection host from `connect-src`.

**Repair:** Add exactly `https://fonts.googleapis.com` and `https://cloudflareinsights.com` to the existing connection allowlist. No wildcards or disabled CSP enforcement were introduced.

**Acceptance:** Identical local production browser navigation originally reproduced the Fonts CSP error; the final full navigation matrix reports zero console errors/warnings and zero uncaught exceptions. The original live beacon failure and the corrected generated policy establish the collection-host mismatch; **successful Cloudflare beacon delivery is not claimed**, because the local preview does not inject that deployed integration and no deployment was authorized. Font stylesheet preload failure did not establish that the original normal stylesheet load or all fonts failed.

**Artifacts:** [live requests and console](artifacts/live-browser.json), [local baseline console](artifacts/before-browser.json), [final console](artifacts/after-browser.json).

## Integrated validation

The paired baseline/final browser script captures direct entry, reload, three repeated internal navigation/history cycles, rendered screenshots, DOM geometry, console/network failures, reduced motion, 200% root text size, FAQ controls, and plan highlights. Live and baseline widths: 320, 390, 639/640, 767/768, 1023/1024, 1151/1152, 1280, 1920. Final production matrix adds 1279/1280, 1439/1440 and 1535/1536 to cover additional navbar boundaries. Short viewport: 390×560. Readiness is bounded on rendered content, font readiness and animation frames; no `networkidle` dependency.

Extended verification exercises skip-link focus into main, mobile menu focus and Escape, all four About and twelve Swings FAQs by keyboard, both explanatory anchor links, 48 collected internal link checks (all return 200), registered service worker, and six route cycles with listener inventories. An initial Escape probe ran before opening focus settled; after a bounded focus-readiness check, both menus close correctly. That initial timing observation is retained, not reported as a confirmed menu defect. No missing/duplicated content or hydration exception was confirmed in these tested public flows. Serif fallback on About is intentional in its design token and was not redesigned.

| Command/check                                      | Result                                                                                                                                    |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `vp check`                                         | Pass, zero formatting/lint/type diagnostics after formatting new audit evidence.                                                          |
| `vp test` (`CI=true` to avoid watch mode)          | Pass: 137 files, 3,078 tests; 32 existing skips.                                                                                          |
| `vp build`                                         | Pass, Cloudflare output generated; 87/100 routing rules.                                                                                  |
| `vp pack`                                          | Pass; regenerated tracked bundle restored because no package source changed.                                                              |
| `pnpm --filter revolution-svelte check`            | Pass: zero errors, zero warnings.                                                                                                         |
| `pnpm --filter revolution-svelte lint`             | Pass: final full run exits 0 with zero warnings. Three test-only non-null-assertion warnings in the initial run were corrected.           |
| `pnpm --filter revolution-svelte test:unit`        | Pass: 136 files, 3,070 tests; 32 existing skips.                                                                                          |
| `pnpm --filter revolution-svelte build`            | Pass, including final SVG reduced-motion correction.                                                                                      |
| `FRONTEND_URL=http://127.0.0.1:4174 pnpm test:seo` | **Fail**: 112 pass, two sitemap tests fail on upstream-backed 404 responses. Tests/thresholds unchanged.                                  |
| New `tests/marketing-pages.spec.ts`                | Six fail on baseline; six pass against repaired production build.                                                                         |
| Focused existing SEO cases matching both routes    | 16 pass, including metadata, semantics/axe, SSR parity, links, images and canonicals.                                                     |
| Svelte MCP autofixer                               | All three changed Svelte files: no issues or suggestions.                                                                                 |
| Final browser matrix                               | No audit exception, uncaught page exception, console error/warning, measured offscreen main controls/copy, or main-content axe violation. |

The environment emits Node's `NO_COLOR`/`FORCE_COLOR` conflict notice in some child processes; these are tool-environment notices, not Svelte compiler or browser warnings. Full command logs are retained under `artifacts/`. A passing subset is not substituted for the failing full SEO gate.

## Remaining limits and operations

1. **Publication backend mismatch:** `/sitemap.xml` returns 404 with `Public publication not found`; `/api/seo/sitemaps/news?per_page=1000` returns 404. The unchanged `seo-publications.ts` throws this response when its configured upstream returns 404, and source code registers publication routes in `api/src/routes/seo_sitemaps.rs`. Completing this gate requires a running API revision/configuration supporting the public article/video catalog and sitemap endpoints with its real database. Backend identity and repair are outside this two-page patch; no success-shaped sitemap fallback or fabricated source fixtures were added. See [full gate failures](artifacts/seo-full.log).
2. Deployed pages remain unchanged. Live maintenance behavior, Cloudflare telemetry delivery, and deployment cache invalidation were not retested after implementation. Local production browser results are explicitly separate from the original live evidence.
3. No authorized authenticated test account was supplied or needed for these public pages. Authenticated toolbar/user-menu states, purchases, account actions, and trading operations were not exercised. No authentication bypass was attempted.
4. Chromium only; no claim of Safari/Firefox/physical-device, assistive-technology speech-output, offline upgrade, quantitative CWV profiling, or exhaustive memory-leak coverage. Existing performance and image checks for both route templates pass. Business claims, testimonial provenance, and commercial pricing authority require maintained business records and were not invented or revalidated here.

## References and replay

- [Svelte transitions and reduced motion](https://svelte.dev/docs/svelte/transition), [reactive motion preference](https://svelte.dev/docs/svelte/svelte-motion#prefersReducedMotion).
- [GSAP matchMedia cleanup](<https://gsap.com/docs/v3/GSAP/gsap.matchMedia()/>).
- [MDN stacking isolation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/isolation), [CSP connection directives](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/connect-src), [toggle-button state](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-pressed).
- [WCAG resize text](https://www.w3.org/WAI/WCAG22/Understanding/resize-text.html), [pause, stop, hide](https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html).

Start the existing production preview on 4174, then run:

```sh
AUDIT_ORIGIN=http://127.0.0.1:4174 AUDIT_LABEL=replay node docs/audits/2026-11-05-marketing-pages/browser-audit.mjs
node docs/audits/2026-11-05-marketing-pages/interactions.mjs
FRONTEND_URL=http://127.0.0.1:4174 pnpm --filter revolution-svelte exec playwright test tests/marketing-pages.spec.ts --project=chromium
```

`artifacts/after-trace.zip` can be opened with Playwright's trace viewer. It captures anonymous public rendering/network activity. Redundant recordings and screenshots were removed during the expanded marketing audit; the cited before/after evidence is retained. Original negative regression output is retained independently from later test-run reports.
