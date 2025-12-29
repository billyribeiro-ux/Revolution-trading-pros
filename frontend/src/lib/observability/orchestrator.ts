/**
 * Analytics Orchestrator - Multi-Provider Event Router
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Enterprise-grade analytics orchestration with:
 * - Multi-adapter management (GA4, Backend, Console, etc.)
 * - Unified event routing to all registered adapters
 * - Consent-aware tracking across all providers
 * - Configuration management with environment awareness
 * - Performance monitoring and diagnostics
 * - Graceful degradation on adapter failures
 *
 * Architecture:
 * ┌─────────────────────────────────────────────────────────────────────────┐
 * │                        Analytics Orchestrator                           │
 * ├─────────────────────────────────────────────────────────────────────────┤
 * │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
 * │  │   GA4       │  │   Backend   │  │   Console   │  │   Custom    │   │
 * │  │   Adapter   │  │   Adapter   │  │   Adapter   │  │   Adapter   │   │
 * │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │
 * └─────────────────────────────────────────────────────────────────────────┘
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import { writable, derived } from 'svelte/store';
import type {
	AnalyticsAdapter,
	AnalyticsConfig,
	PageViewPayload,
	CustomEventPayload,
	PurchasePayload,
	IdentifyPayload,
	AdapterMetrics,
} from './adapters/types';
import { getGoogleAnalyticsAdapter } from './adapters/google-analytics';
import { getConsoleAdapter } from './adapters/console';
import { getBackendAdapter } from './adapters/backend';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface OrchestratorState {
	initialized: boolean;
	adapters: string[];
	consent: {
		analytics: boolean;
		marketing: boolean;
	};
	userId: string | null;
	sessionId: string;
	environment: 'development' | 'staging' | 'production';
}

export interface OrchestratorMetrics {
	totalEvents: number;
	failedEvents: number;
	adapterMetrics: Record<string, AdapterMetrics>;
	lastEventTime: number | null;
	uptime: number;
}

// ═══════════════════════════════════════════════════════════════════════════
// Default Configuration
// ═══════════════════════════════════════════════════════════════════════════

const getDefaultConfig = (): AnalyticsConfig => ({
	enabled: true,
	environment: (import.meta.env.MODE as 'development' | 'staging' | 'production') || 'development',
	googleAnalytics: {
		measurementId: import.meta.env['PUBLIC_GA4_MEASUREMENT_ID'] || '',
		debug: import.meta.env.DEV,
		sendPageView: false,
		anonymizeIp: true,
		allowGoogleSignals: false,
		allowAdPersonalization: false,
	},
	backend: {
		endpoint: import.meta.env['VITE_ANALYTICS_ENDPOINT'] || '/api/analytics/batch',
		flushIntervalMs: 5000,
		maxBatchSize: 50,
		useSendBeacon: true,
	},
	console: {
		enabled: import.meta.env.DEV,
		prettyPrint: true,
		groupLogs: true,
	},
	consent: {
		analytics: false,
		marketing: false,
	},
	debug: import.meta.env.DEV,
});

// ═══════════════════════════════════════════════════════════════════════════
// Analytics Orchestrator Class
// ═══════════════════════════════════════════════════════════════════════════

class AnalyticsOrchestrator {
	private _adapters: Map<string, AnalyticsAdapter> = new Map();
	private _config: AnalyticsConfig = getDefaultConfig();
	private _initialized = false;
	private _startTime = Date.now();
	private _totalEvents = 0;
	private _failedEvents = 0;
	private _userId: string | null = null;
	private _sessionId: string;

	// Svelte store for reactive state
	private _store = writable<OrchestratorState>({
		initialized: false,
		adapters: [],
		consent: { analytics: false, marketing: false },
		userId: null,
		sessionId: '',
		environment: 'development',
	});

	constructor() {
		this._sessionId = this._generateSessionId();
		this._updateStore();
	}

	/**
	 * Get the reactive store for UI integration.
	 */
	get store() {
		return { subscribe: this._store.subscribe };
	}

	/**
	 * Check if orchestrator is initialized.
	 */
	get initialized(): boolean {
		return this._initialized;
	}

	/**
	 * Get current consent state.
	 */
	get consent(): { analytics: boolean; marketing: boolean } {
		return this._config.consent;
	}

	/**
	 * Get orchestrator metrics.
	 */
	get metrics(): OrchestratorMetrics {
		const adapterMetrics: Record<string, AdapterMetrics> = {};

		for (const [id, adapter] of this._adapters) {
			if ('metrics' in adapter) {
				adapterMetrics[id] = (adapter as any).metrics;
			}
		}

		return {
			totalEvents: this._totalEvents,
			failedEvents: this._failedEvents,
			adapterMetrics,
			lastEventTime: Date.now(),
			uptime: Date.now() - this._startTime,
		};
	}

	/**
	 * Initialize the orchestrator with configuration.
	 */
	async initialize(customConfig?: Partial<AnalyticsConfig>): Promise<void> {
		if (!browser) return;

		// Merge custom config with defaults
		const defaults = getDefaultConfig();
		const ga = defaults.googleAnalytics!;
		const be = defaults.backend!;
		this._config = {
			...defaults,
			...customConfig,
			googleAnalytics: {
				measurementId: customConfig?.googleAnalytics?.measurementId ?? ga.measurementId,
				...(customConfig?.googleAnalytics?.debug !== undefined ? { debug: customConfig.googleAnalytics.debug } : ga.debug !== undefined ? { debug: ga.debug } : {}),
				...(customConfig?.googleAnalytics?.sendPageView !== undefined ? { sendPageView: customConfig.googleAnalytics.sendPageView } : ga.sendPageView !== undefined ? { sendPageView: ga.sendPageView } : {}),
				...(customConfig?.googleAnalytics?.anonymizeIp !== undefined ? { anonymizeIp: customConfig.googleAnalytics.anonymizeIp } : ga.anonymizeIp !== undefined ? { anonymizeIp: ga.anonymizeIp } : {}),
				...(customConfig?.googleAnalytics?.allowGoogleSignals !== undefined ? { allowGoogleSignals: customConfig.googleAnalytics.allowGoogleSignals } : ga.allowGoogleSignals !== undefined ? { allowGoogleSignals: ga.allowGoogleSignals } : {}),
				...(customConfig?.googleAnalytics?.allowAdPersonalization !== undefined ? { allowAdPersonalization: customConfig.googleAnalytics.allowAdPersonalization } : ga.allowAdPersonalization !== undefined ? { allowAdPersonalization: ga.allowAdPersonalization } : {}),
			},
			backend: {
				endpoint: customConfig?.backend?.endpoint ?? be.endpoint,
				...(customConfig?.backend?.flushIntervalMs !== undefined ? { flushIntervalMs: customConfig.backend.flushIntervalMs } : be.flushIntervalMs !== undefined ? { flushIntervalMs: be.flushIntervalMs } : {}),
				...(customConfig?.backend?.maxBatchSize !== undefined ? { maxBatchSize: customConfig.backend.maxBatchSize } : be.maxBatchSize !== undefined ? { maxBatchSize: be.maxBatchSize } : {}),
				...(customConfig?.backend?.useSendBeacon !== undefined ? { useSendBeacon: customConfig.backend.useSendBeacon } : be.useSendBeacon !== undefined ? { useSendBeacon: be.useSendBeacon } : {}),
			},
			console: {
				...defaults.console,
				...customConfig?.console,
			},
			consent: {
				...defaults.consent,
				...customConfig?.consent,
			},
		};

		// Register default adapters
		await this._registerDefaultAdapters();

		this._initialized = true;
		this._updateStore();

		if (this._config.debug) {
			console.debug('[Orchestrator] Initialized with config:', this._config);
			console.debug('[Orchestrator] Registered adapters:', Array.from(this._adapters.keys()));
		}
	}

	/**
	 * Register a custom adapter.
	 */
	async registerAdapter(adapter: AnalyticsAdapter): Promise<void> {
		if (this._adapters.has(adapter.id)) {
			console.warn(`[Orchestrator] Adapter "${adapter.id}" already registered, replacing...`);
		}

		try {
			await adapter.initialize(this._config);
			this._adapters.set(adapter.id, adapter);
			this._updateStore();

			if (this._config.debug) {
				console.debug(`[Orchestrator] Registered adapter: ${adapter.id}`);
			}
		} catch (error) {
			console.error(`[Orchestrator] Failed to register adapter "${adapter.id}":`, error);
		}
	}

	/**
	 * Unregister an adapter.
	 */
	unregisterAdapter(adapterId: string): void {
		const adapter = this._adapters.get(adapterId);
		if (adapter) {
			adapter.destroy?.();
			this._adapters.delete(adapterId);
			this._updateStore();
		}
	}

	/**
	 * Update consent state across all adapters.
	 */
	updateConsent(consent: { analytics: boolean; marketing: boolean }): void {
		this._config.consent = consent;

		for (const adapter of this._adapters.values()) {
			adapter.onConsentChange?.(consent);
		}

		this._updateStore();

		if (this._config.debug) {
			console.debug('[Orchestrator] Consent updated:', consent);
		}
	}

	/**
	 * Track a page view across all adapters.
	 */
	trackPageView(payload?: PageViewPayload): void {
		if (!this._config.enabled || !this._config.consent.analytics) return;

		const enrichedPayload: PageViewPayload = {
			...payload,
			page_location: payload?.page_location ?? (browser ? window.location.href : ''),
			page_path: payload?.page_path ?? (browser ? window.location.pathname : ''),
			page_title: payload?.page_title ?? (browser ? document.title : ''),
			page_referrer: payload?.page_referrer ?? (browser ? document.referrer : ''),
			timestamp: Date.now(),
			session_id: this._sessionId,
			user_id: this._userId,
		};

		this._broadcast('trackPageView', enrichedPayload);
	}

	/**
	 * Track a custom event across all adapters.
	 */
	trackEvent(eventName: string, payload?: CustomEventPayload): void {
		if (!this._config.enabled || !this._config.consent.analytics) return;

		const enrichedPayload: CustomEventPayload = {
			...payload,
			timestamp: Date.now(),
			session_id: this._sessionId,
			user_id: this._userId,
		};

		this._broadcast('trackEvent', eventName, enrichedPayload);
	}

	/**
	 * Track a purchase across all adapters.
	 */
	trackPurchase(payload: PurchasePayload): void {
		if (!this._config.enabled || !this._config.consent.analytics) return;

		const enrichedPayload: PurchasePayload = {
			...payload,
			timestamp: Date.now(),
			session_id: this._sessionId,
			user_id: this._userId,
		};

		this._broadcast('trackPurchase', enrichedPayload);
	}

	/**
	 * Identify a user across all adapters.
	 */
	identify(userId: string, properties?: Record<string, unknown>): void {
		this._userId = userId;
		this._updateStore();

		const payload: IdentifyPayload = {
			user_id: userId,
			...properties,
		};

		this._broadcast('identify', payload);
	}

	/**
	 * Set user properties across all adapters.
	 */
	setUserProperties(properties: Record<string, unknown>): void {
		this._broadcast('setUserProperties', properties);
	}

	/**
	 * Reset user identity across all adapters.
	 */
	reset(): void {
		this._userId = null;
		this._sessionId = this._generateSessionId();
		this._updateStore();

		this._broadcast('reset');
	}

	/**
	 * Flush all adapters.
	 */
	async flush(): Promise<void> {
		const promises: Promise<void>[] = [];

		for (const adapter of this._adapters.values()) {
			if (adapter.flush) {
				promises.push(adapter.flush());
			}
		}

		await Promise.allSettled(promises);
	}

	/**
	 * Destroy all adapters and cleanup.
	 */
	destroy(): void {
		for (const adapter of this._adapters.values()) {
			adapter.destroy?.();
		}

		this._adapters.clear();
		this._initialized = false;
		this._updateStore();
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Private Methods
	// ─────────────────────────────────────────────────────────────────────────

	/**
	 * Register default adapters based on configuration.
	 */
	private async _registerDefaultAdapters(): Promise<void> {
		const registrations: Promise<void>[] = [];

		// Google Analytics (if measurement ID is configured)
		if (this._config.googleAnalytics?.measurementId) {
			registrations.push(this.registerAdapter(getGoogleAnalyticsAdapter()));
		}

		// Backend API (always register)
		if (this._config.backend?.endpoint) {
			registrations.push(this.registerAdapter(getBackendAdapter()));
		}

		// Console (development only)
		if (this._config.console?.enabled) {
			registrations.push(this.registerAdapter(getConsoleAdapter()));
		}

		await Promise.allSettled(registrations);
	}

	/**
	 * Broadcast a method call to all adapters.
	 */
	private _broadcast(method: string, ...args: unknown[]): void {
		this._totalEvents++;

		for (const adapter of this._adapters.values()) {
			try {
				const fn = (adapter as any)[method];
				if (typeof fn === 'function') {
					fn.apply(adapter, args);
				}
			} catch (error) {
				this._failedEvents++;
				console.error(`[Orchestrator] Error in adapter "${adapter.id}":`, error);
			}
		}
	}

	/**
	 * Generate a unique session ID.
	 */
	private _generateSessionId(): string {
		return `${Date.now()}-${Math.random().toString(36).substring(2, 11)}`;
	}

	/**
	 * Update the reactive store.
	 */
	private _updateStore(): void {
		this._store.set({
			initialized: this._initialized,
			adapters: Array.from(this._adapters.keys()),
			consent: this._config.consent,
			userId: this._userId,
			sessionId: this._sessionId,
			environment: this._config.environment,
		});
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Singleton Instance
// ═══════════════════════════════════════════════════════════════════════════

let instance: AnalyticsOrchestrator | null = null;

/**
 * Get the analytics orchestrator singleton.
 */
export function getOrchestrator(): AnalyticsOrchestrator {
	if (!instance) {
		instance = new AnalyticsOrchestrator();
	}
	return instance;
}

/**
 * Create a new orchestrator instance (for testing).
 */
export function createOrchestrator(): AnalyticsOrchestrator {
	return new AnalyticsOrchestrator();
}

/**
 * Reset the singleton (for testing).
 */
export function resetOrchestrator(): void {
	if (instance) {
		instance.destroy();
		instance = null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Derived Stores
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Derived store for orchestrator state.
 */
export const orchestratorState = derived(
	{ subscribe: getOrchestrator().store.subscribe },
	($state) => $state
);

/**
 * Derived store for consent status.
 */
export const analyticsConsent = derived(
	{ subscribe: getOrchestrator().store.subscribe },
	($state) => $state.consent
);

/**
 * Derived store for registered adapters.
 */
export const registeredAdapters = derived(
	{ subscribe: getOrchestrator().store.subscribe },
	($state) => $state.adapters
);

export default getOrchestrator;
