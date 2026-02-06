/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Real-Time Price Feed Service
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Polling-based price feed for stock tickers with caching
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+
 *
 * Features:
 * - Real-time price updates via polling
 * - Price caching for offline resilience
 * - Automatic reconnection with exponential backoff
 * - Memory-efficient subscription management
 * - Market hours awareness
 *
 * Built for the next 10 years with extensibility in mind.
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/** Price feed polling interval during market hours (ms) */
const POLLING_INTERVAL_MARKET_OPEN = 15000;

/** Price feed polling interval outside market hours (ms) */
const POLLING_INTERVAL_MARKET_CLOSED = 60000;

/** Maximum age for cached prices before considered stale (ms) */
const PRICE_CACHE_MAX_AGE = 60000;

/** Reconnection delay base (ms) - uses exponential backoff */
const RECONNECT_BASE_DELAY = 1000;

/** Maximum reconnection delay (ms) */
const RECONNECT_MAX_DELAY = 30000;

/** Maximum reconnection attempts before extended backoff */
const MAX_RECONNECT_ATTEMPTS = 5;

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

export interface PriceData {
	/** Stock ticker symbol */
	ticker: string;
	/** Current price */
	price: number;
	/** Price change from previous close */
	change: number;
	/** Percentage change from previous close */
	changePercent: number;
	/** Timestamp when price was last updated */
	timestamp: number;
	/** Whether the market is currently open */
	marketOpen: boolean;
}

export interface PriceFeedState {
	/** Map of ticker to latest price data */
	prices: Map<string, PriceData>;
	/** Whether the price feed is connected */
	isConnected: boolean;
	/** Whether the price feed is loading */
	isLoading: boolean;
	/** Connection error if any */
	error: string | null;
	/** Last update timestamp */
	lastUpdate: number | null;
}

type PriceUpdateCallback = (prices: Map<string, PriceData>) => void;

// ═══════════════════════════════════════════════════════════════════════════════
// MARKET HOURS UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Checks if US stock market is currently open.
 * Market hours: 9:30 AM - 4:00 PM ET, Monday-Friday
 */
function isMarketOpen(): boolean {
	const now = new Date();
	const etOptions: Intl.DateTimeFormatOptions = { timeZone: 'America/New_York' };

	// Get current ET time
	const etTimeString = now.toLocaleString('en-US', {
		...etOptions,
		hour: '2-digit',
		minute: '2-digit',
		hour12: false
	});
	const [hours, minutes] = etTimeString.split(':').map(Number);
	const currentMinutes = hours * 60 + minutes;

	// Market hours: 9:30 AM (570 min) - 4:00 PM (960 min) ET
	const marketOpen = 9 * 60 + 30; // 9:30 AM
	const marketClose = 16 * 60; // 4:00 PM

	// Check if weekday
	const dayOfWeek = new Date(
		now.toLocaleString('en-US', { ...etOptions, weekday: 'short' })
	).getDay();
	const isWeekday = dayOfWeek >= 1 && dayOfWeek <= 5;

	return isWeekday && currentMinutes >= marketOpen && currentMinutes < marketClose;
}

/**
 * Gets the appropriate polling interval based on market hours.
 */
function getPollingInterval(): number {
	return isMarketOpen() ? POLLING_INTERVAL_MARKET_OPEN : POLLING_INTERVAL_MARKET_CLOSED;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PRICE FEED SERVICE CLASS
// ═══════════════════════════════════════════════════════════════════════════════

class PriceFeedService {
	private static instance: PriceFeedService | null = null;

	private state: PriceFeedState = {
		prices: new Map(),
		isConnected: false,
		isLoading: false,
		error: null,
		lastUpdate: null
	};

	private subscribedTickers: Set<string> = new Set();
	private pollingInterval: ReturnType<typeof setInterval> | null = null;
	private reconnectAttempts = 0;
	private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
	private callbacks: Set<PriceUpdateCallback> = new Set();

	private constructor() {
		logger.debug('[PriceFeed] Service initialized');
	}

	static getInstance(): PriceFeedService {
		if (!PriceFeedService.instance) {
			PriceFeedService.instance = new PriceFeedService();
		}
		return PriceFeedService.instance;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE ACCESS
	// ═══════════════════════════════════════════════════════════════════════════

	getState(): Readonly<PriceFeedState> {
		return this.state;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// SUBSCRIPTION MANAGEMENT
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Register a callback for price updates.
	 */
	onUpdate(callback: PriceUpdateCallback): () => void {
		this.callbacks.add(callback);
		return () => this.callbacks.delete(callback);
	}

	/**
	 * Subscribe to price updates for given tickers.
	 */
	subscribe(tickers: string[]): void {
		if (!browser) return;

		const normalizedTickers = tickers.map((t) => t.toUpperCase().trim()).filter(Boolean);
		const newTickers = normalizedTickers.filter((t) => !this.subscribedTickers.has(t));

		for (const ticker of normalizedTickers) {
			this.subscribedTickers.add(ticker);
		}

		logger.debug('[PriceFeed] Subscribed to:', normalizedTickers);

		// Fetch prices for new tickers immediately
		if (newTickers.length > 0) {
			this.fetchPrices(newTickers);
		}

		// Start polling if not already running
		if (this.subscribedTickers.size > 0 && !this.pollingInterval) {
			this.startPolling();
		}
	}

	/**
	 * Unsubscribe from price updates for given tickers.
	 */
	unsubscribe(tickers: string[]): void {
		const normalizedTickers = tickers.map((t) => t.toUpperCase().trim());

		for (const ticker of normalizedTickers) {
			this.subscribedTickers.delete(ticker);
		}

		logger.debug('[PriceFeed] Unsubscribed from:', normalizedTickers);

		// Stop polling if no more subscriptions
		if (this.subscribedTickers.size === 0) {
			this.stopPolling();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PRICE ACCESS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get current price data for a ticker.
	 */
	getPrice(ticker: string): PriceData | null {
		const normalizedTicker = ticker.toUpperCase().trim();
		const priceData = this.state.prices.get(normalizedTicker);

		if (!priceData) return null;

		// Check if price is stale
		const age = Date.now() - priceData.timestamp;
		if (age > PRICE_CACHE_MAX_AGE) {
			logger.debug(`[PriceFeed] Stale price for ${normalizedTicker}, age: ${age}ms`);
		}

		return priceData;
	}

	/**
	 * Get current price value for a ticker (convenience method).
	 */
	getPriceValue(ticker: string): number | null {
		const priceData = this.getPrice(ticker);
		return priceData?.price ?? null;
	}

	/**
	 * Get all current prices.
	 */
	getAllPrices(): Map<string, PriceData> {
		return new Map(this.state.prices);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PRICE FETCHING
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Fetches current prices from the API for given tickers.
	 */
	private async fetchPrices(tickers: string[]): Promise<void> {
		if (!browser || tickers.length === 0) return;

		this.state.isLoading = true;
		this.state.error = null;

		try {
			const tickerList = tickers.join(',');
			const response = await fetch(`/api/prices?tickers=${encodeURIComponent(tickerList)}`);

			if (!response.ok) {
				throw new Error(`Price fetch failed: ${response.status}`);
			}

			const data = await response.json();

			if (data.success && Array.isArray(data.prices)) {
				const now = Date.now();
				const marketOpen = isMarketOpen();

				for (const priceData of data.prices as Partial<PriceData>[]) {
					if (priceData.ticker && typeof priceData.price === 'number') {
						this.state.prices.set(priceData.ticker.toUpperCase(), {
							ticker: priceData.ticker.toUpperCase(),
							price: priceData.price,
							change: priceData.change ?? 0,
							changePercent: priceData.changePercent ?? 0,
							timestamp: now,
							marketOpen
						});
					}
				}

				this.state.lastUpdate = now;
				this.state.isConnected = true;
				this.reconnectAttempts = 0;

				// Notify all callbacks
				this.notifyCallbacks();

				logger.debug('[PriceFeed] Prices updated:', this.state.prices.size, 'tickers');
			}
		} catch (e) {
			const errorMessage = e instanceof Error ? e.message : 'Failed to fetch prices';
			this.state.error = errorMessage;
			logger.warn('[PriceFeed] Fetch error:', errorMessage);

			// Schedule reconnect on failure
			if (this.reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
				this.scheduleReconnect();
			}
		} finally {
			this.state.isLoading = false;
		}
	}

	/**
	 * Force refresh prices for all subscribed tickers.
	 */
	async refresh(): Promise<void> {
		if (this.subscribedTickers.size > 0) {
			await this.fetchPrices(Array.from(this.subscribedTickers));
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// POLLING MANAGEMENT
	// ═══════════════════════════════════════════════════════════════════════════

	private startPolling(): void {
		if (this.pollingInterval) return;

		const interval = getPollingInterval();
		logger.debug(`[PriceFeed] Starting polling with ${interval}ms interval`);

		this.pollingInterval = setInterval(() => {
			if (this.subscribedTickers.size > 0) {
				this.fetchPrices(Array.from(this.subscribedTickers));
			}
		}, interval);
	}

	private stopPolling(): void {
		if (this.pollingInterval) {
			clearInterval(this.pollingInterval);
			this.pollingInterval = null;
			logger.debug('[PriceFeed] Stopped polling');
		}
	}

	private scheduleReconnect(): void {
		if (this.reconnectTimeout) {
			clearTimeout(this.reconnectTimeout);
		}

		this.reconnectAttempts++;
		const delay = Math.min(
			RECONNECT_BASE_DELAY * Math.pow(2, this.reconnectAttempts - 1),
			RECONNECT_MAX_DELAY
		);

		logger.debug(
			`[PriceFeed] Scheduling reconnect attempt ${this.reconnectAttempts} in ${delay}ms`
		);

		this.reconnectTimeout = setTimeout(() => {
			if (this.subscribedTickers.size > 0) {
				this.fetchPrices(Array.from(this.subscribedTickers));
			}
		}, delay);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// CALLBACK MANAGEMENT
	// ═══════════════════════════════════════════════════════════════════════════

	private notifyCallbacks(): void {
		for (const callback of this.callbacks) {
			try {
				callback(this.state.prices);
			} catch (e) {
				logger.warn('[PriceFeed] Callback error:', e);
			}
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// CLEANUP
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Disconnect and cleanup all resources.
	 */
	disconnect(): void {
		this.stopPolling();

		if (this.reconnectTimeout) {
			clearTimeout(this.reconnectTimeout);
			this.reconnectTimeout = null;
		}

		this.subscribedTickers.clear();
		this.callbacks.clear();
		this.state.isConnected = false;
		this.state.error = null;

		logger.debug('[PriceFeed] Disconnected');
	}

	/**
	 * Reset the singleton instance.
	 */
	static reset(): void {
		if (PriceFeedService.instance) {
			PriceFeedService.instance.disconnect();
			PriceFeedService.instance = null;
		}
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Gets the global price feed service instance.
 */
export function getPriceFeed(): PriceFeedService {
	return PriceFeedService.getInstance();
}

/**
 * Resets the global price feed service.
 */
export function resetPriceFeed(): void {
	PriceFeedService.reset();
}

export { PriceFeedService };
export default getPriceFeed;
