import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

// Validate user-controlled query params before they reach the upstream URL.
const TICKER_RE = /^[A-Z.\-]{1,10}$/i;
const EXPIRATION_RE = /^\d{4}-\d{2}-\d{2}$/;

export const GET: RequestHandler = async ({ url }) => {
	const rawTicker = url.searchParams.get('ticker');
	const expiration = url.searchParams.get('expiration');
	const provider = url.searchParams.get('provider') ?? 'polygon';

	if (!rawTicker || !expiration) {
		throw error(400, 'Missing ticker or expiration parameter');
	}

	const ticker = rawTicker.toUpperCase();
	if (!TICKER_RE.test(ticker)) {
		throw error(400, 'Invalid ticker format');
	}
	if (!EXPIRATION_RE.test(expiration)) {
		throw error(400, 'Invalid expiration format (expected YYYY-MM-DD)');
	}

	if (provider === 'polygon') {
		const apiKey = env.POLYGON_API_KEY;
		if (!apiKey) throw error(401, 'Polygon API key not configured');

		try {
			const response = await fetch(
				`https://api.polygon.io/v3/snapshot/options/${encodeURIComponent(ticker)}?expiration_date=${encodeURIComponent(expiration)}&apiKey=${apiKey}&limit=250`
			);

			if (!response.ok) {
				throw error(response.status, `Polygon API error: ${response.statusText}`);
			}

			const data = await response.json();
			const results = data.results ?? [];

			const calls = results
				.filter((r: Record<string, unknown>) => {
					const details = r.details as Record<string, unknown> | undefined;
					return details?.contract_type === 'call';
				})
				.map((r: Record<string, unknown>) => {
					const details = r.details as Record<string, unknown>;
					const greeks = r.greeks as Record<string, number> | undefined;
					const lastQuote = r.last_quote as Record<string, number> | undefined;
					const day = r.day as Record<string, number> | undefined;
					const underlying = r.underlying_asset as Record<string, unknown> | undefined;

					return {
						symbol: details.ticker,
						underlying: ticker,
						type: 'call',
						strike: details.strike_price,
						expiration: details.expiration_date,
						lastPrice: day?.close ?? 0,
						bid: lastQuote?.bid ?? 0,
						ask: lastQuote?.ask ?? 0,
						mid: lastQuote?.midpoint ?? 0,
						volume: day?.volume ?? 0,
						openInterest: r.open_interest ?? 0,
						impliedVolatility: r.implied_volatility ?? 0,
						delta: greeks?.delta,
						gamma: greeks?.gamma,
						theta: greeks?.theta,
						vega: greeks?.vega,
						inTheMoney: ((underlying?.price as number) ?? 0) > (details.strike_price as number),
						source: 'polygon'
					};
				});

			const puts = results
				.filter((r: Record<string, unknown>) => {
					const details = r.details as Record<string, unknown> | undefined;
					return details?.contract_type === 'put';
				})
				.map((r: Record<string, unknown>) => {
					const details = r.details as Record<string, unknown>;
					const greeks = r.greeks as Record<string, number> | undefined;
					const lastQuote = r.last_quote as Record<string, number> | undefined;
					const day = r.day as Record<string, number> | undefined;
					const underlying = r.underlying_asset as Record<string, unknown> | undefined;

					return {
						symbol: details.ticker,
						underlying: ticker,
						type: 'put',
						strike: details.strike_price,
						expiration: details.expiration_date,
						lastPrice: day?.close ?? 0,
						bid: lastQuote?.bid ?? 0,
						ask: lastQuote?.ask ?? 0,
						mid: lastQuote?.midpoint ?? 0,
						volume: day?.volume ?? 0,
						openInterest: r.open_interest ?? 0,
						impliedVolatility: r.implied_volatility ?? 0,
						delta: greeks?.delta,
						gamma: greeks?.gamma,
						theta: greeks?.theta,
						vega: greeks?.vega,
						inTheMoney: ((underlying?.price as number) ?? 0) < (details.strike_price as number),
						source: 'polygon'
					};
				});

			const underlyingPrice = results[0]?.underlying_asset?.price ?? 0;

			return json({
				underlying: ticker,
				underlyingPrice,
				expiration,
				calls,
				puts,
				timestamp: new Date().toISOString(),
				source: 'polygon'
			});
		} catch (err) {
			throw error(502, `Polygon request failed: ${err instanceof Error ? err.message : 'Unknown'}`);
		}
	}

	throw error(400, `Unsupported provider: ${provider}`);
};
