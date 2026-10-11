import { error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { FRED_API_KEY } from '$app/env/private';

/** FRED series for each tenor of the treasury curve. */
const FRED_SERIES = {
	rate1M: 'DGS1MO',
	rate3M: 'DGS3MO',
	rate6M: 'DGS6MO',
	rate1Y: 'DGS1',
	rate2Y: 'DGS2',
	rate10Y: 'DGS10'
} as const;

type Tenor = keyof typeof FRED_SERIES;

/** Used for any tenor FRED has no recent observation for. */
const FALLBACK_RATES: Record<Tenor, number> = {
	rate1M: 0.043,
	rate3M: 0.043,
	rate6M: 0.042,
	rate1Y: 0.04,
	rate2Y: 0.039,
	rate10Y: 0.04
};

type JsonObject = Record<string, unknown>;

/** Narrows an upstream JSON value to an object; anything else reads as empty. */
function asObject(value: unknown): JsonObject {
	return typeof value === 'object' && value !== null ? (value as JsonObject) : {};
}

/** The latest published observation of a series (FRED marks missing days with "."). */
interface Observation {
	rate: number;
	date: string;
}

/**
 * Fetches the most recent observation of one FRED series. An upstream error status yields
 * `null` (that tenor falls back); a network or parse failure rejects.
 */
async function latestObservation(
	fetch: typeof globalThis.fetch,
	seriesId: string,
	apiKey: string
): Promise<Observation | null> {
	const response = await fetch(
		`https://api.stlouisfed.org/fred/series/observations?series_id=${seriesId}&api_key=${apiKey}&file_type=json&sort_order=desc&limit=5`
	);
	if (!response.ok) return null;

	const { observations } = asObject(await response.json());
	if (!Array.isArray(observations)) return null;

	for (const raw of observations) {
		const { value, date } = asObject(raw);
		if (typeof value !== 'string' || value === '.' || typeof date !== 'string') continue;
		const percent = Number.parseFloat(value);
		if (Number.isFinite(percent)) return { rate: percent / 100, date };
	}
	return null;
}

export const GET: RequestHandler = async ({ url, fetch }) => {
	const provider = url.searchParams.get('provider') ?? 'fred';
	const healthcheck = url.searchParams.get('healthcheck');

	if (provider !== 'fred') error(400, `Unsupported provider: ${provider}`);

	const apiKey = FRED_API_KEY;
	if (!apiKey) error(401, 'FRED API key not configured');
	if (healthcheck) return Response.json({ ok: true });

	const tenors = Object.keys(FRED_SERIES) as Tenor[];
	let observations: (Observation | null)[];
	try {
		observations = await Promise.all(
			tenors.map((tenor) => latestObservation(fetch, FRED_SERIES[tenor], apiKey))
		);
	} catch (err) {
		error(502, `FRED request failed: ${err instanceof Error ? err.message : 'Unknown'}`);
	}

	const rates = { ...FALLBACK_RATES };
	let latestDate = '';
	tenors.forEach((tenor, i) => {
		const observation = observations[i];
		if (!observation) return;
		rates[tenor] = observation.rate;
		if (observation.date > latestDate) latestDate = observation.date;
	});

	return Response.json({
		...rates,
		date: latestDate || new Date().toISOString().slice(0, 10),
		source: 'fred'
	});
};
