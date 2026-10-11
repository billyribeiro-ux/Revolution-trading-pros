import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { POLYGON_API_KEY } from '$app/env/private';

// Validate user-controlled query params before they reach the upstream URL.
const TICKER_RE = /^[A-Z.-]{1,10}$/;
const EXPIRATION_RE = /^\d{4}-\d{2}-\d{2}$/;

type JsonObject = Record<string, unknown>;

/** Narrows an upstream JSON value to an object; anything else reads as empty. */
function asObject(value: unknown): JsonObject {
	return typeof value === 'object' && value !== null ? (value as JsonObject) : {};
}

function asNumber(value: unknown): number | undefined {
	return typeof value === 'number' && Number.isFinite(value) ? value : undefined;
}

function asString(value: unknown): string | undefined {
	return typeof value === 'string' ? value : undefined;
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

/** Maps one Polygon option snapshot to our `OptionQuote`, or `null` if it is unusable. */
function toOptionQuote(raw: unknown, underlying: string) {
	const snapshot = asObject(raw);
	const details = asObject(snapshot.details);
	const type = details.contract_type;
	const symbol = asString(details.ticker);
	const strike = asNumber(details.strike_price);
	const expiration = asString(details.expiration_date);
	if ((type !== 'call' && type !== 'put') || !symbol || strike === undefined || !expiration) {
		return null;
	}

	const greeks = asObject(snapshot.greeks);
	const lastQuote = asObject(snapshot.last_quote);
	const day = asObject(snapshot.day);
	const underlyingPrice = asNumber(asObject(snapshot.underlying_asset).price) ?? 0;

	return {
		symbol,
		underlying,
		type,
		strike,
		expiration,
		lastPrice: asNumber(day.close) ?? 0,
		bid: asNumber(lastQuote.bid) ?? 0,
		ask: asNumber(lastQuote.ask) ?? 0,
		mid: asNumber(lastQuote.midpoint) ?? 0,
		volume: asNumber(day.volume) ?? 0,
		openInterest: asNumber(snapshot.open_interest) ?? 0,
		impliedVolatility: asNumber(snapshot.implied_volatility) ?? 0,
		delta: asNumber(greeks.delta),
		gamma: asNumber(greeks.gamma),
		theta: asNumber(greeks.theta),
		vega: asNumber(greeks.vega),
		inTheMoney: type === 'call' ? underlyingPrice > strike : underlyingPrice < strike,
		source: 'polygon' as const
	};
}

export const GET: RequestHandler = async ({ url, fetch }) => {
	const rawTicker = url.searchParams.get('ticker');
	const expiration = url.searchParams.get('expiration');
	const provider = url.searchParams.get('provider') ?? 'polygon';

	if (!rawTicker || !expiration) error(400, 'Missing ticker or expiration parameter');

	const ticker = rawTicker.toUpperCase();
	if (!TICKER_RE.test(ticker)) error(400, 'Invalid ticker format');
	if (!EXPIRATION_RE.test(expiration)) {
		error(400, 'Invalid expiration format (expected YYYY-MM-DD)');
	}

	if (provider !== 'polygon') error(400, `Unsupported provider: ${provider}`);

	const apiKey = POLYGON_API_KEY;
	if (!apiKey) error(401, 'Polygon API key not configured');

	const data = asObject(
		await fetchPolygon(
			fetch,
			`https://api.polygon.io/v3/snapshot/options/${encodeURIComponent(ticker)}?expiration_date=${encodeURIComponent(expiration)}&apiKey=${apiKey}&limit=250`
		)
	);
	const results: unknown[] = Array.isArray(data.results) ? data.results : [];

	const underlyingPrice = asNumber(asObject(asObject(results[0]).underlying_asset).price) ?? 0;

	const quotes = results.flatMap((raw) => toOptionQuote(raw, ticker) ?? []);

	return Response.json({
		underlying: ticker,
		underlyingPrice,
		expiration,
		calls: quotes.filter((quote) => quote.type === 'call'),
		puts: quotes.filter((quote) => quote.type === 'put'),
		timestamp: new Date().toISOString(),
		source: 'polygon'
	});
};
