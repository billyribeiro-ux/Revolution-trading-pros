/**
 * Analytics Adapters - Barrel Export
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Central export point for all analytics adapters.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

// Types
export type {
	AnalyticsAdapter,
	AnalyticsConfig,
	AdapterState,
	AdapterFactory,
	AdapterMetrics,
	// Event Types
	AnalyticsEventName,
	BaseEventPayload,
	PageViewPayload,
	CustomEventPayload,
	PurchasePayload,
	IdentifyPayload,
	ErrorPayload,
	EventPayload,
	EcommerceItem,
	QueuedEvent,
	// Config Types
	GoogleAnalyticsConfig,
	BackendAnalyticsConfig,
	ConsoleAdapterConfig,
} from './types';

export { AnalyticsEvents } from './types';

// Google Analytics Adapter
export {
	getGoogleAnalyticsAdapter,
	createGoogleAnalyticsAdapter,
	resetGoogleAnalyticsAdapter,
} from './google-analytics';

// Console Adapter
export {
	getConsoleAdapter,
	createConsoleAdapter,
} from './console';

// Backend Adapter
export {
	getBackendAdapter,
	createBackendAdapter,
} from './backend';
