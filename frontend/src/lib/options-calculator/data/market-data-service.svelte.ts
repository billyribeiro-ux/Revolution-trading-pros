// ============================================================
// MARKET DATA SERVICE — Central Data Layer
// Svelte 5 runes-based reactive service.
// Single point of contact for all market data needs.
// ============================================================

import { createProviderRouter } from './provider-router.js';
import { createCache, CACHE_TTL } from './cache.js';
import { createMockAdapter } from './adapters/mock.js';
import { createPolygonAdapter } from './adapters/polygon.js';
import { createFREDAdapter } from './adapters/fred.js';
import type {
import { logger } from '$lib/utils/logger';
	StockQuote,
	OptionsChain,
	ExpirationList,
	TickerSearchResult,
	RiskFreeRateData,
	DividendInfo,
	EarningsInfo,
	HistoricalVolData,
	MarketSnapshot,
	ProviderStatus,
	DataMode
} from './types.js';

/**
 * Create the market data service.
 * Uses Svelte 5 runes for reactive state management.
 */
export function createMarketDataService() {
	// ── State ────────────────────────────────────────────
	let dataMode = $state<DataMode>('manual');
	let activeTicker = $state<string>('');
	let isLoading = $state(false);
	let lastError = $state<string | null>(null);
	let providerStatuses = $state<ProviderStatus[]>([]);

	let currentQuote = $state<StockQuote | null>(null);
	let currentExpirations = $state<ExpirationList | null>(null);
	let currentChain = $state<OptionsChain | null>(null);
	let currentRiskFreeRate = $state<RiskFreeRateData | null>(null);
	let currentDividend = $state<DividendInfo | null>(null);
	let currentEarnings = $state<EarningsInfo | null>(null);
	let currentHistoricalVol = $state<HistoricalVolData | null>(null);
	let searchResults = $state<TickerSearchResult[]>([]);
	let isSearching = $state(false);

	// ── Initialize providers ─────────────────────────────
	const cache = createCache();
	const mockAdapter = createMockAdapter();

	const router = createProviderRouter({
		providers: [createPolygonAdapter(), mockAdapter],
		maxRetries: 1,
		timeoutMs: 8000,
		onFallback(from, to, error) {
			logger.warn(`[MarketData] Falling back from ${from} to ${to}: ${error.message}`);
		},
		onAllFailed(capability, errors) {
			logger.error(`[MarketData] All providers failed for ${capability}:`, errors);
			lastError = `Unable to fetch ${capability}. Please try again.`;
		}
	});

	const fredRouter = createProviderRouter({
		providers: [createFREDAdapter(), mockAdapter],
		maxRetries: 1,
		timeoutMs: 5000
	});

	// ── Debounce utility ─────────────────────────────────
	let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;

	// ── Methods ──────────────────────────────────────────

	async function searchTicker(query: string): Promise<void> {
		if (query.length < 1) {
			searchResults = [];
			isSearching = false;
			return;
		}

		if (searchDebounceTimer) clearTimeout(searchDebounceTimer);
		isSearching = true;

		searchDebounceTimer = setTimeout(async () => {
			try {
				const cacheKey = cache.makeKey('search', query);
				const cached = cache.get<TickerSearchResult[]>(cacheKey);
				if (cached) {
					searchResults = cached;
					isSearching = false;
					return;
				}

				const results = await router.route('searchTickers', (p) => p.searchTickers(query, 8));
				cache.set(cacheKey, results, CACHE_TTL.TICKER_SEARCH, 'mock');
				searchResults = results;
			} catch (error) {
				logger.error('[MarketData] Search failed:', error);
				searchResults = [];
			} finally {
				isSearching = false;
			}
		}, 250);
	}

	async function selectTicker(ticker: string): Promise<MarketSnapshot | null> {
		activeTicker = ticker.toUpperCase();
		isLoading = true;
		lastError = null;
		searchResults = [];

		try {
			const [quote, dividend, earnings, historicalVol, riskFreeRate] = await Promise.allSettled([
				fetchQuote(activeTicker),
				fetchDividend(activeTicker),
				fetchEarnings(activeTicker),
				fetchHistoricalVol(activeTicker),
				fetchRiskFreeRate()
			]);

			const quoteData = quote.status === 'fulfilled' ? quote.value : null;
			const divData = dividend.status === 'fulfilled' ? dividend.value : null;
			const earningsData = earnings.status === 'fulfilled' ? earnings.value : null;
			const hvData = historicalVol.status === 'fulfilled' ? historicalVol.value : null;
			const rateData = riskFreeRate.status === 'fulfilled' ? riskFreeRate.value : null;

			if (!quoteData) {
				lastError = `Could not fetch quote for ${activeTicker}`;
				return null;
			}

			await fetchExpirations(activeTicker);

			const snapshot: MarketSnapshot = {
				quote: quoteData,
				dividends: divData ?? {
					ticker: activeTicker,
					annualDividend: 0,
					dividendYield: 0,
					frequency: 'none',
					source: 'mock'
				},
				earnings: earningsData ?? {
					ticker: activeTicker,
					source: 'mock'
				},
				historicalVol: hvData ?? {
					ticker: activeTicker,
					hv10: 0.25,
					hv20: 0.25,
					hv30: 0.25,
					hv60: 0.25,
					hv90: 0.25,
					source: 'mock'
				},
				riskFreeRate: rateData ?? {
					rate1M: 0.043,
					rate3M: 0.043,
					rate6M: 0.042,
					rate1Y: 0.04,
					rate2Y: 0.039,
					rate10Y: 0.04,
					date: new Date().toISOString().split('T')[0],
					source: 'mock'
				},
				suggestedInputs: {
					spotPrice: quoteData.price,
					volatility: hvData?.hv30 ?? 0.25,
					dividendYield: divData?.dividendYield ?? 0,
					riskFreeRate: rateData?.rate3M ?? 0.043
				}
			};

			return snapshot;
		} catch (error) {
			lastError = `Error loading data for ${activeTicker}`;
			logger.error('[MarketData]', error);
			return null;
		} finally {
			isLoading = false;
		}
	}

	async function fetchQuote(ticker: string): Promise<StockQuote> {
		const cacheKey = cache.makeKey('quote', ticker);
		const cached = cache.get<StockQuote>(cacheKey);
		if (cached) {
			currentQuote = cached;
			return cached;
		}

		const quote = await router.route('getQuote', (p) => p.getQuote(ticker));
		cache.set(cacheKey, quote, CACHE_TTL.QUOTE, quote.source);
		currentQuote = quote;
		return quote;
	}

	async function fetchExpirations(ticker: string): Promise<ExpirationList> {
		const cacheKey = cache.makeKey('expirations', ticker);
		const cached = cache.get<ExpirationList>(cacheKey);
		if (cached) {
			currentExpirations = cached;
			return cached;
		}

		const expirations = await router.route('getExpirations', (p) => p.getExpirations(ticker));
		cache.set(cacheKey, expirations, CACHE_TTL.EXPIRATIONS, expirations.source);
		currentExpirations = expirations;
		return expirations;
	}

	async function fetchOptionsChain(ticker: string, expiration: string): Promise<OptionsChain> {
		const cacheKey = cache.makeKey('chain', ticker, expiration);
		const cached = cache.get<OptionsChain>(cacheKey);
		if (cached) {
			currentChain = cached;
			return cached;
		}

		const chain = await router.route('getOptionsChain', (p) =>
			p.getOptionsChain(ticker, expiration)
		);
		cache.set(cacheKey, chain, CACHE_TTL.OPTIONS_CHAIN, chain.source);
		currentChain = chain;
		return chain;
	}

	async function fetchRiskFreeRate(): Promise<RiskFreeRateData> {
		const cacheKey = cache.makeKey('riskfree', 'treasury');
		const cached = cache.get<RiskFreeRateData>(cacheKey);
		if (cached) {
			currentRiskFreeRate = cached;
			return cached;
		}

		const rate = await fredRouter.route('getRiskFreeRate', (p) => p.getRiskFreeRate());
		cache.set(cacheKey, rate, CACHE_TTL.RISK_FREE_RATE, rate.source);
		currentRiskFreeRate = rate;
		return rate;
	}

	async function fetchDividend(ticker: string): Promise<DividendInfo> {
		const cacheKey = cache.makeKey('dividend', ticker);
		const cached = cache.get<DividendInfo>(cacheKey);
		if (cached) {
			currentDividend = cached;
			return cached;
		}

		const div = await router.route('getDividendInfo', (p) => p.getDividendInfo(ticker));
		cache.set(cacheKey, div, CACHE_TTL.DIVIDEND, div.source);
		currentDividend = div;
		return div;
	}

	async function fetchEarnings(ticker: string): Promise<EarningsInfo> {
		const cacheKey = cache.makeKey('earnings', ticker);
		const cached = cache.get<EarningsInfo>(cacheKey);
		if (cached) {
			currentEarnings = cached;
			return cached;
		}

		const earnings = await router.route('getEarningsInfo', (p) => p.getEarningsInfo(ticker));
		cache.set(cacheKey, earnings, CACHE_TTL.EARNINGS, earnings.source);
		currentEarnings = earnings;
		return earnings;
	}

	async function fetchHistoricalVol(ticker: string): Promise<HistoricalVolData> {
		const cacheKey = cache.makeKey('hv', ticker);
		const cached = cache.get<HistoricalVolData>(cacheKey);
		if (cached) {
			currentHistoricalVol = cached;
			return cached;
		}

		const hv = await router.route('getHistoricalVol', (p) => p.getHistoricalVol(ticker));
		cache.set(cacheKey, hv, CACHE_TTL.HISTORICAL_VOL, hv.source);
		currentHistoricalVol = hv;
		return hv;
	}

	async function refreshProviderStatuses(): Promise<void> {
		providerStatuses = await router.getAllStatuses();
	}

	function setDataMode(mode: DataMode): void {
		dataMode = mode;
	}

	function clearData(): void {
		currentQuote = null;
		currentExpirations = null;
		currentChain = null;
		currentDividend = null;
		currentEarnings = null;
		currentHistoricalVol = null;
		activeTicker = '';
		lastError = null;
	}

	return {
		get dataMode() {
			return dataMode;
		},
		get activeTicker() {
			return activeTicker;
		},
		get isLoading() {
			return isLoading;
		},
		get lastError() {
			return lastError;
		},
		get providerStatuses() {
			return providerStatuses;
		},
		get currentQuote() {
			return currentQuote;
		},
		get currentExpirations() {
			return currentExpirations;
		},
		get currentChain() {
			return currentChain;
		},
		get currentRiskFreeRate() {
			return currentRiskFreeRate;
		},
		get currentDividend() {
			return currentDividend;
		},
		get currentEarnings() {
			return currentEarnings;
		},
		get currentHistoricalVol() {
			return currentHistoricalVol;
		},
		get searchResults() {
			return searchResults;
		},
		get isSearching() {
			return isSearching;
		},

		searchTicker,
		selectTicker,
		fetchQuote,
		fetchExpirations,
		fetchOptionsChain,
		fetchRiskFreeRate,
		fetchDividend,
		fetchEarnings,
		fetchHistoricalVol,
		refreshProviderStatuses,
		setDataMode,
		clearData,
		cache
	};
}

export type MarketDataService = ReturnType<typeof createMarketDataService>;
