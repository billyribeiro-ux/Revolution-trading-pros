/**
 * Member Analytics — Remote Function (query)
 * ─────────────────────────────────────────────────────────────────────────────
 * Replaces the page's six parallel `fetch('/api/admin/members/analytics/…')`
 * calls + manual `Promise.allSettled` bookkeeping with ONE typed `query`. The
 * page consumes it reactively (`$derived(getMemberAnalytics(dateRange))`), so
 * changing the date range re-fetches automatically and identical ranges dedupe —
 * and with `compilerOptions.experimental.async` on, the result is server-resolved
 * on first paint.
 *
 * The six upstream endpoints are fetched in parallel and degrade independently:
 * a failing/empty endpoint contributes its empty default rather than failing the
 * whole dashboard (mirrors the old `allSettled` behaviour). `hasData` tells the
 * page whether to show the charts or the "not connected" state.
 *
 * Auth: `getRequestEvent().fetch` forwards the request cookies to the admin
 * analytics proxies, which enforce `requireAdmin` — the same path the old client
 * `fetch` took.
 */
import * as v from 'valibot';
import { getRequestEvent, query } from '$app/server';
import {
	EMPTY_METRICS,
	type ChurnRow,
	type CohortRow,
	type GrowthRow,
	type MemberAnalytics,
	type Metrics,
	type RevenueRow,
	type SegmentRow
} from './analytics.types';

const RangeSchema = v.optional(v.picklist(['7d', '30d', '90d', '12m']), '30d');

export const getMemberAnalytics = query(RangeSchema, async (range): Promise<MemberAnalytics> => {
	const { fetch } = getRequestEvent();

	// Non-throwing fetch+parse so one bad endpoint can't sink the dashboard.
	const get = async (path: string): Promise<unknown> => {
		try {
			const res = await fetch(`/api/admin/members/analytics/${path}?range=${range}`);
			return res.ok ? await res.json() : null;
		} catch {
			return null;
		}
	};

	const [metricsRaw, growth, cohorts, revenue, churn, segments] = await Promise.all([
		get('metrics'),
		get('growth'),
		get('cohorts'),
		get('revenue'),
		get('churn-reasons'),
		get('segments')
	]);

	const hasMetrics =
		!!metricsRaw && typeof (metricsRaw as { totalMembers?: unknown }).totalMembers === 'number';
	const metrics: Metrics = hasMetrics ? (metricsRaw as Metrics) : EMPTY_METRICS;

	const rows = <T>(x: unknown): T[] => (Array.isArray(x) && x.length > 0 ? (x as T[]) : []);
	const g = rows<GrowthRow>(growth);
	const c = rows<CohortRow>(cohorts);
	const rev = rows<RevenueRow>(revenue);
	const ch = rows<ChurnRow>(churn);
	const seg = rows<SegmentRow>(segments);

	const hasData =
		hasMetrics || g.length > 0 || c.length > 0 || rev.length > 0 || ch.length > 0 || seg.length > 0;

	return { metrics, growth: g, cohorts: c, revenue: rev, churn: ch, segments: seg, hasData };
});
