# Analytics Dashboard — Forensic End-to-End Audit

**Auditor:** Claude (Opus 4.7), Distinguished Principal Engineer (ICT 7+).
**Date:** 2026-04-25
**Scope:** `frontend/src/routes/admin/analytics/**`, `frontend/src/routes/admin/dashboard/+page.svelte`, `frontend/src/lib/components/analytics/**`.

---

## Executive summary

| Dimension | Current | Linear/Vercel bar | Gap |
|-----------|---------|-------------------|-----|
| Data freshness | 60-second polling | 5-10s + WebSocket | high |
| Chart library | hand-rolled SVG | Recharts / ECharts | **critical** |
| KPI cards | flat + GSAP entrance only | 3D tilt + sparklines + micro-interactions | medium |
| Icon system | `@tabler/icons-svelte-runes` (partial) + inline `<svg>` mixed in | `@tabler` exclusively, sized via tokens | medium |
| Real vs. stubbed | Real-data-first, but Goals/Funnels/Cohorts APIs stubbed | 100% real | high |
| Drill-down | Funnels + Cohorts only | every KPI clickable | medium |

**Grade: C+ → B with the recommendations below.**

---

## 1. Real vs. mock vs. placeholder

### Real (production-grade)
- `frontend/src/routes/admin/dashboard/+page.svelte` — admin home. Live polling, real connections, real currency formatting:
  - `loadDashboard()` polls every 60 s (line 178)
  - Connection status from `/api/admin/connections/summary` (line 65)
  - Stripe MRR pulled if `connections.stripe === true`
  - GA visitors pulled if `connections.google_analytics === true`
- `frontend/src/lib/components/analytics/KpiCard.svelte` — production-grade, GSAP-animated, viewport-intersection-aware (lines 51-62).

### Stubbed (UI complete, backend pending)
| Surface | File:line | Symptom |
|---------|-----------|---------|
| Goals | `routes/admin/analytics/goals/+page.svelte:79` | Inline comment: "Using a mock structure since goals API isn't defined yet" — fetches `/api/admin/analytics/goals?period=30d`, falls back to empty array (line 81) |
| Funnels | `routes/admin/analytics/funnels/+page.svelte:49` | `analyticsApi.getFunnels()` exists in client; backend route absent |
| Cohorts | `routes/admin/analytics/cohorts/+page.svelte:57` | Same shape — client method present, backend missing |
| Reports | `routes/admin/analytics/reports/+page.svelte` | Empty/skeleton |
| Heatmaps, Recordings, Events, Segments, Attribution | various | Placeholder pages |

### Sample three numbers — where do they come from?

1. "Visitors Today" → `metrics.visitors` ← `adminFetch('/api/analytics/realtime')` (`admin/dashboard/+page.svelte:299`). Falls back to `—` when GA isn't connected.
2. "Monthly Recurring Revenue" → `metrics.mrr` ← `adminFetch('/api/payments/summary')` (`admin/dashboard/+page.svelte:395`). Hidden when Stripe isn't connected.
3. "Funnel Overall Conversion" → `funnel.overall_conversion` (`analytics/funnels/+page.svelte:242`). Real once API ships.

---

## 2. KPI cards

### Inventory (admin home)

| Label | Source | Icon | Animation | Sparkline |
|-------|--------|------|-----------|-----------|
| Visitors Today | `metrics.visitors` | inline SVG (👥-shape) | GSAP entrance + hover scale | ❌ |
| Revenue This Month | `metrics.revenue` | inline SVG | scale + shadow | ❌ |
| SEO Health Score | `metrics.seo_score` | inline SVG | fly+y | ❌ |
| MRR | `metrics.mrr` | inline SVG | fly+delay 250ms | ❌ |

### Recommended `KpiCard3D` pattern

CSS-3D-only (no three.js — too heavy for KPIs). Tilt on hover via `perspective` + `rotateX` / `rotateY`. Add an optional 30-pt sparkline slot.

```svelte
<!-- frontend/src/lib/components/analytics/KpiCard3D.svelte -->
<script lang="ts">
    let { kpi, sparklineData = [] } = $props<{ kpi: Kpi; sparklineData?: number[] }>();
    let rotationX = $state(0);
    let rotationY = $state(0);

    function onMouseMove(e: MouseEvent) {
        const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
        rotationX = ((e.clientY - rect.top) / rect.height - 0.5) * 8;
        rotationY = ((e.clientX - rect.left) / rect.width - 0.5) * -8;
    }
    function reset() { rotationX = 0; rotationY = 0; }
</script>

<div
    class="card"
    onmousemove={onMouseMove}
    onmouseleave={reset}
    style="transform: perspective(1000px) rotateX({rotationX}deg) rotateY({rotationY}deg);"
>
    <!-- gradient light leak -->
    <div class="leak" aria-hidden="true"></div>
    <div class="content">
        <h3>{kpi.name}</h3>
        <div class="value">{kpi.formatted_value}</div>
        {#if sparklineData.length > 1}
            <Sparkline data={sparklineData} />
        {/if}
    </div>
</div>
```

Adopt incrementally — primary KPIs first, A/B-test perceived quality. Wraps the existing `KpiCard.svelte` rather than replacing it.

---

## 3. Charts

**No chart library is in use** in the analytics dashboards. Every chart is hand-rolled SVG.

| Component | Type | Real vs mock | Issues |
|-----------|------|--------------|--------|
| `analytics/FunnelChart.svelte` | horizontal bar funnel | Real-ready | drop-off badges absolute-positioned (line 91); width calc uses possibly-undefined property (line 27) |
| `analytics/TimeSeriesChart.svelte` | line + area, custom SVG | Real-ready | no grid lines; no tooltip; 5 hardcoded ticks; mobile labels overlap |
| `analytics/CohortMatrix.svelte` | heatmap table | Real-ready | period averages calculated (lines 49-60) but never rendered |

**Already installed but unused:** `lightweight-charts@5.2.0`, `d3@7.9.0`. The team installed financial-chart and low-level d3 libraries but didn't reach for either.

### Recommendation

Adopt **Recharts** (or its Svelte port) for the analytics dashboards:
- 1 import for `<LineChart>`, `<BarChart>`, `<AreaChart>`, `<PieChart>`.
- Animations, tooltips, zoom, pan come for free.
- Removes ~300 lines of custom SVG layout code.

Keep `CohortMatrix.svelte` as an HTML table — heatmaps are best as accessible tables anyway.

Keep `lightweight-charts` for the trading-room candlestick feeds elsewhere in the app.

---

## 4. Icon system

Two libraries in flight:

- `@tabler/icons-svelte-runes` (1000+ icons, used by analytics, sidebar, most admin pages). 1.5px stroke, consistent.
- Inline `<svg>` paths in `admin/dashboard/+page.svelte:233-295` (custom paths with hardcoded `stroke-width="2"`).

The Tabler library is the right pick (already installed, comprehensive, themed correctly for dark UI). The inline SVGs are remnants from a pre-Tabler era.

### Recommendation

Create `frontend/src/lib/components/Icon.svelte`:

```svelte
<script lang="ts">
    import * as TablerIcons from '$lib/icons';
    let { name, size = 20, label = undefined } = $props<{
        name: keyof typeof TablerIcons;
        size?: number | string;
        label?: string;
    }>();
    const Component = $derived(TablerIcons[name]);
</script>

{#if label}
    <Component {size} aria-label={label} />
{:else}
    <Component {size} aria-hidden="true" />
{/if}
```

Then sweep:
1. Replace every inline `<svg>` in `admin/dashboard/+page.svelte` with `<Icon name="…" />`.
2. Update `KpiCard.svelte:74-86` — drop the emoji-icon fallback, use Tabler.
3. Standardize size tokens: `xs=14 / sm=16 / md=20 / lg=24 / xl=32`.

---

## 5. Refresh strategy

Polling (60 s) on the admin home; no auto-refresh elsewhere; no WebSocket.

- Add a "Last updated · 5 m ago" timestamp on every analytics page (already present on admin home — port the helper to a shared component).
- Add a manual refresh button on Goals/Funnels/Cohorts pages.
- WebSocket is overkill for analytics; the existing `services/event_broadcaster.rs` is for trading-room alerts.

---

## 6. Top 20 ranked issues

| # | Severity | File:line | Issue | One-line fix |
|---|----------|-----------|-------|--------------|
| 1 | high | `analytics/goals/+page.svelte:79` | Goals API stubbed | implement `/api/admin/analytics/goals` |
| 2 | high | `lib/components/analytics/TimeSeriesChart.svelte` | custom SVG, no library | add Recharts; migrate |
| 3 | high | `analytics/+page.svelte:336` | KPI cards not clickable | wire `onKpiClick={showDetailModal}` |
| 4 | high | `analytics/funnels/+page.svelte:49` | Funnels API stubbed | implement backend |
| 5 | high | `analytics/cohorts/+page.svelte:57` | Cohorts API stubbed | implement backend |
| 6 | medium | `admin/dashboard/+page.svelte:233-295` | inline SVGs alongside Tabler | replace with `<Icon>` wrapper |
| 7 | medium | `lib/components/analytics/TimeSeriesChart.svelte` | no grid lines | add `<line>` Y-axis grid |
| 8 | medium | `lib/components/analytics/KpiCard.svelte:74-86` | emoji icons instead of Tabler | map keys to Tabler icons |
| 9 | medium | `analytics/+page.svelte` | no KPI detail modal | create `KpiDetailModal.svelte` |
| 10 | low | `admin/dashboard/+page.svelte:118-123` | room stats fallback to zeros | confirm endpoint exists, otherwise drop |
| 11 | medium | `lib/components/analytics/FunnelChart.svelte:27` | width calc on undefined property | use `conversion_rate` not `overall_conversion_rate` |
| 12 | low | `lib/components/analytics/CohortMatrix.svelte:49-60` | period averages computed, never rendered | add "Average Retention" row |
| 13 | medium | `analytics/goals/+page.svelte:87-91` | silent fetch failure | show error state |
| 14 | medium | `lib/components/analytics/TimeSeriesChart.svelte` | Y-axis labels overlap on mobile | reduce ticks on small viewport |
| 15 | low | `lib/components/analytics/KpiGrid.svelte` | no skeleton loader | add Skeleton while data loads |
| 16 | medium | `analytics/goals/+page.svelte:544` | submit not validated | gate on required fields |
| 17 | low | `lib/components/analytics/FunnelChart.svelte:91` | drop-off absolutely positioned | switch to margin |
| 18 | low | `analytics/goals/+page.svelte` | no CSV export | reuse `ExportButton` |
| 19 | medium | `analytics/+page.svelte:145` | connection status not reactive on permission grant | poll on visibility change |
| 20 | low | `lib/components/analytics/KpiCard.svelte:59` | inconsistent animation delay | compute as `index * 50ms` in `KpiGrid` |

---

## 7. Three structural recommendations

1. **Pick `@tabler/icons-svelte-runes` as the only icon library.** Wrap in `<Icon name="…" />`. Replace inline SVGs.
2. **Adopt Recharts for analytics charts.** Migrate `TimeSeriesChart` first; `FunnelChart` after; keep `CohortMatrix` as a table.
3. **Extract a `KpiCard3D` variant** with optional sparkline. Roll out per-page.

Tracked in [`MASTER_UIUX_BACKLOG.md`](MASTER_UIUX_BACKLOG.md).
