# CLS probe (dashboard layout-shift measurement)

Reproducible, measured evidence for Cumulative Layout Shift on `/dashboard`.
Local dev tool — not part of the app build or CI.

## Why
`/dashboard` and its sidebar load membership data; if that data arrives after
first paint the nav/footer reflow and produce CLS. This harness measures the
**real** shift via the browser Layout Instability API (the same metric Google
uses), instead of guessing.

## Run
```bash
# 1) mock backend on :8080 (auth/me, memberships, watchlist)
node .cls-probe/mock-backend.mjs &

# 2) dev server pointed at the mock
API_BASE_URL=http://localhost:8080 BACKEND_URL=http://localhost:8080 \
VITE_API_URL=http://localhost:8080 VITE_API_BASE_URL=http://localhost:8080 \
pnpm dev &

# 3) measure (prints totalCLS + the top shift sources with the DOM node + y-delta)
node .cls-probe/measure-cls.mjs http://localhost:5173 my-run
```

## Measured result for the dashboard sidebar CLS fix
- **Before** (sidebar nav populated by a client `onMount` fetch): `totalCLS = 0.1215`
  — dominant source: the sidebar `<ul>` growing + the footer jumping 656px.
- **After** (membership list pre-fetched in `dashboard/+layout.server.ts` so the
  sidebar renders on first paint): `totalCLS = 0.0016` — ~76× lower, inside
  Google's "good" (<0.1) threshold. Remaining shift is a negligible sub-pixel
  top-nav nudge unrelated to the dashboard body.
