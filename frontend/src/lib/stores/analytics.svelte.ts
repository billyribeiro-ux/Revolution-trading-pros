/**
 * Analytics Store - Enterprise Analytics State Management (Svelte 5 Runes)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized state management for analytics data, real-time metrics,
 * and event tracking across the Revolution Trading platform.
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

import { browser } from '$app/environment';
import { analyticsApi, type DashboardData, type RealTimeMetrics } from '$lib/api/analytics';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface AnalyticsState {
	dashboard: DashboardData | null;
	realtime: RealTimeMetrics | null;
	selectedPeriod: string;
	isLoading: boolean;
	error: string | null;
	lastUpdated: string | null;
}

export interface EventTrackingConfig {
	autoTrackPageViews: boolean;
	autoTrackClicks: boolean;
	autoTrackScrollDepth: boolean;
	autoTrackFormSubmissions: boolean;
	sessionTimeout: number; // minutes
}

// ═══════════════════════════════════════════════════════════════════════════
// Initial State
// ═══════════════════════════════════════════════════════════════════════════

const initialState: AnalyticsState = {
	dashboard: null,
	realtime: null,
	selectedPeriod: '30d',
	isLoading: false,
	error: null,
	lastUpdated: null
};

const defaultTrackingConfig: EventTrackingConfig = {
	autoTrackPageViews: true,
	autoTrackClicks: true,
	autoTrackScrollDepth: true,
	autoTrackFormSubmissions: true,
	sessionTimeout: 30
};

// ═══════════════════════════════════════════════════════════════════════════
// Core Analytics Store (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let analyticsState = $state<AnalyticsState>({ ...initialState });
let realtimeInterval: ReturnType<typeof setInterval> | null = null;

export const analyticsStore = {
	get state() {
		return analyticsState;
	},

	get dashboard() {
		return analyticsState.dashboard;
	},

	get realtime() {
		return analyticsState.realtime;
	},

	get isLoading() {
		return analyticsState.isLoading;
	},

	get selectedPeriod() {
		return analyticsState.selectedPeriod;
	},

	// ───────────────────────────────────────────────────────────────────
	// Dashboard Data
	// ───────────────────────────────────────────────────────────────────

	async loadDashboard(period: string = '30d') {
		analyticsState = { ...analyticsState, isLoading: true, error: null, selectedPeriod: period };

		try {
			const data = await analyticsApi.getDashboard(period);
			analyticsState = {
				...analyticsState,
				dashboard: data,
				isLoading: false,
				lastUpdated: new Date().toISOString()
			};
		} catch (error) {
			analyticsState = {
				...analyticsState,
				error: error instanceof Error ? error.message : 'Failed to load dashboard',
				isLoading: false
			};
		}
	},

	// ───────────────────────────────────────────────────────────────────
	// Real-Time Metrics
	// ───────────────────────────────────────────────────────────────────

	async loadRealtime() {
		try {
			const { metrics } = await analyticsApi.getRealTimeMetrics();
			analyticsState = {
				...analyticsState,
				realtime: metrics
			};
		} catch (error) {
			logger.error('Failed to load realtime metrics:', error);
		}
	},

	startRealtimeUpdates(intervalMs: number = 10000) {
		if (!browser) return;

		this.loadRealtime();

		if (realtimeInterval) {
			clearInterval(realtimeInterval);
		}

		realtimeInterval = setInterval(() => {
			this.loadRealtime();
		}, intervalMs);
	},

	stopRealtimeUpdates() {
		if (realtimeInterval) {
			clearInterval(realtimeInterval);
			realtimeInterval = null;
		}
	},

	// ───────────────────────────────────────────────────────────────────
	// Period Selection
	// ───────────────────────────────────────────────────────────────────

	setPeriod(period: string) {
		analyticsState = { ...analyticsState, selectedPeriod: period };
		this.loadDashboard(period);
	},

	// ───────────────────────────────────────────────────────────────────
	// State Management
	// ───────────────────────────────────────────────────────────────────

	reset() {
		this.stopRealtimeUpdates();
		analyticsState = { ...initialState };
	},

	clearError() {
		analyticsState = { ...analyticsState, error: null };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Event Tracking Store (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let trackingConfig = $state<EventTrackingConfig>({ ...defaultTrackingConfig });
let sessionIdValue = $state<string | null>(null);
const eventQueue: any[] = [];
let flushInterval: ReturnType<typeof setInterval> | null = null;

// Generate session ID
function generateSessionId(): string {
	return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

// Initialize session
function initSession() {
	if (!browser) return;

	let sid = sessionStorage.getItem('analytics_session_id');
	const lastActivity = sessionStorage.getItem('analytics_last_activity');

	const now = Date.now();
	const timeout = trackingConfig.sessionTimeout * 60 * 1000;

	if (!sid || (lastActivity && now - parseInt(lastActivity) > timeout)) {
		sid = generateSessionId();
		sessionStorage.setItem('analytics_session_id', sid);
	}

	sessionStorage.setItem('analytics_last_activity', now.toString());
	sessionIdValue = sid;
}

// Track event
async function track(eventName: string, properties?: Record<string, any>) {
	if (!browser) return;

	initSession();

	const eventData = {
		event_name: eventName,
		event_category: properties?.['category'] || 'user_interaction',
		event_type: properties?.['type'] || 'custom',
		properties: {
			...properties,
			session_id: sessionIdValue,
			page_url: window.location.href,
			page_title: document.title,
			timestamp: new Date().toISOString(),
			user_agent: navigator.userAgent,
			screen_resolution: `${window.screen.width}x${window.screen.height}`,
			viewport_size: `${window.innerWidth}x${window.innerHeight}`
		}
	};

	try {
		await analyticsApi.trackEvent(eventData);
	} catch (error) {
		logger.error('Failed to track event:', error);
		// Queue for retry
		eventQueue.push(eventData);
	}
}

// Track page view
async function trackPageView(data?: { page_type?: string; referrer?: string }) {
	if (!browser || !trackingConfig.autoTrackPageViews) return;

	initSession();

	try {
		await analyticsApi.trackPageView({
			page_url: window.location.href,
			page_title: document.title,
			...(data?.page_type && { page_type: data.page_type }),
			referrer: data?.referrer || document.referrer
		});
	} catch (error) {
		logger.error('Failed to track page view:', error);
	}
}

// Track click
function trackClick(element: HTMLElement, eventName?: string) {
	if (!trackingConfig.autoTrackClicks) return;

	const name = eventName || `click_${element.tagName.toLowerCase()}`;
	const properties = {
		element_type: element.tagName,
		element_id: element.id,
		element_class: element.className,
		element_text: element.textContent?.substring(0, 100),
		category: 'click'
	};

	track(name, properties);
}

// Track scroll depth
let maxScrollDepth = 0;
function setupScrollTracking() {
	if (!browser || !trackingConfig.autoTrackScrollDepth) return;

	let ticking = false;

	const handleScroll = () => {
		if (!ticking) {
			window.requestAnimationFrame(() => {
				const scrollPercentage = Math.round(
					(window.scrollY / (document.documentElement.scrollHeight - window.innerHeight)) * 100
				);

				if (scrollPercentage > maxScrollDepth) {
					maxScrollDepth = scrollPercentage;

					// Track milestones
					if ([25, 50, 75, 90, 100].includes(scrollPercentage)) {
						track('scroll_depth', {
							depth: scrollPercentage,
							category: 'engagement'
						});
					}
				}

				ticking = false;
			});

			ticking = true;
		}
	};

	window.addEventListener('scroll', handleScroll, { passive: true });

	return () => {
		window.removeEventListener('scroll', handleScroll);
	};
}

// Track form submission
function trackFormSubmit(formId: string, formData?: Record<string, any>) {
	if (!trackingConfig.autoTrackFormSubmissions) return;

	track('form_submit', {
		form_id: formId,
		form_data: formData,
		category: 'conversion'
	});
}

// Batch flush
async function flushQueue() {
	if (eventQueue.length === 0) return;

	const events = eventQueue.splice(0, 50); // Batch size

	try {
		await analyticsApi.trackBatch(events);
	} catch (error) {
		logger.error('Failed to flush event queue:', error);
		// Re-add to queue
		eventQueue.unshift(...events);
	}
}

// Start auto-flush
function startAutoFlush(intervalMs: number = 30000) {
	if (flushInterval) clearInterval(flushInterval);
	flushInterval = setInterval(flushQueue, intervalMs);
}

// Initialize
if (browser) {
	initSession();
	startAutoFlush();
	setupScrollTracking();
}

export const eventTracker = {
	get config() {
		return trackingConfig;
	},

	get sessionId() {
		return sessionIdValue;
	},

	track,
	trackPageView,
	trackClick,
	trackFormSubmit,
	flushQueue,

	updateConfig(newConfig: Partial<EventTrackingConfig>) {
		trackingConfig = { ...trackingConfig, ...newConfig };
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════

export function getDashboard() {
	return analyticsState.dashboard;
}
export function getRealtime() {
	return analyticsState.realtime;
}
export function getIsAnalyticsLoading() {
	return analyticsState.isLoading;
}
export function getAnalyticsSelectedPeriod() {
	return analyticsState.selectedPeriod;
}

// KPI helpers
export function getPrimaryKpis() {
	return analyticsState.dashboard?.kpis?.filter((kpi) => kpi.is_primary) || [];
}

export function getAnomalyKpis() {
	return analyticsState.dashboard?.kpis?.filter((kpi) => kpi.is_anomaly) || [];
}
