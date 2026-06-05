/**
 * Media (bandwidth) Analytics — Remote Function (query)
 * ─────────────────────────────────────────────────────────────────────────────
 * Replaces the page's three parallel `fetch('/api/admin/media/analytics/…')`
 * calls + manual `Promise.allSettled` bookkeeping with ONE typed `query`,
 * consumed reactively (`$derived(getMediaAnalytics(timeRange))`). Endpoints
 * degrade independently; `hasData` drives the page's not-connected state.
 *
 * Auth: `getRequestEvent().fetch` forwards the request cookies to the admin
 * media-analytics proxies (`requireAdmin`) — the same path the old client
 * `fetch` took.
 */
import * as v from 'valibot';
import { getRequestEvent, query } from '$app/server';
import type {
	BandwidthData,
	FormatStats,
	MediaAnalytics,
	SavingsOverview
} from './media-analytics.types';

const RangeSchema = v.optional(v.picklist(['7d', '30d', '90d', '1y']), '30d');

export const getMediaAnalytics = query(RangeSchema, async (range): Promise<MediaAnalytics> => {
	const { fetch } = getRequestEvent();

	const get = async (path: string): Promise<unknown> => {
		try {
			const res = await fetch(`/api/admin/media/analytics/${path}`);
			return res.ok ? await res.json() : null;
		} catch {
			return null;
		}
	};

	const [overviewRaw, bandwidth, formats] = await Promise.all([
		get('overview'),
		get(`bandwidth?range=${range}`),
		get('formats')
	]);

	const hasOverview =
		!!overviewRaw && typeof (overviewRaw as { totalOriginal?: unknown }).totalOriginal === 'number';
	const overview: SavingsOverview | null = hasOverview ? (overviewRaw as SavingsOverview) : null;

	const rows = <T>(x: unknown): T[] => (Array.isArray(x) && x.length > 0 ? (x as T[]) : []);
	const bw = rows<BandwidthData>(bandwidth);
	const fmt = rows<FormatStats>(formats);

	const hasData = hasOverview || bw.length > 0 || fmt.length > 0;

	return { overview, bandwidth: bw, formats: fmt, hasData };
});
