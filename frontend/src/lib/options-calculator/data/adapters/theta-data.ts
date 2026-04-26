// ============================================================
// THETA DATA ADAPTER — Options data specialist
// API docs: https://www.thetadata.net/
// All requests go through SvelteKit API routes
// ============================================================

import type { MarketDataProvider } from '../provider-interface.js';
import {
	CapabilityNotSupportedError,
	AuthenticationError,
	RateLimitExceededError
} from '../provider-interface.js';
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
	const BASE = '/tools/options-calculator/api';

	async function fetchAPI<T>(path: string, params: Record<string, string> = {}): Promise<T> {
		const url = new URL(path, window.location.origin);
		url.searchParams.set('provider', 'theta-data');
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, value);
		}
		const response = await fetch(url.toString());
		if (response.status === 401) throw new AuthenticationError('theta-data');
		if (response.status === 429) throw new RateLimitExceededError('theta-data');
		if (!response.ok) throw new Error(`Theta Data API error: ${response.status}`);
		return response.json();
	}

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

	return {
		name: 'theta-data',
		displayName: 'Theta Data',
		capabilities,

		async isAvailable(): Promise<boolean> {
			try {
				const res = await fetch(`${BASE}/quote?provider=theta-data&ticker=AAPL&healthcheck=true`);
				return res.ok;
			} catch {
				return false;
			}
		},

		async getStatus(): Promise<ProviderStatus> {
			const available = await this.isAvailable();
			return {
				name: 'theta-data',
				displayName: 'Theta Data',
				isAvailable: available,
				isConfigured: available
			};
		},

		async getQuote(ticker: string): Promise<StockQuote> {
			return fetchAPI<StockQuote>(`${BASE}/quote`, { ticker });
		},

		async getHistoricalPrices(ticker: string, from: string, to: string): Promise<OHLCV[]> {
			return fetchAPI<OHLCV[]>(`${BASE}/historical-vol`, { ticker, from, to, type: 'prices' });
		},

		async getOptionsChain(ticker: string, expiration: string): Promise<OptionsChain> {
			return fetchAPI<OptionsChain>(`${BASE}/chain`, { ticker, expiration });
		},

		async getExpirations(ticker: string): Promise<ExpirationList> {
			return fetchAPI<ExpirationList>(`${BASE}/expirations`, { ticker });
		},

		async getHistoricalVol(ticker: string): Promise<HistoricalVolData> {
			return fetchAPI<HistoricalVolData>(`${BASE}/historical-vol`, { ticker });
		},

		async getDividendInfo(_ticker: string): Promise<DividendInfo> {
			throw new CapabilityNotSupportedError('theta-data', 'dividends');
		},

		async getEarningsInfo(_ticker: string): Promise<EarningsInfo> {
			throw new CapabilityNotSupportedError('theta-data', 'earnings');
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			throw new CapabilityNotSupportedError('theta-data', 'riskFreeRate');
		},

		async searchTickers(_query: string, _limit?: number): Promise<TickerSearchResult[]> {
			throw new CapabilityNotSupportedError('theta-data', 'tickerSearch');
		}
	};
}
