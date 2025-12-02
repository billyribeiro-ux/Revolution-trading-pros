/**
 * Observability - Enterprise Analytics & Telemetry
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Unified observability infrastructure providing:
 * - Multi-provider analytics (GA4, Backend, Console)
 * - OpenTelemetry-based telemetry
 * - A/B testing & experiments
 * - Performance monitoring
 *
 * Quick Start:
 * ```typescript
 * import { initializeAnalytics, track, trackPageView } from '$lib/observability';
 *
 * // Initialize on client mount
 * await initializeAnalytics({ consent: { analytics: true, marketing: false } });
 *
 * // Track events
 * trackPageView();
 * track('button_clicked', { button_id: 'cta' });
 * ```
 *
 * @version 3.0.0
 * @author Revolution Trading Pros
 */

// ═══════════════════════════════════════════════════════════════════════════
// Metrics (Primary Analytics API)
// ═══════════════════════════════════════════════════════════════════════════

export {
	metrics,
	track,
	trackPageView,
	identify,
	startTimer,
	trackPurchase,
	trackSignUp,
	trackLogin,
	trackLogout,
	trackSearch,
	trackShare,
	trackError,
	trackFormSubmit,
	trackFormError,
	Events,
	type EventProperties,
	type PageViewProperties,
	type UserProperties,
	type EventName,
	type PageViewPayload,
	type CustomEventPayload,
	type PurchasePayload,
	type EcommerceItem,
} from './metrics';

// ═══════════════════════════════════════════════════════════════════════════
// Orchestrator (Advanced Usage)
// ═══════════════════════════════════════════════════════════════════════════

export {
	getOrchestrator,
	createOrchestrator,
	resetOrchestrator,
	orchestratorState,
	analyticsConsent,
	registeredAdapters,
	type OrchestratorState,
	type OrchestratorMetrics,
} from './orchestrator';

// ═══════════════════════════════════════════════════════════════════════════
// Adapters (For Custom Implementations)
// ═══════════════════════════════════════════════════════════════════════════

export {
	// Types
	type AnalyticsAdapter,
	type AnalyticsConfig,
	type AdapterState,
	type AdapterFactory,
	type AdapterMetrics,
	type AnalyticsEventName,
	type BaseEventPayload,
	type IdentifyPayload,
	type ErrorPayload,
	type EventPayload,
	type QueuedEvent,
	type GoogleAnalyticsConfig,
	type BackendAnalyticsConfig,
	type ConsoleAdapterConfig,
	// Event constants
	AnalyticsEvents,
	// Google Analytics
	getGoogleAnalyticsAdapter,
	createGoogleAnalyticsAdapter,
	resetGoogleAnalyticsAdapter,
	// Console
	getConsoleAdapter,
	createConsoleAdapter,
	// Backend
	getBackendAdapter,
	createBackendAdapter,
} from './adapters';

// ═══════════════════════════════════════════════════════════════════════════
// Experiments (A/B Testing)
// ═══════════════════════════════════════════════════════════════════════════

export {
	useExperiment,
	useFeatureFlag,
	trackConversion,
	overrideExperiment,
	clearOverrides,
	experimentAssignments,
	EXPERIMENTS,
	FEATURE_FLAGS,
	type Experiment,
	type ExperimentAssignment,
	type FeatureFlag,
} from './experiments';

// ═══════════════════════════════════════════════════════════════════════════
// Telemetry (OpenTelemetry)
// ═══════════════════════════════════════════════════════════════════════════

export * from './telemetry';

// ═══════════════════════════════════════════════════════════════════════════
// Initialization Helper
// ═══════════════════════════════════════════════════════════════════════════

import { browser } from '$app/environment';
import { metrics } from './metrics';
import { getOrchestrator } from './orchestrator';
import type { AnalyticsConfig } from './adapters';

/**
 * Initialize the complete analytics system.
 *
 * Call this once in your root layout's onMount:
 * ```svelte
 * <script>
 *   import { onMount } from 'svelte';
 *   import { initializeAnalytics } from '$lib/observability';
 *
 *   onMount(async () => {
 *     await initializeAnalytics({
 *       consent: { analytics: true, marketing: false }
 *     });
 *   });
 * </script>
 * ```
 *
 * @param config - Optional configuration overrides
 */
export async function initializeAnalytics(
	config?: Partial<AnalyticsConfig>
): Promise<void> {
	if (!browser) return;

	await metrics.initialize(config);

	console.debug('[Observability] Analytics initialized');
}

/**
 * Update analytics consent state.
 *
 * Call this when user consent changes:
 * ```typescript
 * import { updateAnalyticsConsent } from '$lib/observability';
 *
 * // When consent is granted
 * updateAnalyticsConsent({ analytics: true, marketing: true });
 *
 * // When consent is revoked
 * updateAnalyticsConsent({ analytics: false, marketing: false });
 * ```
 */
export function updateAnalyticsConsent(
	consent: { analytics: boolean; marketing: boolean }
): void {
	if (!browser) return;

	getOrchestrator().updateConsent(consent);

	console.debug('[Observability] Consent updated:', consent);
}

/**
 * Get analytics metrics for monitoring/debugging.
 */
export function getAnalyticsMetrics() {
	return getOrchestrator().metrics;
}

/**
 * Flush all pending analytics events.
 * Useful before page unload or navigation.
 */
export async function flushAnalytics(): Promise<void> {
	if (!browser) return;
	await getOrchestrator().flush();
}

/**
 * Destroy and cleanup analytics.
 */
export function destroyAnalytics(): void {
	if (!browser) return;
	getOrchestrator().destroy();
}
