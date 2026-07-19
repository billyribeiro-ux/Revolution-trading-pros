import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

// Validate user-controlled query params before they reach the upstream URL.
const TICKER_RE = /^[A-Z.\-]{1,10}$/i;
const DATE_RE = /^\d{4}-\d{2}-\d{2}$/;

export const GET: RequestHandler = async ({ url }) => {
	const rawTicker = url.searchParams.get('ticker');
	const provider = url.searchParams.get('provider') ?? 'polygon';
	const type = url.searchParams.get('type') ?? 'vol';

	if (!rawTicker) {
		throw error(400, 'Missing ticker parameter');
	}

	const ticker = rawTicker.toUpperCase();
	if (!TICKER_RE.test(ticker)) {
		throw error(400, 'Invalid ticker format');
	}

	if (provider === 'polygon') {
		const apiKey = env.POLYGON_API_KEY;
		if (!apiKey) throw error(401, 'Polygon API key not configured');

		// Validate the user-supplied date window up front (outside the try/catch)
		// so an invalid date returns a clean 400 rather than the catch's 502.
		let priceFrom = '';
		let priceTo = '';
		if (type === 'prices') {
			priceFrom = url.searchParams.get('from') ?? '';
			priceTo = url.searchParams.get('to') ?? '';
			if (!DATE_RE.test(priceFrom) || !DATE_RE.test(priceTo)) {
				throw error(400, 'Invalid from/to date format (expected YYYY-MM-DD)');
			}
		}

		try {
			if (type === 'prices') {
				const response = await fetch(
					`https://api.polygon.io/v2/aggs/ticker/${encodeURIComponent(ticker)}/range/1/day/${encodeURIComponent(priceFrom)}/${encodeURIComponent(priceTo)}?adjusted=true&sort=asc&apiKey=${apiKey}`
				);

				if (!response.ok) {
					throw error(response.status, `Polygon API error: ${response.statusText}`);
				}

				const data = await response.json();
				const results = (data.results ?? []).map((r: Record<string, number>) => ({
					date: new Date(r.t).toISOString().split('T')[0],
					open: r.o,
					high: r.h,
					low: r.l,
					close: r.c,
					volume: r.v
				}));

				return json(results);
			}

			// Calculate historical volatility from daily prices
			const to = new Date().toISOString().split('T')[0];
			const fromDate = new Date();
			fromDate.setDate(fromDate.getDate() - 120);
			const from = fromDate.toISOString().split('T')[0];

			const response = await fetch(
				`https://api.polygon.io/v2/aggs/ticker/${encodeURIComponent(ticker)}/range/1/day/${from}/${to}?adjusted=true&sort=asc&apiKey=${apiKey}`
			);

			if (!response.ok) {
				throw error(response.status, `Polygon API error: ${response.statusText}`);
			}

			const data = await response.json();
			const prices: number[] = (data.results ?? []).map((r: Record<string, number>) => r.c);

			if (prices.length < 20) {
				throw error(422, 'Insufficient price data for volatility calculation');
			}

			// Calculate log returns
			const logReturns: number[] = [];
			for (let i = 1; i < prices.length; i++) {
				logReturns.push(Math.log(prices[i] / prices[i - 1]));
			}

			function calcHV(returns: number[], window: number): number {
				const subset = returns.slice(-window);
				if (subset.length < window) return 0;
				const mean = subset.reduce((s, v) => s + v, 0) / subset.length;
				const variance = subset.reduce((s, v) => s + (v - mean) ** 2, 0) / (subset.length - 1);
				return Math.sqrt(variance * 252);
			}

			return json({
				ticker,
				hv10: calcHV(logReturns, 10),
				hv20: calcHV(logReturns, 20),
				hv30: calcHV(logReturns, 30),
				hv60: calcHV(logReturns, 60),
				hv90: calcHV(logReturns, Math.min(90, logReturns.length)),
				source: 'polygon'
			});
		} catch (err) {
			throw error(502, `Polygon request failed: ${err instanceof Error ? err.message : 'Unknown'}`);
		}
	}

	throw error(400, `Unsupported provider: ${provider}`);
};
