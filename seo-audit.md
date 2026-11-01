---
description: Evidence-gated technical SEO audit → implement → re-audit for a Svelte 5 / SvelteKit repo, against docs/seo/SEO_STANDARD.md
---

# MISSION

Audit **every route and every concrete URL** in this repository against `docs/seo/SEO_STANDARD.md`. Implement every missing or incorrect item end to end. Then re-run the **identical** audit and prove, with artifacts, that each requirement is PASS, N-A, or BLOCKED with a stated reason.

You are done only when the final matrix (Phase 8) has **zero FAIL rows** and every PASS row cites evidence produced **in this session**.

---

# INPUTS (fill in before running; if a value is blank, ask the owner before Phase 4)

```
PRODUCTION_ORIGIN   = https://                # canonical scheme+host, no trailing slash
PREFERRED_HOST      = apex | www
PREVIEW_URL         =                         # deployed preview, if any; else local `pnpm preview`
LOCALES             = en                      # comma list; first is base locale
NOINDEX_SECTIONS    = /account, /checkout, /admin, /search   # adjust
SITE_TYPES          = marketing, blog, video, membership-product, community-forum   # which templates exist
AI_SEARCH_OPT_OUT   = undecided               # owner decision — never decide this yourself
AFFILIATE_DOMAINS   =                         # broker/tool domains you're paid by → rel="sponsored"
GATED_SECTIONS      =                         # member-only routes that should still be indexed → paywall markup (SD-13)
AUTHORS             =                         # real authors + profile data source; never invent
APPS_IN_SCOPE       = all                     # for monorepos: list app directories, or "all"
```

---

# NON-NEGOTIABLE EVIDENCE RULES

These rules exist so that nothing in your output is unverified. Breaking any of them makes the run invalid.

1. **No claim without an evidence tag.** Every factual statement about the code, the site, a library, Svelte/SvelteKit, or Google must carry one of:
   - `[CODE path:line]` — a file and line you read in this session.
   - `[CMD]` — the exact command, its exit code, and the relevant output excerpt (trimmed, never paraphrased).
   - `[HTTP]` — URL, status, and the relevant headers/body excerpt from a request made in this session.
   - `[DOC url §section]` — documentation you **fetched in this session**: Svelte MCP `get-documentation`, or WebFetch for Google/npm/library docs.

   An untagged claim is treated as false.

2. **Memory is not evidence.** Never rely on your training data for an API name, a config option, a default value, a Google rule, or a package version. Fetch it. If you can't fetch it, mark the item `BLOCKED: unverifiable` and move on. Don't guess.
3. **Installed version wins.** Read `package.json` and the lockfile's **resolved** versions for svelte, @sveltejs/kit, the adapter, vite, and every SEO-related package. The Svelte MCP may serve docs ahead of the installed version (e.g. `$app/env` / `#lib` vs `$app/environment` / `$lib`). When docs and installed types disagree, read the installed package's `.d.ts` files in `node_modules`, cite them with `[CODE]`, and follow them.
4. **Read before you edit.** Never modify a file you haven't read in this session. Never reconstruct a file from memory.
5. **No fabricated content.** Never invent reviews, ratings, prices, author names, credentials, dates, or business facts to fill structured data or meta tags. If the data doesn't exist in the codebase/CMS, record `BLOCKED: needs owner data` with the exact fields required.
6. **No cheating the gates.** Don't delete or skip tests. Don't lower budgets or thresholds. Don't set prerender handlers to `'warn'` or `'ignore'`. Don't add `any`, `@ts-ignore`, `@ts-expect-error`, or `eslint-disable`. Don't mark something PASS because it "should" work.
7. **Tool failure is reported, not hidden.** If a command fails, show the error. Fix the cause, or mark BLOCKED. Never substitute an estimate.
   8a. **WARN status:** requirements labelled "WARN, not FAIL" (e.g. META-11 and META-13 title/description length) are reported as `WARN` with each offending URL and its character count. Fix them when the fix is just rewriting copy from content already on the page. Never pad or truncate text into meaningless copy to hit a number. Remaining WARNs don't block completion, but every one is listed in the final report.
8. **Things that can't be proven from the repo are never PASS.** These are always `REQUIRES-PRODUCTION` or `REQUIRES-HUMAN`, with exact steps for the owner:
   - field Core Web Vitals (CrUX)
   - Search Console status
   - Rich Results Test screenshots
   - Bing Webmaster verification
   - author credentials
   - the AI-search opt-out decision
9. **Business decisions belong to the owner.** Preferred host, locales, which sections are noindex, the AI opt-out, `Google-Extended`, and removing or redirecting any public URL: ask, then record the answer in `docs/seo/DECISIONS.md`.
10. **pnpm only. TypeScript strict. Zero errors, zero warnings.**

---

# PHASE 0 — PREFLIGHT (no edits yet)

1. Confirm `docs/seo/SEO_STANDARD.md` exists and read **all** of it. If it's missing, STOP and tell the owner.
2. Confirm the Svelte MCP is available (`list-sections` succeeds). If it isn't, tell the owner to run
   `claude mcp add -t stdio -s project svelte -- npx -y @sveltejs/mcp`
   and STOP.
3. Record the versions from rule 3 in `docs/seo/audit/00-environment.md`.
4. Run `pnpm install --frozen-lockfile`, `pnpm check`, `pnpm lint` (if defined), `pnpm test` (if defined), and `pnpm build`. Record exit codes and warning counts.
   - If the baseline build fails, fix **only** what blocks the build, in a separate commit labelled `chore: unblock build for SEO audit`, then continue.
5. Use `git status` to confirm the working tree is clean. Create the branch `seo/audit-<YYYY-MM-DD>`.

# PHASE 1 — EVIDENCE FETCH

For every requirement ID in the standard that is marked **†**, or whose source is `G:*`:

- Fetch the cited Google Search Central page (WebFetch).
- Record in `docs/seo/audit/01-sources.md`: the requirement ID, the URL, the fetch date, and a ≤2-sentence paraphrase of the governing rule.
- If the live doc **contradicts** the standard, the live doc wins. Flag the row `STANDARD-OUTDATED` and implement the live rule.

Then fetch, via Svelte MCP, every `SK:*` page the standard cites, plus `kit/configuration` (or its current equivalent), `kit/hooks`, `kit/load`, and `kit/routing`.

Also fetch the live Google structured-data **Search Gallery** and record which types are currently supported. This list governs SD-10.

# PHASE 2 — INVENTORY (every page, every site)

1. **Apps:** enumerate every SvelteKit app in scope (each `svelte.config.*`). Run Phases 2–7 per app.
2. **Routes:** glob `src/routes/**/+page.svelte`, `+server.ts|js`, `+layout*`, and `+error.svelte`. For each route record:
   - path pattern
   - params
   - `prerender` / `ssr` / `csr` / `trailingSlash` values (and which file sets them)
   - `load` type
   - whether it's indexable (per NOINDEX_SECTIONS and auth)
3. **Concrete URLs:** expand every dynamic route to its real param values from the actual data source: `entries()`, the CMS/DB query, or content files. Cite the source. If values can't be obtained, mark the route `BLOCKED: no param source`.
4. Write `docs/seo/audit/02-inventory.json` as an array of `{ app, route, url, indexable, template, paramSource }`. Template = the page type: home, article, video, product, forum-thread, profile, listing, static, etc.
5. Print the counts: routes, concrete URLs, indexable URLs, templates.

# PHASE 3 — BASELINE AUDIT (measure, don't fix)

Write the audit tooling as **committed, typed code** under `tests/seo/`. It becomes the permanent regression gate (MON-05). It must be fully implemented — no stubs, no TODOs.

### 3a. Crawler (Playwright + Node fetch, TypeScript strict)

Start from `/` plus every URL in `02-inventory.json` plus every `<loc>` in the sitemap(s). For **each URL**, capture into `docs/seo/audit/03-baseline.json`:

**Raw HTTP response (no JS):**

- final status
- full redirect chain with status codes
- headers: `content-type`, `x-robots-tag`, `cache-control`, `content-encoding`, `strict-transport-security`
- whether `<title>`, `<h1>` and the main text exist in the **raw** HTML

**From the SSR HTML:**

- character length of `<title>` and meta description; brand occurrences in the title; most-repeated term count
- counts **and** values of `<title>`, meta description, canonical, meta robots, every `og:*` and `twitter:*` tag, and hreflang links
- `html[lang]` and `html[dir]`
- h1 count and the heading outline
- every JSON-LD block, parsed (and parse errors)
- the visible breadcrumb text sequence
- the LCP-candidate `<img>` attributes
- every `<img>`'s `alt`, `width`, `height`, `loading`, `fetchpriority`

**From the rendered DOM:**

- every `<a>`: href, resolved URL, anchor text, and whether it's a real navigable href

**Derived:**

- final URL == canonical == sitemap loc
- duplicate titles and descriptions across the whole site
- orphan URLs (in the sitemap/inventory but never linked)
- click depth from `/`
- hreflang reciprocity
- og:image fetch status and content-type
- soft-404 detection: request `/<random-uuid>`, an unknown slug for **each** dynamic route, and ensure each returns 404

**Trust and spam signals (YMYL-_, SPAM-_, SD-13):**

- byline, author-page link, visible published/updated dates, and disclosure blocks on every article, alert or lesson URL
- the presence and crawlability of About, Contact, Editorial policy, Privacy, Terms and Refund pages
- every outbound link: domain, `rel` values, and whether it's in a UGC region (forum, comments, profiles) or matches an affiliate domain from the owner-supplied list
- paywall: whether gated content exists; `isAccessibleForFree`/`hasPart`/`cssSelector` JSON-LD vs the DOM class
- Googlebot user-agent vs browser user-agent response diff (cloaking check)
- hidden-text/link detection
- per-page term-frequency outliers
- near-duplicate similarity across the template pages
- grep `history.pushState`/`replaceState` outside SvelteKit's router

Content truth (credentials, accuracy, legal disclosures) is never PASS from the crawler: presence = PASS-STRUCTURE, and the truth check = REQUIRES-HUMAN.

**Variants:**

- each indexable URL with `?utm_source=x`, plus each sort/filter param found in the code → canonical must equal the clean URL
- with and without `Accept-Language` / locale cookie → main content must be identical (I18N-02)
- at viewports 390 px and 1440 px → text diff for HTML-05

**Also fetch:**

- `/robots.txt`
- every sitemap file
- the HTTP→HTTPS and non-preferred-host redirect chains (if a deployed URL exists)
- `PREVIEW_URL` headers for IDX-09

### 3b. Static checks

Write `docs/seo/audit/03-static.md` from grep/AST checks for:

- `ssr = false`
- `trailingSlash`
- prerender handler config
- Paraglide packages
- `$app/stores` usage
- the JSON-LD serializer's escaping
- `<enhanced:img>` vs `<img>` usage
- the `enhancedImages()` plugin order
- font preload in `handle`
- third-party `<script>` tags
- `changefreq`/`priority` emission

Cite `[CODE path:line]` for each finding.

### 3c. Lab performance and accessibility

- `pnpm build && pnpm preview` (or `PREVIEW_URL`).
- Run `pnpm dlx unlighthouse-ci --site <url> --build-static` across all routes.
- Run `@lhci/cli` on one URL per template.
- Run `@axe-core/playwright` on one URL per template.

Store the reports under `docs/seo/audit/03-lab/`.

**Field CWV:** if a production URL exists **and** the owner has provided a PSI/CrUX API key in the environment, query CrUX per template origin/URL. Otherwise, mark PERF-01 `REQUIRES-PRODUCTION`.

### 3d. Baseline matrix

Write `docs/seo/audit/03-matrix-baseline.md` with one row per **requirement ID × template**: status, the failing URLs (or the count plus 5 examples), and the evidence tag. URL-level requirements must be evaluated on **every** URL, not a sample.

# PHASE 4 — PLAN

Write `docs/seo/audit/04-plan.md`:

- each FAIL, the root cause `[CODE]`, the exact files to change, the change, and the requirement IDs it closes
- the order: build/indexing blockers → canonical/URL → metadata → structured data → sitemap/robots → i18n → performance → HTML/a11y
- every owner decision still open (rule 9), asked as **one** batched question set

Wait for the owner's answers on the decisions. Don't wait on the rest.

# PHASE 5 — IMPLEMENT

Implementation contracts (adapt the names to the installed versions you verified):

- **Config:** `src/lib/seo/config.ts` exports a typed, frozen `SITE` object: `origin` from `PRODUCTION_ORIGIN` via a `$env/static/public` variable, `name`, `alternateName`, `logo`, `sameAs`, `defaultOgImage`, `locales`. Nothing else builds absolute URLs.
- **Types:** a typed `SeoData` interface (title, description, canonicalPath, robots, og overrides, `jsonLd: Thing[]` using `schema-dts`, breadcrumbs), declared through `App.PageData` in `src/app.d.ts`, so every route's `load` is type-checked to return it (or inherit the defaults).
- **Head:** **one** `Seo.svelte` component rendered once in the root `+layout.svelte`. It reads `page.data` from `$app/state`, merges layout defaults with page overrides (deep merge for objects, explicit rules for arrays), and emits each tag exactly once. Using `svelte-meta-tags` v4+ instead is acceptable if META-09 is satisfied.
- **JSON-LD:** `src/lib/seo/jsonld.ts` builds an `@graph` with stable `@id`s. `serializeJsonLd()` escapes `<`, `>`, `&`, U+2028 and U+2029. Builders exist for every template type in SITE_TYPES — Organization, WebSite, BreadcrumbList, Article/BlogPosting, VideoObject, Product+Offer, DiscussionForumPosting, ProfilePage — but **only for types still in the live Search Gallery** or with a documented non-rich-result reason. Every field maps to real data shown on the page (rule 5).
- **Breadcrumbs:** a visible `Breadcrumbs.svelte` driven by the **same** `breadcrumbs` array that feeds the BreadcrumbList JSON-LD. It's one source, so DOM and markup can't drift.
- **Status codes:** `error(404)`, `error(410)`, and `redirect(308)` inside `load`, plus a typed redirect map for retired slugs.
- **Robots:** `src/routes/robots.txt/+server.ts` (prerendered). `X-Robots-Tag` set in `hooks.server.ts` for noindex sections and for every non-production host.
- **Sitemap:** `src/routes/sitemap.xml/+server.ts` (use the `sitemap[[page]].xml` pattern if over 50k URLs):
  - every dynamic route's param values, or an explicit exclusion
  - real `lastmod` from content timestamps
  - no `changefreq` or `priority`
  - hreflang alternates when there is more than one locale
- **i18n (if more than one locale):** Paraglide JS 2 via `pnpm dlx sv add paraglide`, remove `@inlang/paraglide-sveltekit`, URL strategy first, hreflang links plus `x-default`, and per-locale `lang`/`dir`.
- **Performance:** enhanced-img migration for local images, a CDN transform for remote images, LCP image priority, font subsetting and preload, third-party script reduction, `csr = false` where no JS is needed, and streaming for non-critical data.
- **Tests:** turn the Phase 3 crawler into `tests/seo/*.spec.ts` assertions — one assertion per requirement ID, named with the ID (e.g. `test('URL-02 canonical self-referencing', …)`). Add a unit test for `serializeJsonLd` covering the `</script>` injection case. Add `pnpm` scripts `seo:audit` and `seo:test`, and a CI job running `pnpm build`, `pnpm seo:test`, and `unlighthouse-ci --budget`.

Rules while implementing:

- After **every** changed `.svelte` file, run Svelte MCP `svelte-autofixer` until it returns zero issues.
- After each requirement group, run `pnpm check && pnpm lint && pnpm test && pnpm build` and record the exit codes.
- Commit once per requirement group, using conventional commits that list the requirement IDs closed (e.g. `feat(seo): canonical + robots (URL-02, URL-03, IDX-07, IDX-08)`).

# PHASE 6 — RE-AUDIT (identical method)

1. Re-run **exactly** the Phase 3 tooling, unchanged except for bug fixes to the tooling itself (disclosed and justified), against a fresh `pnpm build`.
2. Write `docs/seo/audit/06-matrix-final.md` and `06-diff.md`. The diff shows every row's baseline status → final status.
3. Any row still FAIL means you return to Phase 5 for that row. Loop until no FAIL remains or the row is legitimately BLOCKED.
4. **Independent verification.** Spawn a fresh subagent with **no** access to your reasoning. Give it only the standard, the raw artifacts (`03-*.json`, `06-*.json`, the lab reports, test output), and the repo. Instruct it to:
   - re-derive the status of **every row that was FAIL at baseline**, plus a random 20% of the other rows, from the raw artifacts and fresh commands;
   - report every disagreement with evidence.

   Resolve every disagreement with new evidence before continuing.

# PHASE 7 — STANDARDS FOR FUTURE PAGES (AGENTS.md + CLAUDE.md)

Write the conventions you **actually implemented and verified** into the repo, so every future page follows them without guessing.

1. **`AGENTS.md` at the repo root.** If one already exists, read it fully first, keep every existing instruction that doesn't conflict, and list any conflicts in the report. Keep it under ~300 lines. Link to `docs/seo/SEO_STANDARD.md` instead of copying it. It must contain:
   - **Verified stack:** the exact installed versions of svelte, @sveltejs/kit, the adapter, vite and typescript (from the lockfile), plus a version-specific API note (e.g. `$app/environment` vs `$app/env`).
   - **The SEO contract,** with real file paths:
     - `src/lib/seo/config.ts` (SITE)
     - the `SeoData` type and how `load` returns it
     - `Seo.svelte` (the only place head tags are emitted)
     - `jsonld.ts` builders and `serializeJsonLd`
     - `Breadcrumbs.svelte` and its shared breadcrumbs array
     - the robots/noindex mechanism
     - how a new route enters the sitemap (`paramValues` / exclusions)
     - Paraglide usage, if multilingual
   - **Forbidden patterns:**
     - a second metadata system or a raw `<meta>` in pages
     - hardcoded origins
     - `ssr = false` on indexable routes
     - `changefreq`/`priority`
     - retired schema types (SD-10)
     - invented structured-data values
     - `$app/stores`
     - `any` / `@ts-ignore`
     - lowering test budgets
   - **New-page checklist:** a numbered list where every line cites requirement IDs, e.g.:
     1. Route options set (IDX-01, IDX-03, URL-01)
     2. `load` returns `SeoData` with unique title and description (META-01/02/10–13)
     3. canonicalPath (URL-02)
     4. breadcrumbs (SD-05)
     5. JSON-LD builder for the template type (SD-*)
     6. byline, dates and disclosures for YMYL content (YMYL-*)
     7. images via `<enhanced:img>` or CDN, with LCP priority (PERF-02/03/04)
     8. outbound and UGC `rel` attributes (SPAM-05/06)
     9. added to the sitemap or explicitly excluded (MAP-02)
     10. a Playwright SEO spec covers the new template
     11. gate commands pass
   - **Gate commands:** the exact pnpm scripts that must exit 0 before any commit.
2. **`CLAUDE.md` at the repo root.** Claude Code reads `AGENTS.md` only when no `CLAUDE.md` exists. If `CLAUDE.md` is missing, create it containing `@AGENTS.md` on the first line. If it exists, add `@AGENTS.md` as its first line and keep the rest unchanged.
3. **Verify the file itself:**
   - every path in `AGENTS.md` exists: `[CMD] test -e <path>` for each
   - every command it lists runs and exits 0: `[CMD]`
   - every requirement ID it cites exists in the standard

# PHASE 8 — FINAL REPORT

Write `docs/seo/audit/08-report.md` containing exactly:

1. **Final matrix:** requirement ID × template → PASS / PASS-STRUCTURE / WARN / N-A / BLOCKED / REQUIRES-PRODUCTION / REQUIRES-HUMAN, each with an evidence tag. Include a WARN table: URL, title length, description length.
2. **Counts:** URLs audited, rows per status, baseline FAIL count → final FAIL count (which must be 0).
3. **Commits:** a list mapping each commit to the requirement IDs it closed.
4. **STANDARD-OUTDATED items**, with the live-doc citations.
5. **Owner action list:** every REQUIRES-HUMAN, REQUIRES-PRODUCTION, and BLOCKED item, each with the exact next step and URL. This covers:
   - Search Console: verify the property, submit the sitemap, run URL Inspection per template, record the AI opt-out decision
   - Rich Results Test per template
   - CrUX re-check 28 days after deploy
   - author credentials
   - missing business data
6. **Gate commands** with their final output: `pnpm check`, `pnpm lint`, `pnpm test`, `pnpm seo:test`, `pnpm build`, and `unlighthouse-ci`, each with exit code 0 and 0 warnings.

Don't summarize beyond this report. Don't claim the site will rank #1. Report what is technically verified.
