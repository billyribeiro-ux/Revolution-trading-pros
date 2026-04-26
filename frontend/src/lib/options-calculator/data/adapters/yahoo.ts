// ============================================================
// YAHOO FINANCE ADAPTER — Free fallback provider
// Supports quotes, basic options chains, ticker search
// All requests go through SvelteKit API routes
// ============================================================

import type { MarketDataProvider } from '../provider-interface.js';
import { CapabilityNotSupportedError } from '../provider-interface.js';
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
	const BASE = '/tools/options-calculator/api';

	async function fetchAPI<T>(path: string, params: Record<string, string> = {}): Promise<T> {
		const url = new URL(path, window.location.origin);
		url.searchParams.set('provider', 'yahoo');
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, value);
		}
		const response = await fetch(url.toString());
		if (!response.ok) throw new Error(`Yahoo Finance API error: ${response.status}`);
		return response.json();
	}

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

	return {
		name: 'yahoo',
		displayName: 'Yahoo Finance',
		capabilities,

		async isAvailable(): Promise<boolean> {
			try {
				const res = await fetch(`${BASE}/quote?provider=yahoo&ticker=AAPL&healthcheck=true`);
				return res.ok;
			} catch {
				return false;
			}
		},

		async getStatus(): Promise<ProviderStatus> {
			const available = await this.isAvailable();
			return {
				name: 'yahoo',
				displayName: 'Yahoo Finance',
				isAvailable: available,
				isConfigured: true
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
			throw new CapabilityNotSupportedError('yahoo', 'historicalVol');
		},

		async getDividendInfo(ticker: string): Promise<DividendInfo> {
			return fetchAPI<DividendInfo>(`${BASE}/quote`, { ticker, include: 'dividends' });
		},

		async getEarningsInfo(ticker: string): Promise<EarningsInfo> {
			return fetchAPI<EarningsInfo>(`${BASE}/quote`, { ticker, include: 'earnings' });
		},

		async getRiskFreeRate(): Promise<RiskFreeRateData> {
			throw new CapabilityNotSupportedError('yahoo', 'riskFreeRate');
		},

		async searchTickers(query: string, limit: number = 10): Promise<TickerSearchResult[]> {
			return fetchAPI<TickerSearchResult[]>(`${BASE}/search`, { q: query, limit: String(limit) });
		}
	};
}
