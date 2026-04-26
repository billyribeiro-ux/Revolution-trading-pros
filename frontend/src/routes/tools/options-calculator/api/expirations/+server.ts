import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const GET: RequestHandler = async ({ url }) => {
	const ticker = url.searchParams.get('ticker');
	const provider = url.searchParams.get('provider') ?? 'polygon';

	if (!ticker) {
		return error(400, 'Missing ticker parameter');
	}

	if (provider === 'polygon') {
		const apiKey = env.POLYGON_API_KEY;
		if (!apiKey) return error(401, 'Polygon API key not configured');

		try {
			const response = await fetch(
				`https://api.polygon.io/v3/reference/options/contracts?underlying_ticker=${ticker}&apiKey=${apiKey}&limit=1000&order=asc&sort=expiration_date`
			);

			if (!response.ok) {
				return error(response.status, `Polygon API error: ${response.statusText}`);
			}

			const data = await response.json();
			const results = data.results ?? [];

			const expirationSet = new Set<string>();
			for (const contract of results) {
				if (contract.expiration_date) {
					expirationSet.add(contract.expiration_date);
				}
			}

			return json({
				underlying: ticker,
				expirations: [...expirationSet].sort(),
				source: 'polygon'
			});
		} catch (err) {
			return error(
				502,
				`Polygon request failed: ${err instanceof Error ? err.message : 'Unknown'}`
			);
		}
	}

	return error(400, `Unsupported provider: ${provider}`);
};
