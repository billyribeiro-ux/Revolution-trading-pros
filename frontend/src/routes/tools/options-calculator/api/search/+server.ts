import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { POLYGON_API_KEY } from '$app/env/private';

const DEFAULT_LIMIT = 10;
const MAX_LIMIT = 100;

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
	const query = url.searchParams.get('q');
	const provider = url.searchParams.get('provider') ?? 'polygon';
	// Clamp limit to a bounded integer so raw user input can't reshape the upstream URL.
	const limit = Math.min(
		Math.max(Number.parseInt(url.searchParams.get('limit') ?? '', 10) || DEFAULT_LIMIT, 1),
		MAX_LIMIT
	);

	if (!query) error(400, 'Missing q parameter');

	if (provider !== 'polygon') error(400, `Unsupported provider: ${provider}`);

	const apiKey = POLYGON_API_KEY;
	if (!apiKey) error(401, 'Polygon API key not configured');

	const { results } = asObject(
		await fetchPolygon(
			fetch,
			`https://api.polygon.io/v3/reference/tickers?search=${encodeURIComponent(query)}&active=true&limit=${limit}&apiKey=${apiKey}`
		)
	);

	const matches = (Array.isArray(results) ? results : []).flatMap((raw: unknown) => {
		const result = asObject(raw);
		const { ticker, name } = result;
		if (typeof ticker !== 'string' || typeof name !== 'string') return [];
		const exchange =
			typeof result.primary_exchange === 'string' ? result.primary_exchange : undefined;
		return {
			ticker,
			name,
			type: result.type === 'ETF' ? 'etf' : 'stock',
			exchange: exchange ?? '',
			primaryExchange: exchange
		};
	});

	return Response.json(matches);
};
