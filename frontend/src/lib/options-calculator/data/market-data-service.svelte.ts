// ============================================================
// MARKET DATA SERVICE — Central Data Layer
// Svelte 5 runes-based reactive service.
// Single point of contact for all market data needs.
// ============================================================

import type * as v from 'valibot';
import { on } from 'svelte/events';
import { createProviderRouter } from './provider-router.js';
import { createCache, CACHE_TTL } from './cache.js';
import { createMockAdapter } from './adapters/mock.js';
import { createPolygonAdapter } from './adapters/polygon.js';
import { createFREDAdapter } from './adapters/fred.js';
import {
	dividendInfoSchema,
	earningsInfoSchema,
	expirationListSchema,
	historicalVolSchema,
	optionsChainSchema,
	riskFreeRateSchema,
	stockQuoteSchema,
	tickerSearchResultsSchema
} from './schemas.js';
import { logger } from '#lib/utils/logger.js';
import type { RequestOptions } from './provider-interface.js';
import type {
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
	DataMode,
	DataProviderName
} from './types.js';

const SEARCH_DEBOUNCE_MS = 250;
const SEARCH_LIMIT = 8;

/**
 * A latest-wins request slot: starting a request aborts the one still in
 * flight, so a slow, superseded response can never overwrite a newer one.
 */
class RequestSlot {
	#controller: AbortController | null = null;

	/** Abort the in-flight request (if any) and track a new one. */
	start(): AbortController {
		this.#controller?.abort();
		this.#controller = new AbortController();
		return this.#controller;
	}

	/** Whether `controller` is still this slot's newest request. */
	owns(controller: AbortController): boolean {
		return this.#controller === controller;
	}

	/** Forget `controller` once it settled; a no-op if it was superseded. */
	release(controller: AbortController): void {
		if (this.#controller === controller) this.#controller = null;
	}

	/** Abort whatever is in flight. */
	abort(): void {
		this.#controller?.abort();
		this.#controller = null;
	}
}

/** A cache-first read that falls back to a provider router. */
interface CachedRequest<T> {
	key: string;
	/** Validates a payload read back from localStorage. */
	schema: v.GenericSchema<unknown, T>;
	ttl: number;
	/** Fetches through a provider router on a miss. */
	load: () => Promise<T>;
	/** The provider credited in the cache entry. */
	sourceOf: (value: T) => DataProviderName;
	signal?: AbortSignal;
}

const bySource = (value: { source: DataProviderName }): DataProviderName => value.source;

function settledValue<T>(result: PromiseSettledResult<T>): T | null {
	return result.status === 'fulfilled' ? result.value : null;
}

/** Assemble a snapshot, filling any secondary data that failed to load with neutral defaults. */
function buildSnapshot(
	ticker: string,
	data: {
		quote: StockQuote;
		dividend: DividendInfo | null;
		earnings: EarningsInfo | null;
		historicalVol: HistoricalVolData | null;
		riskFreeRate: RiskFreeRateData | null;
	}
): MarketSnapshot {
	const { quote, dividend, earnings, historicalVol, riskFreeRate } = data;
	return {
		quote,
		dividends: dividend ?? {
			ticker,
			annualDividend: 0,
			dividendYield: 0,
			frequency: 'none',
			source: 'mock'
		},
		earnings: earnings ?? {
			ticker,
			source: 'mock'
		},
		historicalVol: historicalVol ?? {
			ticker,
			hv10: 0.25,
			hv20: 0.25,
			hv30: 0.25,
			hv60: 0.25,
			hv90: 0.25,
			source: 'mock'
		},
		riskFreeRate: riskFreeRate ?? {
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
			spotPrice: quote.price,
			volatility: historicalVol?.hv30 ?? 0.25,
			dividendYield: dividend?.dividendYield ?? 0,
			riskFreeRate: riskFreeRate?.rate3M ?? 0.043
		}
	};
}

/**
 * Reactive market data service: quotes, chains and reference data with
 * provider failover and a two-tier cache.
 *
 * Construct it during component initialisation (`Calculator.svelte` does, and
 * shares it through `setMarketData`): the owning component's lifetime bounds
 * the search debounce timer and every request still in flight.
 */
export class MarketDataService {
	// ── State (read-only outside the class) ──────────────
	#dataMode = $state<DataMode>('manual');
	#activeTicker = $state('');
	#isLoading = $state(false);
	#lastError = $state<string | null>(null);
	#isSearching = $state(false);

	// Fetched payloads are only ever replaced, never mutated: no deep proxies.
	#providerStatuses = $state.raw<ProviderStatus[]>([]);
	#currentQuote = $state.raw<StockQuote | null>(null);
	#currentExpirations = $state.raw<ExpirationList | null>(null);
	#currentChain = $state.raw<OptionsChain | null>(null);
	#currentRiskFreeRate = $state.raw<RiskFreeRateData | null>(null);
	#currentDividend = $state.raw<DividendInfo | null>(null);
	#currentEarnings = $state.raw<EarningsInfo | null>(null);
	#currentHistoricalVol = $state.raw<HistoricalVolData | null>(null);
	#searchResults = $state.raw<TickerSearchResult[]>([]);

	get dataMode() {
		return this.#dataMode;
	}
	get activeTicker() {
		return this.#activeTicker;
	}
	get isLoading() {
		return this.#isLoading;
	}
	get lastError() {
		return this.#lastError;
	}
	get providerStatuses() {
		return this.#providerStatuses;
	}
	get currentQuote() {
		return this.#currentQuote;
	}
	get currentExpirations() {
		return this.#currentExpirations;
	}
	get currentChain() {
		return this.#currentChain;
	}
	get currentRiskFreeRate() {
		return this.#currentRiskFreeRate;
	}
	get currentDividend() {
		return this.#currentDividend;
	}
	get currentEarnings() {
		return this.#currentEarnings;
	}
	get currentHistoricalVol() {
		return this.#currentHistoricalVol;
	}
	get searchResults() {
		return this.#searchResults;
	}
	get isSearching() {
		return this.#isSearching;
	}

	// ── Providers ────────────────────────────────────────
	readonly cache = createCache();
	#mockAdapter = createMockAdapter();

	#router = createProviderRouter({
		providers: [createPolygonAdapter(), this.#mockAdapter],
		maxRetries: 1,
		timeoutMs: 8000,
		onFallback: (from, to, error) => {
			logger.warn(`[MarketData] Falling back from ${from} to ${to}: ${error.message}`);
		},
		onAllFailed: (capability, errors) => {
			logger.error(`[MarketData] All providers failed for ${capability}:`, errors);
			this.#lastError = `Unable to fetch ${capability}. Please try again.`;
		}
	});

	#fredRouter = createProviderRouter({
		providers: [createFREDAdapter(), this.#mockAdapter],
		maxRetries: 1,
		timeoutMs: 5000
	});

	// ── In-flight work ───────────────────────────────────
	#searchTimer: ReturnType<typeof setTimeout> | undefined;
	#search = new RequestSlot();
	#selection = new RequestSlot();
	#chain = new RequestSlot();

	constructor() {
		// When the owning component unmounts, drop the pending search and abort
		// every request, so nothing keeps running against a discarded service.
		$effect(() => () => this.#cancelAll());
	}

	// ── Methods ──────────────────────────────────────────

	/**
	 * Debounced ticker search. Each call supersedes the previous one: its timer
	 * is cleared and its request aborted. Resolves once the search is scheduled.
	 */
	searchTicker = async (query: string): Promise<void> => {
		this.#cancelSearch();

		if (query.length < 1) {
			this.#searchResults = [];
			this.#isSearching = false;
			return;
		}

		this.#isSearching = true;
		const controller = this.#search.start();
		this.#searchTimer = setTimeout(() => {
			this.#searchTimer = undefined;
			void this.#runSearch(query, controller);
		}, SEARCH_DEBOUNCE_MS);
	};

	/**
	 * Load everything the calculator needs for `ticker`. Selecting another
	 * ticker (or clearing) while this runs aborts it and resolves it to `null`.
	 */
	selectTicker = async (ticker: string): Promise<MarketSnapshot | null> => {
		const symbol = ticker.toUpperCase();

		// A selection supersedes a pending search and the previous ticker's requests.
		this.#cancelSearch();
		this.#chain.abort();
		const controller = this.#selection.start();
		const { signal } = controller;

		this.#activeTicker = symbol;
		this.#isLoading = true;
		this.#isSearching = false;
		this.#lastError = null;
		this.#searchResults = [];
		// The previous ticker's chain and expirations must not show (or drive mispricing
		// alerts) under the new ticker's badge while its data loads.
		this.#currentChain = null;
		this.#currentExpirations = null;

		try {
			const [quote, dividend, earnings, historicalVol, riskFreeRate] = await Promise.allSettled([
				this.fetchQuote(symbol, { signal }),
				this.fetchDividend(symbol, { signal }),
				this.fetchEarnings(symbol, { signal }),
				this.fetchHistoricalVol(symbol, { signal }),
				this.fetchRiskFreeRate({ signal })
			]);
			if (signal.aborted) return null;

			const quoteData = settledValue(quote);
			if (!quoteData) {
				this.#lastError = `Could not fetch quote for ${symbol}`;
				return null;
			}

			await this.fetchExpirations(symbol, { signal });

			return buildSnapshot(symbol, {
				quote: quoteData,
				dividend: settledValue(dividend),
				earnings: settledValue(earnings),
				historicalVol: settledValue(historicalVol),
				riskFreeRate: settledValue(riskFreeRate)
			});
		} catch (error) {
			if (signal.aborted) return null;
			this.#lastError = `Error loading data for ${symbol}`;
			logger.error('[MarketData]', error);
			return null;
		} finally {
			if (this.#selection.owns(controller)) {
				this.#selection.release(controller);
				this.#isLoading = false;
			}
		}
	};

	fetchQuote = async (ticker: string, { signal }: RequestOptions = {}): Promise<StockQuote> => {
		const quote = await this.#cached({
			key: this.cache.makeKey('quote', ticker),
			schema: stockQuoteSchema,
			ttl: CACHE_TTL.QUOTE,
			load: () =>
				this.#router.route('getQuote', (p, s) => p.getQuote(ticker, { signal: s }), { signal }),
			sourceOf: bySource,
			signal
		});
		this.#currentQuote = quote;
		return quote;
	};

	fetchExpirations = async (
		ticker: string,
		{ signal }: RequestOptions = {}
	): Promise<ExpirationList> => {
		const expirations = await this.#cached({
			key: this.cache.makeKey('expirations', ticker),
			schema: expirationListSchema,
			ttl: CACHE_TTL.EXPIRATIONS,
			load: () =>
				this.#router.route('getExpirations', (p, s) => p.getExpirations(ticker, { signal: s }), {
					signal
				}),
			sourceOf: bySource,
			signal
		});
		this.#currentExpirations = expirations;
		return expirations;
	};

	/**
	 * Latest wins: requesting another chain (or selecting another ticker)
	 * aborts the one in flight, which then rejects with an `AbortError`.
	 * An optional caller `signal` (e.g. `getAbortSignal()`) aborts it too.
	 */
	fetchOptionsChain = async (
		ticker: string,
		expiration: string,
		{ signal }: RequestOptions = {}
	): Promise<OptionsChain> => {
		signal?.throwIfAborted();
		const controller = this.#chain.start();
		const stopForwarding = signal
			? on(signal, 'abort', () => controller.abort(signal.reason))
			: undefined;

		try {
			const chain = await this.#cached({
				key: this.cache.makeKey('chain', ticker, expiration),
				schema: optionsChainSchema,
				ttl: CACHE_TTL.OPTIONS_CHAIN,
				load: () =>
					this.#router.route(
						'getOptionsChain',
						(p, s) => p.getOptionsChain(ticker, expiration, { signal: s }),
						{ signal: controller.signal }
					),
				sourceOf: bySource,
				signal: controller.signal
			});
			this.#currentChain = chain;
			return chain;
		} finally {
			stopForwarding?.();
			this.#chain.release(controller);
		}
	};

	fetchRiskFreeRate = async ({ signal }: RequestOptions = {}): Promise<RiskFreeRateData> => {
		const rate = await this.#cached({
			key: this.cache.makeKey('riskfree', 'treasury'),
			schema: riskFreeRateSchema,
			ttl: CACHE_TTL.RISK_FREE_RATE,
			load: () =>
				this.#fredRouter.route('getRiskFreeRate', (p, s) => p.getRiskFreeRate({ signal: s }), {
					signal
				}),
			sourceOf: bySource,
			signal
		});
		this.#currentRiskFreeRate = rate;
		return rate;
	};

	fetchDividend = async (
		ticker: string,
		{ signal }: RequestOptions = {}
	): Promise<DividendInfo> => {
		const dividend = await this.#cached({
			key: this.cache.makeKey('dividend', ticker),
			schema: dividendInfoSchema,
			ttl: CACHE_TTL.DIVIDEND,
			load: () =>
				this.#router.route('getDividendInfo', (p, s) => p.getDividendInfo(ticker, { signal: s }), {
					signal
				}),
			sourceOf: bySource,
			signal
		});
		this.#currentDividend = dividend;
		return dividend;
	};

	fetchEarnings = async (
		ticker: string,
		{ signal }: RequestOptions = {}
	): Promise<EarningsInfo> => {
		const earnings = await this.#cached({
			key: this.cache.makeKey('earnings', ticker),
			schema: earningsInfoSchema,
			ttl: CACHE_TTL.EARNINGS,
			load: () =>
				this.#router.route('getEarningsInfo', (p, s) => p.getEarningsInfo(ticker, { signal: s }), {
					signal
				}),
			sourceOf: bySource,
			signal
		});
		this.#currentEarnings = earnings;
		return earnings;
	};

	fetchHistoricalVol = async (
		ticker: string,
		{ signal }: RequestOptions = {}
	): Promise<HistoricalVolData> => {
		const historicalVol = await this.#cached({
			key: this.cache.makeKey('hv', ticker),
			schema: historicalVolSchema,
			ttl: CACHE_TTL.HISTORICAL_VOL,
			load: () =>
				this.#router.route(
					'getHistoricalVol',
					(p, s) => p.getHistoricalVol(ticker, { signal: s }),
					{ signal }
				),
			sourceOf: bySource,
			signal
		});
		this.#currentHistoricalVol = historicalVol;
		return historicalVol;
	};

	refreshProviderStatuses = async (): Promise<void> => {
		this.#providerStatuses = await this.#router.getAllStatuses();
	};

	setDataMode = (mode: DataMode): void => {
		this.#dataMode = mode;
	};

	/** Reset the selection, aborting any selection or chain request still in flight. */
	clearData = (): void => {
		this.#selection.abort();
		this.#chain.abort();
		this.#isLoading = false;
		this.#currentQuote = null;
		this.#currentExpirations = null;
		this.#currentChain = null;
		this.#currentDividend = null;
		this.#currentEarnings = null;
		this.#currentHistoricalVol = null;
		this.#activeTicker = '';
		this.#lastError = null;
	};

	// ── Internals ────────────────────────────────────────

	/**
	 * Cache-first read. A hit is returned as is (persisted hits are validated by
	 * the cache); a miss is loaded and cached. Rejects with the abort reason if
	 * `signal` aborted meanwhile, so the caller never commits a stale result.
	 */
	async #cached<T>({ key, schema, ttl, load, sourceOf, signal }: CachedRequest<T>): Promise<T> {
		signal?.throwIfAborted();
		const hit = this.cache.get(key, schema);
		if (hit !== null) return hit;

		const fresh = await load();
		this.cache.set(key, fresh, ttl, sourceOf(fresh));
		signal?.throwIfAborted();
		return fresh;
	}

	async #runSearch(query: string, controller: AbortController): Promise<void> {
		const { signal } = controller;
		try {
			const results = await this.#cached({
				key: this.cache.makeKey('search', query),
				schema: tickerSearchResultsSchema,
				ttl: CACHE_TTL.TICKER_SEARCH,
				load: () =>
					this.#router.route(
						'searchTickers',
						(p, s) => p.searchTickers(query, SEARCH_LIMIT, { signal: s }),
						{ signal }
					),
				sourceOf: () => this.#router.getActiveProvider('searchTickers') ?? 'mock',
				signal
			});
			this.#searchResults = results;
		} catch (error) {
			if (signal.aborted) return;
			logger.error('[MarketData] Search failed:', error);
			this.#searchResults = [];
		} finally {
			if (this.#search.owns(controller)) {
				this.#search.release(controller);
				this.#isSearching = false;
			}
		}
	}

	#cancelSearch(): void {
		clearTimeout(this.#searchTimer);
		this.#searchTimer = undefined;
		this.#search.abort();
	}

	#cancelAll(): void {
		this.#cancelSearch();
		this.#selection.abort();
		this.#chain.abort();
	}
}
