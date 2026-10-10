// ============================================================
// POLYGON.IO ADAPTER
// Primary provider — real-time quotes, options, historical data
// All requests go through SvelteKit API routes to keep keys secure
// ============================================================

import type { MarketDataProvider, RequestOptions } from '../provider-interface.js';
import {
	CapabilityNotSupportedError,
	fetchProviderJSON,
	probeProvider
} from '../provider-interface.js';
import {
	expirationListSchema,
	ohlcvListSchema,
	optionsChainSchema,
	stockQuoteSchema,
	tickerSearchResultsSchema
} from '../schemas.js';
import type {
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
} from '../types.js';

export function createPolygonAdapter(): MarketDataProvider {
	const NAME = 'polygon';
	const LABEL = 'Polygon';

	const capabilities: ProviderCapabilities = {
		quotes: true,
		optionsChain: true,
		historicalPrices: true,
		historicalVol: false,
		// The quote proxy route has no dividends lookup (it ignores `include=dividends` and
		// returns a quote), so dividend data comes from the next provider.
		dividends: false,
		earnings: false,
		riskFreeRate: false,
		tickerSearch: true,
		streaming: true
	};

	function isAvailable(options?: RequestOptions): Promise<boolean> {
		return probeProvider(NAME, 'quote', { ticker: 'AAPL' }, options);
	}

	return {
		name: NAME,
		displayName: 'Polygon.io',
		capabilities,

		isAvailable,

		async getStatus(options?: RequestOptions): Promise<ProviderStatus> {
			const available = await isAvailable(options);
			return {
				name: NAME,
				displayName: 'Polygon.io',
				isAvailable: available,
				isConfigured: available
			};
		},

		getQuote(ticker: string, options?: RequestOptions): Promise<StockQuote> {
			return fetchProviderJSON(NAME, LABEL, 'quote', { ticker }, stockQuoteSchema, options);
		},

		getHistoricalPrices(
			ticker: string,
			from: string,
			to: string,
			options?: RequestOptions
		): Promise<OHLCV[]> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'historical-vol',
				{ ticker, from, to, type: 'prices' },
				ohlcvListSchema,
				options
			);
		},

		getOptionsChain(
			ticker: string,
			expiration: string,
			options?: RequestOptions
		): Promise<OptionsChain> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'chain',
				{ ticker, expiration },
				optionsChainSchema,
				options
			);
		},

		getExpirations(ticker: string, options?: RequestOptions): Promise<ExpirationList> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'expirations',
				{ ticker },
				expirationListSchema,
				options
			);
		},

		async getHistoricalVol(_ticker: string): Promise<HistoricalVolData> {
			throw new CapabilityNotSupportedError(NAME, 'historicalVol');
		},

		async getDividendInfo(_ticker: string): Promise<DividendInfo> {
			throw new CapabilityNotSupportedError(NAME, 'dividends');
		},

		async getEarningsInfo(_ticker: string): Promise<EarningsInfo> {
			throw new CapabilityNotSupportedError(NAME, 'earnings');
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			throw new CapabilityNotSupportedError(NAME, 'riskFreeRate');
		},

		searchTickers(
			query: string,
			limit: number = 10,
			options?: RequestOptions
		): Promise<TickerSearchResult[]> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'search',
				{ q: query, limit: String(limit) },
				tickerSearchResultsSchema,
				options
			);
		}
	};
}
