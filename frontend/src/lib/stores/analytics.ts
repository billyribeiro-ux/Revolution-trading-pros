/**
 * Analytics Store - Enterprise Analytics State Management
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Centralized state management for analytics data, real-time metrics,
 * and event tracking across the Revolution Trading platform.
 *
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import {
	analyticsApi,
	type DashboardData,
	type RealTimeMetrics
} from '$lib/api/analytics';

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
// Core Analytics Store
// ═══════════════════════════════════════════════════════════════════════════

function createAnalyticsStore() {
	const { subscribe, set, update } = writable<AnalyticsState>(initialState);

	let realtimeInterval: ReturnType<typeof setInterval> | null = null;

	return {
		subscribe,

		// ───────────────────────────────────────────────────────────────────
		// Dashboard Data
		// ───────────────────────────────────────────────────────────────────

		async loadDashboard(period: string = '30d') {
			update((state) => ({ ...state, isLoading: true, error: null, selectedPeriod: period }));

			try {
				const data = await analyticsApi.getDashboard(period);
				update((state) => ({
					...state,
					dashboard: data,
					isLoading: false,
					lastUpdated: new Date().toISOString()
				}));
			} catch (error) {
				update((state) => ({
					...state,
					error: error instanceof Error ? error.message : 'Failed to load dashboard',
					isLoading: false
				}));
			}
		},

		// ───────────────────────────────────────────────────────────────────
		// Real-Time Metrics
		// ───────────────────────────────────────────────────────────────────

		async loadRealtime() {
			try {
				const { metrics } = await analyticsApi.getRealTimeMetrics();
				update((state) => ({
					...state,
					realtime: metrics
				}));
			} catch (error) {
				console.error('Failed to load realtime metrics:', error);
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
			update((state) => ({ ...state, selectedPeriod: period }));
			this.loadDashboard(period);
		},

		// ───────────────────────────────────────────────────────────────────
		// State Management
		// ───────────────────────────────────────────────────────────────────

		reset() {
			this.stopRealtimeUpdates();
			set(initialState);
		},

		clearError() {
			update((state) => ({ ...state, error: null }));
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Event Tracking Store
// ═══════════════════════════════════════════════════════════════════════════

function createEventTracker() {
	const config = writable<EventTrackingConfig>(defaultTrackingConfig);
	const sessionId = writable<string | null>(null);
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
		const timeout = get(config).sessionTimeout * 60 * 1000;

		if (!sid || (lastActivity && now - parseInt(lastActivity) > timeout)) {
			sid = generateSessionId();
			sessionStorage.setItem('analytics_session_id', sid);
		}

		sessionStorage.setItem('analytics_last_activity', now.toString());
		sessionId.set(sid);
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
				session_id: get(sessionId),
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
			console.error('Failed to track event:', error);
			// Queue for retry
			eventQueue.push(eventData);
		}
	}

	// Track page view
	async function trackPageView(data?: { page_type?: string; referrer?: string }) {
		if (!browser || !get(config).autoTrackPageViews) return;

		initSession();

		try {
			await analyticsApi.trackPageView({
				page_url: window.location.href,
				page_title: document.title,
				...(data?.page_type && { page_type: data.page_type }),
				referrer: data?.referrer || document.referrer
			});
		} catch (error) {
			console.error('Failed to track page view:', error);
		}
	}

	// Track click
	function trackClick(element: HTMLElement, eventName?: string) {
		if (!get(config).autoTrackClicks) return;

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
		if (!browser || !get(config).autoTrackScrollDepth) return;

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
		if (!get(config).autoTrackFormSubmissions) return;

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
			console.error('Failed to flush event queue:', error);
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

	return {
		config,
		sessionId: { subscribe: sessionId.subscribe },
		track,
		trackPageView,
		trackClick,
		trackFormSubmit,
		flushQueue,
		updateConfig: (newConfig: Partial<EventTrackingConfig>) => {
			config.update((c) => ({ ...c, ...newConfig }));
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Exports
// ═══════════════════════════════════════════════════════════════════════════

export const analyticsStore = createAnalyticsStore();
export const eventTracker = createEventTracker();

// Derived stores
export const dashboard = derived(analyticsStore, ($analytics) => $analytics.dashboard);
export const realtime = derived(analyticsStore, ($analytics) => $analytics.realtime);
export const isLoading = derived(analyticsStore, ($analytics) => $analytics.isLoading);
export const selectedPeriod = derived(analyticsStore, ($analytics) => $analytics.selectedPeriod);

// KPI helpers
export const primaryKpis = derived(
	dashboard,
	($dashboard) => $dashboard?.kpis?.filter((kpi) => kpi.is_primary) || []
);

export const anomalyKpis = derived(
	dashboard,
	($dashboard) => $dashboard?.kpis?.filter((kpi) => kpi.is_anomaly) || []
);
