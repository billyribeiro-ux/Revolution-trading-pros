// ============================================================
// POLYGON.IO ADAPTER
// Primary provider — real-time quotes, options, historical data
// All requests go through SvelteKit API routes to keep keys secure
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

export function createPolygonAdapter(): MarketDataProvider {
	const BASE = '/tools/options-calculator/api';

	async function fetchAPI<T>(path: string, params: Record<string, string> = {}): Promise<T> {
		const url = new URL(path, window.location.origin);
		url.searchParams.set('provider', 'polygon');
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, value);
		}

		const response = await fetch(url.toString());

		if (response.status === 401) throw new AuthenticationError('polygon');
		if (response.status === 429) {
			const resetAt = response.headers.get('X-RateLimit-Reset') ?? undefined;
			throw new RateLimitExceededError('polygon', resetAt);
		}
		if (!response.ok) {
			throw new Error(`Polygon API error: ${response.status} ${response.statusText}`);
		}

		return response.json();
	}

	const capabilities: ProviderCapabilities = {
		quotes: true,
		optionsChain: true,
		historicalPrices: true,
		historicalVol: false,
		dividends: true,
		earnings: false,
		riskFreeRate: false,
		tickerSearch: true,
		streaming: true
	};

	return {
		name: 'polygon',
		displayName: 'Polygon.io',
		capabilities,

		async isAvailable(): Promise<boolean> {
			try {
				const res = await fetch(`${BASE}/quote?provider=polygon&ticker=AAPL&healthcheck=true`);
				return res.ok;
			} catch {
				return false;
			}
		},

		async getStatus(): Promise<ProviderStatus> {
			const available = await this.isAvailable();
			return {
				name: 'polygon',
				displayName: 'Polygon.io',
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
			throw new CapabilityNotSupportedError('polygon', 'historicalVol');
		},

		async getDividendInfo(ticker: string): Promise<DividendInfo> {
			return fetchAPI<DividendInfo>(`${BASE}/quote`, { ticker, include: 'dividends' });
		},

		async getEarningsInfo(_ticker: string): Promise<EarningsInfo> {
			throw new CapabilityNotSupportedError('polygon', 'earnings');
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			throw new CapabilityNotSupportedError('polygon', 'riskFreeRate');
		},

		async searchTickers(query: string, limit: number = 10): Promise<TickerSearchResult[]> {
			return fetchAPI<TickerSearchResult[]>(`${BASE}/search`, { q: query, limit: String(limit) });
		}
	};
}
