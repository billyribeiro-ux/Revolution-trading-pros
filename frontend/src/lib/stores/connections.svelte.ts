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
			throw new Error('Connection check timed out');
		}
		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store Actions
// ═══════════════════════════════════════════════════════════════════════════════

let refreshInterval: ReturnType<typeof setInterval> | null = null;

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
	 */
	async load(force = false): Promise<void> {
		// Check cache
		if (
			!force &&
			connectionsState.lastFetched &&
			Date.now() - connectionsState.lastFetched < CACHE_TTL
		) {
			return;
		}

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
			// On error, set default state with built-in services marked as connected
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
		}
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
		connectionsState = { ...initialState };
	}
};

// ═══════════════════════════════════════════════════════════════════════════════
// Connection Check Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Check if analytics is connected
 */
export function getIsAnalyticsConnected(): boolean {
	const analyticsServices = FEATURE_SERVICES['analytics'];
	return analyticsServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if SEO tools are connected
 */
export function getIsSeoConnected(): boolean {
	const seoServices = FEATURE_SERVICES['seo'];
	return seoServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if email is connected
 */
export function getIsEmailConnected(): boolean {
	const emailServices = FEATURE_SERVICES['email'];
	return emailServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if payment is connected
 */
export function getIsPaymentConnected(): boolean {
	const paymentServices = FEATURE_SERVICES['payment'];
	return paymentServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if CRM is connected
 */
export function getIsCrmConnected(): boolean {
	const crmServices = FEATURE_SERVICES['crm'];
	return crmServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if Fluent ecosystem is connected (any Fluent product)
 */
export function getIsFluentConnected(): boolean {
	const fluentServices = FEATURE_SERVICES['fluent'];
	return fluentServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if Forms is connected (FluentForms Pro)
 */
export function getIsFormsConnected(): boolean {
	const formsServices = FEATURE_SERVICES['forms'];
	return formsServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Check if behavior tracking is connected
 */
export function getIsBehaviorConnected(): boolean {
	const behaviorServices = FEATURE_SERVICES['behavior'];
	return behaviorServices?.some((key) => connectionsState.connections[key]?.isConnected) ?? false;
}

/**
 * Get all connection statuses (for connection health panel)
 */
export function getAllConnectionStatuses(): Record<string, ConnectionState> {
	const statuses: Record<string, ConnectionState> = {};
	for (const [key, conn] of Object.entries(connectionsState.connections)) {
		statuses[key] = conn.status;
	}
	return statuses;
}

/**
 * Get all connected services count
 */
export function getConnectedCount(): number {
	return Object.values(connectionsState.connections).filter((c) => c.isConnected).length;
}

/**
 * Get services with errors
 */
export function getServicesWithErrors(): ConnectionStatus[] {
	return Object.values(connectionsState.connections).filter((c) => c.status === 'error');
}

/**
 * Overall connection health
 */
export function getOverallHealth(): number {
	const connected = Object.values(connectionsState.connections).filter((c) => c.isConnected);
	if (connected.length === 0) return 0;

	const totalHealth = connected.reduce((sum, c) => sum + c.healthScore, 0);
	return Math.round(totalHealth / connected.length);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Export
// ═══════════════════════════════════════════════════════════════════════════════

export default connections;
