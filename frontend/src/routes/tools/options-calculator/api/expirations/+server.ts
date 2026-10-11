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

	if (!rawTicker) error(400, 'Missing ticker parameter');

	const ticker = rawTicker.toUpperCase();
	if (!TICKER_RE.test(ticker)) error(400, 'Invalid ticker format');

	if (provider !== 'polygon') error(400, `Unsupported provider: ${provider}`);

	const apiKey = POLYGON_API_KEY;
	if (!apiKey) error(401, 'Polygon API key not configured');

	const data = asObject(
		await fetchPolygon(
			fetch,
			`https://api.polygon.io/v3/reference/options/contracts?underlying_ticker=${encodeURIComponent(ticker)}&apiKey=${apiKey}&limit=1000&order=asc&sort=expiration_date`
		)
	);
	const contracts: unknown[] = Array.isArray(data.results) ? data.results : [];

	const expirations = new Set<string>();
	for (const contract of contracts) {
		const date = asObject(contract).expiration_date;
		if (typeof date === 'string' && date) expirations.add(date);
	}

	return Response.json({
		underlying: ticker,
		expirations: [...expirations].sort(),
		source: 'polygon'
	});
};
