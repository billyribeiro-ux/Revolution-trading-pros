// ============================================================
// TRADIER ADAPTER — Free sandbox, good options chain support
// API docs: https://developer.tradier.com/
// All requests go through SvelteKit API routes
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

export function createTradierAdapter(): MarketDataProvider {
	const NAME = 'tradier';
	const LABEL = 'Tradier';

	const capabilities: ProviderCapabilities = {
		quotes: true,
		optionsChain: true,
		historicalPrices: true,
		historicalVol: false,
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
		displayName: 'Tradier',
		capabilities,

		isAvailable,

		async getStatus(options?: RequestOptions): Promise<ProviderStatus> {
			const available = await isAvailable(options);
			return {
				name: NAME,
				displayName: 'Tradier',
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
