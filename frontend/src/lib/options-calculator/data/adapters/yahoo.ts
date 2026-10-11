// ============================================================
// YAHOO FINANCE ADAPTER — Free fallback provider
// Supports quotes, basic options chains, ticker search
// All requests go through SvelteKit API routes
// ============================================================

import type { MarketDataProvider, RequestOptions } from '../provider-interface.js';
import {
	CapabilityNotSupportedError,
	fetchProviderJSON,
	probeProvider
} from '../provider-interface.js';
import {
	dividendInfoSchema,
	earningsInfoSchema,
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

export function createYahooAdapter(): MarketDataProvider {
	const NAME = 'yahoo';
	const LABEL = 'Yahoo Finance';

	const capabilities: ProviderCapabilities = {
		quotes: true,
		optionsChain: true,
		historicalPrices: true,
		historicalVol: false,
		dividends: true,
		earnings: true,
		riskFreeRate: false,
		tickerSearch: true,
		streaming: false
	};

	function isAvailable(options?: RequestOptions): Promise<boolean> {
		return probeProvider(NAME, 'quote', { ticker: 'AAPL' }, options);
	}

	return {
		name: NAME,
		displayName: 'Yahoo Finance',
		capabilities,

		isAvailable,

		async getStatus(options?: RequestOptions): Promise<ProviderStatus> {
			const available = await isAvailable(options);
			return {
				name: NAME,
				displayName: 'Yahoo Finance',
				isAvailable: available,
				isConfigured: true
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

		getDividendInfo(ticker: string, options?: RequestOptions): Promise<DividendInfo> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'quote',
				{ ticker, include: 'dividends' },
				dividendInfoSchema,
				options
			);
		},

		getEarningsInfo(ticker: string, options?: RequestOptions): Promise<EarningsInfo> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'quote',
				{ ticker, include: 'earnings' },
				earningsInfoSchema,
				options
			);
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
