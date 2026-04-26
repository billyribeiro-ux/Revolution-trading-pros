import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const GET: RequestHandler = async ({ url }) => {
	const ticker = url.searchParams.get('ticker');
	const provider = url.searchParams.get('provider') ?? 'polygon';
	const healthcheck = url.searchParams.get('healthcheck');

	if (!ticker && !healthcheck) {
		return error(400, 'Missing ticker parameter');
	}

	if (provider === 'polygon') {
		const apiKey = env.POLYGON_API_KEY;
		if (!apiKey) return error(401, 'Polygon API key not configured');

		if (healthcheck) return json({ ok: true });

		try {
			const response = await fetch(
				`https://api.polygon.io/v2/snapshot/locale/us/markets/stocks/tickers/${ticker}?apiKey=${apiKey}`
			);

			if (!response.ok) {
				return error(response.status, `Polygon API error: ${response.statusText}`);
			}

			const data = await response.json();
			const snap = data.ticker;

			const quote = {
				ticker: snap?.ticker ?? ticker,
				price: snap?.lastTrade?.p ?? snap?.day?.c ?? 0,
				open: snap?.day?.o ?? 0,
				high: snap?.day?.h ?? 0,
				low: snap?.day?.l ?? 0,
				close: snap?.day?.c ?? 0,
				previousClose: snap?.prevDay?.c ?? 0,
				volume: snap?.day?.v ?? 0,
				change: (snap?.lastTrade?.p ?? 0) - (snap?.prevDay?.c ?? 0),
				changePercent: snap?.todaysChangePerc ?? 0,
				timestamp: new Date().toISOString(),
				source: 'polygon'
			};

			return json(quote);
		} catch (err) {
			return error(
				502,
				`Polygon request failed: ${err instanceof Error ? err.message : 'Unknown'}`
			);
		}
	}

	return error(400, `Unsupported provider: ${provider}`);
};
