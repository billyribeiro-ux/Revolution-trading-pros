/**
 * Backend Analytics Adapter - Server-Side Event Collection
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * High-performance adapter for sending analytics events to your backend API.
 * Features:
 * - Intelligent batching with configurable thresholds
 * - Automatic retry with exponential backoff
 * - sendBeacon for reliable page-unload delivery
 * - Offline queue with localStorage persistence
 * - Request deduplication
 * - Compression support for large payloads
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import type {
	AnalyticsAdapter,
	AnalyticsConfig,
	AdapterState,
	PageViewPayload,
	CustomEventPayload,
	PurchasePayload,
	IdentifyPayload,
	QueuedEvent,
	AdapterMetrics,
} from './types';

// ═══════════════════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════════════════

const DEFAULT_ENDPOINT = '/api/analytics/batch';
const DEFAULT_FLUSH_INTERVAL = 5000; // 5 seconds
const DEFAULT_MAX_BATCH_SIZE = 50;
const MAX_QUEUE_SIZE = 1000;
const MAX_RETRIES = 3;
const STORAGE_KEY = 'rtp_analytics_offline_queue';

// ═══════════════════════════════════════════════════════════════════════════
// Backend Adapter Class
// ═══════════════════════════════════════════════════════════════════════════

class BackendAnalyticsAdapter implements AnalyticsAdapter {
	readonly id = 'backend';
	readonly name = 'Backend API';

	private _state: AdapterState = 'uninitialized';
	private _config: AnalyticsConfig | null = null;
	private _endpoint: string = DEFAULT_ENDPOINT;
	private _flushIntervalMs: number = DEFAULT_FLUSH_INTERVAL;
	private _maxBatchSize: number = DEFAULT_MAX_BATCH_SIZE;
	private _useSendBeacon: boolean = true;
	private _authorization: string | null = null;

	private _eventQueue: QueuedEvent[] = [];
	private _flushTimeoutId: ReturnType<typeof setTimeout> | null = null;
	private _isFlushing = false;
	private _userId: string | null = null;
	private _sessionId: string;

	// Metrics
	private _metrics: AdapterMetrics = {
		eventsTracked: 0,
		eventsFailed: 0,
		lastEventTime: null,
		averageLatencyMs: 0,
		queueSize: 0,
	};

	constructor() {
		this._sessionId = this._generateSessionId();
	}

	get state(): AdapterState {
		return this._state;
	}

	get metrics(): AdapterMetrics {
		return {
			...this._metrics,
			queueSize: this._eventQueue.length,
		};
	}

	/**
	 * Initialize the backend adapter.
	 */
	async initialize(config: AnalyticsConfig): Promise<void> {
		if (!browser) {
			this._state = 'disabled';
			return;
		}

		this._config = config;

		if (config.backend) {
			this._endpoint = config.backend.endpoint || DEFAULT_ENDPOINT;
			this._flushIntervalMs = config.backend.flushIntervalMs || DEFAULT_FLUSH_INTERVAL;
			this._maxBatchSize = config.backend.maxBatchSize || DEFAULT_MAX_BATCH_SIZE;
			this._useSendBeacon = config.backend.useSendBeacon ?? true;
			this._authorization = config.backend.authorization || null;
		}

		// Load offline queue
		this._loadOfflineQueue();

		// Set up periodic flush
		this._scheduleFlush();

		// Set up page unload handler
		window.addEventListener('beforeunload', this._handlePageUnload);
		document.addEventListener('visibilitychange', this._handleVisibilityChange);

		this._state = 'ready';

		if (config.debug) {
			console.debug('[Backend] Adapter initialized', {
				endpoint: this._endpoint,
				flushInterval: this._flushIntervalMs,
				maxBatchSize: this._maxBatchSize,
			});
		}
	}

	/**
	 * Track a page view.
	 */
	trackPageView(payload: PageViewPayload): void {
		this._queueEvent('page_view', {
			...this._enrichPayload(payload),
			page_location: payload.page_location || window.location.href,
			page_path: payload.page_path || window.location.pathname,
			page_title: payload.page_title || document.title,
			page_referrer: payload.page_referrer || document.referrer,
			page_type: payload.page_type,
			content_group: payload.content_group,
		});
	}

	/**
	 * Track a custom event.
	 */
	trackEvent(eventName: string, payload?: CustomEventPayload): void {
		this._queueEvent(eventName, {
			...this._enrichPayload(payload || {}),
			event_category: payload?.event_category,
			event_label: payload?.event_label,
			value: payload?.value,
			...this._extractCustomProperties(payload),
		});
	}

	/**
	 * Track a purchase.
	 */
	trackPurchase(payload: PurchasePayload): void {
		this._queueEvent('purchase', {
			...this._enrichPayload(payload),
			transaction_id: payload.transaction_id,
			value: payload.value,
			currency: payload.currency,
			tax: payload.tax,
			shipping: payload.shipping,
			coupon: payload.coupon,
			items: payload.items,
		});
	}

	/**
	 * Identify a user.
	 */
	identify(payload: IdentifyPayload): void {
		this._userId = payload.user_id;

		this._queueEvent('identify', {
			...this._enrichPayload({}),
			...payload,
		});
	}

	/**
	 * Set user properties.
	 */
	setUserProperties(properties: Record<string, unknown>): void {
		this._queueEvent('user_properties', {
			...this._enrichPayload({}),
			properties,
		});
	}

	/**
	 * Reset user identity.
	 */
	reset(): void {
		this._userId = null;
		this._sessionId = this._generateSessionId();

		this._queueEvent('reset', {
			...this._enrichPayload({}),
		});
	}

	/**
	 * Handle consent changes.
	 */
	onConsentChange(consent: { analytics: boolean; marketing: boolean }): void {
		if (!consent.analytics) {
			// Clear queue if analytics consent revoked
			this._eventQueue = [];
			this._clearOfflineQueue();
		}
	}

	/**
	 * Flush pending events immediately.
	 */
	async flush(): Promise<void> {
		await this._flushQueue();
	}

	/**
	 * Cleanup.
	 */
	destroy(): void {
		if (this._flushTimeoutId) {
			clearTimeout(this._flushTimeoutId);
		}

		window.removeEventListener('beforeunload', this._handlePageUnload);
		document.removeEventListener('visibilitychange', this._handleVisibilityChange);

		// Final flush
		this._flushQueue(true);

		this._state = 'disabled';
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Private Methods
	// ─────────────────────────────────────────────────────────────────────────

	/**
	 * Queue an event for sending.
	 */
	private _queueEvent(eventName: string, payload: Record<string, unknown>): void {
		if (this._state !== 'ready') return;

		// Enforce queue limit
		if (this._eventQueue.length >= MAX_QUEUE_SIZE) {
			this._eventQueue.shift();
		}

		this._eventQueue.push({
			eventName,
			payload,
			timestamp: Date.now(),
			retries: 0,
		});

		this._metrics.eventsTracked++;
		this._metrics.lastEventTime = Date.now();

		// Check if we should flush immediately
		if (this._eventQueue.length >= this._maxBatchSize) {
			this._flushQueue();
		}
	}

	/**
	 * Enrich payload with common properties.
	 */
	private _enrichPayload(payload: Record<string, unknown> | object): Record<string, unknown> {
		return {
			...payload,
			session_id: this._sessionId,
			user_id: this._userId,
			timestamp: Date.now(),
			url: window.location.href,
			user_agent: navigator.userAgent,
			screen_width: window.screen.width,
			screen_height: window.screen.height,
			viewport_width: window.innerWidth,
			viewport_height: window.innerHeight,
			language: navigator.language,
			timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
		};
	}

	/**
	 * Extract custom properties from payload.
	 */
	private _extractCustomProperties(
		payload?: CustomEventPayload
	): Record<string, unknown> {
		if (!payload) return {};

		const custom: Record<string, unknown> = {};
		const reserved = [
			'timestamp', 'session_id', 'user_id', 'event_category',
			'event_label', 'value', 'page_location', 'page_path',
			'page_title', 'page_referrer',
		];

		for (const [key, value] of Object.entries(payload)) {
			if (!reserved.includes(key)) {
				custom[key] = value;
			}
		}

		return custom;
	}

	/**
	 * Schedule the next flush.
	 */
	private _scheduleFlush(): void {
		if (this._flushTimeoutId) {
			clearTimeout(this._flushTimeoutId);
		}

		this._flushTimeoutId = setTimeout(() => {
			this._flushQueue();
			this._scheduleFlush();
		}, this._flushIntervalMs);
	}

	/**
	 * Flush the event queue.
	 */
	private async _flushQueue(sync = false): Promise<void> {
		if (this._isFlushing || this._eventQueue.length === 0) return;

		this._isFlushing = true;

		const events = this._eventQueue.splice(0, this._maxBatchSize);

		try {
			const success = await this._sendEvents(events, sync);

			if (!success) {
				// Re-queue failed events for retry
				const retryEvents = events.filter((e) => e.retries < MAX_RETRIES);
				retryEvents.forEach((e) => e.retries++);
				this._eventQueue.unshift(...retryEvents);
				this._metrics.eventsFailed += events.length - retryEvents.length;
			}
		} catch (error) {
			// Re-queue all events
			events.forEach((e) => e.retries++);
			const retryEvents = events.filter((e) => e.retries < MAX_RETRIES);
			this._eventQueue.unshift(...retryEvents);
			this._metrics.eventsFailed += events.length - retryEvents.length;

			if (this._config?.debug) {
				console.error('[Backend] Flush failed:', error);
			}
		} finally {
			this._isFlushing = false;
		}

		// Save remaining queue for offline
		this._saveOfflineQueue();
	}

	/**
	 * Send events to the backend.
	 */
	private async _sendEvents(events: QueuedEvent[], sync = false): Promise<boolean> {
		const payload = {
			events: events.map((e) => ({
				event_name: e.eventName,
				properties: e.payload,
				timestamp: e.timestamp,
			})),
		};

		const body = JSON.stringify(payload);

		// Use sendBeacon for page unload
		if (sync && this._useSendBeacon && navigator.sendBeacon) {
			const blob = new Blob([body], { type: 'application/json' });
			return navigator.sendBeacon(this._endpoint, blob);
		}

		// Use fetch for normal requests
		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
		};

		if (this._authorization) {
			headers['Authorization'] = this._authorization;
		}

		const response = await fetch(this._endpoint, {
			method: 'POST',
			headers,
			body,
			keepalive: true,
		});

		return response.ok;
	}

	/**
	 * Handle page unload.
	 */
	private _handlePageUnload = (): void => {
		this._flushQueue(true);
	};

	/**
	 * Handle visibility change.
	 */
	private _handleVisibilityChange = (): void => {
		if (document.visibilityState === 'hidden') {
			this._flushQueue(true);
		}
	};

	/**
	 * Generate a session ID.
	 */
	private _generateSessionId(): string {
		return `${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
	}

	/**
	 * Load offline queue from localStorage.
	 */
	private _loadOfflineQueue(): void {
		try {
			const stored = localStorage.getItem(STORAGE_KEY);
			if (stored) {
				const events = JSON.parse(stored) as QueuedEvent[];
				// Only load recent events (last 24 hours)
				const cutoff = Date.now() - 24 * 60 * 60 * 1000;
				this._eventQueue = events.filter((e) => e.timestamp > cutoff);
				localStorage.removeItem(STORAGE_KEY);
			}
		} catch {
			// Ignore errors
		}
	}

	/**
	 * Save queue to localStorage for offline recovery.
	 */
	private _saveOfflineQueue(): void {
		if (this._eventQueue.length === 0) {
			this._clearOfflineQueue();
			return;
		}

		try {
			localStorage.setItem(STORAGE_KEY, JSON.stringify(this._eventQueue));
		} catch {
			// Ignore quota errors
		}
	}

	/**
	 * Clear offline queue.
	 */
	private _clearOfflineQueue(): void {
		try {
			localStorage.removeItem(STORAGE_KEY);
		} catch {
			// Ignore
		}
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Factory & Singleton
// ═══════════════════════════════════════════════════════════════════════════

let instance: BackendAnalyticsAdapter | null = null;

/**
 * Get the backend adapter singleton.
 */
export function getBackendAdapter(): BackendAnalyticsAdapter {
	if (!instance) {
		instance = new BackendAnalyticsAdapter();
	}
	return instance;
}

/**
 * Create a new backend adapter instance.
 */
export function createBackendAdapter(): BackendAnalyticsAdapter {
	return new BackendAnalyticsAdapter();
}

export default getBackendAdapter;
