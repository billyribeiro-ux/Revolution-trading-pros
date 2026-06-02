/**
 * Member Analytics — shared types.
 * ─────────────────────────────────────────────────────────────────────────────
 * Hoisted out of the page and out of `analytics.remote.ts` (a `.remote.ts` may
 * only export remote functions). One combined payload (`MemberAnalytics`) backs
 * the whole dashboard so a single query drives every chart.
 */

/** Supported date-range windows for the analytics query. */
export type DateRange = '7d' | '30d' | '90d' | '12m';

/** Headline metric cards — each value is nullable until the backend reports it. */
export interface Metrics {
	totalMembers: number | null;
	memberGrowth: number | null;
	mrr: number | null;
	mrrGrowth: number | null;
	churnRate: number | null;
	churnChange: number | null;
	avgLtv: number | null;
	ltvGrowth: number | null;
}

export interface GrowthRow {
	month: string;
	members: number;
	new: number;
	churned: number;
}

export interface CohortRow {
	cohort: string;
	m0: number;
	m1: number;
	m2: number;
	m3: number;
	m4: number;
	m5: number;
}

export interface RevenueRow {
	month: string;
	mrr: number;
	expansion: number;
	contraction: number;
	churn: number;
}

export interface ChurnRow {
	reason: string;
	count: number;
	percentage: number;
}

export interface SegmentRow {
	segment: string;
	count: number;
	revenue: number;
	churnRate: number;
}

/** Everything the analytics page renders, plus a `hasData` flag the page uses to
 *  switch between the "not connected" state and the charts. */
export interface MemberAnalytics {
	metrics: Metrics;
	growth: GrowthRow[];
	cohorts: CohortRow[];
	revenue: RevenueRow[];
	churn: ChurnRow[];
	segments: SegmentRow[];
	hasData: boolean;
}

/** All-null metrics — the default before any data arrives. */
export const EMPTY_METRICS: Metrics = {
	totalMembers: null,
	memberGrowth: null,
	mrr: null,
	mrrGrowth: null,
	churnRate: null,
	churnChange: null,
	avgLtv: null,
	ltvGrowth: null
};
