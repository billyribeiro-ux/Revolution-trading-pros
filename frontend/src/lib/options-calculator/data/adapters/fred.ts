// ============================================================
// FRED API ADAPTER — Federal Reserve Economic Data
// ONLY supports getRiskFreeRate() — Treasury yields
// All other methods throw CapabilityNotSupportedError
// ============================================================

import type { MarketDataProvider } from '../provider-interface.js';
import { CapabilityNotSupportedError, AuthenticationError } from '../provider-interface.js';
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
	const BASE = '/tools/options-calculator/api';

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

	return {
		name: 'fred',
		displayName: 'FRED (Federal Reserve)',
		capabilities,

		async isAvailable(): Promise<boolean> {
			try {
				const res = await fetch(`${BASE}/treasury?provider=fred&healthcheck=true`);
				return res.ok;
			} catch {
				return false;
			}
		},

		async getStatus(): Promise<ProviderStatus> {
			const available = await this.isAvailable();
			return {
				name: 'fred',
				displayName: 'FRED (Federal Reserve)',
				isAvailable: available,
				isConfigured: available
			};
		},

		async getQuote(_ticker: string): Promise<StockQuote> {
			throw new CapabilityNotSupportedError('fred', 'quotes');
		},

		async getHistoricalPrices(_ticker: string, _from: string, _to: string): Promise<OHLCV[]> {
			throw new CapabilityNotSupportedError('fred', 'historicalPrices');
		},

		async getOptionsChain(_ticker: string, _expiration: string): Promise<OptionsChain> {
			throw new CapabilityNotSupportedError('fred', 'optionsChain');
		},

		async getExpirations(_ticker: string): Promise<ExpirationList> {
			throw new CapabilityNotSupportedError('fred', 'expirations');
		},

		async getHistoricalVol(_ticker: string): Promise<HistoricalVolData> {
			throw new CapabilityNotSupportedError('fred', 'historicalVol');
		},

		async getDividendInfo(_ticker: string): Promise<DividendInfo> {
			throw new CapabilityNotSupportedError('fred', 'dividends');
		},

		async getEarningsInfo(_ticker: string): Promise<EarningsInfo> {
			throw new CapabilityNotSupportedError('fred', 'earnings');
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			const response = await fetch(`${BASE}/treasury?provider=fred`);
			if (response.status === 401) throw new AuthenticationError('fred');
			if (!response.ok) throw new Error(`FRED API error: ${response.status}`);
			return response.json();
		},

		async searchTickers(_query: string, _limit?: number): Promise<TickerSearchResult[]> {
			throw new CapabilityNotSupportedError('fred', 'tickerSearch');
		}
	};
}
