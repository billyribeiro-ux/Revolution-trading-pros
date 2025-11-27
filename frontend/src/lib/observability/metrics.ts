/**
 * Metrics - Standardized event tracking and analytics
 * Netflix E6 Level Observability
 * 
 * @version 2.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';

// ═══════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════

export interface EventProperties {
	[key: string]: string | number | boolean | null | undefined;
}

export interface PageViewProperties {
	title?: string;
	referrer?: string;
	path?: string;
	search?: string;
}

export interface UserProperties {
	userId?: string;
	email?: string;
	name?: string;
	plan?: string;
	[key: string]: string | number | boolean | null | undefined;
}

// ═══════════════════════════════════════════════════════════════════════════
// Standard Event Names
// ═══════════════════════════════════════════════════════════════════════════

export const Events = {
	// Page Events
	PAGE_VIEW: 'page_view',
	PAGE_LEAVE: 'page_leave',
	
	// User Events
	USER_SIGNED_UP: 'user_signed_up',
	USER_LOGGED_IN: 'user_logged_in',
	USER_LOGGED_OUT: 'user_logged_out',
	USER_PROFILE_UPDATED: 'user_profile_updated',
	
	// Navigation Events
	BUTTON_CLICKED: 'button_clicked',
	LINK_CLICKED: 'link_clicked',
	NAV_ITEM_CLICKED: 'nav_item_clicked',
	
	// Form Events
	FORM_STARTED: 'form_started',
	FORM_SUBMITTED: 'form_submitted',
	FORM_ERROR: 'form_error',
	FORM_ABANDONED: 'form_abandoned',
	
	// E-commerce Events
	PRODUCT_VIEWED: 'product_viewed',
	PRODUCT_ADDED_TO_CART: 'product_added_to_cart',
	PRODUCT_REMOVED_FROM_CART: 'product_removed_from_cart',
	CHECKOUT_STARTED: 'checkout_started',
	CHECKOUT_COMPLETED: 'checkout_completed',
	CHECKOUT_ABANDONED: 'checkout_abandoned',
	
	// Trading Events
	TRADING_ROOM_JOINED: 'trading_room_joined',
	TRADING_ROOM_LEFT: 'trading_room_left',
	TRADE_ALERT_RECEIVED: 'trade_alert_received',
	TRADE_ALERT_CLICKED: 'trade_alert_clicked',
	
	// Content Events
	VIDEO_STARTED: 'video_started',
	VIDEO_COMPLETED: 'video_completed',
	VIDEO_PAUSED: 'video_paused',
	COURSE_STARTED: 'course_started',
	COURSE_COMPLETED: 'course_completed',
	LESSON_COMPLETED: 'lesson_completed',
	
	// Engagement Events
	SEARCH_PERFORMED: 'search_performed',
	FILTER_APPLIED: 'filter_applied',
	SORT_APPLIED: 'sort_applied',
	SHARE_CLICKED: 'share_clicked',
	
	// Error Events
	ERROR_OCCURRED: 'error_occurred',
	API_ERROR: 'api_error',
} as const;

export type EventName = typeof Events[keyof typeof Events];

// ═══════════════════════════════════════════════════════════════════════════
// Metrics Service
// ═══════════════════════════════════════════════════════════════════════════

class MetricsService {
	private static instance: MetricsService;
	private userId: string | null = null;
	private sessionId: string;
	private queue: Array<{ name: string; properties: EventProperties; timestamp: number }> = [];
	private flushInterval: number | null = null;
	private isInitialized = false;

	private config = {
		endpoint: import.meta.env.VITE_ANALYTICS_ENDPOINT || '/api/analytics/batch',
		flushIntervalMs: 5000,
		maxQueueSize: 50,
		debug: import.meta.env.DEV,
	};

	private constructor() {
		this.sessionId = this.generateSessionId();
		if (browser) {
			this.initialize();
		}
	}

	static getInstance(): MetricsService {
		if (!MetricsService.instance) {
			MetricsService.instance = new MetricsService();
		}
		return MetricsService.instance;
	}

	private initialize(): void {
		if (this.isInitialized) return;
		this.isInitialized = true;

		// Start periodic flush
		this.flushInterval = window.setInterval(() => {
			this.flush();
		}, this.config.flushIntervalMs);

		// Flush on page unload
		window.addEventListener('beforeunload', () => {
			this.flush(true);
		});

		// Track page visibility changes
		document.addEventListener('visibilitychange', () => {
			if (document.visibilityState === 'hidden') {
				this.flush(true);
			}
		});
	}

	private generateSessionId(): string {
		return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
	}

	/**
	 * Set the current user for analytics
	 */
	identify(userId: string, properties?: UserProperties): void {
		this.userId = userId;
		
		if (this.config.debug) {
			console.log('[Metrics] Identify:', { userId, properties });
		}

		// Send identify event
		this.track('user_identified', {
			...properties,
			user_id: userId,
		});
	}

	/**
	 * Clear user identity (on logout)
	 */
	reset(): void {
		this.userId = null;
		this.sessionId = this.generateSessionId();
		
		if (this.config.debug) {
			console.log('[Metrics] Reset');
		}
	}

	/**
	 * Track a page view
	 */
	trackPageView(properties?: PageViewProperties): void {
		const pageProps: EventProperties = {
			path: properties?.path || (browser ? window.location.pathname : ''),
			title: properties?.title || (browser ? document.title : ''),
			referrer: properties?.referrer || (browser ? document.referrer : ''),
			search: properties?.search || (browser ? window.location.search : ''),
		};

		this.track(Events.PAGE_VIEW, pageProps);
	}

	/**
	 * Track a custom event
	 */
	track(name: string, properties?: EventProperties): void {
		if (!browser) return;

		const event = {
			name,
			properties: {
				...properties,
				session_id: this.sessionId,
				user_id: this.userId,
				timestamp: Date.now(),
				url: window.location.href,
				user_agent: navigator.userAgent,
				screen_width: window.innerWidth,
				screen_height: window.innerHeight,
			},
			timestamp: Date.now(),
		};

		if (this.config.debug) {
			console.log('[Metrics] Track:', event);
		}

		// Add to queue
		this.queue.push(event);

		// Flush if queue is full
		if (this.queue.length >= this.config.maxQueueSize) {
			this.flush();
		}

		// Also send to Google Analytics if available
		this.sendToGA(name, properties);
	}

	/**
	 * Track a timed event (measure duration)
	 */
	startTimer(eventName: string): () => void {
		const startTime = Date.now();
		
		return () => {
			const duration = Date.now() - startTime;
			this.track(eventName, { duration_ms: duration });
		};
	}

	/**
	 * Flush events to the server
	 */
	private async flush(sync = false): Promise<void> {
		if (this.queue.length === 0) return;

		const events = [...this.queue];
		this.queue = [];

		try {
			if (sync && navigator.sendBeacon) {
				// Use sendBeacon for reliable delivery on page unload
				navigator.sendBeacon(
					this.config.endpoint,
					JSON.stringify({ events })
				);
			} else {
				// Use fetch for normal delivery
				await fetch(this.config.endpoint, {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ events }),
					keepalive: true,
				});
			}
		} catch (error) {
			// Re-queue events on failure
			this.queue = [...events, ...this.queue].slice(0, this.config.maxQueueSize);
			
			if (this.config.debug) {
				console.error('[Metrics] Flush failed:', error);
			}
		}
	}

	/**
	 * Send event to Google Analytics
	 */
	private sendToGA(name: string, properties?: EventProperties): void {
		if (typeof window !== 'undefined' && 'gtag' in window) {
			(window as any).gtag('event', name, properties);
		}
	}

	/**
	 * Cleanup
	 */
	destroy(): void {
		if (this.flushInterval) {
			clearInterval(this.flushInterval);
		}
		this.flush(true);
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Exports
// ═══════════════════════════════════════════════════════════════════════════

export const metrics = MetricsService.getInstance();

// Convenience functions
export const track = (name: string, properties?: EventProperties) => 
	metrics.track(name, properties);

export const trackPageView = (properties?: PageViewProperties) => 
	metrics.trackPageView(properties);

export const identify = (userId: string, properties?: UserProperties) => 
	metrics.identify(userId, properties);

export const startTimer = (eventName: string) => 
	metrics.startTimer(eventName);

export default metrics;
