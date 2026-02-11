// ============================================================
// TRADIER ADAPTER — Free sandbox, good options chain support
// API docs: https://developer.tradier.com/
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

export function createTradierAdapter(): MarketDataProvider {
	const BASE = '/tools/options-calculator/api';

	async function fetchAPI<T>(path: string, params: Record<string, string> = {}): Promise<T> {
		const url = new URL(path, window.location.origin);
		url.searchParams.set('provider', 'tradier');
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, value);
		}
		const response = await fetch(url.toString());
		if (response.status === 401) throw new AuthenticationError('tradier');
		if (response.status === 429) throw new RateLimitExceededError('tradier');
		if (!response.ok) throw new Error(`Tradier API error: ${response.status}`);
		return response.json();
	}

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

	return {
		name: 'tradier',
		displayName: 'Tradier',
		capabilities,

		async isAvailable(): Promise<boolean> {
			try {
				const res = await fetch(`${BASE}/quote?provider=tradier&ticker=AAPL&healthcheck=true`);
				return res.ok;
			} catch {
				return false;
			}
		},

		async getStatus(): Promise<ProviderStatus> {
			const available = await this.isAvailable();
			return {
				name: 'tradier',
				displayName: 'Tradier',
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

		async getHistoricalVol(_ticker: string): Promise<HistoricalVolData> {
			throw new CapabilityNotSupportedError('tradier', 'historicalVol');
		},

		async getDividendInfo(_ticker: string): Promise<DividendInfo> {
			throw new CapabilityNotSupportedError('tradier', 'dividends');
		},

		async getEarningsInfo(_ticker: string): Promise<EarningsInfo> {
			throw new CapabilityNotSupportedError('tradier', 'earnings');
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			throw new CapabilityNotSupportedError('tradier', 'riskFreeRate');
		},

		async searchTickers(query: string, limit: number = 10): Promise<TickerSearchResult[]> {
			return fetchAPI<TickerSearchResult[]>(`${BASE}/search`, { q: query, limit: String(limit) });
		}
	};
}
