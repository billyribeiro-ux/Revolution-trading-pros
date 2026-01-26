/**
 * Analytics Components - Enterprise Analytics UI Kit
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Comprehensive set of analytics visualization components
 * for the Revolution Trading enterprise analytics engine.
 *
 * @version 1.0.0
 */

// KPI Components
export { default as KpiCard } from './KpiCard.svelte';
export { default as KpiGrid } from './KpiGrid.svelte';

// Chart Components
export { default as TimeSeriesChart } from './TimeSeriesChart.svelte';
export { default as FunnelChart } from './FunnelChart.svelte';
export { default as CohortMatrix } from './CohortMatrix.svelte';
export { default as AttributionChart } from './AttributionChart.svelte';

// Widget Components
export { default as RealTimeWidget } from './RealTimeWidget.svelte';
// SegmentList retired 2026-01-26 - zero imports found

// Control Components
export { default as PeriodSelector } from './PeriodSelector.svelte';

// Advanced Components - retired 2026-01-26 (zero imports):
// BehaviorHeatmap, AIInsightsPanel, EventExplorer, RevenueBreakdown, UserJourneyMap, RetentionCurve
