/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Hooks Module
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized exports for all Svelte 5 runes-based hooks
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax
 *
 * Usage:
 * ```typescript
 * import { useAlerts, useTrades, useStats, useAnalytics } from './hooks';
 *
 * // In a .svelte component or .svelte.ts file
 * const alerts = useAlerts({ autoRefresh: true });
 * const trades = useTrades();
 * const stats = useStats();
 * const analytics = useAnalytics({ period: '30d' });
 * ```
 */

// Hook exports
export { useAlerts } from './useAlerts.svelte';
export { useTrades } from './useTrades.svelte';
export { useStats } from './useStats.svelte';
export { useAnalytics } from './useAnalytics.svelte';

// Type exports
export type { UseAlertsOptions, UseAlertsReturn } from './useAlerts.svelte';
export type {
	UseTradesOptions,
	UseTradesReturn,
	CloseTradeData,
	UpdateTradeData,
	NewTradeData
} from './useTrades.svelte';
export type { UseStatsOptions, UseStatsReturn } from './useStats.svelte';
export type {
	UseAnalyticsOptions,
	UseAnalyticsReturn,
	AnalyticsPeriod,
	PerformanceMetrics,
	DailyPerformance,
	TickerPerformance
} from './useAnalytics.svelte';
