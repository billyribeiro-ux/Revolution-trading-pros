/**
 * Console Analytics Adapter - Development & Debug Mode
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Beautiful, colorized console output for analytics events during development.
 * Features:
 * - Colorized output with event type indicators
 * - Grouped logs for related events
 * - Performance timing visualization
 * - Pretty-printed payloads
 * - Event filtering by type
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { browser } from '$app/environment';
import type {
import { logger } from '$lib/utils/logger';
	AnalyticsAdapter,
	AnalyticsConfig,
	AdapterState,
	PageViewPayload,
	CustomEventPayload,
	PurchasePayload,
	IdentifyPayload
} from './types';

// ═══════════════════════════════════════════════════════════════════════════
// Console Styling
// ═══════════════════════════════════════════════════════════════════════════

const STYLES = {
	// Base styles
	badge: 'padding: 2px 6px; border-radius: 3px; font-weight: bold; font-size: 11px;',
	label: 'color: #888; font-size: 11px;',
	value: 'color: #fff; font-size: 11px;',

	// Event type badges
	pageView: 'background: #3b82f6; color: white;',
	event: 'background: #8b5cf6; color: white;',
	purchase: 'background: #10b981; color: white;',
	identify: 'background: #f59e0b; color: white;',
	error: 'background: #ef4444; color: white;',

	// Additional styles
	header: 'font-size: 12px; font-weight: bold; color: #60a5fa;',
	timestamp: 'color: #6b7280; font-size: 10px;',
	payload: 'color: #a3a3a3; font-size: 11px;'
} as const;

// Event type emojis for visual identification
const EVENT_ICONS: Record<string, string> = {
	page_view: '📄',
	scroll: '📜',
	login: '🔐',
	sign_up: '🎉',
	logout: '👋',
	add_to_cart: '🛒',
	remove_from_cart: '🗑️',
	purchase: '💰',
	begin_checkout: '💳',
	form_submit: '📝',
	form_start: '✏️',
	form_error: '❌',
	video_start: '▶️',
	video_complete: '⏹️',
	search: '🔍',
	share: '📤',
	error: '🚨',
	exception: '💥',
	trading_room_join: '📈',
	trading_room_leave: '📉',
	trade_alert_view: '🔔',
	course_start: '📚',
	course_complete: '🎓',
	experiment_exposure: '🧪',
	experiment_conversion: '✅'
};

// ═══════════════════════════════════════════════════════════════════════════
// Console Adapter Class
// ═══════════════════════════════════════════════════════════════════════════

class ConsoleAnalyticsAdapter implements AnalyticsAdapter {
	readonly id = 'console';
	readonly name = 'Console Logger';

	private _state: AdapterState = 'uninitialized';
	private _config: AnalyticsConfig | null = null;
	private _enabled = true;
	private _prettyPrint = true;
	private _groupLogs = true;
	private _eventCount = 0;
	private _sessionStart = Date.now();

	get state(): AdapterState {
		return this._state;
	}

	/**
	 * Initialize the console adapter.
	 */
	async initialize(config: AnalyticsConfig): Promise<void> {
		if (!browser) {
			this._state = 'disabled';
			return;
		}

		this._config = config;
		this._enabled = config.console?.enabled ?? config.environment === 'development';
		this._prettyPrint = config.console?.prettyPrint ?? true;
		this._groupLogs = config.console?.groupLogs ?? true;

		if (this._enabled) {
			this._state = 'ready';
			this._printBanner();
		} else {
			this._state = 'disabled';
		}
	}

	/**
	 * Print initialization banner.
	 */
	private _printBanner(): void {
		logger.info(
			`%c ═══════════════════════════════════════════════════════════════
 ║  🚀 Analytics Console Adapter - Active                      ║
 ║  Environment: ${this._config?.environment?.padEnd(15) ?? 'unknown'}                        ║
 ║  GA4 ID: ${this._config?.googleAnalytics?.measurementId?.padEnd(20) ?? 'Not configured'}               ║
 ═══════════════════════════════════════════════════════════════`,
			'color: #60a5fa; font-family: monospace;'
		);
	}

	/**
	 * Track a page view.
	 */
	trackPageView(payload: PageViewPayload): void {
		if (!this._enabled) return;

		this._eventCount++;

		const icon = EVENT_ICONS['page_view'] || '📄';
		const title = `${icon} PAGE VIEW`;

		if (this._groupLogs) {
			console.groupCollapsed(
				`%c${title}%c ${payload.page_path || window.location.pathname}`,
				`${STYLES.badge} ${STYLES.pageView}`,
				STYLES.value
			);
		}

		this._logPayload({
			'Page Path': payload.page_path || window.location.pathname,
			'Page Title': payload.page_title || document.title,
			'Page Type': payload.page_type || 'unknown',
			Referrer: payload.page_referrer || document.referrer || '(direct)',
			Timestamp: new Date().toISOString()
		});

		if (this._groupLogs) {
			console.groupEnd();
		}
	}

	/**
	 * Track a custom event.
	 */
	trackEvent(eventName: string, payload?: CustomEventPayload): void {
		if (!this._enabled) return;

		this._eventCount++;

		const icon = EVENT_ICONS[eventName] || '📊';
		const title = `${icon} ${eventName.toUpperCase().replace(/_/g, ' ')}`;

		if (this._groupLogs) {
			console.groupCollapsed(`%c${title}`, `${STYLES.badge} ${STYLES.event}`);
		}

		const displayPayload: Record<string, unknown> = {
			'Event Name': eventName,
			Timestamp: new Date().toISOString()
		};

		if (payload) {
			if (payload.event_category) displayPayload['Category'] = payload.event_category;
			if (payload.event_label) displayPayload['Label'] = payload.event_label;
			if (payload.value !== undefined) displayPayload['Value'] = payload.value;

			// Add custom properties
			for (const [key, value] of Object.entries(payload)) {
				if (
					![
						'event_category',
						'event_label',
						'value',
						'timestamp',
						'session_id',
						'user_id'
					].includes(key)
				) {
					displayPayload[this._formatKey(key)] = value;
				}
			}
		}

		this._logPayload(displayPayload);

		if (this._groupLogs) {
			console.groupEnd();
		}
	}

	/**
	 * Track a purchase event.
	 */
	trackPurchase(payload: PurchasePayload): void {
		if (!this._enabled) return;

		this._eventCount++;

		const icon = EVENT_ICONS['purchase'] || '💰';
		const title = `${icon} PURCHASE`;

		if (this._groupLogs) {
			console.groupCollapsed(
				`%c${title}%c $${payload.value.toFixed(2)} ${payload.currency}`,
				`${STYLES.badge} ${STYLES.purchase}`,
				'color: #10b981; font-weight: bold;'
			);
		}

		this._logPayload({
			'Transaction ID': payload.transaction_id,
			Value: `$${payload.value.toFixed(2)} ${payload.currency}`,
			Tax: payload.tax !== undefined ? `$${payload.tax.toFixed(2)}` : 'N/A',
			Shipping: payload.shipping !== undefined ? `$${payload.shipping.toFixed(2)}` : 'N/A',
			Coupon: payload.coupon || 'None',
			Items: payload.items?.length || 0,
			Timestamp: new Date().toISOString()
		});

		// Log items if present
		if (payload.items && payload.items.length > 0) {
			console.table(payload.items);
		}

		if (this._groupLogs) {
			console.groupEnd();
		}
	}

	/**
	 * Identify a user.
	 */
	identify(payload: IdentifyPayload): void {
		if (!this._enabled) return;

		this._eventCount++;

		const icon = '👤';
		const title = `${icon} USER IDENTIFIED`;

		if (this._groupLogs) {
			console.groupCollapsed(
				`%c${title}%c ${payload.user_id}`,
				`${STYLES.badge} ${STYLES.identify}`,
				STYLES.value
			);
		}

		const displayPayload: Record<string, unknown> = {
			'User ID': payload.user_id,
			Timestamp: new Date().toISOString()
		};

		for (const [key, value] of Object.entries(payload)) {
			if (key !== 'user_id' && value !== undefined) {
				displayPayload[this._formatKey(key)] = value;
			}
		}

		this._logPayload(displayPayload);

		if (this._groupLogs) {
			console.groupEnd();
		}
	}

	/**
	 * Set user properties.
	 */
	setUserProperties(properties: Record<string, unknown>): void {
		if (!this._enabled) return;

		logger.info('%c👤 USER PROPERTIES%c', `${STYLES.badge} ${STYLES.identify}`, STYLES.value);
		this._logPayload(properties);
	}

	/**
	 * Reset user identity.
	 */
	reset(): void {
		if (!this._enabled) return;

		logger.info(
			'%c🔄 USER RESET%c Identity cleared',
			`${STYLES.badge} ${STYLES.identify}`,
			STYLES.value
		);
	}

	/**
	 * Handle consent changes.
	 */
	onConsentChange(consent: { analytics: boolean; marketing: boolean }): void {
		if (!this._enabled) return;

		logger.info('%c🔒 CONSENT UPDATED', `${STYLES.badge} background: #6366f1; color: white;`);
		this._logPayload({
			Analytics: consent.analytics ? '✅ Granted' : '❌ Denied',
			Marketing: consent.marketing ? '✅ Granted' : '❌ Denied'
		});
	}

	/**
	 * Flush events (no-op for console).
	 */
	async flush(): Promise<void> {
		if (!this._enabled) return;

		const sessionDuration = ((Date.now() - this._sessionStart) / 1000).toFixed(1);
		logger.info(
			`%c📊 SESSION STATS%c ${this._eventCount} events in ${sessionDuration}s`,
			`${STYLES.badge} background: #374151; color: white;`,
			STYLES.value
		);
	}

	/**
	 * Cleanup.
	 */
	destroy(): void {
		this._state = 'disabled';
	}

	// ─────────────────────────────────────────────────────────────────────────
	// Private Helpers
	// ─────────────────────────────────────────────────────────────────────────

	/**
	 * Log payload with pretty formatting.
	 */
	private _logPayload(payload: Record<string, unknown>): void {
		if (this._prettyPrint) {
			for (const [key, value] of Object.entries(payload)) {
				logger.info(`%c  ${key}:%c ${this._formatValue(value)}`, STYLES.label, STYLES.value);
			}
		} else {
			logger.info(payload);
		}
	}

	/**
	 * Format a key for display.
	 */
	private _formatKey(key: string): string {
		return key
			.split('_')
			.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
			.join(' ');
	}

	/**
	 * Format a value for display.
	 */
	private _formatValue(value: unknown): string {
		if (value === null || value === undefined) {
			return '(none)';
		}
		if (typeof value === 'object') {
			return JSON.stringify(value, null, 2);
		}
		return String(value);
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Factory & Singleton
// ═══════════════════════════════════════════════════════════════════════════

let instance: ConsoleAnalyticsAdapter | null = null;

/**
 * Get the console adapter singleton.
 */
export function getConsoleAdapter(): ConsoleAnalyticsAdapter {
	if (!instance) {
		instance = new ConsoleAnalyticsAdapter();
	}
	return instance;
}

/**
 * Create a new console adapter instance.
 */
export function createConsoleAdapter(): ConsoleAnalyticsAdapter {
	return new ConsoleAnalyticsAdapter();
}

export default getConsoleAdapter;
