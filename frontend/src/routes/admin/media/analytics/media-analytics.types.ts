/**
 * Media (bandwidth) Analytics — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Hoisted out of the page and out of `media-analytics.remote.ts` (a `.remote.ts`
 * may only export remote functions). One combined `MediaAnalytics` payload backs
 * the whole dashboard.
 */

/** Time-range windows for the bandwidth dashboard. */
export type TimeRange = '7d' | '30d' | '90d' | '1y';

export interface BandwidthData {
	date: string;
	original: number;
	optimized: number;
	savings: number;
	requests: number;
}

export interface FormatStats {
	format: string;
	count: number;
	originalSize: number;
	optimizedSize: number;
	savings: number;
}

export interface SavingsOverview {
	totalOriginal: number;
	totalOptimized: number;
	totalSavings: number;
	savingsPercent: number;
	totalImages: number;
	optimizedImages: number;
	avgCompressionRatio: number;
	estimatedCostSavings: number;
	co2Saved: number;
}

/** Everything the dashboard renders, plus `hasData` for the not-connected state. */
export interface MediaAnalytics {
	overview: SavingsOverview | null;
	bandwidth: BandwidthData[];
	formats: FormatStats[];
	hasData: boolean;
}
