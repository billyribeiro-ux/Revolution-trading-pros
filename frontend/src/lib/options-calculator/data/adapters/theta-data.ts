// ============================================================
// THETA DATA ADAPTER — Options data specialist
// API docs: https://www.thetadata.net/
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
	historicalVolSchema,
	ohlcvListSchema,
	optionsChainSchema,
	stockQuoteSchema
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

export function createThetaDataAdapter(): MarketDataProvider {
	const NAME = 'theta-data';
	const LABEL = 'Theta Data';

	const capabilities: ProviderCapabilities = {
		quotes: true,
		optionsChain: true,
		historicalPrices: true,
		historicalVol: true,
		dividends: false,
		earnings: false,
		riskFreeRate: false,
		tickerSearch: false,
		streaming: false
	};

	function isAvailable(options?: RequestOptions): Promise<boolean> {
		return probeProvider(NAME, 'quote', { ticker: 'AAPL' }, options);
	}

	return {
		name: NAME,
		displayName: 'Theta Data',
		capabilities,

		isAvailable,

		async getStatus(options?: RequestOptions): Promise<ProviderStatus> {
			const available = await isAvailable(options);
			return {
				name: NAME,
				displayName: 'Theta Data',
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

		getHistoricalVol(ticker: string, options?: RequestOptions): Promise<HistoricalVolData> {
			return fetchProviderJSON(
				NAME,
				LABEL,
				'historical-vol',
				{ ticker },
				historicalVolSchema,
				options
			);
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

		async searchTickers(_query: string, _limit?: number): Promise<TickerSearchResult[]> {
			throw new CapabilityNotSupportedError(NAME, 'tickerSearch');
		}
	};
}
