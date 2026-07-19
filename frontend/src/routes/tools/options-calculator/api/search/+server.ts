import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const GET: RequestHandler = async ({ url }) => {
	const query = url.searchParams.get('q');
	const provider = url.searchParams.get('provider') ?? 'polygon';
	// Clamp limit to a bounded integer so raw user input can't reshape the
	// upstream URL. Falls back to 10, capped at 100.
	const limit = Math.min(
		Math.max(parseInt(url.searchParams.get('limit') ?? '10', 10) || 10, 1),
		100
	);

	if (!query) {
		throw error(400, 'Missing q parameter');
	}

	if (provider === 'polygon') {
		const apiKey = env.POLYGON_API_KEY;
		if (!apiKey) throw error(401, 'Polygon API key not configured');

		try {
			const response = await fetch(
				`https://api.polygon.io/v3/reference/tickers?search=${encodeURIComponent(query)}&active=true&limit=${limit}&apiKey=${apiKey}`
			);

			if (!response.ok) {
				throw error(response.status, `Polygon API error: ${response.statusText}`);
			}

			const data = await response.json();
			const results = (data.results ?? []).map((r: Record<string, unknown>) => ({
				ticker: r.ticker,
				name: r.name,
				type: r.type === 'ETF' ? 'etf' : 'stock',
				exchange: r.primary_exchange ?? '',
				primaryExchange: r.primary_exchange
			}));

			return json(results);
		} catch (err) {
			throw error(502, `Polygon request failed: ${err instanceof Error ? err.message : 'Unknown'}`);
		}
	}

	throw error(400, `Unsupported provider: ${provider}`);
};
