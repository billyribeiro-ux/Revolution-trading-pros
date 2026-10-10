import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { POLYGON_API_KEY } from '$app/env/private';

// Validate the user-controlled ticker before it reaches the upstream URL.
const TICKER_RE = /^[A-Z.-]{1,10}$/;

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

export const GET: RequestHandler = async ({ url, fetch }) => {
	const rawTicker = url.searchParams.get('ticker');
	const provider = url.searchParams.get('provider') ?? 'polygon';
	const healthcheck = url.searchParams.get('healthcheck');

	if (!rawTicker && !healthcheck) error(400, 'Missing ticker parameter');

	const ticker = rawTicker?.toUpperCase() ?? null;
	if (ticker && !TICKER_RE.test(ticker)) error(400, 'Invalid ticker format');

	if (provider !== 'polygon') error(400, `Unsupported provider: ${provider}`);

	const apiKey = POLYGON_API_KEY;
	if (!apiKey) error(401, 'Polygon API key not configured');
	if (healthcheck) return Response.json({ ok: true });

	if (!ticker) error(400, 'Missing ticker parameter');

	const snapshot = asObject(
		asObject(
			await fetchPolygon(
				fetch,
				`https://api.polygon.io/v2/snapshot/locale/us/markets/stocks/tickers/${encodeURIComponent(ticker)}?apiKey=${apiKey}`
			)
		).ticker
	);
	const day = asObject(snapshot.day);
	const prevDay = asObject(snapshot.prevDay);
	const lastPrice = asNumber(asObject(snapshot.lastTrade).p);
	const previousClose = asNumber(prevDay.c) ?? 0;

	return Response.json({
		ticker: typeof snapshot.ticker === 'string' ? snapshot.ticker : ticker,
		price: lastPrice ?? asNumber(day.c) ?? 0,
		open: asNumber(day.o) ?? 0,
		high: asNumber(day.h) ?? 0,
		low: asNumber(day.l) ?? 0,
		close: asNumber(day.c) ?? 0,
		previousClose,
		volume: asNumber(day.v) ?? 0,
		change: (lastPrice ?? 0) - previousClose,
		changePercent: asNumber(snapshot.todaysChangePerc) ?? 0,
		timestamp: new Date().toISOString(),
		source: 'polygon'
	});
};
