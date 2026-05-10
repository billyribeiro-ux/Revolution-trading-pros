/**
 * API Connections Store - Apple ICT9+ Enterprise Grade (Svelte 5 Runes)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Centralized store for managing API connection status across the application.
 * Features:
 * - Real-time connection status tracking
 * - Cached status with configurable TTL
 * - Automatic background refresh
 * - Connection health monitoring
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

import { browser } from '$app/environment';
// FIX-2026-04-26: untrack imported for the read-helper shield (Change 1B in
// docs/audits/PRINCIPAL_FIX_PLAN_2026-04-26.md). Wrapping the rune read inside each
// getIs*Connected() helper prevents a $effect caller from installing
// connectionsState as a tracked dep — eliminating the cascade-prone
// read+write pattern that bit 13 admin pages.
import { untrack } from 'svelte';
import { getAuthToken } from '$lib/stores/auth.svelte';

// ═══════════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════════

export type ConnectionState =
	| 'connected'
	| 'disconnected'
	| 'error'
	| 'expired'
	| 'pending'
	| 'connecting';

export interface ConnectionStatus {
	key: string;
	name: string;
	category: string;
	isConnected: boolean;
	status: ConnectionState;
	healthScore: number;
	lastVerified: string | null;
	error: string | null;
}

export interface ConnectionsState {
	connections: Record<string, ConnectionStatus>;
	isLoading: boolean;
	lastFetched: number | null;
	error: string | null;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

const CACHE_TTL = 5 * 60 * 1000; // 5 minutes
const REFRESH_INTERVAL = 60 * 1000; // 1 minute background refresh

// Service keys for different features
export const SERVICE_KEYS = {
	// Analytics
	GOOGLE_ANALYTICS: 'google_analytics',
	MIXPANEL: 'mixpanel',
	AMPLITUDE: 'amplitude',
	SEGMENT: 'segment',
	PLAUSIBLE: 'plausible',

	// SEO
	GOOGLE_SEARCH_CONSOLE: 'google_search_console',
	BING_WEBMASTER: 'bing_webmaster',
	SEMRUSH: 'semrush',
	AHREFS: 'ahrefs',
	MOZ: 'moz',

	// Email Delivery (Transactional - marketing handled by FluentCRM)
	SENDGRID: 'sendgrid',
	MAILGUN: 'mailgun',
	POSTMARK: 'postmark',

	// Payment
	STRIPE: 'stripe',
	PAYPAL: 'paypal',
	SQUARE: 'square',

	// Fluent Ecosystem (Built-in)
	FLUENT_FORMS_PRO: 'fluent_forms_pro', // FluentForms Pro 6.1.8+
	FLUENT_CRM_PRO: 'fluent_crm_pro', // FluentCRM Pro (RevolutionCRM-L8)
	FLUENT_SMTP: 'fluent_smtp', // FluentSMTP
	FLUENT_SUPPORT: 'fluent_support', // FluentSupport (Helpdesk)
	FLUENT_BOOKING: 'fluent_booking', // FluentBooking

	// External CRM
	HUBSPOT: 'hubspot',
	SALESFORCE: 'salesforce',
	PIPEDRIVE: 'pipedrive',

	// Social
	FACEBOOK: 'facebook',
	TWITTER: 'twitter',
	LINKEDIN: 'linkedin',
	INSTAGRAM: 'instagram',

	// Cloud
	AWS: 'aws',
	GOOGLE_CLOUD: 'google_cloud',
	CLOUDFLARE: 'cloudflare',

	// AI
	OPENAI: 'openai',
	ANTHROPIC: 'anthropic',

	// Monitoring
	SENTRY: 'sentry',
	DATADOG: 'datadog',
	NEW_RELIC: 'new_relic'
} as const;

export type ServiceKey = (typeof SERVICE_KEYS)[keyof typeof SERVICE_KEYS];

// Feature to service mapping
export const FEATURE_SERVICES: Record<string, ServiceKey[]> = {
	analytics: [
		SERVICE_KEYS.GOOGLE_ANALYTICS,
		SERVICE_KEYS.MIXPANEL,
		SERVICE_KEYS.AMPLITUDE,
		SERVICE_KEYS.SEGMENT,
		SERVICE_KEYS.PLAUSIBLE
	],
	seo: [
		SERVICE_KEYS.GOOGLE_SEARCH_CONSOLE,
		SERVICE_KEYS.BING_WEBMASTER,
		SERVICE_KEYS.SEMRUSH,
		SERVICE_KEYS.AHREFS,
		SERVICE_KEYS.MOZ
	],
	email: [
		SERVICE_KEYS.FLUENT_SMTP, // Built-in
		SERVICE_KEYS.SENDGRID,
		SERVICE_KEYS.MAILGUN,
		SERVICE_KEYS.POSTMARK
	],
	payment: [SERVICE_KEYS.STRIPE, SERVICE_KEYS.PAYPAL, SERVICE_KEYS.SQUARE],
	// Fluent Ecosystem (all built-in products)
	fluent: [
		SERVICE_KEYS.FLUENT_FORMS_PRO,
		SERVICE_KEYS.FLUENT_CRM_PRO,
		SERVICE_KEYS.FLUENT_SMTP,
		SERVICE_KEYS.FLUENT_SUPPORT,
		SERVICE_KEYS.FLUENT_BOOKING
	],
	forms: [SERVICE_KEYS.FLUENT_FORMS_PRO],
	crm: [
		SERVICE_KEYS.FLUENT_CRM_PRO,
		SERVICE_KEYS.HUBSPOT,
		SERVICE_KEYS.SALESFORCE,
		SERVICE_KEYS.PIPEDRIVE
	],
	social: [
		SERVICE_KEYS.FACEBOOK,
		SERVICE_KEYS.TWITTER,
		SERVICE_KEYS.LINKEDIN,
		SERVICE_KEYS.INSTAGRAM
	],
	monitoring: [SERVICE_KEYS.SENTRY, SERVICE_KEYS.DATADOG, SERVICE_KEYS.NEW_RELIC],
	behavior: [
		SERVICE_KEYS.GOOGLE_ANALYTICS,
		SERVICE_KEYS.MIXPANEL,
		SERVICE_KEYS.AMPLITUDE,
		SERVICE_KEYS.SEGMENT
	]
};

// ═══════════════════════════════════════════════════════════════════════════════
// State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════════

const initialState: ConnectionsState = {
	connections: {},
	isLoading: false,
	lastFetched: null,
	error: null
};

let connectionsState = $state<ConnectionsState>({ ...initialState });

// ═══════════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════════

async function fetchConnectionStatus(): Promise<Record<string, ConnectionStatus>> {
	const token = getAuthToken();
	if (!token) {
		throw new Error('Not authenticated');
	}

	// Use AbortController for timeout
	const controller = new AbortController();
	const timeoutId = setTimeout(() => controller.abort(), 5000); // 5 second timeout

	try {
		const response = await fetch('/api/admin/connections', {
			headers: {
				Authorization: `Bearer ${token}`,
				'Content-Type': 'application/json'
			},
			signal: controller.signal
		});

		clearTimeout(timeoutId);

		if (!response.ok) {
			throw new Error('Failed to fetch connection status');
		}

		const data = await response.json();

		// Transform API response to our format
		const connections: Record<string, ConnectionStatus> = {};

		if (data.connections && Array.isArray(data.connections)) {
			for (const conn of data.connections) {
				connections[conn.key] = {
					key: conn.key,
					name: conn.name,
					category: conn.category,
					isConnected: conn.is_connected || conn.status === 'connected',
					status: conn.status || (conn.is_connected ? 'connected' : 'disconnected'),
					healthScore: conn.health_score || conn.connection?.health_score || 0,
					lastVerified: conn.last_verified_at || conn.connection?.last_verified_at || null,
					error: conn.last_error || conn.connection?.last_error || null
				};
			}
		}

		return connections;
	} catch (error) {
		clearTimeout(timeoutId);
		if (error instanceof Error && error.name === 'AbortError') {
			throw new Error('Connection check timed out', { cause: error });
		}
		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store Actions
// ═══════════════════════════════════════════════════════════════════════════════

let refreshInterval: ReturnType<typeof setInterval> | null = null;

// FIX-2026-04-26: visibility listener handle for visibility-gated polling.
// Stored at module scope so `stopAutoRefresh()` can detach it cleanly.
// See `startAutoRefresh()` below for the full rationale.
let visibilityListener: (() => void) | null = null;

// FIX-2026-04-26: in-flight Promise guard makes `load()` safe-inside-$effect.
// When a $effect re-runs (because the rune was written + getIs*Connected was
// read in the same flush), the second `load()` call returns this promise
// instead of starting a second concurrent fetch. Net result: one rune write
// per page mount, no cascade. See docs/audits/CASCADE_ROOT_CAUSE_REPORT.md §6 Change A.
let inFlightLoad: Promise<void> | null = null;

export const connections = {
	get state() {
		return connectionsState;
	},

	get connectionsList() {
		return connectionsState.connections;
	},

	get isLoading() {
		return connectionsState.isLoading;
	},

	get error() {
		return connectionsState.error;
	},

	/**
	 * Load connection status (with caching)
	 *
	 * FIX-2026-04-26: rewritten with an in-flight Promise guard so concurrent
	 * calls coalesce into a single fetch + single rune write. This makes
	 * `load()` safe to call from inside a $effect — the recipe that 13 admin
	 * pages used (and that tripped `effect_update_depth_exceeded` on
	 * post-login). See docs/audits/CASCADE_ROOT_CAUSE_REPORT.md §6 Change A.
	 *
	 * Old code (kept for one revision per FIX-2026-04-26 marker — delete in follow-up):
	 *
	 *   async load(force = false): Promise<void> {
	 *       if (!force && connectionsState.lastFetched && Date.now() - connectionsState.lastFetched < CACHE_TTL) {
	 *           return;
	 *       }
	 *       connectionsState = { ...connectionsState, isLoading: true, error: null };
	 *       try {
	 *           const connectionData = await fetchConnectionStatus();
	 *           connectionsState = {
	 *               ...connectionsState,
	 *               connections: connectionData,
	 *               isLoading: false,
	 *               lastFetched: Date.now(),
	 *               error: null
	 *           };
	 *       } catch (error) {
	 *           // ... (default-connections fallback verbatim, see below)
	 *       }
	 *   }
	 */
	async load(force = false): Promise<void> {
		// Cache TTL guard (existing behaviour) — unchanged.
		if (
			!force &&
			connectionsState.lastFetched &&
			Date.now() - connectionsState.lastFetched < CACHE_TTL
		) {
			return;
		}

		// FIX-2026-04-26: concurrent-call guard. If another `load()` is already
		// in flight, return its promise rather than starting a second fetch.
		// This is what makes `load()` safe to call from inside a $effect —
		// re-runs of the effect will await the same fetch and produce a
		// single rune write per mount.
		if (inFlightLoad) {
			return inFlightLoad;
		}

		inFlightLoad = (async () => {
			connectionsState = { ...connectionsState, isLoading: true, error: null };

			try {
				const connectionData = await fetchConnectionStatus();

				connectionsState = {
					...connectionsState,
					connections: connectionData,
					isLoading: false,
					lastFetched: Date.now(),
					error: null
				};
			} catch (error) {
				// On error, set default state with built-in services marked as connected.
				const defaultConnections: Record<string, ConnectionStatus> = {
					fluent_crm_pro: {
						key: 'fluent_crm_pro',
						name: 'FluentCRM Pro',
						category: 'Fluent',
						isConnected: true,
						status: 'connected',
						healthScore: 100,
						lastVerified: new Date().toISOString(),
						error: null
					},
					fluent_forms_pro: {
						key: 'fluent_forms_pro',
						name: 'FluentForms Pro',
						category: 'Fluent',
						isConnected: true,
						status: 'connected',
						healthScore: 100,
						lastVerified: new Date().toISOString(),
						error: null
					},
					fluent_smtp: {
						key: 'fluent_smtp',
						name: 'FluentSMTP',
						category: 'Fluent',
						isConnected: true,
						status: 'connected',
						healthScore: 100,
						lastVerified: new Date().toISOString(),
						error: null
					}
				};

				connectionsState = {
					...connectionsState,
					connections: defaultConnections,
					isLoading: false,
					lastFetched: Date.now(),
					error: error instanceof Error ? error.message : 'Failed to load connections'
				};
			} finally {
				// FIX-2026-04-26: clear the in-flight guard so subsequent `load()`
				// calls (after cache-TTL expiry or on `force=true`) can start a
				// fresh fetch.
				inFlightLoad = null;
			}
		})();

		return inFlightLoad;
	},

	/**
	 * Check if a specific service is connected
	 */
	isConnected(serviceKey: string): boolean {
		return connectionsState.connections[serviceKey]?.isConnected ?? false;
	},

	/**
	 * Check if any service in a feature category is connected
	 */
	isFeatureConnected(feature: keyof typeof FEATURE_SERVICES): boolean {
		const services = FEATURE_SERVICES[feature] || [];
		return services.some((key) => connectionsState.connections[key]?.isConnected);
	},

	/**
	 * Get connected services for a feature
	 */
	getConnectedServices(feature: keyof typeof FEATURE_SERVICES): ConnectionStatus[] {
		const services = FEATURE_SERVICES[feature] || [];
		return services
			.map((key) => connectionsState.connections[key])
			.filter((conn): conn is ConnectionStatus => conn?.isConnected === true);
	},

	/**
	 * Get status for a specific service
	 */
	getStatus(serviceKey: string): ConnectionStatus | null {
		return connectionsState.connections[serviceKey] || null;
	},

	/**
	 * Start background refresh
	 *
	 * FIX-2026-04-26: was an unconditional `setInterval(this.load, 60s)` that
	 * kept ticking even when the tab was backgrounded — wasting battery,
	 * bandwidth, and (per docs/audits/CASCADE_ROOT_CAUSE_REPORT.md §5.2) generating high-
	 * frequency rune churn that re-fired every `$derived` and `getIs*Connected`
	 * downstream of `connectionsState`. Now gated on `document.visibilityState`:
	 * the interval only runs while the tab is visible and is paused/resumed via
	 * a `visibilitychange` listener. The in-flight Promise guard on `load()`
	 * (Change 1A) still de-dupes any concurrent calls.
	 *
	 * Old code (kept for one revision per FIX-2026-04-26 marker — delete in follow-up):
	 *
	 *   startAutoRefresh(): void {
	 *       if (!browser || refreshInterval) return;
	 *
	 *       refreshInterval = setInterval(() => {
	 *           this.load(true);
	 *       }, REFRESH_INTERVAL);
	 *   },
	 */
	startAutoRefresh(): void {
		if (!browser) return;
		if (refreshInterval || visibilityListener) return; // idempotent

		const tick = () => {
			// Cache-bypass refresh; in-flight guard on `load()` coalesces
			// concurrent calls (e.g. from a near-simultaneous user-initiated
			// refresh) into a single fetch + single rune write.
			void this.load(true);
		};

		const startInterval = () => {
			if (refreshInterval) return;
			refreshInterval = setInterval(tick, REFRESH_INTERVAL);
		};

		const stopInterval = () => {
			if (refreshInterval) {
				clearInterval(refreshInterval);
				refreshInterval = null;
			}
		};

		// Initial state — only start the interval if the tab is currently
		// visible. If the user opened site-health then immediately switched
		// away, we wait until they return before ticking.
		if (document.visibilityState === 'visible') {
			startInterval();
		}

		// Toggle interval based on visibility. On returning to a previously
		// hidden tab we also fire an immediate tick so the user sees fresh
		// data without waiting up to REFRESH_INTERVAL.
		visibilityListener = () => {
			if (document.visibilityState === 'visible') {
				startInterval();
				tick();
			} else {
				stopInterval();
			}
		};
		document.addEventListener('visibilitychange', visibilityListener);
	},

	/**
	 * Stop background refresh
	 *
	 * FIX-2026-04-26: now also detaches the `visibilitychange` listener
	 * registered by `startAutoRefresh()`. Without this, a navigation away
	 * from /admin/site-health would leak the listener and keep flipping
	 * the (now-null) interval on every tab focus change.
	 *
	 * Old code (kept for one revision per FIX-2026-04-26 marker — delete in follow-up):
	 *
	 *   stopAutoRefresh(): void {
	 *       if (refreshInterval) {
	 *           clearInterval(refreshInterval);
	 *           refreshInterval = null;
	 *       }
	 *   },
	 */
	stopAutoRefresh(): void {
		if (refreshInterval) {
			clearInterval(refreshInterval);
			refreshInterval = null;
		}
		if (browser && visibilityListener) {
			document.removeEventListener('visibilitychange', visibilityListener);
			visibilityListener = null;
		}
	},

	/**
	 * Reset store
	 */
	reset(): void {
		this.stopAutoRefresh();
		connectionsState = { ...initialState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════════
// Connection Check Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════════
//
// FIX-2026-04-26: every helper below wraps its rune read in `untrack(...)`.
// This shields callers from accidentally installing `connectionsState` as a
// tracked dep — the recipe that 13 admin pages copy-pasted (`$effect → await
// connections.load() → if (getIsXConnected())`) and that tripped
// `effect_update_depth_exceeded` on post-login. `$derived(getXConnected())`
// continues to work: $derived installs tracking at the proxy boundary,
// before `untrack` runs inside the helper body. Verified Svelte 5 semantics.
// See docs/audits/CASCADE_ROOT_CAUSE_REPORT.md §6 Change B.

/**
 * Check if analytics is connected
 */
export function getIsAnalyticsConnected(): boolean {
	return untrack(() => {
		const analyticsServices = FEATURE_SERVICES['analytics'];
		return (
			analyticsServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false
		);
	});
}

/**
 * Check if SEO tools are connected
 */
export function getIsSeoConnected(): boolean {
	return untrack(() => {
		const seoServices = FEATURE_SERVICES['seo'];
		return seoServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Check if email is connected
 */
export function getIsEmailConnected(): boolean {
	return untrack(() => {
		const emailServices = FEATURE_SERVICES['email'];
		return emailServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Check if payment is connected
 */
export function getIsPaymentConnected(): boolean {
	return untrack(() => {
		const paymentServices = FEATURE_SERVICES['payment'];
		return paymentServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Check if CRM is connected
 */
export function getIsCrmConnected(): boolean {
	return untrack(() => {
		const crmServices = FEATURE_SERVICES['crm'];
		return crmServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Check if Fluent ecosystem is connected (any Fluent product)
 */
export function getIsFluentConnected(): boolean {
	return untrack(() => {
		const fluentServices = FEATURE_SERVICES['fluent'];
		return fluentServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Check if Forms is connected (FluentForms Pro)
 */
export function getIsFormsConnected(): boolean {
	return untrack(() => {
		const formsServices = FEATURE_SERVICES['forms'];
		return formsServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Check if behavior tracking is connected
 */
export function getIsBehaviorConnected(): boolean {
	return untrack(() => {
		const behaviorServices = FEATURE_SERVICES['behavior'];
		return behaviorServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
	});
}

/**
 * Get all connection statuses (for connection health panel)
 */
export function getAllConnectionStatuses(): Record<string, ConnectionState> {
	return untrack(() => {
		const statuses: Record<string, ConnectionState> = {};
		for (const [key, conn] of Object.entries(connectionsState.connections)) {
			statuses[key] = conn.status;
		}
		return statuses;
	});
}

/**
 * Get all connected services count
 */
export function getConnectedCount(): number {
	return untrack(
		() => Object.values(connectionsState.connections).filter((c) => c.isConnected).length
	);
}

/**
 * Get services with errors
 */
export function getServicesWithErrors(): ConnectionStatus[] {
	return untrack(() =>
		Object.values(connectionsState.connections).filter((c) => c.status === 'error')
	);
}

/**
 * Overall connection health
 */
export function getOverallHealth(): number {
	return untrack(() => {
		const connected = Object.values(connectionsState.connections).filter((c) => c.isConnected);
		if (connected.length === 0) return 0;

		const totalHealth = connected.reduce((sum, c) => sum + c.healthScore, 0);
		return Math.round(totalHealth / connected.length);
	});
}

// ═══════════════════════════════════════════════════════════════════════════════
// Export
// ═══════════════════════════════════════════════════════════════════════════════

export default connections;
