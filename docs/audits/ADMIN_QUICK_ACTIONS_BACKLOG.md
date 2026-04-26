# Admin Quick Actions — Missing Routes Backlog

**Effective:** 2026-04-25 · **Source:** `frontend/src/routes/admin/+page.svelte` Quick Actions panel

The Quick Actions tile grid on `/admin` exposes 12 shortcuts. Six of them point
to dedicated routes that **don't exist yet**. As of this date, those tiles are
aliased to the closest existing surface so the user never lands on a 404.

Each row below tracks the tile's intended destination, the alias it currently
uses, and the work required to give it a real home.

---

## Aliased tiles — current behaviour

| Tile | Intended route | Alias (current) | Notes |
|------|----------------|-----------------|-------|
| Notifications | `/admin/notifications` | `/admin/popups` | Popups overlap conceptually (in-app overlays). A dedicated push/email/SMS notification centre is owed. |
| Links | `/admin/links` | `/admin/seo` | SEO already covers redirects and internal-link management; a standalone link manager is owed once we ship UTM tagging + short links. |
| Filters | `/admin/filters` | `/admin/categories` | Filters and categories share data plumbing today. Once filter rules diverge from taxonomy, split this. |
| Broadcast | `/admin/broadcast` | `/admin/email` | Broadcast is a special-case email send. Promote to its own route when we add SMS / push channels. |
| Global | `/admin/international` | `/admin/settings` | i18n / locale config currently lives under settings; carve out its own surface when locale count > 2. |
| Alerts | `/admin/alerts` | `/admin/indicators` | Trading alerts and indicators share state today. Once user-facing alert subscriptions ship, give them a dedicated admin page. |

---

## Already-real tiles (no work needed)

`/admin/email`, `/admin/forms`, `/admin/media`, `/admin/videos`,
`/admin/categories`, `/admin/seo` — all routes exist and resolve.

---

## How to use this file

When you build one of the missing routes, update
`frontend/src/routes/admin/+page.svelte` so the tile points to the real path,
and remove the matching row from the table above. Don't drop a tile from the
grid — the icon set was sized for 12.
