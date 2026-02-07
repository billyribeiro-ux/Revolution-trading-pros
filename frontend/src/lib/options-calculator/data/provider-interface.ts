// ============================================================
// UNIVERSAL MARKET DATA PROVIDER INTERFACE
// Every adapter implements this contract.
// Swap providers without touching any UI code.
// ============================================================

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
	OHLCV,
} from './types.js';

/**
 * Universal Market Data Provider Interface.
 * Every data provider adapter MUST implement this interface.
 */
export interface MarketDataProvider {
	readonly name: DataProviderName;
	readonly displayName: string;
	readonly capabilities: ProviderCapabilities;

	isAvailable(): Promise<boolean>;
	getStatus(): Promise<ProviderStatus>;

	getQuote(ticker: string): Promise<StockQuote>;
	getHistoricalPrices(ticker: string, from: string, to: string): Promise<OHLCV[]>;

	getOptionsChain(ticker: string, expiration: string): Promise<OptionsChain>;
	getExpirations(ticker: string): Promise<ExpirationList>;

	getHistoricalVol(ticker: string): Promise<HistoricalVolData>;

	getDividendInfo(ticker: string): Promise<DividendInfo>;
	getEarningsInfo(ticker: string): Promise<EarningsInfo>;
	getRiskFreeRate(): Promise<RiskFreeRateData>;

	searchTickers(query: string, limit?: number): Promise<TickerSearchResult[]>;
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
