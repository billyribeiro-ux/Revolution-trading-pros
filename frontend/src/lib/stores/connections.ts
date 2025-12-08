/**
 * API Connections Store - Apple ICT9+ Enterprise Grade
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Centralized store for managing API connection status across the application.
 * Features:
 * - Real-time connection status tracking
 * - Cached status with configurable TTL
 * - Automatic background refresh
 * - Connection health monitoring
 *
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { getAuthToken } from '$lib/stores/auth';

// ═══════════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════════

export type ConnectionState = 'connected' | 'disconnected' | 'error' | 'expired' | 'pending' | 'connecting';

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

	// Email
	MAILCHIMP: 'mailchimp',
	SENDGRID: 'sendgrid',
	MAILGUN: 'mailgun',
	POSTMARK: 'postmark',
	SENDINBLUE: 'sendinblue',

	// Payment
	STRIPE: 'stripe',
	PAYPAL: 'paypal',
	SQUARE: 'square',

	// CRM
	REVOLUTION_CRM: 'revolution_crm', // Built-in CRM Pro
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
		SERVICE_KEYS.MAILCHIMP,
		SERVICE_KEYS.SENDGRID,
		SERVICE_KEYS.MAILGUN,
		SERVICE_KEYS.POSTMARK,
		SERVICE_KEYS.SENDINBLUE
	],
	payment: [SERVICE_KEYS.STRIPE, SERVICE_KEYS.PAYPAL, SERVICE_KEYS.SQUARE],
	crm: [SERVICE_KEYS.REVOLUTION_CRM, SERVICE_KEYS.HUBSPOT, SERVICE_KEYS.SALESFORCE, SERVICE_KEYS.PIPEDRIVE],
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
// Store
// ═══════════════════════════════════════════════════════════════════════════════

const initialState: ConnectionsState = {
	connections: {},
	isLoading: false,
	lastFetched: null,
	error: null
};

const connectionsStore = writable<ConnectionsState>(initialState);

// ═══════════════════════════════════════════════════════════════════════════════
// API Functions
// ═══════════════════════════════════════════════════════════════════════════════

async function fetchConnectionStatus(): Promise<Record<string, ConnectionStatus>> {
	const token = getAuthToken();
	if (!token) {
		throw new Error('Not authenticated');
	}

	const response = await fetch('/api/admin/connections/status', {
		headers: {
			Authorization: `Bearer ${token}`,
			'Content-Type': 'application/json'
		}
	});

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
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store Actions
// ═══════════════════════════════════════════════════════════════════════════════

let refreshInterval: ReturnType<typeof setInterval> | null = null;

export const connections = {
	subscribe: connectionsStore.subscribe,

	/**
	 * Load connection status (with caching)
	 */
	async load(force = false): Promise<void> {
		const state = get(connectionsStore);

		// Check cache
		if (!force && state.lastFetched && Date.now() - state.lastFetched < CACHE_TTL) {
			return;
		}

		connectionsStore.update((s) => ({ ...s, isLoading: true, error: null }));

		try {
			const connectionData = await fetchConnectionStatus();

			connectionsStore.update((s) => ({
				...s,
				connections: connectionData,
				isLoading: false,
				lastFetched: Date.now(),
				error: null
			}));
		} catch (error) {
			connectionsStore.update((s) => ({
				...s,
				isLoading: false,
				error: error instanceof Error ? error.message : 'Failed to load connections'
			}));
		}
	},

	/**
	 * Check if a specific service is connected
	 */
	isConnected(serviceKey: string): boolean {
		const state = get(connectionsStore);
		return state.connections[serviceKey]?.isConnected ?? false;
	},

	/**
	 * Check if any service in a feature category is connected
	 */
	isFeatureConnected(feature: keyof typeof FEATURE_SERVICES): boolean {
		const state = get(connectionsStore);
		const services = FEATURE_SERVICES[feature] || [];
		return services.some((key) => state.connections[key]?.isConnected);
	},

	/**
	 * Get connected services for a feature
	 */
	getConnectedServices(feature: keyof typeof FEATURE_SERVICES): ConnectionStatus[] {
		const state = get(connectionsStore);
		const services = FEATURE_SERVICES[feature] || [];
		return services
			.map((key) => state.connections[key])
			.filter((conn): conn is ConnectionStatus => conn?.isConnected === true);
	},

	/**
	 * Get status for a specific service
	 */
	getStatus(serviceKey: string): ConnectionStatus | null {
		const state = get(connectionsStore);
		return state.connections[serviceKey] || null;
	},

	/**
	 * Start background refresh
	 */
	startAutoRefresh(): void {
		if (!browser || refreshInterval) return;

		refreshInterval = setInterval(() => {
			this.load(true);
		}, REFRESH_INTERVAL);
	},

	/**
	 * Stop background refresh
	 */
	stopAutoRefresh(): void {
		if (refreshInterval) {
			clearInterval(refreshInterval);
			refreshInterval = null;
		}
	},

	/**
	 * Reset store
	 */
	reset(): void {
		this.stopAutoRefresh();
		connectionsStore.set(initialState);
	}
};

// ═══════════════════════════════════════════════════════════════════════════════
// Derived Stores
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Check if analytics is connected
 */
export const isAnalyticsConnected = derived(connectionsStore, ($state) => {
	const analyticsServices = FEATURE_SERVICES.analytics;
	return analyticsServices.some((key) => $state.connections[key]?.isConnected);
});

/**
 * Check if SEO tools are connected
 */
export const isSeoConnected = derived(connectionsStore, ($state) => {
	const seoServices = FEATURE_SERVICES.seo;
	return seoServices.some((key) => $state.connections[key]?.isConnected);
});

/**
 * Check if email is connected
 */
export const isEmailConnected = derived(connectionsStore, ($state) => {
	const emailServices = FEATURE_SERVICES.email;
	return emailServices.some((key) => $state.connections[key]?.isConnected);
});

/**
 * Check if payment is connected
 */
export const isPaymentConnected = derived(connectionsStore, ($state) => {
	const paymentServices = FEATURE_SERVICES.payment;
	return paymentServices.some((key) => $state.connections[key]?.isConnected);
});

/**
 * Check if CRM is connected
 */
export const isCrmConnected = derived(connectionsStore, ($state) => {
	const crmServices = FEATURE_SERVICES.crm;
	return crmServices.some((key) => $state.connections[key]?.isConnected);
});

/**
 * Check if behavior tracking is connected
 */
export const isBehaviorConnected = derived(connectionsStore, ($state) => {
	const behaviorServices = FEATURE_SERVICES.behavior;
	return behaviorServices.some((key) => $state.connections[key]?.isConnected);
});

/**
 * Get all connection statuses (for connection health panel)
 */
export const allConnectionStatuses = derived(connectionsStore, ($state) => {
	const statuses: Record<string, ConnectionState> = {};
	for (const [key, conn] of Object.entries($state.connections)) {
		statuses[key] = conn.status;
	}
	return statuses;
});

/**
 * Get all connected services count
 */
export const connectedCount = derived(connectionsStore, ($state) => {
	return Object.values($state.connections).filter((c) => c.isConnected).length;
});

/**
 * Get services with errors
 */
export const servicesWithErrors = derived(connectionsStore, ($state) => {
	return Object.values($state.connections).filter((c) => c.status === 'error');
});

/**
 * Overall connection health
 */
export const overallHealth = derived(connectionsStore, ($state) => {
	const connected = Object.values($state.connections).filter((c) => c.isConnected);
	if (connected.length === 0) return 0;

	const totalHealth = connected.reduce((sum, c) => sum + c.healthScore, 0);
	return Math.round(totalHealth / connected.length);
});

// ═══════════════════════════════════════════════════════════════════════════════
// Export
// ═══════════════════════════════════════════════════════════════════════════════

export default connections;
