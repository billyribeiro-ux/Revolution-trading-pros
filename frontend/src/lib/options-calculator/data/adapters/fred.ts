// ============================================================
// FRED API ADAPTER — Federal Reserve Economic Data
// ONLY supports getRiskFreeRate() — Treasury yields
// All other methods throw CapabilityNotSupportedError
// ============================================================

import type { MarketDataProvider, RequestOptions } from '../provider-interface.js';
import {
	CapabilityNotSupportedError,
	fetchProviderJSON,
	probeProvider
} from '../provider-interface.js';
import { riskFreeRateSchema } from '../schemas.js';
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

export function createFREDAdapter(): MarketDataProvider {
	const NAME = 'fred';

	const capabilities: ProviderCapabilities = {
		quotes: false,
		optionsChain: false,
		historicalPrices: false,
		historicalVol: false,
		dividends: false,
		earnings: false,
		riskFreeRate: true,
		tickerSearch: false,
		streaming: false
	};

	function isAvailable(options?: RequestOptions): Promise<boolean> {
		return probeProvider(NAME, 'treasury', {}, options);
	}

	return {
		name: NAME,
		displayName: 'FRED (Federal Reserve)',
		capabilities,

		isAvailable,

		async getStatus(options?: RequestOptions): Promise<ProviderStatus> {
			const available = await isAvailable(options);
			return {
				name: NAME,
				displayName: 'FRED (Federal Reserve)',
				isAvailable: available,
				isConfigured: available
			};
		},

		async getQuote(_ticker: string): Promise<StockQuote> {
			throw new CapabilityNotSupportedError(NAME, 'quotes');
		},

		async getHistoricalPrices(_ticker: string, _from: string, _to: string): Promise<OHLCV[]> {
			throw new CapabilityNotSupportedError(NAME, 'historicalPrices');
		},

		async getOptionsChain(_ticker: string, _expiration: string): Promise<OptionsChain> {
			throw new CapabilityNotSupportedError(NAME, 'optionsChain');
		},

		async getExpirations(_ticker: string): Promise<ExpirationList> {
			throw new CapabilityNotSupportedError(NAME, 'expirations');
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

		getRiskFreeRate(options?: RequestOptions): Promise<RiskFreeRateData> {
			return fetchProviderJSON(NAME, 'FRED', 'treasury', {}, riskFreeRateSchema, options);
		},

		async searchTickers(_query: string, _limit?: number): Promise<TickerSearchResult[]> {
			throw new CapabilityNotSupportedError(NAME, 'tickerSearch');
		}
	};
}
