import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const GET: RequestHandler = async ({ url }) => {
	const ticker = url.searchParams.get('ticker');
	const expiration = url.searchParams.get('expiration');
	const provider = url.searchParams.get('provider') ?? 'polygon';

	if (!ticker || !expiration) {
		return error(400, 'Missing ticker or expiration parameter');
	}

	if (provider === 'polygon') {
		const apiKey = env.POLYGON_API_KEY;
		if (!apiKey) return error(401, 'Polygon API key not configured');

		try {
			const response = await fetch(
				`https://api.polygon.io/v3/snapshot/options/${ticker}?expiration_date=${expiration}&apiKey=${apiKey}&limit=250`,
			);

			if (!response.ok) {
				return error(response.status, `Polygon API error: ${response.statusText}`);
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
						inTheMoney: (underlying?.price as number ?? 0) > (details.strike_price as number),
						source: 'polygon',
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
						inTheMoney: (underlying?.price as number ?? 0) < (details.strike_price as number),
						source: 'polygon',
					};
				});

			const underlyingPrice =
				results[0]?.underlying_asset?.price ?? 0;

			return json({
				underlying: ticker,
				underlyingPrice,
				expiration,
				calls,
				puts,
				timestamp: new Date().toISOString(),
				source: 'polygon',
			});
		} catch (err) {
			return error(502, `Polygon request failed: ${err instanceof Error ? err.message : 'Unknown'}`);
		}
	}

	return error(400, `Unsupported provider: ${provider}`);
};
