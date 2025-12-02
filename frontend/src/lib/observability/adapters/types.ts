/**
 * Analytics Adapter Types - Enterprise-Grade Type Safety
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Google L8+ / Netflix L11+ / Apple L8+ Level Architecture
 *
 * Provides strongly-typed event definitions, adapter interfaces,
 * and configuration types for the analytics infrastructure.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// Core Event Types - Strongly Typed Event Names
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Standard analytics event names aligned with GA4 recommended events.
 * @see https://developers.google.com/analytics/devguides/collection/ga4/reference/events
 */
export const AnalyticsEvents = {
	// ─────────────────────────────────────────────────────────────────────────
	// Page & Navigation Events
	// ─────────────────────────────────────────────────────────────────────────
	PAGE_VIEW: 'page_view',
	PAGE_LEAVE: 'page_leave',
	SCROLL: 'scroll',
	SCREEN_VIEW: 'screen_view',

	// ─────────────────────────────────────────────────────────────────────────
	// User Events
	// ─────────────────────────────────────────────────────────────────────────
	LOGIN: 'login',
	SIGN_UP: 'sign_up',
	LOGOUT: 'logout',
	USER_ENGAGEMENT: 'user_engagement',

	// ─────────────────────────────────────────────────────────────────────────
	// Content & Engagement Events
	// ─────────────────────────────────────────────────────────────────────────
	VIEW_ITEM: 'view_item',
	VIEW_ITEM_LIST: 'view_item_list',
	SELECT_ITEM: 'select_item',
	SELECT_CONTENT: 'select_content',
	SHARE: 'share',
	SEARCH: 'search',

	// ─────────────────────────────────────────────────────────────────────────
	// E-commerce Events (GA4 Standard)
	// ─────────────────────────────────────────────────────────────────────────
	ADD_TO_CART: 'add_to_cart',
	REMOVE_FROM_CART: 'remove_from_cart',
	VIEW_CART: 'view_cart',
	BEGIN_CHECKOUT: 'begin_checkout',
	ADD_PAYMENT_INFO: 'add_payment_info',
	ADD_SHIPPING_INFO: 'add_shipping_info',
	PURCHASE: 'purchase',
	REFUND: 'refund',

	// ─────────────────────────────────────────────────────────────────────────
	// Form Events
	// ─────────────────────────────────────────────────────────────────────────
	FORM_START: 'form_start',
	FORM_SUBMIT: 'form_submit',
	FORM_ERROR: 'form_error',
	FORM_ABANDON: 'form_abandon',

	// ─────────────────────────────────────────────────────────────────────────
	// Video Events
	// ─────────────────────────────────────────────────────────────────────────
	VIDEO_START: 'video_start',
	VIDEO_PROGRESS: 'video_progress',
	VIDEO_COMPLETE: 'video_complete',

	// ─────────────────────────────────────────────────────────────────────────
	// Trading Platform Events (Custom)
	// ─────────────────────────────────────────────────────────────────────────
	TRADING_ROOM_JOIN: 'trading_room_join',
	TRADING_ROOM_LEAVE: 'trading_room_leave',
	TRADE_ALERT_VIEW: 'trade_alert_view',
	TRADE_ALERT_CLICK: 'trade_alert_click',
	COURSE_START: 'course_start',
	COURSE_COMPLETE: 'course_complete',
	LESSON_COMPLETE: 'lesson_complete',

	// ─────────────────────────────────────────────────────────────────────────
	// Experiment Events
	// ─────────────────────────────────────────────────────────────────────────
	EXPERIMENT_EXPOSURE: 'experiment_exposure',
	EXPERIMENT_CONVERSION: 'experiment_conversion',

	// ─────────────────────────────────────────────────────────────────────────
	// Error Events
	// ─────────────────────────────────────────────────────────────────────────
	ERROR: 'error',
	EXCEPTION: 'exception',
} as const;

export type AnalyticsEventName = (typeof AnalyticsEvents)[keyof typeof AnalyticsEvents];

// ═══════════════════════════════════════════════════════════════════════════
// Event Payloads - Strongly Typed Properties
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Base properties included with every event.
 */
export interface BaseEventPayload {
	/** Event timestamp (auto-generated if not provided) */
	timestamp?: number;
	/** Session identifier */
	session_id?: string;
	/** User identifier (if known) */
	user_id?: string | null;
	/** Page URL */
	page_location?: string;
	/** Page path */
	page_path?: string;
	/** Page title */
	page_title?: string;
	/** Referrer URL */
	page_referrer?: string;
	/** Screen resolution */
	screen_resolution?: string;
	/** Viewport size */
	viewport_size?: string;
	/** User agent */
	user_agent?: string;
	/** Language */
	language?: string;
}

/**
 * Page view event payload.
 */
export interface PageViewPayload extends BaseEventPayload {
	/** Page type (e.g., 'home', 'product', 'checkout') */
	page_type?: string;
	/** Content group */
	content_group?: string;
}

/**
 * User identification payload.
 */
export interface IdentifyPayload {
	/** Unique user identifier */
	user_id: string;
	/** User email (hashed for privacy) */
	email?: string;
	/** User name */
	name?: string;
	/** Subscription plan */
	plan?: string;
	/** Account creation date */
	created_at?: string;
	/** User properties */
	[key: string]: string | number | boolean | null | undefined;
}

/**
 * E-commerce item for purchase events.
 */
export interface EcommerceItem {
	/** Product/Item ID */
	item_id: string;
	/** Product/Item name */
	item_name: string;
	/** Price per unit */
	price?: number;
	/** Quantity */
	quantity?: number;
	/** Brand */
	item_brand?: string;
	/** Category */
	item_category?: string;
	/** Variant */
	item_variant?: string;
	/** Coupon applied */
	coupon?: string;
	/** Discount amount */
	discount?: number;
	/** Position in list */
	index?: number;
	/** List name */
	item_list_name?: string;
}

/**
 * Purchase/conversion event payload.
 */
export interface PurchasePayload extends BaseEventPayload {
	/** Transaction ID */
	transaction_id: string;
	/** Total value */
	value: number;
	/** Currency code (ISO 4217) */
	currency: string;
	/** Tax amount */
	tax?: number;
	/** Shipping amount */
	shipping?: number;
	/** Coupon code */
	coupon?: string;
	/** Items purchased */
	items?: EcommerceItem[];
}

/**
 * Custom event payload with flexible properties.
 */
export interface CustomEventPayload extends BaseEventPayload {
	/** Event category */
	event_category?: string;
	/** Event label */
	event_label?: string;
	/** Numeric value */
	value?: number;
	/** Additional custom properties */
	[key: string]: string | number | boolean | null | undefined | EcommerceItem[];
}

/**
 * Error event payload.
 */
export interface ErrorPayload extends BaseEventPayload {
	/** Error message */
	message: string;
	/** Error stack trace */
	stack?: string;
	/** Error type/name */
	error_type?: string;
	/** Is fatal error */
	fatal?: boolean;
	/** Additional context */
	context?: Record<string, unknown>;
}

/**
 * Union of all event payloads.
 */
export type EventPayload =
	| PageViewPayload
	| IdentifyPayload
	| PurchasePayload
	| ErrorPayload
	| CustomEventPayload;

// ═══════════════════════════════════════════════════════════════════════════
// Analytics Configuration
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Configuration for Google Analytics adapter.
 */
export interface GoogleAnalyticsConfig {
	/** GA4 Measurement ID (G-XXXXXXXXXX) */
	measurementId: string;
	/** Enable debug mode */
	debug?: boolean;
	/** Send page views automatically */
	sendPageView?: boolean;
	/** Anonymize IP addresses */
	anonymizeIp?: boolean;
	/** Allow Google signals (for remarketing) */
	allowGoogleSignals?: boolean;
	/** Allow ad personalization */
	allowAdPersonalization?: boolean;
	/** Cookie domain */
	cookieDomain?: string;
	/** Cookie expiry in seconds */
	cookieExpires?: number;
	/** Cookie flags */
	cookieFlags?: string;
	/** Link attribution */
	linkAttribution?: boolean;
	/** Transport type */
	transportType?: 'beacon' | 'xhr' | 'image';
}

/**
 * Configuration for backend analytics adapter.
 */
export interface BackendAnalyticsConfig {
	/** API endpoint for batch events */
	endpoint: string;
	/** Flush interval in milliseconds */
	flushIntervalMs?: number;
	/** Maximum batch size */
	maxBatchSize?: number;
	/** Use sendBeacon for page unload */
	useSendBeacon?: boolean;
	/** Authorization header value */
	authorization?: string;
}

/**
 * Configuration for console adapter (development).
 */
export interface ConsoleAdapterConfig {
	/** Enable console output */
	enabled?: boolean;
	/** Log level */
	logLevel?: 'debug' | 'info' | 'warn';
	/** Pretty print events */
	prettyPrint?: boolean;
	/** Group console logs */
	groupLogs?: boolean;
}

/**
 * Master analytics configuration.
 */
export interface AnalyticsConfig {
	/** Enable analytics globally */
	enabled: boolean;
	/** Environment (development, staging, production) */
	environment: 'development' | 'staging' | 'production';
	/** Google Analytics configuration */
	googleAnalytics?: GoogleAnalyticsConfig;
	/** Backend analytics configuration */
	backend?: BackendAnalyticsConfig;
	/** Console adapter configuration */
	console?: ConsoleAdapterConfig;
	/** Consent status */
	consent: {
		analytics: boolean;
		marketing: boolean;
	};
	/** Debug mode */
	debug?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// Analytics Adapter Interface
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Adapter state for lifecycle management.
 */
export type AdapterState = 'uninitialized' | 'initializing' | 'ready' | 'error' | 'disabled';

/**
 * Analytics adapter interface - implement this to add new providers.
 */
export interface AnalyticsAdapter {
	/** Unique adapter identifier */
	readonly id: string;

	/** Human-readable name */
	readonly name: string;

	/** Current adapter state */
	readonly state: AdapterState;

	/**
	 * Initialize the adapter with configuration.
	 * Should be non-blocking and queue events if needed.
	 */
	initialize(config: AnalyticsConfig): Promise<void>;

	/**
	 * Track a page view event.
	 */
	trackPageView(payload: PageViewPayload): void;

	/**
	 * Track a custom event.
	 */
	trackEvent(eventName: string, payload?: CustomEventPayload): void;

	/**
	 * Track a purchase/conversion event.
	 */
	trackPurchase?(payload: PurchasePayload): void;

	/**
	 * Identify a user.
	 */
	identify?(payload: IdentifyPayload): void;

	/**
	 * Set user properties.
	 */
	setUserProperties?(properties: Record<string, unknown>): void;

	/**
	 * Reset user identity (on logout).
	 */
	reset?(): void;

	/**
	 * Called when consent status changes.
	 */
	onConsentChange?(consent: { analytics: boolean; marketing: boolean }): void;

	/**
	 * Flush any pending events immediately.
	 */
	flush?(): Promise<void>;

	/**
	 * Cleanup and destroy the adapter.
	 */
	destroy?(): void;
}

// ═══════════════════════════════════════════════════════════════════════════
// Adapter Factory Type
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Factory function for creating adapters.
 */
export type AdapterFactory<C = unknown> = (config?: C) => AnalyticsAdapter;

// ═══════════════════════════════════════════════════════════════════════════
// Utility Types
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Event with metadata for queuing.
 */
export interface QueuedEvent {
	eventName: string;
	payload: EventPayload;
	timestamp: number;
	retries: number;
}

/**
 * Adapter metrics for monitoring.
 */
export interface AdapterMetrics {
	eventsTracked: number;
	eventsFailed: number;
	lastEventTime: number | null;
	averageLatencyMs: number;
	queueSize: number;
}
