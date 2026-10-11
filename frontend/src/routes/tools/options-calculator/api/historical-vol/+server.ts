import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { POLYGON_API_KEY } from '$app/env/private';

// Validate user-controlled query params before they reach the upstream URL.
const TICKER_RE = /^[A-Z.-]{1,10}$/;
const DATE_RE = /^\d{4}-\d{2}-\d{2}$/;

/** Calendar days of daily closes fetched for the volatility windows (enough for HV90). */
const LOOKBACK_DAYS = 120;
const TRADING_DAYS_PER_YEAR = 252;

type JsonObject = Record<string, unknown>;

/** Narrows an upstream JSON value to an object; anything else reads as empty. */
function asObject(value: unknown): JsonObject {
	return typeof value === 'object' && value !== null ? (value as JsonObject) : {};
}

function asNumber(value: unknown): number | undefined {
	return typeof value === 'number' && Number.isFinite(value) ? value : undefined;
}

function reason(err: unknown): string {
	return err instanceof Error ? err.message : 'Unknown';
}

/** `YYYY-MM-DD` in UTC. */
function isoDate(date: Date): string {
	return date.toISOString().slice(0, 10);
}

/**
 * GET a Polygon URL and return its JSON body. Network and parse failures become a 502;
 * an upstream error status is passed through so the client can tell 401/429 apart.
 * `error(...)` throws, and it is never called inside a `try` here, so SvelteKit always
 * receives it unaltered.
 */
async function fetchPolygon(fetch: typeof globalThis.fetch, href: string): Promise<unknown> {
	let response: Response;
	try {
		response = await fetch(href);
	} catch (err) {
		error(502, `Polygon request failed: ${reason(err)}`);
	}

	if (!response.ok) error(response.status, `Polygon API error: ${response.statusText}`);

	try {
		return await response.json();
	} catch (err) {
		error(502, `Polygon request failed: ${reason(err)}`);
	}
}

/** The daily aggregate bars of a Polygon `/v2/aggs` response. */
function aggregateBars(data: unknown): JsonObject[] {
	const { results } = asObject(data);
	return Array.isArray(results) ? results.map(asObject) : [];
}

/** Annualised sample standard deviation of the last `window` log returns (0 if too few). */
function historicalVol(returns: number[], window: number): number {
	const subset = returns.slice(-window);
	if (subset.length < window || subset.length < 2) return 0;
	const mean = subset.reduce((sum, r) => sum + r, 0) / subset.length;
	const variance = subset.reduce((sum, r) => sum + (r - mean) ** 2, 0) / (subset.length - 1);
	return Math.sqrt(variance * TRADING_DAYS_PER_YEAR);
}

export const GET: RequestHandler = async ({ url, fetch }) => {
	const rawTicker = url.searchParams.get('ticker');
	const provider = url.searchParams.get('provider') ?? 'polygon';
	const type = url.searchParams.get('type') ?? 'vol';

	if (!rawTicker) error(400, 'Missing ticker parameter');

	const ticker = rawTicker.toUpperCase();
	if (!TICKER_RE.test(ticker)) error(400, 'Invalid ticker format');

	if (provider !== 'polygon') error(400, `Unsupported provider: ${provider}`);

	const apiKey = POLYGON_API_KEY;
	if (!apiKey) error(401, 'Polygon API key not configured');

	const aggregatesURL = (from: string, to: string) =>
		`https://api.polygon.io/v2/aggs/ticker/${encodeURIComponent(ticker)}/range/1/day/${encodeURIComponent(from)}/${encodeURIComponent(to)}?adjusted=true&sort=asc&apiKey=${apiKey}`;

	if (type === 'prices') {
		const from = url.searchParams.get('from') ?? '';
		const to = url.searchParams.get('to') ?? '';
		if (!DATE_RE.test(from) || !DATE_RE.test(to)) {
			error(400, 'Invalid from/to date format (expected YYYY-MM-DD)');
		}

		const bars = aggregateBars(await fetchPolygon(fetch, aggregatesURL(from, to)));
		return Response.json(
			bars.flatMap((bar) => {
				const t = asNumber(bar.t);
				if (t === undefined) return [];
				return {
					date: isoDate(new Date(t)),
					open: asNumber(bar.o) ?? 0,
					high: asNumber(bar.h) ?? 0,
					low: asNumber(bar.l) ?? 0,
					close: asNumber(bar.c) ?? 0,
					volume: asNumber(bar.v) ?? 0
				};
			})
		);
	}

	// Historical volatility from the last LOOKBACK_DAYS of daily closes.
	const toDate = new Date();
	const fromDate = new Date(toDate);
	fromDate.setDate(fromDate.getDate() - LOOKBACK_DAYS);

	const bars = aggregateBars(
		await fetchPolygon(fetch, aggregatesURL(isoDate(fromDate), isoDate(toDate)))
	);
	const closes = bars.flatMap((bar) => {
		const close = asNumber(bar.c);
		return close !== undefined && close > 0 ? [close] : [];
	});

	if (closes.length < 20) error(422, 'Insufficient price data for volatility calculation');

	const logReturns = closes.slice(1).map((close, i) => Math.log(close / closes[i]));

	return Response.json({
		ticker,
		hv10: historicalVol(logReturns, 10),
		hv20: historicalVol(logReturns, 20),
		hv30: historicalVol(logReturns, 30),
		hv60: historicalVol(logReturns, 60),
		hv90: historicalVol(logReturns, Math.min(90, logReturns.length)),
		source: 'polygon'
	});
};
