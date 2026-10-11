// ============================================================
// UNIVERSAL MARKET DATA PROVIDER INTERFACE
// Every adapter implements this contract.
// Swap providers without touching any UI code.
// ============================================================

import * as v from 'valibot';
import type {
	DataProviderName,
	ProviderCapabilities,
	ProviderStatus,
	StockQuote,
	OptionsChain,
	ExpirationList,
	TickerSearchResult,
	RiskFreeRateData,
	DividendInfo,
	EarningsInfo,
	HistoricalVolData,
	OHLCV
} from './types.js';

/** Per-request options shared by every provider method. */
export interface RequestOptions {
	/**
	 * Aborts the request (superseded search, ticker switch, timeout).
	 * Network-backed adapters MUST forward it to `fetch`.
	 */
	signal?: AbortSignal;
}

/**
 * Universal Market Data Provider Interface.
 * Every data provider adapter MUST implement this interface.
 */
export interface MarketDataProvider {
	readonly name: DataProviderName;
	readonly displayName: string;
	readonly capabilities: ProviderCapabilities;

	isAvailable(options?: RequestOptions): Promise<boolean>;
	getStatus(options?: RequestOptions): Promise<ProviderStatus>;

	getQuote(ticker: string, options?: RequestOptions): Promise<StockQuote>;
	getHistoricalPrices(
		ticker: string,
		from: string,
		to: string,
		options?: RequestOptions
	): Promise<OHLCV[]>;

	getOptionsChain(
		ticker: string,
		expiration: string,
		options?: RequestOptions
	): Promise<OptionsChain>;
	getExpirations(ticker: string, options?: RequestOptions): Promise<ExpirationList>;

	getHistoricalVol(ticker: string, options?: RequestOptions): Promise<HistoricalVolData>;

	getDividendInfo(ticker: string, options?: RequestOptions): Promise<DividendInfo>;
	getEarningsInfo(ticker: string, options?: RequestOptions): Promise<EarningsInfo>;
	getRiskFreeRate(options?: RequestOptions): Promise<RiskFreeRateData>;

	searchTickers(
		query: string,
		limit?: number,
		options?: RequestOptions
	): Promise<TickerSearchResult[]>;
}

/**
 * Error thrown when a provider doesn't support a requested capability.
 */
export class CapabilityNotSupportedError extends Error {
	constructor(provider: DataProviderName, capability: string) {
		super(`${provider} does not support ${capability}`);
		this.name = 'CapabilityNotSupportedError';
	}
}

/**
 * Error thrown when API rate limit is exceeded.
 */
export class RateLimitExceededError extends Error {
	constructor(provider: DataProviderName, resetAt?: string) {
		super(`${provider} rate limit exceeded${resetAt ? `, resets at ${resetAt}` : ''}`);
		this.name = 'RateLimitExceededError';
	}
}

/**
 * Error thrown when API key is missing or invalid.
 */
export class AuthenticationError extends Error {
	constructor(provider: DataProviderName) {
		super(`${provider} authentication failed — check API key configuration`);
		this.name = 'AuthenticationError';
	}
}

/**
 * Error thrown when a provider answers with a body that is not the shape we
 * expect (not JSON, or failing its valibot schema). Retrying cannot fix it.
 */
export class InvalidResponseError extends Error {
	constructor(provider: DataProviderName, endpoint: string, detail: string) {
		super(`${provider} returned an invalid ${endpoint} response: ${detail}`);
		this.name = 'InvalidResponseError';
	}
}

/**
 * Error thrown when no provider can fulfill a request.
 */
export class AllProvidersFailedError extends Error {
	public readonly errors: { provider: DataProviderName; error: Error }[];

	constructor(capability: string, errors: { provider: DataProviderName; error: Error }[]) {
		const providerList = errors.map((e) => `${e.provider}: ${e.error.message}`).join('; ');
		super(`All providers failed for ${capability}: ${providerList}`);
		this.name = 'AllProvidersFailedError';
		this.errors = errors;
	}
}

// ── Shared HTTP plumbing ─────────────────────────────────────
// Network adapters never call third-party APIs directly: they go through the
// SvelteKit proxy routes so API keys stay on the server.

/** Base path of the SvelteKit proxy routes. */
export const PROVIDER_API_BASE = '/tools/options-calculator/api';

function proxyURL(
	provider: DataProviderName,
	endpoint: string,
	params: Record<string, string>
): string {
	const query = new URLSearchParams({ provider, ...params });
	return `${PROVIDER_API_BASE}/${endpoint}?${query}`;
}

/**
 * GET a proxy route and validate the JSON body against `schema`.
 * Maps 401 → `AuthenticationError`, 429 → `RateLimitExceededError`, a body
 * that is not JSON or fails validation → `InvalidResponseError`; an abort
 * propagates untouched so the router can tell it apart from a failure.
 */
export async function fetchProviderJSON<T>(
	provider: DataProviderName,
	label: string,
	endpoint: string,
	params: Record<string, string>,
	schema: v.GenericSchema<unknown, T>,
	{ signal }: RequestOptions = {}
): Promise<T> {
	const response = await fetch(proxyURL(provider, endpoint, params), { signal });

	if (response.status === 401) throw new AuthenticationError(provider);
	if (response.status === 429) {
		const resetAt = response.headers.get('X-RateLimit-Reset') ?? undefined;
		throw new RateLimitExceededError(provider, resetAt);
	}
	if (!response.ok) {
		throw new Error(`${label} API error: ${response.status} ${response.statusText}`.trimEnd());
	}

	let body: unknown;
	try {
		body = await response.json();
	} catch (error) {
		// Reading the body also rejects when the request is aborted mid-stream.
		signal?.throwIfAborted();
		throw new InvalidResponseError(
			provider,
			endpoint,
			error instanceof Error ? error.message : 'body is not JSON'
		);
	}

	const result = v.safeParse(schema, body);
	if (!result.success) {
		throw new InvalidResponseError(provider, endpoint, v.summarize(result.issues));
	}
	return result.output;
}

/** Health-check a proxy route: `true` when it answers 2xx, `false` on any failure. */
export async function probeProvider(
	provider: DataProviderName,
	endpoint: string,
	params: Record<string, string> = {},
	{ signal }: RequestOptions = {}
): Promise<boolean> {
	try {
		const response = await fetch(proxyURL(provider, endpoint, { ...params, healthcheck: 'true' }), {
			signal
		});
		return response.ok;
	} catch {
		return false;
	}
}
