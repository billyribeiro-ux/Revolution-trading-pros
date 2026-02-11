import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

const FRED_SERIES: Record<string, string> = {
	rate1M: 'DGS1MO',
	rate3M: 'DGS3MO',
	rate6M: 'DGS6MO',
	rate1Y: 'DGS1',
	rate2Y: 'DGS2',
	rate10Y: 'DGS10'
};

export const GET: RequestHandler = async ({ url }) => {
	const provider = url.searchParams.get('provider') ?? 'fred';
	const healthcheck = url.searchParams.get('healthcheck');

	if (provider === 'fred') {
		const apiKey = env.FRED_API_KEY;
		if (!apiKey) return error(401, 'FRED API key not configured');

		if (healthcheck) return json({ ok: true });

		try {
			const results: Record<string, number> = {};
			let latestDate = '';

			const fetches = Object.entries(FRED_SERIES).map(async ([key, seriesId]) => {
				const response = await fetch(
					`https://api.stlouisfed.org/fred/series/observations?series_id=${seriesId}&api_key=${apiKey}&file_type=json&sort_order=desc&limit=5`
				);

				if (!response.ok) return;

				const data = await response.json();
				const observations = data.observations ?? [];

				for (const obs of observations) {
					if (obs.value !== '.') {
						results[key] = parseFloat(obs.value) / 100;
						if (!latestDate || obs.date > latestDate) {
							latestDate = obs.date;
						}
						break;
					}
				}
			});

			await Promise.all(fetches);

			return json({
				rate1M: results.rate1M ?? 0.043,
				rate3M: results.rate3M ?? 0.043,
				rate6M: results.rate6M ?? 0.042,
				rate1Y: results.rate1Y ?? 0.04,
				rate2Y: results.rate2Y ?? 0.039,
				rate10Y: results.rate10Y ?? 0.04,
				date: latestDate || new Date().toISOString().split('T')[0],
				source: 'fred'
			});
		} catch (err) {
			return error(502, `FRED request failed: ${err instanceof Error ? err.message : 'Unknown'}`);
		}
	}

	return error(400, `Unsupported provider: ${provider}`);
};
